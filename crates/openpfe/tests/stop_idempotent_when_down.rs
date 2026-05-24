//! `openpfe stop` exits successfully when the server is not running.

mod support;

use openpfe::adapters::IpcProjectControl;
use openpfe::client::run_stop;
use support::TestProject;

#[tokio::test]
async fn stop_idempotent_when_down() {
    let project = TestProject::new();
    project.chdir();

    let control = IpcProjectControl::from_paths(&project.paths);
    run_stop(&control).await.expect("stop when down is ok");
}
