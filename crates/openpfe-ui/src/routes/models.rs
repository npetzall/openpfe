use std::sync::Arc;

use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use openpfe_llm::types::{DownloadStatus, JobId, ModelEntry};
use serde::Serialize;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/models", get(list_models))
        .route("/models/{id}", get(get_model))
        .route("/models/{id}/download", post(start_download))
        .route("/models/downloads/{job_id}", get(download_status))
}

#[derive(Debug, Serialize)]
struct ModelsResponse {
    models: Vec<ModelEntry>,
}

async fn list_models(State(state): State<AppState>) -> ApiResult<Json<ModelsResponse>> {
    let llm = Arc::clone(&state.llm);
    let models = tokio::task::spawn_blocking(move || llm.list_models())
        .await
        .map_err(|_| ApiError::internal("task join failed"))?
        .map_err(ApiError::from)?;
    Ok(Json(ModelsResponse { models }))
}

async fn get_model(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<ModelEntry>> {
    let llm = Arc::clone(&state.llm);
    let entry = tokio::task::spawn_blocking(move || {
        let models = llm.list_models()?;
        models
            .into_iter()
            .find(|m| m.id == id)
            .ok_or_else(|| openpfe_llm::LlmError::not_found(format!("model {id}")))
    })
    .await
    .map_err(|_| ApiError::internal("task join failed"))?
    .map_err(ApiError::from)?;
    Ok(Json(entry))
}

#[derive(Debug, Serialize)]
struct StartDownloadResponse {
    job_id: JobId,
}

async fn start_download(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<Json<StartDownloadResponse>> {
    let llm = Arc::clone(&state.llm);
    let job_id = tokio::task::spawn_blocking(move || llm.start_download(&id))
        .await
        .map_err(|_| ApiError::internal("task join failed"))?
        .map_err(ApiError::from)?;
    Ok(Json(StartDownloadResponse { job_id }))
}

/// Poll download progress. When a job reaches `complete` and the active model is installed but
/// not yet loaded, calls [`openpfe_llm::LlmService::reload_engine`] so weights become available
/// without a separate client step.
async fn download_status(
    State(state): State<AppState>,
    Path(job_id): Path<String>,
) -> ApiResult<Json<DownloadStatus>> {
    let job_id = job_id
        .parse::<JobId>()
        .map_err(|e| ApiError::invalid_request(format!("invalid job_id: {e}")))?;
    let llm = Arc::clone(&state.llm);
    let status = tokio::task::spawn_blocking(move || {
        let status = llm.download_status(&job_id)?;
        if matches!(status, DownloadStatus::Complete)
            && !llm.status().loaded
            && let Ok(file) = llm.load_config()
            && let Some(active) = &file.llm.model
        {
            let installed = llm
                .list_models()?
                .into_iter()
                .any(|m| &m.id == active && m.installed);
            if installed {
                let _ = llm.reload_engine();
            }
        }
        Ok::<_, openpfe_llm::LlmError>(status)
    })
    .await
    .map_err(|_| ApiError::internal("task join failed"))?
    .map_err(ApiError::from)?;
    Ok(Json(status))
}
