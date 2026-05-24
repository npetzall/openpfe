# Coding — Rust

**Read when:** implementing or reviewing any workspace crate under `crates/`, the `openpfe` binary, or Rust integration tests.

**Normative refs:** [workspace-crates.md](../workspace-crates.md), per-crate [specification.md](../crates/) files, [architcture.md](../architcture.md).

## Workspace layout

- One crate per directory under `crates/<name>/` matching [workspace-crates.md#physical-workspace-layout](../workspace-crates.md#physical-workspace-layout).
- Crate `lib` vs `bin`: `openpfe` is the only primary binary; others are libraries consumed by `openpfe-server` or the bin.

## Async and I/O boundaries

| Layer | Async (tokio) | Sync preferred |
|-------|---------------|----------------|
| `openpfe-server`, `openpfe-ipc` | Yes — listeners, accept loops | — |
| `openpfe-ui` | Yes — axum handlers | Delegate heavy CPU to blocking pool where needed |
| `openpfe-graph` | No direct sockets | Public APIs sync where practical |
| `openpfe-llm` | `spawn_blocking` for inference inside server runtime | — |

Do not pull HTTP or UDS into `openpfe-graph`.

## Dependencies

- Obey [dependency rules](../workspace-crates.md#dependency-rules-normative); run `cargo tree` mentally before adding an edge.
- **Path deps** between workspace members: declare in member `Cargo.toml` as today; no `.dev/dependencies/` intake.
- **External crates** (crates.io / git): full intake in [dependencies/README.md](../dependencies/README.md) — record under `.dev/dependencies/<crate-name>/` **before** changing any real `Cargo.toml`.
- Workspace versions: centralize in root `[workspace.dependencies]` when adopted; member crates use `{ workspace = true }`.
- Prefer std and crates already chosen in [architcture.md](../architcture.md) — do not introduce a parallel stack (e.g. a second HTTP framework).

### Adding an external crate (order)

1. Create `.dev/dependencies/<crate-name>/` and write **`rational.md`**.
2. Preview lock impact: [.dev/scripts/dependency-lock-diff.sh](../scripts/dependency-lock-diff.sh) (`<crate-name>@<version>`); record diff in **`lock-update.md`**.
3. Add the dependency to the real `Cargo.toml`(s) (workspace table + member edges as needed).
4. **Immediately** from repo root: **`cargo audit`** — first Cargo command after the manifest edit; paste output into **`scan.md`**.
5. Complete other scans (when defined) → append **`scan.md`**; write **`verdict.md`**; merge with the PR.

Do not run `cargo build`, `cargo check`, or `cargo update` between step 3 and step 4 unless a scan explicitly requires it — **`cargo audit`** is the mandatory next step after the manifest change.

Details: [security-rust.md](./security-rust.md).

When work is driven by an [implementation plan](../plans/README.md) that adds external crates, complete intake and the **human pause** in [plans.md](./plans.md) before writing product `src/` or running `cargo test` / `cargo build`.

## Workspace crate boundaries (ports)

When crate **A** depends on workspace crate **B**, **A** must not call **B**’s API from command/handler/business modules directly. Use a **crate-local port** so tests can mock **B** without real I/O.

1. Define a **trait** in **A** (`ports.rs` or `ports/`) describing only what **A** needs (consumer-driven surface).
2. Implement it in **`adapters/<b>.rs`** (e.g. wrapping `openpfe_ipc::IpcClient`).
3. Pass the port into logic (`Arc<dyn …>` or generic); unit tests use a **mock adapter**; integration tests may use the real **B**.

**Adapters** are the only modules that `use openpfe_<b>::…` outside `lib.rs` wiring. Keep traits **small** (one concern per port). Normative wire/API detail stays in **B**’s [specification.md](../crates/) — the port is **A**’s narrowed view, not a second spec.

This does **not** remove **B** from `Cargo.toml`; it limits **source** coupling and enables mocks.

### Exceptions

- **B** already exposes the right abstraction — use **B**’s trait or type directly (e.g. `GraphStore` in `openpfe-graph`, `LlmService` in `openpfe-llm`). Do **not** add a duplicate wrapper trait in **A** unless **A** needs a strictly narrower surface.
- **A** is the workspace **composer** (`openpfe-server`) — may call sibling factories and listeners (`api_router`, `IpcListener::bind`, `run_server`) in **wiring** modules only (`main`, `run`, `mount`, `spawn` tasks). Domain rules stay in owning crates; do not re-trait every sibling behind another layer.
- **A** has **no** workspace path dependencies (leaf crates such as `openpfe-ipc`, `openpfe-graph`).
- **One-off wiring** at the binary boundary (e.g. `openpfe --server` → `openpfe_server::run_server`) — a single thin adapter or direct call in `main` is enough; no trait per line of delegation.

### Example layout (`openpfe`)

```
crates/openpfe/src/
  ports.rs           # traits
  adapters/
    ipc.rs           # uses openpfe_ipc
    server.rs        # uses openpfe_server (--server)
  client.rs          # uses ports only
```

See [testing-rust.md](./testing-rust.md) for mock vs integration test expectations.

## Error handling

- Library crates: typed errors (`thiserror` or crate-local enums); avoid `unwrap()`/`expect()` except in tests or proven invariants.
- Binary / server: map errors to user-visible messages; IPC/HTTP responses use stable JSON error shapes once specified in crate `specification.md`.
- Fail closed on lock/socket conflicts (singleton server) — see [openpfe-server/design.md](../crates/openpfe-server/design.md).

## API design

- HTTP handlers live in **`openpfe-ui`** only; export a `Router` (or factory) for `openpfe-server` to mount at `/api/v1`.
- IPC message types: `echo`, `mcp`, `shutdown` only unless [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md) is extended.
- **cwd** = project root: [cross-cutting.md](../cross-cutting.md#project-root-convention).
- **Paths**: normative tables in each owning crate’s `specification.md` — no shared path-helper module.
- **`server.json`**: **`openpfe-server`** — [openpfe-server/specification.md](../crates/openpfe-server/specification.md).
- **`llm.json`** + shared weights: **`openpfe-llm`** — [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md).
- Graph: **`openpfe-graph`**. Config serialization: **`serde_json`** only (no TOML).

## Style and quality

- Rust 2024 edition; `rustfmt` defaults; `clippy` clean for new code.
- Public items: doc comments with purpose and link to `.dev/crates/<crate>/specification.md` when behavior is contractually normative.
- Keep modules small; `mod` per concern (e.g. `ipc/framing.rs`, `routes/graph.rs`).

## Documentation changes

- Behavior that other crates or clients rely on → update `specification.md` (and `requirements.md` if FR-related).
- Implementation choices → `design.md`.

## Related

- [security-rust.md](./security-rust.md) — `cargo audit`, new external crate intake
- [protocols.md](./protocols.md) — IPC/HTTP shapes
- [testing-rust.md](./testing-rust.md) — tests
- [mcp.md](./mcp.md) — MCP handler crate only
