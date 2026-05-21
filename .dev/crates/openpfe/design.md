# openpfe (binary) — design

CLI binary: subcommands, IPC **client**, detached server spawn, browser launch, stdio↔IPC MCP bridge. Does **not** implement MCP tools, HTTP routes, or graph logic.

Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Decisions

| Topic | Decision |
|-------|----------|
| **Project root** | Assume **cwd** is project root; paths under `./.openpfe/server/`. |
| **Server detection** | Echo on `socket` ⇒ client path; else flock / spawn / lock-held wait. |
| **MCP attachment** | Thin **stdio↔IPC bridge** in this binary only — not a server-owned stdio pipe. MCP tools/resources in `openpfe-mcp` on the server. |
| **Runtime (v1)** | **tokio** for IPC client (echo, shutdown, MCP bridge) — same workspace runtime as server; see [architcture.md](../../architcture.md#technology-stack-v1). |
| **Detached spawn (v1)** | Re-exec same binary with hidden `--server` flag. Parent uses `Command` with stdio null; on Unix, `pre_exec` calls **`setsid()`** (no `daemon(3)`, no `detach` crate). Child logs to `.openpfe/server/openpfe.log` (see [openpfe-server/design.md](../openpfe-server/design.md)). |
| **Browser (v1)** | [`webbrowser`](https://docs.rs/webbrowser) opens `http_base_url` from echo as-is. **No** auth query param — localhost-only, no auth layer in v1 ([protocols.md](../../guidelines/protocols.md)). |
| **`--timeout` default** | **5s**; override via `OPENPFE_TIMEOUT` (seconds). Lock-held echo retry uses backoff steps **50ms → 200ms → 500ms** (repeat last step) until deadline. |
| **`-v`** | Off by default; when set, log each flock/echo retry and spawn/connect milestones to stderr. |
| **v1 commands** | Normative: default (no subcommand), `mcp`, `stop`, internal `--server`. **`init` / `status` / `logs` deferred** — see [specification.md](./specification.md). |
| **`openpfe stop` when down** | Exit **0**, stderr message `server not running` (idempotent). |
| **IPC version (v1)** | No negotiation. Client sends envelope `"v": 1`; server rejects other values ([openpfe-ipc/specification.md](../openpfe-ipc/specification.md)). |
| **Headless / CI** | `--no-browser` or `OPENPFE_NO_BROWSER=1` skips launch. If browser open fails after server is up, **warn on stderr and exit 0** (browser is optional). |

## Client process

1. Assume **cwd is project root**; paths under `./.openpfe/server/`.
2. If `socket` exists → **echo**; if OK, server is up → handle subcommand.
3. Otherwise try **non-blocking flock** on `pid`:
   - **Acquired** → become server (delegate to `openpfe-server` entry).
   - **Failed** → **lock-held wait** (below).
4. If flock failed earlier and no server was spawned yet → **start server** (detached child), then **lock-held wait**.
5. Branch on subcommand when echo succeeds.

If `./.openpfe/` is missing, fail clearly (unless a future command creates the project).

## Lock held, waiting for IPC (retry)

When `flock` on `pid` fails, another process is (or was) the server. **Do not** exit immediately if echo fails — peer may still be starting.

```
loop until `--timeout` deadline (backoff: 50ms → 200ms → 500ms, then repeat 500ms):
  try connect + echo on socket
  if echo OK → success (client path)
sleep step
exit with diagnostics (lock contention failure)
```

Same loop when a parent waits on a child it just spawned. Configurable via `--timeout` / env — [specification.md](./specification.md).

## Detached server spawn

Parent must not hold the terminal for the server’s lifetime:

```
CLI (parent)
  ├─ spawn openpfe --server (or internal flag)
  ├─ wait for IPC echo (with timeout/backoff)
  ├─ optional: open browser
  └─ exit 0
Server (child, detached)
  ├─ new session / no controlling tty
  └─ run listeners until shutdown
```

## MCP over IPC (`openpfe mcp`)

```
IDE ─stdio─► CLI (mcp mode) ─IPC─► Server ─► openpfe-mcp ─► Graph / tools
```

- CLI does **not** reimplement MCP business logic.
- If server was just spawned, wait for echo before bridging stdio.
- Each `mcp` invocation = one IPC connection; server multiplexes.

## Lock contention diagnostics

If **lock is held** and **echo never succeeds** within the retry budget: non-zero exit, **stderr** diagnostics:

| Field | Source |
|-------|--------|
| **Recorded PID** | ASCII body of `./.openpfe/server/pid` |
| **Recorded PID alive?** | `kill(pid, 0)` or equivalent |
| **Recorded process name** | OS query — or “not running” |
| **Lock holder PID** | `lsof` / `fuser` on `pid` when available |
| **Lock holder process name** | OS query when known |
| **Hint** | Retry, `openpfe stop`, or manual cleanup if dead |

Example (illustrative):

```
error: openpfe server for this project did not respond on IPC
  pid file: .openpfe/server/pid (locked by another process)
  recorded pid: 12345 (alive, comm: openpfe)
  lock holder pid: 12345 (comm: openpfe)
  socket: .openpfe/server/socket (no echo after 5.0s)
hint: wait and retry, run `openpfe stop`, or remove stale socket if no openpfe process is running
```

Platform-specific lock-holder detection behind a trait for future Windows.

## CLI without lock

Echo on `socket` ⇒ client. No echo → try flock; if flock fails → **lock-held wait**.

**Race:** two starters — one acquires `pid` flock, one fails lock and retries echo until winner binds `socket`.

## Logging

- CLI: quiet by default; `-v` for connection / lock / echo retries.

## Related

- [openpfe-server/design.md](../openpfe-server/design.md) — server process, bind, shutdown
- [openpfe-ipc/design.md](../openpfe-ipc/design.md) — echo, framing
- [cross-cutting.md](../../cross-cutting.md) — project root, flow index
