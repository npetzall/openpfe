//! Loopback HTTP — `openpfe-ui` API plus minimal root probe.

use std::sync::{Arc, Mutex};

use axum::{Router, routing::get};
use openpfe_graph::GraphStore;
use openpfe_ui::{api_router, AppState};
use tokio::net::TcpListener;

use crate::error::ServerError;

/// Bound `127.0.0.1:0` server; [`Self::base_url`] is returned in IPC echo.
pub struct HttpServer {
    base_url: Arc<String>,
    listener: TcpListener,
    app_state: AppState,
}

impl HttpServer {
    pub async fn bind(app_state: AppState) -> Result<Self, ServerError> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let base_url = Arc::new(format!("http://{addr}"));
        Ok(Self {
            base_url,
            listener,
            app_state,
        })
    }

    pub fn base_url(&self) -> Arc<String> {
        Arc::clone(&self.base_url)
    }

    pub fn graph_handle(&self) -> Arc<Mutex<dyn GraphStore + Send>> {
        Arc::clone(&self.app_state.graph)
    }

    pub async fn serve(
        self,
        mut shutdown: tokio::sync::broadcast::Receiver<()>,
    ) -> Result<(), ServerError> {
        let router = Router::new()
            .merge(api_router(self.app_state))
            .route("/", get(|| async { "OK" }));
        let shutdown_signal = async move {
            let _ = shutdown.recv().await;
        };
        axum::serve(self.listener, router)
            .with_graceful_shutdown(shutdown_signal)
            .await?;
        Ok(())
    }
}
