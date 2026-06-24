# Protocols (IPC and HTTP)

**Read when:** defining or implementing wire formats, client discovery, or transport behavior — **not** MCP tool schemas (see [mcp.md](./mcp.md)).

**Normative refs:** [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md), [cross-cutting.md](../cross-cutting.md).

## Transport matrix (v1)

| Client | Transport | API surface |
|--------|-----------|-------------|
| CLI (`openpfe`, `stop`) | Unix domain socket | IPC envelopes |
| Browser / Web UI | HTTP | `/api/v1/…` — graph + LLM REST; MCP via `POST /debug/mcp` (Debug panel) |
| TUI (future) | HTTP (product data); IPC (admin) | HTTP: graph + `llm.json` (same as browser). IPC: `echo`, `shutdown`, **`server_config_*`** |
| IDE agent | stdio MCP → CLI bridge → IPC `type: mcp` | MCP JSON-RPC in envelope `payload` — **no** `server.json` |

**MCP:** one shared **`McpHandler`** (`openpfe-mcp`) — IPC for agents, HTTP debug route for browser ([openpfe-mcp/specification.md](../crates/openpfe-mcp/specification.md)).

**Do not** add graph or **`llm.json`** over IPC for browser or TUI. **`server.json`** is **IPC admin only** ([openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md)) — not MCP, not human HTTP.

## IPC (project control plane)

### Framing

- **4-byte little-endian length** + UTF-8 JSON body — [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md).

### Envelope

```json
{
  "v": 1,
  "type": "echo | mcp | shutdown | server_config_get | server_config_put",
  "id": "<correlation-id>",
  "payload": { }
}
```

- Bump `"v"` only with a documented migration. **v1:** `"v": 1` on **every frame** — no post-connect handshake ([openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md)).
- Unknown `type` → error response; do not ignore.

### Message types

| `type` | Use | Response expectations |
|--------|-----|------------------------|
| `echo` | Discover server; obtain **`http_base_url`** | `http_base_url` **required** in success payload |
| `shutdown` | Graceful stop (`openpfe stop`) | `ok` then connection close |
| `server_config_get` | Read **`server.json`** (admin) | Full document in `payload` |
| `server_config_put` | Replace **`server.json`** (admin) | `{ "ok": true }`; runtime apply in [openpfe-server/specification.md](../crates/openpfe-server/specification.md) |
| `mcp` | MCP JSON-RPC (or batch) in `payload` | MCP response in `payload` (v1: no IPC streaming) |
| `error` | (server only) protocol/validation failure | `code`, `message` in `payload` |

Clients must obtain HTTP base URL from **echo**, not from disk or env (see [cross-cutting.md#data-placement](../cross-cutting.md#data-placement)).

### LLM init (CLI)

**`openpfe llm init`** uses IPC **`echo`** only to obtain **`http_base_url`**, then HTTP for download — **no LLM envelopes over IPC**. See [openpfe/specification.md](../crates/openpfe/specification.md#llm-init).

## HTTP (human API)

- Base path: **`/api/v1/`** — [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md).
- JSON request/response bodies unless endpoint spec says otherwise.
- Errors: stable machine-readable shape (document in `openpfe-ui/specification.md` when finalized); use appropriate HTTP status codes.
- Body limits: axum `DefaultBodyLimit` (or equivalent) on upload routes — document per route in spec.

## Static assets

- Embedded UI served at `/` (after API mount) from `openpfe-webui` — not `tower-http` `ServeDir` from disk.
- API routes registered before static fallback so `/api/v1` is never shadowed.

## Versioning and compatibility

- IPC: `"v": 1` in envelope; reject or negotiate unsupported versions explicitly.
- HTTP: path prefix `/api/v1` for breaking changes; additive fields allowed within v1.
- Record open questions in crate `design.md`, not only in code comments.

## Security (localhost v1)

- Bind HTTP to loopback unless spec changes.
- UDS socket path under `./.openpfe/server/` — project-scoped.
- No auth layer in v1; still validate JSON shape and reject malformed frames early.

## Related

- [mcp.md](./mcp.md) — MCP payload semantics
- [coding-rust.md](./coding-rust.md) — where handlers live
- [openpfe/specification.md](../crates/openpfe/specification.md) — CLI wait/lock behavior
