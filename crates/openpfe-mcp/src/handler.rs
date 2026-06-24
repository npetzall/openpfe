//! Transport-agnostic MCP JSON-RPC handler (v1 stub — graph tools deferred).

use std::sync::{Arc, Mutex};

use openpfe_graph::GraphStore;
use serde_json::{Value, json};

/// Shared MCP handler — wired to IPC `type: mcp` and HTTP `POST /debug/mcp`.
///
/// **Stub (plan 008):** `initialize` and `tools/list` return minimal success; all other
/// methods return JSON-RPC **`-32601`** until the full tool surface lands.
pub struct McpHandler {
    _graph: Arc<Mutex<dyn GraphStore + Send>>,
}

impl McpHandler {
    pub fn new(graph: Arc<Mutex<dyn GraphStore + Send>>) -> Self {
        Self { _graph: graph }
    }

    /// Single JSON-RPC object or batch array in/out — same contract as IPC `type: mcp` payload.
    pub fn handle_jsonrpc(&self, payload: Value) -> Value {
        if payload.is_array() {
            let responses: Vec<Value> = payload
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|item| self.handle_one(item))
                .collect();
            return Value::Array(responses);
        }
        self.handle_one(&payload).unwrap_or_else(|| {
            error_response(Value::Null, -32600, "Invalid Request")
        })
    }
}

impl McpHandler {
    fn handle_one(&self, request: &Value) -> Option<Value> {
        let id = request.get("id").cloned();
        let Some(id) = id else {
            // JSON-RPC notification — process silently, no response object.
            if is_valid_request(request) {
                let _ = self.dispatch_method(request.get("method").and_then(|m| m.as_str()));
            }
            return None;
        };

        if !is_valid_request(request) {
            return Some(error_response(id, -32600, "Invalid Request"));
        }

        let method = request
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or_default();

        Some(match self.dispatch_method(Some(method)) {
            Dispatch::Result(result) => success_response(id, result),
            Dispatch::MethodNotFound => {
                error_response(id, -32601, &format!("Method not found: {method}"))
            }
        })
    }

    fn dispatch_method(&self, method: Option<&str>) -> Dispatch {
        match method {
            Some("initialize") => Dispatch::Result(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "openpfe",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            Some("tools/list") => Dispatch::Result(json!({ "tools": [] })),
            Some(_) => Dispatch::MethodNotFound,
            None => Dispatch::MethodNotFound,
        }
    }
}

enum Dispatch {
    Result(Value),
    MethodNotFound,
}

fn is_valid_request(request: &Value) -> bool {
    let Some(obj) = request.as_object() else {
        return false;
    };
    matches!(obj.get("jsonrpc"), Some(v) if v == "2.0")
        && obj.get("method").is_some_and(|m| m.is_string())
}

fn success_response(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    })
}

fn error_response(id: Value, code: i32, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use openpfe_graph::GrafeoGraphStore;
    use tempfile::tempdir;

    fn test_handler() -> McpHandler {
        let dir = tempdir().expect("tempdir");
        let store = GrafeoGraphStore::open(dir.path()).expect("open graph");
        McpHandler::new(Arc::new(Mutex::new(store)))
    }

    #[test]
    fn tools_list_returns_jsonrpc_success() {
        let handler = test_handler();
        let req = json!({"jsonrpc":"2.0","id":1,"method":"tools/list"});
        let resp = handler.handle_jsonrpc(req);
        assert_eq!(resp["jsonrpc"], "2.0");
        assert_eq!(resp["id"], 1);
        assert!(resp.get("result").is_some());
        assert_eq!(resp["result"]["tools"], json!([]));
    }

    #[test]
    fn initialize_returns_jsonrpc_success() {
        let handler = test_handler();
        let req = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0" }
            }
        });
        let resp = handler.handle_jsonrpc(req);
        assert_eq!(resp["jsonrpc"], "2.0");
        assert_eq!(resp["id"], 0);
        assert!(resp["result"]["serverInfo"]["name"].is_string());
    }

    #[test]
    fn unknown_method_returns_32601() {
        let handler = test_handler();
        let req = json!({"jsonrpc":"2.0","id":7,"method":"tools/call"});
        let resp = handler.handle_jsonrpc(req);
        assert_eq!(resp["error"]["code"], -32601);
    }

    #[test]
    fn batch_returns_two_responses() {
        let handler = test_handler();
        let batch = json!([
            {"jsonrpc":"2.0","id":1,"method":"tools/list"},
            {"jsonrpc":"2.0","id":2,"method":"tools/call","params":{}}
        ]);
        let resp = handler.handle_jsonrpc(batch);
        let arr = resp.as_array().expect("batch response");
        assert_eq!(arr.len(), 2);
        assert!(arr[0].get("result").is_some());
        assert_eq!(arr[1]["error"]["code"], -32601);
    }

    #[test]
    fn invalid_request_returns_32600() {
        let handler = test_handler();
        let req = json!({"id":1,"method":"tools/list"});
        let resp = handler.handle_jsonrpc(req);
        assert_eq!(resp["error"]["code"], -32600);
    }

    #[test]
    fn non_object_top_level_returns_32600() {
        let handler = test_handler();
        let resp = handler.handle_jsonrpc(json!("not-json-rpc"));
        assert_eq!(resp["error"]["code"], -32600);
    }
}
