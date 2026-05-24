use std::fmt;
use std::path::PathBuf;
use std::time::Duration;

/// Client-side errors for the `openpfe` binary (not IPC wire errors).
#[derive(Debug)]
pub enum ClientError {
    ProjectNotInitialized { path: PathBuf },
    Io(std::io::Error),
    EchoTimeout { elapsed: Duration },
    Spawn(String),
    Echo(String),
    Shutdown(String),
    Mcp(String),
    InvalidPid { contents: String },
    Bridge(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProjectNotInitialized { path } => {
                write!(f, "project not initialized: missing {}", path.display())
            }
            Self::Io(e) => write!(f, "io error: {e}"),
            Self::EchoTimeout { elapsed } => {
                write!(f, "server did not respond on IPC within {elapsed:?}")
            }
            Self::Spawn(msg) => write!(f, "spawn detached server: {msg}"),
            Self::Echo(msg) => write!(f, "echo failed: {msg}"),
            Self::Shutdown(msg) => write!(f, "shutdown: {msg}"),
            Self::Mcp(msg) => write!(f, "mcp request failed: {msg}"),
            Self::InvalidPid { contents } => write!(f, "invalid pid in pid file: {contents}"),
            Self::Bridge(msg) => write!(f, "stdio mcp bridge: {msg}"),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
