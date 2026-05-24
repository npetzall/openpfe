//! Second default command does not spawn again when echo already succeeds.

mod support;

use openpfe::adapters::IpcProjectControl;
use support::{
    CountingSpawn, TestProject, run_default_no_browser, start_background_server, stop_server,
};

#[tokio::test]
async fn second_default_no_double_server() {
    let project = TestProject::new();
    project.chdir();

    let _server = start_background_server(true).await;
    support::wait_for_echo_ready(&project.paths, 5).await;

    let control = IpcProjectControl::from_paths(&project.paths);
    let spawn = CountingSpawn::new().expect("spawn");

    run_default_no_browser(&project.paths, &control, &spawn)
        .await
        .expect("first default");
    assert_eq!(spawn.spawns(), 0, "server already up — no spawn");

    run_default_no_browser(&project.paths, &control, &spawn)
        .await
        .expect("second default");
    assert_eq!(spawn.spawns(), 0, "still no spawn");

    stop_server(&control).await;
}
