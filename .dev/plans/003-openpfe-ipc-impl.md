# Plan 003: `openpfe-ipc`

**Status:** Complete (2026-05-23).

**Read when:** implementing `crates/openpfe-ipc/`. **Parallel with** [002-openpfe-impl.md](./002-openpfe-impl.md) and [004-openpfe-server-impl.md](./004-openpfe-server-impl.md). **Wiring:** [005-openpfe-wiring.md](./005-openpfe-wiring.md).

**Trait pattern:** **Required** in this crate (internal abstractions + public API). Not excluded by [coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md) | Framing, envelopes, error codes |
| [openpfe-ipc/design.md](../crates/openpfe-ipc/design.md) | Layering, `IpcTransport`, limits |
| [openpfe-ipc/requirements.md](../crates/openpfe-ipc/requirements.md) | FR-5, NFR-1, NFR-6 |
| [002-openpfe-impl.md](./002-openpfe-impl.md#port-contract-align-with-003) | Consumer port contract (`ProjectControl`) |
| [guidelines/protocols.md](../guidelines/protocols.md) | Wire rules |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Unit vs integration tests |
| [guidelines/plans.md](../guidelines/plans.md) | Intake → pause → implement |

## Prerequisites

- [x] [001-scaffolding.md](./001-scaffolding.md) complete.
- [x] [002-openpfe-impl.md](./002-openpfe-impl.md) port shapes stable (`ProjectControl`, `EchoInfo` in `crates/openpfe/src/ports.rs`).

**Not required before starting intake:** 004 or 005 — server and binary adapters consume this crate later.

### Dependency intake (phase A — complete before implementation)

New or changed **external** deps for `openpfe-ipc`. Follow [guidelines/plans.md](../guidelines/plans.md#dependency-intake-gate-required); one batch, then pause for human review.

- [x] `.dev/dependencies/serde/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`) — envelope (de)serialization
- [x] `.dev/dependencies/thiserror/rational.md` (+ …) — crate-local `IpcError`
- [x] Reuse / extend **tokio** ([tokio/rational.md](../dependencies/tokio/rational.md)): add workspace feature **`net`** for UDS; update rational if needed; `scan.md` / `verdict.md` reflect the feature change
- [x] Reuse **serde_json** ([serde_json/rational.md](../dependencies/serde_json/rational.md)) via `{ workspace = true }` on `openpfe-ipc` (no second rational unless scope changes)
- [x] `crates/openpfe-ipc/Cargo.toml` + root `[workspace.dependencies]` updated; **`cargo audit`** run immediately after manifest edit ([security-rust.md](../guidelines/security-rust.md))
- [x] **Human intake approval** (verdicts accepted; see [guidelines/plans.md#pause-checkpoint-manual-inspection](../guidelines/plans.md#pause-checkpoint-manual-inspection))

## Goal

Implement a **domain-free** IPC library: length-prefixed JSON envelopes over Unix domain sockets (v1), with **internal traits** for codec, transport, connection, and server dispatch, and a thin **public API** (`IpcClient`, `IpcListener`) that [005](./005-openpfe-wiring.md) can wrap to satisfy [002](./002-openpfe-impl.md) `ProjectControl`. Business modules in this crate must not call `tokio::net` directly — only adapter/unix modules.

## Consumer contract (align with 002)

[002 port contract](./002-openpfe-impl.md#port-contract-align-with-003) maps to public methods implementers and 005 adapters rely on:

| `ProjectControl` (002) | `IpcClient` (003) | Notes |
|------------------------|-------------------|--------|
| `echo` → `EchoInfo { http_base_url }` | `echo` | Parse `payload.http_base_url` from `type: echo` response |
| `shutdown` | `shutdown` | `type: shutdown` envelope |
| `send_mcp(payload)` | `send_mcp` | `type: mcp`; opaque JSON in/out |

Connection target: `./.openpfe/server/socket` ([specification](../crates/openpfe-ipc/specification.md)). Errors: crate-local `IpcError` (mapped to `ClientError` in 005).

## Tasks

### Implementation (phase C — after intake approval only)

#### 1. Layout and errors

- [x] `error.rs` — `IpcError` (`thiserror`); protocol vs I/O distinction
- [x] `lib.rs` — module exports; no product logic in root

#### 2. Envelope and framing

- [x] `envelope.rs` — types per [specification](../crates/openpfe-ipc/specification.md); `v == 1`; `type: error` codes
- [x] `frame.rs` — `FrameCodec` trait + `LeJsonCodec` (4-byte LE, 16 MiB cap)

#### 3. Transport (internal traits)

- [x] `transport.rs` — `IpcTransport` trait (`connect` / `bind` / `accept`); `UnixTransport` in `adapters/unix.rs` (or `unix.rs`) using `tokio` UDS only here
- [x] `connection.rs` — `Connection` trait: read/write one framed envelope per call

#### 4. Public API

- [x] `client.rs` — `IpcClient`: `connect`, `echo`, `shutdown`, `send_mcp` (uses `Connection` + `FrameCodec`)
- [x] `server.rs` — `IpcListener`: `bind`, `serve(handler)`; per-connection loop
- [x] `handler.rs` — `RequestHandler` trait: `handle(Envelope) -> Envelope` (server dispatch hook for 004)

#### 5. Tests

- [x] Unit tests: `LeJsonCodec` (valid frame, oversize, bad length, bad JSON)
- [x] Unit tests: mock `IpcTransport` / `Connection` (no real socket)
- [x] Integration test: UDS echo round-trip with a minimal test `RequestHandler`

## Acceptance criteria

- [x] All implementation task boxes above are `[x]`
- [x] `cargo test -p openpfe-ipc` passes (unit + one UDS integration test)
- [x] `cargo clippy -p openpfe-ipc -- -D warnings` clean (or documented exception in `verdict.md`)
- [x] Public API covers 002 `ProjectControl` needs without MCP/HTTP semantics in this crate
- [x] [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) can call `IpcListener` / `IpcClient` without duplicating framing
- [x] Intake artifacts present; audit clean or documented in `verdict.md`

## Out of scope

- Flock, HTTP, MCP tool semantics, `openpfe` ports (consumer wiring is [005](./005-openpfe-wiring.md))
- Windows named pipes / TCP transport (trait only; Unix v1 impl)
- Post-v1 streaming MCP over IPC

## Next

[004-openpfe-server-impl.md](./004-openpfe-server-impl.md) (depends on this plan), then [005-openpfe-wiring.md](./005-openpfe-wiring.md).
