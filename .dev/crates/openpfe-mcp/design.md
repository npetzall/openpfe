# openpfe-mcp — design

MCP server implementation (tools/resources) for IDE agents. Invoked from **`openpfe-server`** over IPC envelopes — not stdio. Stdio bridge lives in **`openpfe`** binary.

## Decisions

| Topic | Decision |
|-------|----------|
| **v1 tool surface** | **Graph + contract read/validate only** — agents use the IDE’s model; openpfe supplies **context shield** data ([openpfe_tooling.md](../../../openpfe_tooling.md)). |
| **LLM MCP tools** | **Deferred** (`openpfe_suggest_subproblems`, `openpfe_draft_contract`, …) — human-driven refinement uses HTTP + `openpfe-llm` first. |
| **Streaming** | **None in v1** — one IPC `type: mcp` envelope in/out per JSON-RPC call ([openpfe-ipc/design.md](../openpfe-ipc/design.md)). |
| **Context shield** | All graph tools enforce [openpfe-graph/specification.md](../openpfe-graph/specification.md#traversal-limits-context-shield) defaults unless caller passes lower limits. |
| **Git / spike automation** | **Out of v1** MCP (branching, C-SDD orchestration) — product feature, not protocol. |
| **Handler** | `McpHandler::handle_jsonrpc(payload) -> payload` using `rmcp` or thin JSON-RPC router; state: `Arc<GraphStore>`, server settings snapshot from **`server.json`** (loaded at startup by `openpfe-server`). |

## Scope

- MCP tools/resources → `openpfe-graph` (and server settings from startup state only in v1)
- **Not** HTTP; **not** static assets; **not** direct `openpfe-llm` in v1

## Related

- [specification.md](./specification.md) — tool/resource tables
- [openpfe-ipc/specification.md](../openpfe-ipc/specification.md) — `type: mcp` envelope
- [openpfe/design.md](../openpfe/design.md) — stdio bridge
- [guidelines/mcp.md](../../guidelines/mcp.md)
