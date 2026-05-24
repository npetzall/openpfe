//! MCP JSON-RPC forwarded over real IPC (phase 1 stub response).

mod support;

use openpfe::adapters::IpcProjectControl;
use openpfe::ports::ProjectControl;
use serde_json::json;
use support::{TestProject, start_background_server, stop_server};

#[tokio::test]
async fn mcp_bridge_roundtrip() {
    let project = TestProject::new();
    project.chdir();

    let _server = start_background_server(true).await;
    let control = IpcProjectControl::from_paths(&project.paths);

    support::wait_for_echo_ready(&project.paths, 5).await;

    let req = json!({"jsonrpc":"2.0","id":42,"method":"tools/list"});
    let resp = control.send_mcp(req).await.expect("mcp roundtrip");

    assert_eq!(resp["jsonrpc"], "2.0");
    assert_eq!(resp["id"], 42);
    assert_eq!(resp["error"]["code"], -32601);

    stop_server(&control).await;
}
