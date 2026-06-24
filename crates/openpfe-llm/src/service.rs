use std::path::PathBuf;
use std::sync::{Mutex, RwLock};

use crate::config::{load_config, write_config};
use crate::download::DownloadManager;
use crate::engine::InferenceEngine;
use crate::error::{LlmError, Result};
use crate::registry::{is_installed, list_models, resolve_gguf_path};
use crate::types::{
    CompleteOptions, DownloadStatus, JobId, LlmFile, LlmService, LlmStatus,
};

/// Default [`LlmService`] — `llama-cpp-2` engine, HTTPS downloads, project `llm.json`.
///
/// HTTP/async callers should invoke [`LlmService::complete`] and [`LlmService::reload_engine`]
/// from `spawn_blocking` (see FR inference NFR).
pub struct LlamaLlmService {
    project_root: PathBuf,
    home: PathBuf,
    config: RwLock<LlmFile>,
    engine: InferenceEngine,
    downloads: DownloadManager,
    status: Mutex<LlmStatus>,
    /// Integration-test hook: artificial delay while holding the inference busy flag.
    test_complete_delay: Option<std::time::Duration>,
}

impl LlamaLlmService {
    /// Resolve paths from process cwd and `HOME`; load `llm.json`; `try_load` when installed.
    pub fn new() -> Result<Self> {
        let project_root = std::env::current_dir().map_err(LlmError::Io)?;
        let home = user_home()?;
        Self::new_with_paths(project_root, home)
    }

    /// Test hook: override project root and shared models home.
    pub fn new_with_paths(project_root: PathBuf, home: PathBuf) -> Result<Self> {
        Self::build(project_root, home, None)
    }

    /// Test hook: override paths and optional `complete` delay (holds busy flag).
    pub fn new_for_test(
        project_root: PathBuf,
        home: PathBuf,
        complete_delay: Option<std::time::Duration>,
    ) -> Result<Self> {
        Self::build(project_root, home, complete_delay)
    }

    fn build(
        project_root: PathBuf,
        home: PathBuf,
        test_complete_delay: Option<std::time::Duration>,
    ) -> Result<Self> {
        let file = load_config(&project_root)?;
        let engine = InferenceEngine::new()?;
        let service = Self {
            project_root,
            home,
            config: RwLock::new(file),
            engine,
            downloads: DownloadManager::new(),
            status: Mutex::new(LlmStatus::default()),
            test_complete_delay,
        };
        service.try_load_active()?;
        Ok(service)
    }

    fn try_load_active(&self) -> Result<()> {
        let file = self.config.read().expect("config read lock");
        let Some(model_id) = file.llm.model.clone() else {
            self.set_status_loaded(false, None, None);
            return Ok(());
        };
        if model_id.is_empty() {
            self.set_status_loaded(false, None, None);
            return Ok(());
        }
        let entry = match file.catalog.iter().find(|e| e.id == model_id) {
            Some(e) => e.clone(),
            None => {
                self.set_status_loaded(
                    false,
                    Some(model_id.clone()),
                    Some(format!("active model {model_id} not in catalog")),
                );
                return Ok(());
            }
        };
        drop(file);

        match resolve_gguf_path(&self.home, &entry) {
            Ok(path) => {
                let settings = self.config.read().expect("config read lock").llm.clone();
                if let Err(e) = self.engine.load(
                    &path,
                    settings.n_ctx,
                    settings.n_threads,
                ) {
                    self.set_status_loaded(
                        false,
                        Some(model_id.to_string()),
                        Some(e.to_string()),
                    );
                } else {
                    self.set_status_loaded(true, Some(model_id.to_string()), None);
                }
            }
            Err(e) => {
                self.set_status_loaded(false, Some(model_id.to_string()), Some(e.to_string()));
            }
        }
        Ok(())
    }

    fn set_status_loaded(&self, loaded: bool, model_id: Option<String>, error: Option<String>) {
        let mut status = self.status.lock().expect("status lock");
        status.loaded = loaded;
        status.model_id = model_id;
        status.error = error;
        status.busy = self.engine.is_busy();
    }

    fn catalog_entry(&self, id: &str) -> Result<crate::types::CatalogEntry> {
        let file = self.config.read().expect("config read lock");
        file.catalog
            .iter()
            .find(|e| e.id == id)
            .cloned()
            .ok_or_else(|| LlmError::not_found(format!("catalog id {id}")))
    }
}

impl LlmService for LlamaLlmService {
    fn load_config(&self) -> Result<LlmFile> {
        load_config(&self.project_root)
    }

    fn write_config(&self, file: &LlmFile) -> Result<()> {
        write_config(&self.project_root, file)?;
        let mut guard = self.config.write().expect("config write lock");
        *guard = file.clone();
        Ok(())
    }

    fn list_models(&self) -> Result<Vec<crate::types::ModelEntry>> {
        let file = self.config.read().expect("config read lock");
        list_models(&self.home, &file)
    }

    fn start_download(&self, id: &str) -> Result<JobId> {
        let entry = self.catalog_entry(id)?;
        // When the active model finishes downloading, the HTTP layer should call
        // `reload_engine` after polling `DownloadStatus::Complete`.
        self.downloads.start(&self.home, &entry)
    }

    fn download_status(&self, job_id: &JobId) -> Result<DownloadStatus> {
        self.downloads.status(job_id)
    }

    fn set_active_model(&self, id: &str) -> Result<()> {
        self.catalog_entry(id)?;
        let mut file = self.config.write().expect("config write lock");
        file.llm.model = Some(id.to_string());
        write_config(&self.project_root, &file)?;
        drop(file);
        if is_installed(&self.home, &self.catalog_entry(id)?) {
            self.reload_engine()?;
        } else {
            self.engine.unload();
            self.set_status_loaded(false, Some(id.to_string()), None);
        }
        Ok(())
    }

    fn reload_engine(&self) -> Result<()> {
        self.engine.unload();
        self.try_load_active()
    }

    fn status(&self) -> LlmStatus {
        let mut status = self.status.lock().expect("status lock").clone();
        status.busy = self.engine.is_busy();
        status
    }

    fn complete(&self, prompt: &str, opts: CompleteOptions) -> Result<String> {
        if !self.engine.try_acquire_busy() {
            return Err(LlmError::Busy);
        }

        if let Some(delay) = self.test_complete_delay {
            std::thread::sleep(delay);
        }

        let settings = self.config.read().expect("config read lock").llm.clone();
        let temperature = opts.temperature.unwrap_or(settings.temperature);
        let max_tokens = opts.max_tokens.unwrap_or(settings.max_tokens);
        let result = self.engine.complete(prompt, temperature, max_tokens);

        self.engine.release_busy();
        {
            let mut status = self.status.lock().expect("status lock");
            status.busy = false;
        }
        result
    }
}

fn user_home() -> Result<PathBuf> {
    if let Some(home) = std::env::var_os("HOME") {
        return Ok(PathBuf::from(home));
    }
    if let Some(home) = std::env::var_os("USERPROFILE") {
        return Ok(PathBuf::from(home));
    }
    Err(LlmError::config("HOME not set"))
}
