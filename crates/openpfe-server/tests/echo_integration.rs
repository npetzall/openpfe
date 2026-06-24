//! Integration: temp project root, background server, echo + LLM HTTP API.

use std::path::Path;
use std::time::Duration;

use openpfe_ipc::IpcClient;
use openpfe_server::{ServerOptions, run_server_with_opts};
use tokio::time::{sleep, timeout};

#[tokio::test]
async fn echo_returns_http_base_url_matching_stub() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::env::set_current_dir(dir.path()).expect("chdir");

    let opts = ServerOptions {
        foreground: true,
        shutdown_timeout: Duration::from_secs(1),
    };

    let server = tokio::spawn(async move { run_server_with_opts(opts).await });

    let echo = timeout(Duration::from_secs(5), wait_for_echo())
        .await
        .expect("echo within timeout")
        .expect("echo ok");

    assert!(
        echo.http_base_url.starts_with("http://127.0.0.1:"),
        "unexpected base url: {}",
        echo.http_base_url
    );

    let body = reqwest::get(format!("{}/", echo.http_base_url))
        .await
        .expect("GET /")
        .text()
        .await
        .expect("body");
    assert_eq!(body, "OK");

    let mut client = IpcClient::connect_default()
        .await
        .expect("connect for shutdown");
    client.shutdown().await.expect("shutdown");

    let _ = timeout(Duration::from_secs(5), server)
        .await
        .expect("server exits after shutdown");
}

#[tokio::test]
async fn llm_status_returns_json() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::env::set_current_dir(dir.path()).expect("chdir");

    let opts = ServerOptions {
        foreground: true,
        shutdown_timeout: Duration::from_secs(1),
    };

    let server = tokio::spawn(async move { run_server_with_opts(opts).await });

    let echo = timeout(Duration::from_secs(5), wait_for_echo())
        .await
        .expect("echo within timeout")
        .expect("echo ok");

    let graph_store = Path::new("./.openpfe/graph/store/");
    assert!(
        graph_store.is_dir(),
        "graph store dir should exist under project root"
    );

    let response = reqwest::get(format!("{}/api/v1/llm/status", echo.http_base_url))
        .await
        .expect("GET /api/v1/llm/status");
    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("json body");
    assert!(body.get("loaded").is_some());
    assert!(body.get("busy").is_some());

    let mut client = IpcClient::connect_default()
        .await
        .expect("connect for shutdown");
    client.shutdown().await.expect("shutdown");

    let _ = timeout(Duration::from_secs(5), server)
        .await
        .expect("server exits after shutdown");
}

async fn wait_for_echo() -> Result<openpfe_ipc::EchoResponse, openpfe_ipc::IpcError> {
    loop {
        if let Ok(mut client) = IpcClient::connect_default().await {
            if let Ok(echo) = client.echo().await {
                return Ok(echo);
            }
        }
        sleep(Duration::from_millis(50)).await;
    }
}
