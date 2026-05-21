# openpfe-ipc — design

Framing, envelope routing types, UDS connect/bind (v1 Unix). **No** domain, MCP tool logic, or HTTP.

## Decisions

| Topic | Decision |
|-------|----------|
| **Transport (v1)** | Unix domain sockets on macOS/Linux. |
| **Abstraction** | `IpcTransport` trait for future Windows (named pipe / TCP). |
| **Framing** | Length-prefixed frames (max size cap). |
| **Runtime (v1)** | **tokio** Unix domain sockets (`tokio::net::UnixListener` / `UnixStream`) — matches async server in `openpfe-server`. |
| **Admin IPC (v1)** | `echo`, `shutdown`, `mcp` only — used by CLI, IDE bridge, and future TUI for control; not for graph/config data. |
| **Message encoding (v1)** | **4-byte LE length** + UTF-8 JSON **envelope** per frame ([specification.md](./specification.md)). MCP JSON-RPC lives only inside `type: mcp` `payload` — not at the framing layer. No CBOR, no bare JSON-RPC framing in v1. |
| **Protocol version (v1)** | **`"v": 1` on every frame** — no post-connect handshake, no negotiation ([openpfe/design.md](../openpfe/design.md)). Unsupported `v` → `type: error` response, then close connection on repeated violations. |
| **Correlation (v1)** | **Per-connection** pairing: client sets envelope `id` (u64); server **echoes the same `id`** on the response envelope. One in-flight request per connection in v1 (stdio bridge is sequential). |
| **Max frame size (v1)** | **16 MiB** body cap (length prefix excluded); oversize or invalid length → protocol error, drop connection. |
| **Echo (v1)** | Admin envelope `type: echo` only (no separate ping opcode). Request `payload`: `{}` or `{ "probe": true }` (equivalent). Response `payload` must include `ok` and **`http_base_url`**. |
| **MCP over IPC (v1)** | **Request/response only** — one MCP JSON-RPC message (or batch array) per `type: mcp` envelope in, one envelope out. **No streaming** over IPC in v1 (defer to phase 3 if MCP stdio streaming is required). |
| **Socket path (v1)** | `./.openpfe/server/socket` — filename **`socket`** ([openpfe-server/specification.md](../openpfe-server/specification.md)). |

## Goals

- Multiple simultaneous clients (MCP proxies, future TUI, `stop` while Web UI active).
- **MCP:** carry MCP messages; server runs handler (may stream).
- **Extensibility:** reserved kinds for CLI (`stop`, status) without breaking MCP clients.

## Layering

| Layer | Responsibility |
|-------|----------------|
| Transport | UDS (v1); trait for later Windows |
| Framing | Length-prefixed frames |
| Envelope | `type`, `id`, `payload` — route to MCP engine or admin |
| MCP bridge (in `openpfe` bin) | Map envelope ↔ MCP JSON-RPC over byte stream |

Normative envelope: [specification.md](./specification.md).

## Echo (health)

Client sends `Echo` / `Ping`. Server replies with at least:

- `ok: true`
- **`http_base_url`** (required — ephemeral HTTP bind)
- optional: `version`, `pid`

Used for: client “server up”, readiness after spawn, stale-**socket** probe. **Singleton** is **`pid` flock**, not echo alone.

## Shutdown envelope

`type: shutdown` — see [specification.md](./specification.md). Server handles lifecycle in `openpfe-server`.

## Related

- [openpfe/design.md](../openpfe/design.md) — MCP stdio bridge
- [openpfe-server/design.md](../openpfe-server/design.md) — accept loop
