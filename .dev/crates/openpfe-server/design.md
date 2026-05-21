# openpfe-server — design

Listeners, **pid** flock, runtime dir lifecycle, mount **`openpfe-webui`** + **`openpfe-ui`**, dispatch IPC to **`openpfe-mcp`**. Wires services; does not own business handlers.

Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Decisions

| Topic | Decision |
|-------|----------|
| **Singleton** | Exclusive **non-blocking `flock`** on `./.openpfe/server/pid`; hold fd until exit. |
| **HTTP bind** | `127.0.0.1:0`; expose `http_base_url` via IPC echo (no address file). |
| **Stale socket** | Before bind, probe connect+echo; unlink dead path. |
| **Runtime (v1)** | **tokio** — async accept loops for IPC and HTTP; mount axum `Router` from `openpfe-ui`. |
| **HTTP composition** | Merge `openpfe-ui` API router + `openpfe-webui` static fallback; apply **tower-http** on the outer router — see [architcture.md](../../architcture.md#tower-http-v1). |
| **tower-http (v1)** | **`TraceLayer`** on the merged router (access-style request logs). **No `CorsLayer` in v1** — see CORS row below. |
| **LLM / graph calls** | Blocking work (`openpfe-llm`, heavy graph IO) on `spawn_blocking` or internal crate locks — do not block the runtime indefinitely on the main task. |
| **IPC socket file** | `./.openpfe/server/socket` (filename **`socket`**, not `openpfe.sock`) — [openpfe-ipc/specification.md](../openpfe-ipc/specification.md). |
| **`pid` flock (Rust)** | **`fd-lock`** crate — hold `LockFile` for process lifetime; write ASCII PID after lock acquired. |
| **Shutdown (v1)** | **Bounded graceful drain** then exit. Default **5s** (`OPENPFE_SHUTDOWN_TIMEOUT` / `[server] shutdown_timeout_secs`). Stop accept → drain in-flight IPC MCP + HTTP handlers → cancel remainder → close listeners → remove runtime files. |
| **Logging (v1)** | **Detached (default):** append to `./.openpfe/server/openpfe.log`. **Foreground:** `--foreground` or `OPENPFE_FOREGROUND=1` → **stderr** (no log file). Level from config `[server] log_level` (default `info`). |
| **Security (v1)** | **`127.0.0.1` bind only** + project-scoped UDS under `./.openpfe/server/` — **no** HTTP bearer tokens, **no** IPC shared secret ([protocols.md](../../guidelines/protocols.md)). API auth deferred — [openpfe-ui/design.md](../openpfe-ui/design.md). |
| **CORS (v1)** | **Not enabled** — embedded Web UI is same-origin; TUI/CLI HTTP clients are not browsers (no preflight). Revisit when a cross-origin dev client is required. |

## Server process

1. Ensure `.openpfe/server/` exists.
2. Open `pid`, try **exclusive non-blocking `flock`**. If fail → not server; return to client path.
3. Write current **PID** to `pid` (lock still held via open fd).
4. If `socket` path exists → **staleness check** (connect + echo); unlink if dead.
5. Bind Unix socket at `socket`, bind HTTP on `127.0.0.1:0`; keep `http_base_url` in memory for echo.
6. Load config (via `openpfe-core`), initialize graph (`openpfe-graph`), model paths, `openpfe-llm`.
7. Serve until shutdown IPC or signal; on exit close lock fd, remove `socket`, remove or truncate `pid`.

**Rust locking:** `fd-lock` on `./.openpfe/server/pid` (macOS + Linux).

## Stale `socket` (server side)

**`pid` (lock):** Released on process exit; no stale lock after crash.

**`socket`:** May outlive process. Before bind, if path exists:

```
try connect + echo on socket
if success → unexpected live server (error if we hold pid lock)
if connect/echo fails → unlink socket path, then bind
```

## Listeners

| Listener | Role |
|----------|------|
| **HTTP** | Static from `openpfe-webui`; human API from `openpfe-ui` `/api/v1/…` |
| **IPC** | Accept; route envelopes to `openpfe-mcp` or admin (`shutdown`) |

## Graceful shutdown

Triggered by IPC `type: shutdown`, **`openpfe stop`**, or **SIGINT/SIGTERM** (same path).

1. **Stop accepting** new IPC and HTTP connections.
2. **Drain** in-flight work on existing connections until **`shutdown_timeout`** (default **5s**):
   - IPC `type: mcp`: allow current JSON-RPC request on each connection to finish.
   - HTTP: allow active handler tasks started before step 1 to finish.
   - `openpfe-llm` / long graph jobs: best-effort cancel at deadline (do not block shutdown indefinitely).
3. **Cancel** remaining tasks; close all IPC connections.
4. **Close** HTTP listener and drop router state.
5. Release **pid** flock, remove `socket` and `pid`, exit **0**.

If drain exceeds timeout, log a warning and proceed (fail-safe stop). Normative ordering: [specification.md](./specification.md).

## Concurrency

- One accept loop (or async task) per IPC listener.
- Per-connection read loop; dispatch to shared `Arc` services (graph vs LLM locking internal to those crates).

## Logging

| Mode | Destination |
|------|-------------|
| Detached server (CLI spawn) | `./.openpfe/server/openpfe.log` (append) |
| `--foreground` / `OPENPFE_FOREGROUND=1` | stderr |

Use `tracing` + subscriber in `openpfe-server`; CLI remains quiet unless `-v`.

## Related

- [openpfe/design.md](../openpfe/design.md) — client flock wait, diagnostics
- [openpfe-ipc/design.md](../openpfe-ipc/design.md) — framing, echo payload
- [architcture.md](../../architcture.md) — system diagram
