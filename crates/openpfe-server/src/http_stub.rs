//! Minimal loopback HTTP stub (`GET /` → OK).

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::error::ServerError;

/// Bound `127.0.0.1:0` stub; [`Self::base_url`] is returned in IPC echo.
pub struct HttpStub {
    base_url: Arc<String>,
    listener: TcpListener,
}

impl HttpStub {
    pub async fn bind() -> Result<Self, ServerError> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let base_url = Arc::new(format!("http://{addr}"));
        Ok(Self { base_url, listener })
    }

    pub fn base_url(&self) -> Arc<String> {
        Arc::clone(&self.base_url)
    }

    pub async fn serve(
        self,
        mut shutdown: tokio::sync::broadcast::Receiver<()>,
    ) -> Result<(), ServerError> {
        let router = Router::new().route("/", get(|| async { "OK" }));
        let shutdown_signal = async move {
            let _ = shutdown.recv().await;
        };
        axum::serve(self.listener, router)
            .with_graceful_shutdown(shutdown_signal)
            .await?;
        Ok(())
    }
}
