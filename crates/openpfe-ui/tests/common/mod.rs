use std::sync::{Arc, Mutex};

use openpfe_graph::{GrafeoGraphStore, GraphStore};
use openpfe_mcp::McpHandler;
use openpfe_ui::AppState;
use tempfile::TempDir;

use crate::MockLlmService;

pub fn test_state(llm: MockLlmService) -> (AppState, TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let graph: Arc<Mutex<dyn GraphStore + Send>> = Arc::new(Mutex::new(
        GrafeoGraphStore::open(dir.path()).expect("open graph"),
    ));
    let mcp = Arc::new(McpHandler::new(Arc::clone(&graph)));
    let state = AppState {
        graph,
        llm: Arc::new(llm),
        mcp,
    };
    (state, dir)
}
