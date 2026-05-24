//! IPC envelope dispatch (`echo`, `shutdown`, MCP stub).

use std::sync::Arc;

use openpfe_ipc::{Envelope, EnvelopeType, ErrorCode, RequestHandler};
use serde_json::{Value, json};
use tokio::sync::broadcast;

/// Shared handler state for [`IpcListener::serve`](openpfe_ipc::IpcListener::serve).
pub struct IpcDispatch {
    http_base_url: Arc<String>,
    shutdown: broadcast::Sender<()>,
}

impl IpcDispatch {
    pub fn new(http_base_url: Arc<String>, shutdown: broadcast::Sender<()>) -> Self {
        Self {
            http_base_url,
            shutdown,
        }
    }
}

impl RequestHandler for IpcDispatch {
    fn handle(&self, envelope: Envelope) -> Envelope {
        match envelope.kind {
            EnvelopeType::Echo => Envelope {
                v: envelope.v,
                kind: EnvelopeType::Echo,
                id: envelope.id,
                payload: json!({
                    "ok": true,
                    "http_base_url": self.http_base_url.as_str(),
                    "pid": std::process::id(),
                    "version": env!("CARGO_PKG_VERSION"),
                }),
            },
            EnvelopeType::Shutdown => {
                let _ = self.shutdown.send(());
                Envelope {
                    v: envelope.v,
                    kind: EnvelopeType::Shutdown,
                    id: envelope.id,
                    payload: json!({ "ok": true }),
                }
            }
            EnvelopeType::Mcp => Envelope {
                v: envelope.v,
                kind: EnvelopeType::Mcp,
                id: envelope.id,
                payload: mcp_not_implemented_stub(&envelope.payload),
            },
            EnvelopeType::Error => Envelope::error_response(
                envelope.id,
                ErrorCode::UnknownType,
                "server must not receive type error",
            ),
        }
    }
}

/// JSON-RPC error stub until `openpfe-mcp` is wired.
fn mcp_not_implemented_stub(request_payload: &Value) -> Value {
    let id = extract_jsonrpc_id(request_payload);
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": -32601,
            "message": "Method not found (openpfe-mcp not mounted in phase 1)"
        }
    })
}

fn extract_jsonrpc_id(payload: &Value) -> Value {
    if let Some(id) = payload.get("id") {
        return id.clone();
    }
    if let Some(arr) = payload.as_array()
        && let Some(first) = arr.first()
        && let Some(id) = first.get("id")
    {
        return id.clone();
    }
    Value::Null
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_stub_preserves_request_id() {
        let req = json!({"jsonrpc":"2.0","id":7,"method":"tools/list"});
        let resp = mcp_not_implemented_stub(&req);
        assert_eq!(resp["id"], 7);
        assert_eq!(resp["error"]["code"], -32601);
    }
}
