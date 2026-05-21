# openpfe-server — specification

## Runtime directory

Path: `./.openpfe/server/` relative to **process cwd** (project root).

| File | Purpose |
|------|---------|
| `socket` | Unix domain socket listen path (`./.openpfe/server/socket`) |
| `pid` | **Exclusive file lock** while server runs; ASCII PID after lock |
| `openpfe.log` | Server log (detached mode; append) |

HTTP base URL is **not** persisted; clients use echo — [openpfe-ipc/specification.md](../openpfe-ipc/specification.md).

## `pid` lifecycle

- Server: `open(O_CREAT)` → `flock(LOCK_EX \| LOCK_NB)` → write PID → hold fd until exit.
- Lock released on process exit; no stale lock after crash.
- Clean shutdown: close fd, delete `pid` and `socket`.

## `socket` lifecycle

- Created on bind; may remain after crash — unlink before re-bind if connect/echo fails — [design.md](./design.md).

## HTTP mount (draft)

| Route | Crate |
|-------|--------|
| **GET /** , **GET /assets/…** | `openpfe-webui` |
| **`/api/v1/…`** | `openpfe-ui` |

Normative API detail: [openpfe-ui/specification.md](../openpfe-ui/specification.md).

## HTTP bind (v1)

| Property | Value |
|----------|--------|
| Address | `127.0.0.1:0` only |
| Discovery | `http_base_url` via IPC echo — not written to disk |
| Auth | None in v1 |
| CORS | None in v1 (`CorsLayer` not mounted) |

## Server flags / env (v1)

| Flag / env | Purpose |
|------------|---------|
| `--foreground` | Log to stderr instead of `openpfe.log` |
| `OPENPFE_FOREGROUND=1` | Same as `--foreground` |
| `OPENPFE_SHUTDOWN_TIMEOUT` | Drain seconds before force cancel (default **5**) |

Config override: `[server] shutdown_timeout_secs`, `[server] log_level` — [openpfe-core/specification.md](../openpfe-core/specification.md).

## Shutdown ordering

On `type: shutdown`, `openpfe stop`, **SIGINT**, or **SIGTERM**:

1. Stop accepting new IPC and HTTP connections.
2. **Drain** in-flight IPC `mcp` requests and HTTP handler tasks for up to **`shutdown_timeout`** (default **5s**).
3. **Cancel** any remaining work; close all IPC connections.
4. Close HTTP listener.
5. Release `pid` flock; delete `pid` and `socket`; exit **0**.

After step 2 timeout: log warning, proceed to steps 3–5 (do not hang on LLM/graph).

## Signals (v1)

| Signal | Behavior |
|--------|----------|
| `SIGINT` / `SIGTERM` | Same graceful shutdown as IPC `shutdown` |
| `SIGHUP` | Ignored (detached server has no controlling tty) |

