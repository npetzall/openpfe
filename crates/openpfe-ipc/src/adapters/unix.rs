//! Tokio Unix domain socket transport.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use tokio::io::split;
use tokio::net::{UnixListener, UnixStream};

use crate::connection::FramedConnection;
use crate::error::IpcError;
use crate::transport::{IpcListenerTransport, IpcTransport};

/// Unix domain socket transport (v1).
#[derive(Debug, Clone, Copy, Default)]
pub struct UnixTransport;

pub type UnixConnection =
    FramedConnection<tokio::io::ReadHalf<UnixStream>, tokio::io::WriteHalf<UnixStream>>;

impl IpcTransport for UnixTransport {
    type Conn = UnixConnection;

    fn connect(path: &Path) -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send>> {
        let path = path.to_path_buf();
        Box::pin(async move {
            let stream = UnixStream::connect(&path).await?;
            let (reader, writer) = split(stream);
            Ok(FramedConnection::new(reader, writer))
        })
    }

    fn bind(path: &Path) -> Pin<Box<dyn Future<Output = Result<Self::Listener, IpcError>> + Send>> {
        let path = path.to_path_buf();
        Box::pin(async move {
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
            let listener = UnixListener::bind(&path)?;
            Ok(UnixListenerTransport { listener, path })
        })
    }

    type Listener = UnixListenerTransport;
}

/// Bound UDS listener.
pub struct UnixListenerTransport {
    listener: UnixListener,
    path: PathBuf,
}

impl UnixListenerTransport {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl IpcListenerTransport for UnixListenerTransport {
    type Conn = UnixConnection;

    fn accept(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send + '_>> {
        Box::pin(async move {
            let (stream, _) = self.listener.accept().await?;
            let (reader, writer) = split(stream);
            Ok(FramedConnection::new(reader, writer))
        })
    }
}

impl Drop for UnixListenerTransport {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
