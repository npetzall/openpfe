//! Server process errors.

use openpfe_graph::GraphError;
use openpfe_ipc::IpcError;
use openpfe_llm::LlmError;

/// Errors from lock acquisition, listeners, and shutdown.
#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error("another server already holds the pid lock")]
    AlreadyRunning,

    #[error("unexpected live server on socket (echo succeeded)")]
    SocketInUse,

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("ipc error: {0}")]
    Ipc(#[from] IpcError),

    #[error("task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("graph error: {0}")]
    Graph(#[from] GraphError),

    #[error("llm error: {0}")]
    Llm(#[from] LlmError),
}
