# Plan 005: Phase 1 wiring

**Status:** Complete (2026-05-24).

**Read when:** connecting [002-openpfe-impl.md](./002-openpfe-impl.md), [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md), and [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) into a working end-to-end CLI.

**Trait pattern:** **Ports in `openpfe` only** — business modules keep using `ProjectControl` / `ServerSpawn`; **only** `adapters/` and `main.rs` import `openpfe_ipc` / `openpfe_server` ([coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports)).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe/design.md](../crates/openpfe/design.md) | Client flow, spawn, `--server` exception |
| [openpfe/requirements.md](../crates/openpfe/requirements.md) | FR-1.3–1.8, FR-2–FR-4 |
| [openpfe/specification.md](../crates/openpfe/specification.md) | CLI surface, paths |
| [002-openpfe-impl.md](./002-openpfe-impl.md#port-contract-align-with-003) | `ProjectControl`, `ServerSpawn` |
| [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md#consumer-contract-align-with-002) | `IpcClient` mapping |
| [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) | `run_server*` |
| [workspace-crates.md#phasing](../workspace-crates.md#phasing) | Phase 1 goal |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Integration vs unit tests |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports) | Adapter boundary |
| [guidelines/plans.md](../guidelines/plans.md) | Progress tracking |

## Prerequisites

**No new crates.io intake for this plan** — only workspace path dependencies on `openpfe-ipc` and `openpfe-server` in `crates/openpfe/Cargo.toml`. Phases A–B ([guidelines/plans.md](../guidelines/plans.md#dependency-intake-gate-required)) are skipped.

- [x] [002-openpfe-impl.md](./002-openpfe-impl.md) **Complete** — ports, CLI, mocks, `cargo test -p openpfe` green.
- [x] [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) **Complete** — `IpcClient` / `IpcListener` per consumer contract.
- [x] [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) **Complete** — `run_server*` + echo integration test.

Existing workspace deps (`tokio`, `clap`, `serde_json`, `async-trait`, `libc`) are unchanged unless a bugfix requires a separate intake batch.

## Goal

Replace mock adapters with **real** `openpfe_ipc` / spawn implementations, add **path dependencies** to the `openpfe` binary, wire **production `main`**, and add **integration tests** so phase 1 (“lock, socket, echo, HTTP stub”) is demonstrable end-to-end from the CLI.

## Tasks

### Implementation

#### 1. Manifest and dependency graph

- [x] `crates/openpfe/Cargo.toml` — add `openpfe-ipc`, `openpfe-server` path dependencies
- [x] `cargo tree -p openpfe` — no forbidden edges to `openpfe-ui` / `openpfe-mcp` / `openpfe-llm` on client paths ([workspace-crates.md](../workspace-crates.md#dependency-rules-normative))

#### 2. Adapters (`openpfe`)

- [x] `adapters/ipc.rs` — `ProjectControl for …` wrapping `openpfe_ipc::IpcClient`; map `IpcError` → `ClientError`
- [x] `adapters/spawn.rs` — `ServerSpawn` using real `Command` + `setsid` (Unix)
- [x] `adapters/mod.rs` — export production adapters

#### 3. Binary wiring

- [x] `main.rs` / `lib.rs` `run` — production: real adapters; unit tests: continue using mocks where appropriate
- [x] Hidden `--server` → `openpfe_server::run_server_with_opts` **directly** in `main` (guideline exception; no `ServerSpawn` for server process)

#### 4. Integration tests (`crates/openpfe/tests/`)

- [x] Temp project root harness (`.openpfe/server/` layout)
- [x] `echo_after_spawn` — server up, echo returns `http_base_url`
- [x] `stop_idempotent_when_down` — exit 0 when already stopped
- [x] `second_default_no_double_server` — no duplicate spawn when echo succeeds
- [x] `mcp_bridge_roundtrip` — stdio MCP envelope forwarded via real IPC (stub response OK)
- [x] Lock-held timeout / diagnostics (real or hybrid with controllable flock)

#### 5. Workspace verification

- [x] `cargo test -p openpfe`
- [x] `cargo test --workspace`
- [x] `cargo fmt --all` and `cargo clippy --workspace -- -D warnings` (or documented exceptions)
- [ ] Manual smoke: `openpfe`, `openpfe stop`, `openpfe mcp` against real server

## Acceptance criteria

- [x] All task boxes above are `[x]` (except optional manual smoke)
- [x] [002](./002-openpfe-impl.md), [003](./003-openpfe-ipc-impl.md), [004](./004-openpfe-server-impl.md) marked **Complete**
- [x] Binary-layer coverage: FR-1.3, FR-1.5–1.8, FR-2.1–2.4, FR-3.1–3.2, FR-3.4, FR-4.1–4.2 ([openpfe/requirements.md](../crates/openpfe/requirements.md))
- [x] Command/handler modules use **ports only**; `openpfe_ipc` / `openpfe_server` appear only under `adapters/` and `main` (`--server` path)
- [x] Phase 1 goal from [workspace-crates.md#phasing](../workspace-crates.md#phasing) met: lock, socket, echo, HTTP stub via real CLI path
- [x] `cargo test --workspace` passes

## Out of scope

- Phase 2 (`openpfe-graph`, `openpfe-webui`, `openpfe-ui` integration)
- Full MCP tool semantics, graph API, LLM routes
- `server.json`, production logging, `init` / `status` / `logs` commands

## Next

Phase 2 per-crate plans for `openpfe-graph`, `openpfe-webui`, `openpfe-ui` (TBD in [plans/README.md](./README.md)).
