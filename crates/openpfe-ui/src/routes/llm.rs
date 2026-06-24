use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post, put};
use axum::{Json, Router};
use openpfe_llm::types::{CompleteOptions, LlmFile, settings_need_reload};
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/llm/config", get(get_config).put(put_config))
        .route("/llm/active", put(put_active))
        .route("/llm/status", get(get_status))
        .route("/llm/complete", post(post_complete))
}

async fn get_config(State(state): State<AppState>) -> ApiResult<Json<LlmFile>> {
    let llm = Arc::clone(&state.llm);
    let file = tokio::task::spawn_blocking(move || llm.load_config())
        .await
        .map_err(|_| ApiError::internal("task join failed"))?
        .map_err(ApiError::from)?;
    Ok(Json(file))
}

async fn put_config(
    State(state): State<AppState>,
    Json(body): Json<LlmFile>,
) -> ApiResult<Json<LlmFile>> {
    let llm = Arc::clone(&state.llm);
    let file = tokio::task::spawn_blocking(move || {
        let old = llm.load_config()?;
        llm.write_config(&body)?;
        if settings_need_reload(&old.llm, &body.llm) {
            llm.reload_engine()?;
        }
        Ok::<_, openpfe_llm::LlmError>(body)
    })
    .await
    .map_err(|_| ApiError::internal("task join failed"))?
    .map_err(ApiError::from)?;
    Ok(Json(file))
}

#[derive(Debug, Deserialize, Serialize)]
struct ActiveModelBody {
    model: String,
}

async fn put_active(
    State(state): State<AppState>,
    Json(body): Json<ActiveModelBody>,
) -> ApiResult<Json<ActiveModelBody>> {
    if body.model.is_empty() {
        return Err(ApiError::invalid_request("model must not be empty"));
    }
    let llm = Arc::clone(&state.llm);
    let model = body.model.clone();
    tokio::task::spawn_blocking(move || llm.set_active_model(&model))
        .await
        .map_err(|_| ApiError::internal("task join failed"))?
        .map_err(ApiError::from)?;
    Ok(Json(body))
}

async fn get_status(State(state): State<AppState>) -> Json<openpfe_llm::LlmStatus> {
    Json(state.llm.status())
}

#[derive(Debug, Deserialize)]
struct CompleteRequest {
    prompt: String,
    #[serde(default)]
    temperature: Option<f32>,
    #[serde(default)]
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize)]
struct CompleteResponse {
    text: String,
}

async fn post_complete(
    State(state): State<AppState>,
    Json(body): Json<CompleteRequest>,
) -> ApiResult<Json<CompleteResponse>> {
    if body.prompt.is_empty() {
        return Err(ApiError::invalid_request("prompt is required"));
    }
    let llm = Arc::clone(&state.llm);
    let opts = CompleteOptions {
        temperature: body.temperature,
        max_tokens: body.max_tokens,
    };
    let prompt = body.prompt;
    let text = tokio::task::spawn_blocking(move || llm.complete(&prompt, opts))
        .await
        .map_err(|_| ApiError::internal("task join failed"))?
        .map_err(ApiError::from)?;
    Ok(Json(CompleteResponse { text }))
}
