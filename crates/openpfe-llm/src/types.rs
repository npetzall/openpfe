use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{LlmError, Result};

/// Opaque download job identifier (`uuid` v4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JobId(pub Uuid);

impl JobId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for JobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for JobId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

/// `llm.json` → `llm` object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LlmSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default = "default_n_ctx")]
    pub n_ctx: u32,
    #[serde(default)]
    pub n_threads: u32,
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

fn default_n_ctx() -> u32 {
    4096
}

fn default_temperature() -> f32 {
    0.7
}

fn default_max_tokens() -> u32 {
    1024
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            model: None,
            n_ctx: default_n_ctx(),
            n_threads: 0,
            temperature: default_temperature(),
            max_tokens: default_max_tokens(),
        }
    }
}

/// `llm.json` → `catalog[]` entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

/// Full on-disk `llm.json` document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LlmFile {
    #[serde(default)]
    pub llm: LlmSettings,
    #[serde(default)]
    pub catalog: Vec<CatalogEntry>,
}

/// On-disk `manifest.json` after install.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelManifest {
    pub id: String,
    pub filename: String,
    pub sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub installed_at: String,
}

/// Catalog row plus install state for `GET /models`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelEntry {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub sha256: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub installed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<ModelManifest>,
}

/// Engine snapshot for `GET /llm/status`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct LlmStatus {
    pub loaded: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    pub busy: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Per-request completion overrides (`POST /llm/complete`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompleteOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Download job poll payload (`GET /models/downloads/:job_id`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum DownloadStatus {
    Queued,
    Running {
        bytes_received: u64,
    },
    Complete,
    Failed {
        message: String,
    },
}

/// Product port — config, registry, download, inference.
pub trait LlmService: Send + Sync {
    fn load_config(&self) -> Result<LlmFile>;
    fn write_config(&self, file: &LlmFile) -> Result<()>;
    fn list_models(&self) -> Result<Vec<ModelEntry>>;
    fn start_download(&self, id: &str) -> Result<JobId>;
    fn download_status(&self, job_id: &JobId) -> Result<DownloadStatus>;
    fn set_active_model(&self, id: &str) -> Result<()>;
    fn reload_engine(&self) -> Result<()>;
    fn status(&self) -> LlmStatus;
    fn complete(&self, prompt: &str, opts: CompleteOptions) -> Result<String>;
}

/// Whether changing `before` → `after` requires `reload_engine`.
pub fn settings_need_reload(before: &LlmSettings, after: &LlmSettings) -> bool {
    before.model != after.model || before.n_ctx != after.n_ctx || before.n_threads != after.n_threads
}

/// Validate a catalog entry has enough fields for its mode.
pub fn validate_catalog_entry(entry: &CatalogEntry) -> Result<()> {
    if entry.id.is_empty() {
        return Err(LlmError::config("catalog entry id must not be empty"));
    }
    if entry.sha256.is_empty() {
        return Err(LlmError::config("catalog entry sha256 is required"));
    }
    if let Some(path) = &entry.path {
        if path.is_empty() {
            return Err(LlmError::config("catalog entry path must not be empty"));
        }
        return Ok(());
    }
    if entry.filename.as_ref().is_none_or(|s| s.is_empty()) {
        return Err(LlmError::config(format!(
            "catalog entry {} requires filename or path",
            entry.id
        )));
    }
    if entry.url.as_ref().is_none_or(|s| s.is_empty()) {
        return Err(LlmError::config(format!(
            "catalog entry {} requires url or path",
            entry.id
        )));
    }
    Ok(())
}
