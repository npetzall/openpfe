# openpfe-server — specification

## Runtime directory

Path: **`./.openpfe/server/`** relative to **process cwd** (project root). Implementations use this literal (no shared path-helper crate).

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

## `server.json` (project config)

Path: **`./.openpfe/server.json`** (JSON). Loaded/saved by **`openpfe-server`** only. Read/write for trusted local clients via IPC admin envelopes **`server_config_get`** / **`server_config_put`** — [openpfe-ipc/specification.md](../openpfe-ipc/specification.md). **Not** on human HTTP (`openpfe-ui`) and **not** on MCP.

Missing file → defaults (`#[serde(default)]`).

### Document shape (v1)

```json
{
  "server": {
    "log_level": "info",
    "shutdown_timeout_secs": 5
  },
  "http": {
    "host": "127.0.0.1"
  }
}
```

| Field | Default | Purpose |
|-------|---------|---------|
| `server.log_level` | `"info"` | Log filter |
| `server.shutdown_timeout_secs` | `5` | Graceful drain ([shutdown ordering](#shutdown-ordering)) |
| `http.host` | `"127.0.0.1"` | Bind address (port remains ephemeral) |

Env `OPENPFE_SHUTDOWN_TIMEOUT` may override drain seconds when set (document precedence at implementation).

### IPC `server_config_put` — runtime apply (v1)

| Field | On successful PUT |
|-------|-------------------|
| `server.log_level` | Apply immediately to active log filter |
| `server.shutdown_timeout_secs` | Apply immediately to in-memory graceful-drain timeout |
| `http.host` | Persist only — **rebind requires server restart** (v1 binds once at startup) |

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

