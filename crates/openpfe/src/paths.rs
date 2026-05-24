use std::path::{Path, PathBuf};

use crate::error::ClientError;

/// Project-relative paths under `./.openpfe/server/` from process cwd.
#[derive(Debug, Clone)]
pub struct ProjectPaths {
    pub openpfe_dir: PathBuf,
    pub server_dir: PathBuf,
    pub pid_file: PathBuf,
    pub socket_path: PathBuf,
}

impl ProjectPaths {
    /// Resolve paths from the current working directory (project root).
    pub fn from_cwd() -> Result<Self, ClientError> {
        let cwd = std::env::current_dir()?;
        Self::from_root(&cwd)
    }

    pub fn from_root(root: &Path) -> Result<Self, ClientError> {
        let openpfe_dir = root.join(".openpfe");
        if !openpfe_dir.is_dir() {
            return Err(ClientError::ProjectNotInitialized { path: openpfe_dir });
        }
        let server_dir = openpfe_dir.join("server");
        Ok(Self {
            pid_file: server_dir.join("pid"),
            socket_path: server_dir.join("socket"),
            server_dir,
            openpfe_dir,
        })
    }
}
