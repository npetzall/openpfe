//! Detached `openpfe --server` spawn; echo returns loopback HTTP base URL.

mod support;

use std::time::Duration;

use openpfe::adapters::IpcProjectControl;
use openpfe::client::ensure_server;
use openpfe::spawn::ReExecSpawn;
use support::{TestProject, stop_server};

#[tokio::test]
async fn echo_after_spawn_returns_http_base_url() {
    let project = TestProject::new();
    project.chdir();

    let control = IpcProjectControl::from_paths(&project.paths);
    let spawner = ReExecSpawn::for_openpfe_binary().expect("openpfe binary");

    let info = ensure_server(
        &project.paths,
        &control,
        &spawner,
        Duration::from_secs(10),
        false,
    )
    .await
    .expect("echo after detached spawn");

    assert!(
        info.http_base_url.starts_with("http://127.0.0.1:"),
        "unexpected base url: {}",
        info.http_base_url
    );

    stop_server(&control).await;
}
