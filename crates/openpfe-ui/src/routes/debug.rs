use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde_json::Value;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/debug/mcp", post(post_mcp))
}

async fn post_mcp(
    State(state): State<AppState>,
    body: axum::body::Bytes,
) -> ApiResult<Json<Value>> {
    let payload: Value = serde_json::from_slice(&body)
        .map_err(|e| ApiError::invalid_request(e.to_string()))?;
    if !payload.is_object() && !payload.is_array() {
        return Err(ApiError::invalid_request(
            "body must be a JSON-RPC object or array",
        ));
    }
    let response = state.mcp.handle_jsonrpc(payload);
    Ok(Json(response))
}
