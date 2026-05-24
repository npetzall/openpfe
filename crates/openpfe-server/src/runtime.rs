//! Runtime directory paths and stale-socket handling.

use std::path::Path;

use openpfe_ipc::{DEFAULT_SOCKET_PATH, IpcClient};

use crate::error::ServerError;

/// `./.openpfe/server/` relative to process cwd.
pub const RUNTIME_DIR: &str = "./.openpfe/server";
pub const PID_FILE: &str = "./.openpfe/server/pid";
pub const SOCKET_PATH: &str = DEFAULT_SOCKET_PATH;
pub const LOG_FILE: &str = "./.openpfe/server/openpfe.log";

/// Ensure the runtime directory exists.
pub fn ensure_runtime_dir() -> Result<(), ServerError> {
    std::fs::create_dir_all(RUNTIME_DIR)?;
    Ok(())
}

/// If `socket` exists, probe with connect+echo; unlink when dead.
pub async fn remove_stale_socket_if_dead() -> Result<(), ServerError> {
    if !Path::new(SOCKET_PATH).exists() {
        return Ok(());
    }

    match IpcClient::connect(SOCKET_PATH).await {
        Ok(mut client) => match client.echo().await {
            Ok(_) => Err(ServerError::SocketInUse),
            Err(_) => remove_socket_file(),
        },
        Err(_) => remove_socket_file(),
    }
}

fn remove_socket_file() -> Result<(), ServerError> {
    if Path::new(SOCKET_PATH).exists() {
        std::fs::remove_file(SOCKET_PATH)?;
    }
    Ok(())
}

/// Remove runtime artifacts after shutdown (best effort).
pub fn cleanup_runtime_files() {
    let _ = std::fs::remove_file(SOCKET_PATH);
    let _ = std::fs::remove_file(PID_FILE);
}
