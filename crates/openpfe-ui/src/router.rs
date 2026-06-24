use axum::Router;
use axum::extract::DefaultBodyLimit;

use crate::routes::{debug, llm, models};
use crate::state::AppState;

/// Human HTTP API under `/api/v1` — LLM, models, and debug MCP routes.
pub fn api_router(state: AppState) -> Router {
    let v1 = Router::new()
        .merge(llm::router())
        .merge(models::router())
        .merge(debug::router())
        .layer(DefaultBodyLimit::max(1024 * 1024));

    Router::new().nest("/api/v1", v1).with_state(state)
}
