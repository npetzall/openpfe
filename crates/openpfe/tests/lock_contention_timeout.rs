//! Stale socket with no server: echo times out (lock-contention path).

mod support;

use std::time::Duration;

use openpfe::adapters::IpcProjectControl;
use openpfe::client::ensure_server;
use openpfe::diagnostics::{LockContentionReport, format_lock_contention};
use openpfe::error::ClientError;
use support::{CountingSpawn, TestProject, hold_pid_lock};

#[tokio::test]
async fn echo_timeout_when_socket_present_without_server() {
    let project = TestProject::new();
    project.chdir();

    std::fs::File::create(&project.paths.socket_path).expect("touch socket");
    let _lock = hold_pid_lock(&project.paths);

    let control = IpcProjectControl::from_paths(&project.paths);
    let spawner = CountingSpawn::new().expect("spawn");

    let err = ensure_server(
        &project.paths,
        &control,
        &spawner,
        Duration::from_millis(300),
        false,
    )
    .await
    .expect_err("echo should time out");

    let ClientError::EchoTimeout { elapsed } = err else {
        panic!("expected EchoTimeout, got {err}");
    };

    let report = LockContentionReport::gather(&project.paths, elapsed);
    let text = format_lock_contention(&report);
    assert!(text.contains("did not respond on IPC"));
    assert!(text.contains("socket:"));
    assert!(text.contains("hint:"));
}
