# openpfe-mcp — specification

## Transport

`McpHandler` is **transport-agnostic**. **`openpfe-server`** constructs one shared instance and attaches:

| Transport | Client | Wire |
|-----------|--------|------|
| **IPC** | IDE agent (`openpfe mcp` stdio bridge) | `type: mcp` envelope `payload` — [openpfe-ipc/specification.md](../openpfe-ipc/specification.md) |
| **HTTP** | Web UI Debug panel (browser) | `POST /api/v1/debug/mcp` — [openpfe-ui/specification.md](../openpfe-ui/specification.md#mcp-debug-json-rpc) |

- Host attachment (agents): `openpfe mcp` stdio per [Model Context Protocol](https://modelcontextprotocol.io/)
- **v1:** request/response JSON-RPC only (no streaming notifications) on both transports
- **Out of scope for MCP:** **`server.json`**, **`llm.json`**, and IPC admin envelopes — graph tools only; humans use REST graph/LLM routes or this MCP debug HTTP bridge per [guidelines/protocols.md](../../guidelines/protocols.md)

## MCP tools (v1)

Stable names — breaking renames require a version note in this file.

| Tool | Input (summary) | Output (summary) |
|------|-----------------|------------------|
| `openpfe_graph_subgraph` | `cluster_id` (UUID), optional `max_depth`, `max_nodes` | JSON subgraph (nodes + edges), bounded |
| `openpfe_graph_get_node` | `node_id` (UUID) | Node properties + `type` |
| `openpfe_graph_list_nodes` | optional `type`, `cluster_id`, `limit` (default 50, max 200) | Array of node summaries |
| `openpfe_graph_neighbors` | `node_id`, optional `edge_type`, `direction` | Adjacent nodes/edges |
| `openpfe_contract_get` | `edge_id` or (`from`, `to`, `type=interfaces`) | Contract body + metadata |
| `openpfe_graph_validate` | — | `{ "ok": true }` or `{ "cycles": [ … ] }` |

### Context shield (normative)

- `openpfe_graph_subgraph` **must** apply defaults `max_depth=3`, `max_nodes=200` when omitted; reject requests above hard caps (`max_depth` ≤ 5, `max_nodes` ≤ 500).
- `openpfe_graph_list_nodes` default `limit=50`, max `200`.
- Tools **must not** return full graph dumps.

### Deferred tools (non-normative v1)

| Tool | Notes |
|------|--------|
| `openpfe_suggest_subproblems` | LLM-assisted drill-down |
| `openpfe_draft_contract` | LLM-assisted contract draft |
| `openpfe_component_spike_plan` | C-SDD / git branch orchestration |

## MCP resources (v1)

| URI template | MIME | Content |
|--------------|------|---------|
| `graph://node/{node_id}` | `application/json` | Single node document |
| `graph://cluster/{cluster_id}/subgraph` | `application/json` | Bounded subgraph (shield defaults) |

Resources are read-only in v1.

## Errors

- JSON-RPC errors per MCP spec; `message` user-safe (no Rust backtraces on stdio).
- Unknown tool → `-32601` method not found equivalent.
- Shield limit exceeded → application error code `context_limit_exceeded`.

## Related

- [openpfe-graph/specification.md](../openpfe-graph/specification.md) — schema
- [guidelines/mcp.md](../../guidelines/mcp.md)
