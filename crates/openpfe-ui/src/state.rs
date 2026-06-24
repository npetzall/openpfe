use std::sync::{Arc, Mutex};

use openpfe_graph::GraphStore;
use openpfe_llm::LlmService;
use openpfe_mcp::McpHandler;

/// Shared handler state — constructed by `openpfe-server`, passed to [`crate::api_router`].
#[derive(Clone)]
pub struct AppState {
    pub graph: Arc<Mutex<dyn GraphStore + Send>>,
    pub llm: Arc<dyn LlmService>,
    pub mcp: Arc<McpHandler>,
}
