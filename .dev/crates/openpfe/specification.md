# openpfe (binary) — specification

## Binary

| Property | Value |
|----------|--------|
| Name | `openpfe` |
| Modes | Client (`default`, `mcp`, `stop`) and server (`--server`, internal) |

## Paths (client)

Relative to **process cwd** (project root). Same literals as [openpfe-server/specification.md](../openpfe-server/specification.md#runtime-directory) — documented here for the binary; no shared path-helper crate.

| Path | Purpose |
|------|---------|
| `./.openpfe/server/pid` | Flock target; read PID for diagnostics |
| `./.openpfe/server/socket` | IPC connect path (echo, shutdown, MCP bridge) |

Server creates and owns runtime files; client reads/waits. If `./.openpfe/` is missing, fail clearly unless a future command creates the project.

## CLI surface

| Command | Behavior summary |
|---------|------------------|
| `openpfe` | Ensure server → open browser → exit |
| `openpfe mcp` | Ensure server → stdio MCP bridge → block until disconnect |
| `openpfe stop` | IPC shutdown if running; if not running, exit **0** with `server not running` on stderr |
| `openpfe llm init` | Write local `llm.json` from `catalog.json` + hardware; start download if needed — [LLM init](#llm-init) |
| `openpfe llm download status <job_id>` | Poll download via IPC echo + HTTP `status_url` |

Future (non-normative): `openpfe status`, `openpfe init` (full project), TUI subcommand. A future TUI uses **HTTP** for data (`openpfe-ui`) and **IPC** only for echo/shutdown — [architcture.md](../../architcture.md#tui-v1).

## Global flags

| Flag | Purpose |
|------|---------|
| `-v` | Verbose: log each lock/echo retry (default: off) |
| `--timeout <secs>` | Max wait when lock held and waiting for IPC echo (default: **5**; env: `OPENPFE_TIMEOUT`) |
| `--no-browser` | Skip browser on default command (env: `OPENPFE_NO_BROWSER=1`) |

## Lock held, IPC wait

When `flock` on `pid` fails:

1. Retry **connect + echo** on `socket` until success or `--timeout`, with backoff **50ms → 200ms → 500ms** (then repeat 500ms) between attempts.
2. On success → continue as client.
3. On timeout → exit non-zero with **lock contention diagnostics** on stderr:

   - `recorded_pid`, `recorded_pid_alive`, `recorded_comm`
   - `lock_holder_pid`, `lock_holder_comm` (from `lsof`/`fuser` when available)
   - `socket` path and echo failure duration
   - Short **hint** line

See [design.md](./design.md) for example message.

## LLM init

**`openpfe llm init`** — CLI-only bootstrap for local **`llm.json`**. Does **not** use HTTP init routes or Web UI.

### Flags

| Flag | Purpose |
|------|---------|
| `--dry-run` | Print hardware-ranked recommendations; do not write `llm.json` or start download |
| `--force` | Overwrite existing `llm.json` |
| `--no-download` | Write `llm.json` only; do not start server for download |

### Flow

1. Read **`./.openpfe/catalog.json`** (committed). Fail clearly if missing or empty when resolution needs entries.
2. **`openpfe-llm`:** probe hardware; **`resolve_initial_config`** — best **recommended ∩ capable** model; set `n_ctx` / `n_threads` from catalog **`requirements`** when present.
3. Write **`./.openpfe/llm.json`** (gitignored) unless `--dry-run`.
4. If chosen model **installed** → stdout JSON `{ "model", "installed": true }`; exit **0**.
5. If not installed and not `--no-download`:
   - **`ensure_server`** (spawn detached if echo fails).
   - **IPC `echo`** → `http_base_url`.
   - **HTTP `POST {http_base_url}/api/v1/catalog/{id}/download`**.
   - Stdout JSON:

```json
{
  "model": "llama-3.2-3b-instruct",
  "installed": false,
  "job_id": "<uuid>",
  "status_url": "http://127.0.0.1:PORT/api/v1/downloads/<uuid>"
}
```

6. **`openpfe llm download status <job_id>`** — echo + `GET status_url`; print `DownloadStatus` JSON.

Init does **not** call **`GET /catalog/discover`**; it reads committed catalog directly.

### Stdout / stderr

- **stdout:** machine-readable JSON (above).
- **stderr:** human progress when `-v`; errors on failure.

## Related

- [openpfe-ipc/specification.md](../openpfe-ipc/specification.md) — echo contract
- [openpfe-server/specification.md](../openpfe-server/specification.md) — runtime dir lifecycle
