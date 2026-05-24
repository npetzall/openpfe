//! Crate-local ports consumed by CLI handlers.
//!
//! Normative consumer contract for [`openpfe-ipc`](../../openpfe-ipc) (plan 003).
//! Real adapters are wired in plan 005; unit tests use [`crate::adapters::mock`].

use async_trait::async_trait;
use serde_json::Value;

use crate::error::ClientError;

/// Successful echo response fields used by the client binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoInfo {
    pub http_base_url: String,
}

/// IPC client operations against the project server socket.
#[async_trait]
pub trait ProjectControl: Send + Sync {
    async fn echo(&self) -> Result<EchoInfo, ClientError>;
    async fn shutdown(&self) -> Result<(), ClientError>;
    async fn send_mcp(&self, payload: Value) -> Result<Value, ClientError>;
}

/// Detached `openpfe --server` spawn (re-exec + session detach on Unix).
pub trait ServerSpawn: Send + Sync {
    fn spawn_detached_server(&self) -> Result<(), ClientError>;
}
