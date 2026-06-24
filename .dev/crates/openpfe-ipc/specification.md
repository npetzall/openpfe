# openpfe-ipc — specification

## Transport path (v1)

| Property | Value |
|----------|--------|
| Socket file | `./.openpfe/server/socket` (relative to project root / process cwd) |
| Platform | Unix domain sockets (macOS, Linux) |

## Framing

| Field | Format |
|-------|--------|
| Length | **4-byte little-endian** unsigned integer — byte length of the JSON body only |
| Body | UTF-8 JSON envelope |
| Max body size | **16 MiB** (16_777_216 bytes). Length `0` or `> max` → protocol error, close connection |

Read loop: read 4 bytes → read `length` bytes → parse JSON → dispatch. Write: serialize envelope → write length → write body.

## Envelope (every frame)

```json
{
  "v": 1,
  "type": "echo | mcp | shutdown | server_config_get | server_config_put | error",
  "id": 1,
  "payload": { }
}
```

| Field | Rules |
|-------|--------|
| `v` | **Required.** Must be `1` in v1. Any other value → respond with `type: error`, `payload.code`: `unsupported_version`. |
| `type` | **Required.** Known types below; unknown → `type: error`, `payload.code`: `unknown_type`. |
| `id` | **Required.** JSON **number** (u64). Client-chosen per connection; server **must** copy into the response envelope for the matching request. |
| `payload` | **Required.** JSON object (may be `{}`). |

**Versioning:** no handshake; every frame carries `"v": 1`. Bump `v` only with a documented migration.

**Correlation:** one in-flight request per connection in v1. Admin handlers (`echo`, `shutdown`, `server_config_get`, `server_config_put`) are synchronous. MCP: one JSON-RPC request or batch per `type: mcp` envelope.

## `type: error` (protocol)

Returned for framing/validation failures and unsupported envelopes (not MCP JSON-RPC errors — those stay inside `type: mcp` `payload`).

```json
{
  "v": 1,
  "type": "error",
  "id": 1,
  "payload": {
    "code": "unsupported_version | unknown_type | invalid_frame | payload_too_large",
    "message": "human-readable detail"
  }
}
```

If the request `id` is unknown (e.g. parse failure), use `"id": 0`.

## `type: echo`

- **Request** `payload`: `{}` or `{ "probe": true }` (equivalent).
- **Response** `payload` (required fields):

```json
{
  "ok": true,
  "http_base_url": "http://127.0.0.1:PORT",
  "pid": 12345,
  "version": "0.1.0"
}
```

| Field | Required |
|-------|----------|
| `ok` | yes (`true`) |
| `http_base_url` | yes — ephemeral HTTP bind from server |
| `pid` | no — server process id |
| `version` | no — `openpfe` binary / workspace version string |

## `type: mcp`

- **Request** `payload`: a single MCP **JSON-RPC 2.0** object, or a **JSON array** of objects (batch), per [Model Context Protocol](https://modelcontextprotocol.io/).
- **Response** `payload`: MCP JSON-RPC response object or batch array for that request.
- **v1:** request/response only — **no** streaming, SSE, or chunked frames over IPC. Large tool results must fit within the frame size cap.
- MCP-level errors use JSON-RPC `error` objects inside `payload`, not `type: error`.

## `type: shutdown`

- **Request** `payload`: `{ "graceful": true }` (`graceful` optional in v1; default true).
- **Response** `payload`: `{ "ok": true }` on the same connection, then server begins shutdown ([openpfe-server/specification.md](../openpfe-server/specification.md)).
- Other connections may be dropped after shutdown starts; in-flight MCP work follows server drain policy ([openpfe-server/specification.md](../openpfe-server/specification.md) — default **5s**).

## Admin envelopes — `server.json` (v1)

**Admin IPC only** — handled in **`openpfe-server`**, not in **`openpfe-ui`** HTTP and **not** inside `type: mcp`. MCP agents **must not** read or write `server.json` ([openpfe-mcp/specification.md](../openpfe-mcp/specification.md)).

Document shape: [openpfe-server/specification.md#serverjson-project-config](../openpfe-server/specification.md#serverjson-project-config).

### `type: server_config_get`

- **Request** `payload`: `{}`.
- **Response** `payload`: full `server.json` document (same JSON as on disk; defaults applied when fields missing).

### `type: server_config_put`

- **Request** `payload`: full `server.json` document (replace).
- **Response** `payload`: `{ "ok": true }`.
- Persist and runtime apply: [openpfe-server/specification.md#serverjson-project-config](../openpfe-server/specification.md#serverjson-project-config) (PUT semantics).

Invalid document → `type: error`, `payload.code`: `invalid_request`.

## Consumers (v1)

| Client | Uses |
|--------|------|
| `openpfe` CLI | `echo`, `shutdown`, `server_config_get`, `server_config_put`, `mcp` (stdio bridge) |
| Future TUI | `echo`, `shutdown`, `server_config_get`, `server_config_put` — **not** graph / `llm.json` (those use HTTP) |
| Browser / Web UI | HTTP only (`openpfe-ui`) — **no** `server.json` routes |

## MCP (reference)

Follows [Model Context Protocol](https://modelcontextprotocol.io/). Tool/resource list: future doc or [openpfe-mcp/specification.md](../openpfe-mcp/specification.md).

## Related

- [openpfe/specification.md](../openpfe/specification.md) — lock-held wait behavior
