# Plan 002: `openpfe` binary (primary)

**Status:** Complete (2026-05-23). Mock adapters and pre-005 `main` wiring; real IPC/server adapters in [005-openpfe-wiring.md](./005-openpfe-wiring.md).

**Read when:** implementing the CLI binary (`crates/openpfe/`) — ports, commands, spawn/wait logic, mocks. **Wiring** to real crates is [005-openpfe-wiring.md](./005-openpfe-wiring.md).

**Parallel with:** [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md), [004-openpfe-server-impl.md](./004-openpfe-server-impl.md). **After all three:** [005-openpfe-wiring.md](./005-openpfe-wiring.md).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe/design.md](../crates/openpfe/design.md) | Client flow, spawn, lock-held wait, MCP bridge, diagnostics |
| [openpfe/requirements.md](../crates/openpfe/requirements.md) | FR-1.3, FR-1.5–1.8, FR-2–FR-4 |
| [openpfe/specification.md](../crates/openpfe/specification.md) | CLI surface, paths, flags |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports) | Ports + adapters; **`openpfe-server` excluded** |
| [guidelines/protocols.md](../guidelines/protocols.md) | Echo discovery |
| [guidelines/mcp.md](../guidelines/mcp.md) | stdio bridge |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Mock unit tests vs integration in 005 |

## Prerequisites

- [x] [001-scaffolding.md](./001-scaffolding.md) complete.
- [x] External crates for **this crate only** (no workspace path deps until 005): `tokio`, `clap`, `webbrowser`; Unix: `libc` for `setsid` — [dependencies/README.md](../dependencies/README.md).

**Not required before starting:** 003, 004, or 005 — define **ports** first; real adapters land in 005.

## Goal

Implement **`openpfe`** business logic behind **crate-local traits** ([coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports)):

- **`ProjectControl`** — echo, shutdown, `send_mcp` (socket path from cwd).
- **`ServerSpawn`** — detached `openpfe --server` (or mock no-op).
- **`--server` path** — excluded from a port: thin call to `openpfe_server::run_server*` only in **005** wiring (guideline exception).

CLI, lock-held wait, browser, diagnostics, and **mock adapters** for unit tests. No `use openpfe_ipc::…` / `use openpfe_server::…` outside `adapters/` (adapters added in 005).

## Port contract (align with 003)

Document in `ports.rs` (normative for 003 implementers):

| Method | Purpose |
|--------|---------|
| `echo(&self) -> Result<EchoInfo, …>` | `EchoInfo` includes `http_base_url: String` |
| `shutdown(&self) -> Result<(), …>` | Graceful stop request |
| `send_mcp(&self, payload: Value) -> Result<Value, …>` | Opaque JSON-RPC in/out |

Async (`async_trait`) if needed. Errors: crate-local `ClientError`.

## Tasks

### 1. Layout

- [x] `ports.rs` — traits + `EchoInfo`.
- [x] `adapters/mock.rs` — in-memory / configurable mock for tests.
- [x] `cli.rs`, `client.rs`, `spawn.rs`, `bridge.rs`, `diagnostics.rs`, `paths.rs` — depend on **ports only**.
- [x] `main.rs` — until 005: construct **mock** wiring via `openpfe::run` (`lib.rs`).

### 2. CLI surface

- [x] Commands: default, `mcp`, `stop`; hidden `--server` (body stub until 005).
- [x] Flags: `-v`, `--timeout` (default **5**), `--no-browser`; env `OPENPFE_TIMEOUT`, `OPENPFE_NO_BROWSER=1`.

### 3. Project paths

- [x] `./.openpfe/server/{pid,socket}` from cwd; fail if `./.openpfe/` missing (FR-1.5).

### 4. Client logic (via `ProjectControl`)

- [x] Lock-held wait: backoff **50ms → 200ms → 500ms** (repeat 500ms).
- [x] `-v`: log retries to stderr.

### 5. Spawn (`ServerSpawn` port)

- [x] Re-exec + `setsid` implementation behind port (`spawn.rs` / `ReExecSpawn`; test with mock spawn).

### 6. Commands

- [x] Default (FR-2): ensure server via port → browser → exit.
- [x] `stop` (FR-4): idempotent when echo fails.
- [x] `mcp` (FR-3): stdio loop calling `send_mcp`; stderr for logs.

### 7. Diagnostics (FR-1.8)

- [x] Lock contention stderr on timeout ([design.md](../crates/openpfe/design.md#lock-contention-diagnostics)).

### 8. Unit tests (mock ports only)

- [x] Lock-held wait, stop idempotent, second default no double spawn (mock), diagnostics text.

## Acceptance criteria (before 005)

- [x] All command/handler code uses **traits**, not workspace crate APIs.
- [x] `cargo test -p openpfe` passes with **mock adapters** only.
- [x] Port shapes documented and stable for [003](./003-openpfe-ipc-impl.md).

## Out of scope (this plan)

- `Cargo.toml` path deps on `openpfe-ipc` / `openpfe-server` (005).
- Real UDS, flock, integration/e2e (005).
- `init` / `status` / `logs`; MCP tool semantics.

## Next

[005-openpfe-wiring.md](./005-openpfe-wiring.md) after 002–004 slices exist.
