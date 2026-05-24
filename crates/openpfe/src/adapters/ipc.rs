//! [`ProjectControl`] via [`openpfe_ipc::IpcClient`].

use std::path::PathBuf;

use async_trait::async_trait;
use openpfe_ipc::IpcClient;
use serde_json::Value;

use crate::error::ClientError;
use crate::paths::ProjectPaths;
use crate::ports::{EchoInfo, ProjectControl};

/// IPC client adapter bound to a project socket path.
#[derive(Debug, Clone)]
pub struct IpcProjectControl {
    socket_path: PathBuf,
}

impl IpcProjectControl {
    pub fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    pub fn from_paths(paths: &ProjectPaths) -> Self {
        Self::new(paths.socket_path.clone())
    }
}

#[async_trait]
impl ProjectControl for IpcProjectControl {
    async fn echo(&self) -> Result<EchoInfo, ClientError> {
        let mut client = IpcClient::connect(&self.socket_path)
            .await
            .map_err(map_ipc_echo)?;
        let resp = client.echo().await.map_err(map_ipc_echo)?;
        Ok(EchoInfo {
            http_base_url: resp.http_base_url,
        })
    }

    async fn shutdown(&self) -> Result<(), ClientError> {
        let mut client = IpcClient::connect(&self.socket_path)
            .await
            .map_err(map_ipc_shutdown)?;
        client.shutdown().await.map_err(map_ipc_shutdown)
    }

    async fn send_mcp(&self, payload: Value) -> Result<Value, ClientError> {
        let mut client = IpcClient::connect(&self.socket_path)
            .await
            .map_err(map_ipc_mcp)?;
        client.send_mcp(payload).await.map_err(map_ipc_mcp)
    }
}

fn map_ipc_echo(err: openpfe_ipc::IpcError) -> ClientError {
    ClientError::Echo(format_ipc(err))
}

fn map_ipc_shutdown(err: openpfe_ipc::IpcError) -> ClientError {
    ClientError::Shutdown(format_ipc(err))
}

fn map_ipc_mcp(err: openpfe_ipc::IpcError) -> ClientError {
    ClientError::Mcp(format_ipc(err))
}

fn format_ipc(err: openpfe_ipc::IpcError) -> String {
    err.to_string()
}
