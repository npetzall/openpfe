//! Server process: pid flock, UDS IPC, loopback HTTP API.

mod app_state;
mod error;
mod http;
mod ipc_dispatch;
mod lock;
mod options;
mod runtime;

pub use error::ServerError;
pub use options::ServerOptions;

use std::io::Write;
use std::time::Duration;

use openpfe_ipc::IpcListener;
use tokio::sync::broadcast;
use tokio::time::sleep;

use crate::app_state::{build_app_state, close_graph_best_effort};
use crate::http::HttpServer;
use crate::ipc_dispatch::IpcDispatch;
use crate::lock::acquire_pid_lock;
use crate::options::ServerOptions as Opts;
use crate::runtime::{LOG_FILE, SOCKET_PATH, cleanup_runtime_files, remove_stale_socket_if_dead};

/// Run the server with default options (detached logging stub, 5s shutdown drain).
pub async fn run_server() -> Result<(), ServerError> {
    run_server_with_opts(Opts::default()).await
}

/// Run the server until IPC shutdown, `openpfe stop`, or SIGINT/SIGTERM.
pub async fn run_server_with_opts(opts: ServerOptions) -> Result<(), ServerError> {
    init_logging(opts.foreground)?;

    let _pid_lock = acquire_pid_lock()?;
    remove_stale_socket_if_dead().await?;

    let app_state = build_app_state()?;
    let http = HttpServer::bind(app_state).await?;
    let http_base_url = http.base_url();
    let graph = http.graph_handle();

    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let handler = IpcDispatch::new(http_base_url, shutdown_tx.clone());

    let listener = IpcListener::bind(SOCKET_PATH).await?;
    let ipc_task = tokio::spawn(async move {
        let _ = listener.serve(handler).await;
    });

    let http_shutdown = shutdown_tx.subscribe();
    let http_task = tokio::spawn(async move { http.serve(http_shutdown).await });

    wait_for_shutdown(shutdown_tx.subscribe()).await;
    drain_and_stop(opts.shutdown_timeout, ipc_task, http_task).await;

    close_graph_best_effort(graph);
    cleanup_runtime_files();
    Ok(())
}

async fn wait_for_shutdown(mut shutdown_rx: broadcast::Receiver<()>) {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigterm = signal(SignalKind::terminate()).expect("register SIGTERM handler");
        let mut sigint = signal(SignalKind::interrupt()).expect("register SIGINT handler");

        tokio::select! {
            _ = shutdown_rx.recv() => {}
            _ = sigterm.recv() => {}
            _ = sigint.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        let ctrl_c = tokio::signal::ctrl_c();
        tokio::select! {
            _ = shutdown_rx.recv() => {}
            _ = ctrl_c => {}
        }
    }
}

async fn drain_and_stop(
    timeout: Duration,
    ipc_task: tokio::task::JoinHandle<()>,
    http_task: tokio::task::JoinHandle<Result<(), ServerError>>,
) {
    sleep(timeout).await;
    ipc_task.abort();
    let _ = http_task.await;
}

fn init_logging(foreground: bool) -> Result<(), ServerError> {
    let line = format!(
        "openpfe-server {} starting (pid {})\n",
        env!("CARGO_PKG_VERSION"),
        std::process::id()
    );
    if foreground {
        eprint!("{line}");
        return Ok(());
    }
    runtime::ensure_runtime_dir()?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)?;
    file.write_all(line.as_bytes())?;
    Ok(())
}
