# openpfe-mcp — design

MCP server implementation (tools/resources). Exposes a **transport-agnostic** `McpHandler`; **`openpfe-server`** wires it to IPC (`type: mcp`) and to the human HTTP debug route in **`openpfe-ui`**. Stdio bridge lives in **`openpfe`** binary (IPC client only).

## Decisions

| Topic | Decision |
|-------|----------|
| **v1 tool surface** | **Graph + contract read/validate only** — agents use the IDE’s model; openpfe supplies **context shield** data ([openpfe_tooling.md](../../../openpfe_tooling.md)). |
| **LLM MCP tools** | **Deferred** (`openpfe_suggest_subproblems`, `openpfe_draft_contract`, …) — human-driven refinement uses HTTP + `openpfe-llm` first. |
| **Streaming** | **None in v1** — one JSON-RPC request (or batch) in/out per call on every transport ([openpfe-ipc/design.md](../openpfe-ipc/design.md)). |
| **Context shield** | All graph tools enforce [openpfe-graph/specification.md](../openpfe-graph/specification.md#traversal-limits-context-shield) defaults unless caller passes lower limits. |
| **Git / spike automation** | **Out of v1** MCP (branching, C-SDD orchestration) — product feature, not protocol. |
| **Handler** | `McpHandler::handle_jsonrpc(payload) -> payload` using `rmcp` or thin JSON-RPC router; state: `Arc<GraphStore>` only. **`server.json`** is **not** readable or writable via MCP — IPC admin only ([openpfe-ipc/specification.md](../openpfe-ipc/specification.md)). |
| **Transports (v1)** | **IPC** — `type: mcp` envelope `payload` (IDE via `openpfe mcp` stdio bridge). **HTTP** — `POST /api/v1/debug/mcp` in `openpfe-ui` (Web UI Debug panel). Same handler instance; tool/resource semantics identical. |

## Scope

- MCP tools/resources → `openpfe-graph` only in v1 (no `server.json`, no `llm.json`)
- **Not** HTTP route definitions (those live in **`openpfe-ui`**); **not** static assets; **not** direct `openpfe-llm` in v1

## Related

- [specification.md](./specification.md) — tool/resource tables
- [openpfe-ipc/specification.md](../openpfe-ipc/specification.md) — `type: mcp` envelope
- [openpfe/design.md](../openpfe/design.md) — stdio bridge
- [guidelines/mcp.md](../../guidelines/mcp.md)
