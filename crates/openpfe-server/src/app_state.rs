//! Construct shared [`openpfe_ui::AppState`] at startup.

use std::sync::{Arc, Mutex};

use openpfe_graph::{GrafeoGraphStore, GraphStore};
use openpfe_llm::LlamaLlmService;
use openpfe_mcp::McpHandler;
use openpfe_ui::AppState;

use crate::error::ServerError;

/// Grafeo store directory relative to process cwd (project root).
pub const GRAPH_STORE_PATH: &str = "./.openpfe/graph/store/";

/// Open graph, LLM service, and MCP handler; build UI [`AppState`].
pub fn build_app_state() -> Result<AppState, ServerError> {
    std::fs::create_dir_all(GRAPH_STORE_PATH)?;

    let graph: Arc<Mutex<dyn GraphStore + Send>> = Arc::new(Mutex::new(
        GrafeoGraphStore::open(GRAPH_STORE_PATH)?,
    ));
    let llm = Arc::new(LlamaLlmService::new()?);
    let mcp = Arc::new(McpHandler::new(Arc::clone(&graph)));

    Ok(AppState { graph, llm, mcp })
}

/// Drop the graph handle after HTTP/IPC shutdown.
///
/// [`GraphStore::close`] consumes `self` and cannot be invoked through
/// `Mutex<dyn GraphStore>`; releasing the last `Arc` drops the store (Grafeo persists via its DB handle).
pub fn close_graph_best_effort(graph: Arc<Mutex<dyn GraphStore + Send>>) {
    drop(graph);
}
