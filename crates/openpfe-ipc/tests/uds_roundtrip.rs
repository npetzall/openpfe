//! UDS integration: echo round-trip with a minimal handler.

use openpfe_ipc::{
    EchoResponse, Envelope, EnvelopeType, ErrorCode, IpcClient, IpcListener, RequestHandler,
};
use serde_json::json;

struct EchoHandler;

impl RequestHandler for EchoHandler {
    fn handle(&self, envelope: Envelope) -> Envelope {
        match envelope.kind {
            EnvelopeType::Echo => Envelope {
                v: envelope.v,
                kind: EnvelopeType::Echo,
                id: envelope.id,
                payload: json!({
                    "ok": true,
                    "http_base_url": "http://127.0.0.1:18080",
                    "pid": 4242,
                    "version": "test",
                }),
            },
            _ => {
                Envelope::error_response(envelope.id, ErrorCode::UnknownType, "unsupported in test")
            }
        }
    }
}

#[tokio::test]
async fn uds_echo_roundtrip() {
    let dir = tempfile::tempdir().expect("tempdir");
    let socket = dir.path().join("socket");

    let listener = IpcListener::bind(&socket).await.expect("bind");
    tokio::spawn(async move {
        let _ = listener.serve(EchoHandler).await;
    });

    let mut client = IpcClient::connect(&socket).await.expect("connect");
    let echo: EchoResponse = client.echo().await.expect("echo");
    assert_eq!(echo.http_base_url, "http://127.0.0.1:18080");
    assert_eq!(echo.pid, Some(4242));
    assert_eq!(echo.version.as_deref(), Some("test"));
}
