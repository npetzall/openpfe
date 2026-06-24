use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use reqwest::Url;
use crate::error::{LlmError, Result};
use crate::paths::model_dir;
use crate::registry::{verify_file_sha256, write_manifest};
use crate::types::{CatalogEntry, DownloadStatus, JobId};

pub const MAX_DOWNLOAD_BYTES: u64 = 32 * 1024 * 1024 * 1024;

#[derive(Debug)]
struct JobRecord {
    status: DownloadStatus,
}

#[derive(Clone)]
pub struct DownloadManager {
    inner: Arc<DownloadState>,
}

struct DownloadState {
    jobs: Mutex<std::collections::HashMap<JobId, JobRecord>>,
    active_by_model: Mutex<std::collections::HashMap<String, JobId>>,
}

impl DownloadManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DownloadState {
                jobs: Mutex::new(std::collections::HashMap::new()),
                active_by_model: Mutex::new(std::collections::HashMap::new()),
            }),
        }
    }

    pub fn status(&self, job_id: &JobId) -> Result<DownloadStatus> {
        let jobs = self.inner.jobs.lock().expect("download jobs lock");
        jobs.get(job_id)
            .map(|j| j.status.clone())
            .ok_or_else(|| LlmError::not_found(format!("download job {job_id}")))
    }

    /// One active download per catalog `id`; a second `start` while running returns `InvalidRequest`.
    pub fn start(&self, home: &Path, entry: &CatalogEntry) -> Result<JobId> {
        if entry.path.is_some() {
            return Err(LlmError::invalid(format!(
                "model {} uses catalog path; download not required",
                entry.id
            )));
        }
        let url = entry.url.as_deref().ok_or_else(|| {
            LlmError::invalid(format!("model {} has no download url", entry.id))
        })?;
        let filename = entry.filename.as_deref().ok_or_else(|| {
            LlmError::invalid(format!("model {} has no filename", entry.id))
        })?;
        validate_download_url(url)?;

        let job_id = JobId::new_v4();
        {
            let mut active = self
                .inner
                .active_by_model
                .lock()
                .expect("active download lock");
            if let Some(existing) = active.get(&entry.id) {
                let jobs = self.inner.jobs.lock().expect("download jobs lock");
                if let Some(record) = jobs.get(existing)
                    && matches!(
                        record.status,
                        DownloadStatus::Queued | DownloadStatus::Running { .. }
                    )
                {
                    return Err(LlmError::invalid(format!(
                        "download already in progress for {}",
                        entry.id
                    )));
                }
            }
            active.insert(entry.id.clone(), job_id);
        }
        {
            let mut jobs = self.inner.jobs.lock().expect("download jobs lock");
            jobs.insert(job_id, JobRecord {
                status: DownloadStatus::Queued,
            });
        }

        let home = home.to_path_buf();
        let entry = entry.clone();
        let url = url.to_string();
        let filename = filename.to_string();
        let inner = Arc::clone(&self.inner);
        thread::spawn(move || {
            let _result = run_download(&home, &entry, &url, &filename, &inner, job_id);
            let mut active = inner.active_by_model.lock().expect("active download lock");
            if active.get(&entry.id) == Some(&job_id) {
                active.remove(&entry.id);
            }
        });

        Ok(job_id)
    }
}

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new()
    }
}

fn run_download(
    home: &Path,
    entry: &CatalogEntry,
    url: &str,
    filename: &str,
    inner: &DownloadState,
    job_id: JobId,
) -> Result<()> {
    update_job(inner, job_id, DownloadStatus::Running { bytes_received: 0 });

    let dir = model_dir(home, &entry.id);
    fs::create_dir_all(&dir)?;
    let partial = dir.join(format!("{filename}.partial"));
    let final_path = dir.join(filename);

    match stream_download(url, &partial, inner, job_id) {
        Ok(()) => {
            if let Err(e) = verify_file_sha256(&partial, &entry.sha256) {
                let _ = fs::remove_file(&partial);
                fail_job(inner, job_id, e.to_string());
                return Err(e);
            }
            if final_path.exists() {
                fs::remove_file(&final_path)?;
            }
            fs::rename(&partial, &final_path)?;
            let installed_at = rfc3339_now();
            write_manifest(
                home,
                &entry.id,
                filename,
                &entry.sha256,
                Some(url),
                &installed_at,
            )?;
            update_job(inner, job_id, DownloadStatus::Complete);
            Ok(())
        }
        Err(e) => {
            let _ = fs::remove_file(&partial);
            fail_job(inner, job_id, e.to_string());
            Err(e)
        }
    }
}

fn stream_download(
    url: &str,
    partial: &Path,
    inner: &DownloadState,
    job_id: JobId,
) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(8))
        .build()
        .map_err(|e| LlmError::download(format!("http client: {e}")))?;

    let mut response = client
        .get(url)
        .send()
        .map_err(|e| LlmError::download(format!("request failed: {e}")))?;

    if !response.status().is_success() {
        return Err(LlmError::download(format!(
            "download failed: HTTP {}",
            response.status()
        )));
    }

    let mut file = File::create(partial)?;
    let mut total: u64 = 0;
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let n = response
            .read(&mut buffer)
            .map_err(|e| LlmError::download(format!("stream error: {e}")))?;
        if n == 0 {
            break;
        }
        total = total.saturating_add(n as u64);
        if total > MAX_DOWNLOAD_BYTES {
            return Err(LlmError::download(format!(
                "download exceeds max size ({MAX_DOWNLOAD_BYTES} bytes)"
            )));
        }
        file.write_all(&buffer[..n])?;
        update_job(
            inner,
            job_id,
            DownloadStatus::Running {
                bytes_received: total,
            },
        );
    }

    Ok(())
}

fn update_job(inner: &DownloadState, job_id: JobId, status: DownloadStatus) {
    let mut jobs = inner.jobs.lock().expect("download jobs lock");
    if let Some(record) = jobs.get_mut(&job_id) {
        record.status = status;
    }
}

fn fail_job(inner: &DownloadState, job_id: JobId, message: String) {
    update_job(
        inner,
        job_id,
        DownloadStatus::Failed {
            message: message.clone(),
        },
    );
}

pub fn validate_download_url(url: &str) -> Result<Url> {
    let parsed = Url::parse(url).map_err(|e| LlmError::download(format!("invalid url: {e}")))?;
    if parsed.scheme() != "https" {
        // Debug / integration tests use a local HTTP mock server on loopback.
        if cfg!(debug_assertions)
            && parsed.scheme() == "http"
            && matches!(
                parsed.host_str(),
                Some("127.0.0.1") | Some("localhost") | Some("[::1]")
            )
        {
            return Ok(parsed);
        }
        return Err(LlmError::download("only https downloads are allowed"));
    }
    Ok(parsed)
}

pub fn rfc3339_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format_unix_rfc3339(dur.as_secs())
}

fn format_unix_rfc3339(secs: u64) -> String {
    let days = secs / 86_400;
    let time = secs % 86_400;
    let (year, month, day) = civil_from_days(days as i64);
    let hour = time / 3600;
    let minute = (time % 3600) / 60;
    let second = time % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if mp < 10 { y + 1 } else { y };
    (year, m, d)
}
