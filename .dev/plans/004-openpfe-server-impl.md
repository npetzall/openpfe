# Plan 004: `openpfe-server` (phase 1 minimal)

**Status:** Complete (2026-05-24).

**Read when:** implementing minimal `crates/openpfe-server/` for phase 1 (lock, socket, echo, HTTP stub). **After** [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) for implementation. **Wiring:** [005-openpfe-wiring.md](./005-openpfe-wiring.md).

**Trait pattern:** **Excluded** — [coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports) composer exception. Use **plain functions** and direct `openpfe_ipc` calls in this crate; no crate-local port traits in `openpfe` for server lifecycle (005 wires `run_server*` from `main` only).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-server/specification.md](../crates/openpfe-server/specification.md) | Runtime dir, shutdown ordering |
| [openpfe-server/design.md](../crates/openpfe-server/design.md) | flock, listeners, stale socket |
| [openpfe-server/requirements.md](../crates/openpfe-server/requirements.md) | FR-1, FR-6 (stub), FR-7 defaults deferred |
| [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md) | Echo payload, socket path |
| [guidelines/protocols.md](../guidelines/protocols.md) | Loopback bind, no v1 auth |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Integration test layout |
| [guidelines/plans.md](../guidelines/plans.md) | Intake → pause → implement |

## Prerequisites

- [x] [001-scaffolding.md](./001-scaffolding.md) complete (path deps to `openpfe-ipc`, `openpfe-ui`, etc. already declared; unused in this slice).
- [ ] [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) **Complete** — `IpcClient`, `IpcListener`, `RequestHandler` available.

**Not required before starting intake:** 005 — binary adapters land in wiring plan.

### Dependency intake (phase A — complete before implementation)

New or changed **external** deps for `openpfe-server`. Follow [guidelines/plans.md](../guidelines/plans.md#dependency-intake-gate-required). Reuse workspace crates already accepted in 002/003 where possible.

- [ ] `.dev/dependencies/axum/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`) — HTTP stub router
- [ ] `.dev/dependencies/fd-lock/rational.md` (+ …) — `pid` exclusive flock
- [ ] Reuse / extend **tokio** ([tokio/rational.md](../dependencies/tokio/rational.md)): server features (`net`, `signal`, `rt-multi-thread`, … as needed); update rational / verdict if feature set changes
- [ ] Reuse **serde** / **serde_json** if already accepted for 003; otherwise complete `serde` intake here
- [ ] `.dev/dependencies/thiserror/rational.md` (+ …) — `ServerError` (skip if accepted in 003 and shared via workspace)
- [ ] `crates/openpfe-server/Cargo.toml` + root `[workspace.dependencies]` updated; **`cargo audit`** immediately after manifest edit
- [ ] **Human intake approval** ([guidelines/plans.md#pause-checkpoint-manual-inspection](../guidelines/plans.md#pause-checkpoint-manual-inspection))

**Do not start tasks in “Implementation” until the human intake approval box is checked.**

After phase A, set **Status** to `Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD).` and stop until approval.

## Goal

Implement the **phase 1 server process**: exclusive `pid` flock, stale-socket probe and bind, IPC dispatch for `echo` / `shutdown` / `mcp` (MCP stub), and a minimal **loopback HTTP stub** that exposes `http_base_url` via echo — without mounting full `openpfe-ui` / `openpfe-webui` routers yet. Public entrypoints are `run_server` and `run_server_with_opts`; bodies must be real at plan completion (no `todo!()`).

## Public API (this plan)

```rust
pub async fn run_server() -> Result<(), ServerError>;
pub async fn run_server_with_opts(opts: ServerOptions) -> Result<(), ServerError>;
```

| Type / field | Purpose |
|--------------|---------|
| `ServerOptions::foreground` | Log to stderr vs `./.openpfe/server/openpfe.log` (stub log file OK in slice) |
| `ServerOptions::shutdown_timeout` | Drain bound (default **5s**) |

## Tasks

### Implementation (phase C — after intake approval and 003 complete)

#### 1. Layout and errors

- [ ] `error.rs` — `ServerError` (`thiserror`)
- [ ] `options.rs` — `ServerOptions` (`foreground`, `shutdown_timeout`)
- [ ] `lib.rs` — export `run_server`, `run_server_with_opts`

#### 2. Runtime directory and lock

- [ ] `lock.rs` — ensure `./.openpfe/server/`; `fd-lock` on `pid`; write ASCII PID; hold for process lifetime
- [ ] `runtime.rs` — stale `socket` probe via `openpfe_ipc::IpcClient::echo`; unlink if dead; coordinate paths with [specification](../crates/openpfe-server/specification.md)

#### 3. Listeners and dispatch

- [ ] `ipc_dispatch.rs` — `IpcListener::serve` + `RequestHandler`: echo (`ok`, `http_base_url`), shutdown, mcp → JSON-RPC not-implemented stub
- [ ] `http_stub.rs` — `127.0.0.1:0` axum `GET /` OK; store `http_base_url` for echo payload

#### 4. Lifecycle

- [ ] `run_server` / `run_server_with_opts` — acquire lock → bind socket + HTTP → serve until shutdown → stop accept → drain (~timeout) → remove `pid` + `socket`
- [ ] SIGINT/SIGTERM → same shutdown path as IPC shutdown (best effort in v1 slice)

#### 5. Tests

- [ ] Integration test: temp cwd, `run_server` in background task, `IpcClient::echo` → `http://127.0.0.1:…` in `http_base_url`

## Acceptance criteria

- [ ] All implementation task boxes above are `[x]`
- [ ] [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) acceptance criteria met (dependency)
- [ ] `cargo test -p openpfe-server` passes (including echo integration test)
- [ ] `cargo clippy -p openpfe-server -- -D warnings` clean (or documented in `verdict.md`)
- [ ] `run_server*` callable from `openpfe --server` after [005](./005-openpfe-wiring.md)
- [ ] Echo returns live `http_base_url` matching bound HTTP stub
- [ ] Intake artifacts present; audit clean or documented in `verdict.md`

## Out of scope

- `server.json` load/save, `openpfe.log` production setup, **`TraceLayer`** / **tower-http**
- Full mounts of `openpfe-ui`, `openpfe-webui`, `openpfe-mcp`, `openpfe-llm` (path deps may stay unused)
- Graph, real MCP tools, LLM inference, CORS, HTTP auth

## Next

[005-openpfe-wiring.md](./005-openpfe-wiring.md).
