//! IPC listener and per-connection serve loop.

use std::path::Path;
use std::sync::Arc;

use crate::adapters::unix::{UnixListenerTransport, UnixTransport};
use crate::client::validate_request;
use crate::connection::Connection;
use crate::envelope::{EnvelopeType, ErrorCode};
use crate::error::IpcError;
use crate::handler::RequestHandler;
use crate::transport::{IpcListenerTransport, IpcTransport};

/// Bound UDS listener; call [`Self::serve`] with a [`RequestHandler`].
pub struct IpcListener {
    inner: UnixListenerTransport,
}

impl IpcListener {
    pub async fn bind(socket_path: impl AsRef<Path>) -> Result<Self, IpcError> {
        let inner = UnixTransport::bind(socket_path.as_ref()).await?;
        Ok(Self { inner })
    }

    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Accept connections until an accept error; each connection runs in its own task.
    pub async fn serve<H>(self, handler: H) -> Result<(), IpcError>
    where
        H: RequestHandler + 'static,
    {
        let handler = Arc::new(handler);
        let mut inner = self.inner;
        loop {
            let mut conn = match inner.accept().await {
                Ok(c) => c,
                Err(e) => return Err(e),
            };
            let handler = Arc::clone(&handler);
            tokio::spawn(async move {
                if let Err(e) = serve_connection(&mut conn, handler.as_ref()).await {
                    let _ = e;
                }
            });
        }
    }
}

async fn serve_connection<C, H>(conn: &mut C, handler: &H) -> Result<(), IpcError>
where
    C: Connection,
    H: RequestHandler + ?Sized,
{
    loop {
        let req = match conn.read_envelope().await {
            Ok(r) => r,
            Err(_) => break,
        };
        let resp = match validate_request(&req) {
            Ok(()) => handler.handle(req),
            Err(err_env) => err_env,
        };
        conn.write_envelope(&resp).await?;
        if resp.kind == EnvelopeType::Error {
            let code = resp.error_code();
            if matches!(
                code,
                Some(ErrorCode::UnsupportedVersion | ErrorCode::UnknownType)
            ) {
                break;
            }
        }
    }
    Ok(())
}
