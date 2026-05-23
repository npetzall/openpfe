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

Future (non-normative): `openpfe status`, `openpfe init`, TUI subcommand. A future TUI uses **HTTP** for data (`openpfe-ui`) and **IPC** only for echo/shutdown — [architcture.md](../../architcture.md#tui-v1).

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

## Related

- [openpfe-ipc/specification.md](../openpfe-ipc/specification.md) — echo contract
- [openpfe-server/specification.md](../openpfe-server/specification.md) — runtime dir lifecycle
