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
| `openpfe-core`, `openpfe-graph` | No direct sockets | Public APIs sync where practical |
| `openpfe-llm` | `spawn_blocking` for inference inside server runtime | — |

Do not pull HTTP or UDS into `openpfe-core` or `openpfe-graph`.

## Dependencies

- Obey [dependency rules](../workspace-crates.md#dependency-rules-normative); run `cargo tree` mentally before adding an edge.
- Workspace dependencies: centralize versions in the root `Cargo.toml` `[workspace.dependencies]` when the workspace exists.
- Prefer std + ecosystem crates already chosen in architecture (axum, hyper, tokio, tower, tower-http) — do not introduce a second HTTP stack.

## Error handling

- Library crates: typed errors (`thiserror` or crate-local enums); avoid `unwrap()`/`expect()` except in tests or proven invariants.
- Binary / server: map errors to user-visible messages; IPC/HTTP responses use stable JSON error shapes once specified in crate `specification.md`.
- Fail closed on lock/socket conflicts (singleton server) — see [openpfe-server/design.md](../crates/openpfe-server/design.md).

## API design

- HTTP handlers live in **`openpfe-ui`** only; export a `Router` (or factory) for `openpfe-server` to mount at `/api/v1`.
- IPC message types: `echo`, `mcp`, `shutdown` only unless [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md) is extended.
- Domain types and path helpers live in **`openpfe-core`**; graph operations in **`openpfe-graph`**.
- Config merge: deep-merge tables; replace scalar arrays; merge `[[models.catalog]]` by `id` — [openpfe-core/specification.md](../crates/openpfe-core/specification.md#merge-semantics-v1).

## Style and quality

- Rust 2024 edition; `rustfmt` defaults; `clippy` clean for new code.
- Public items: doc comments with purpose and link to `.dev/crates/<crate>/specification.md` when behavior is contractually normative.
- Keep modules small; `mod` per concern (e.g. `ipc/framing.rs`, `routes/graph.rs`).

## Documentation changes

- Behavior that other crates or clients rely on → update `specification.md` (and `requirements.md` if FR-related).
- Implementation choices → `design.md`.

## Related

- [protocols.md](./protocols.md) — IPC/HTTP shapes
- [testing-rust.md](./testing-rust.md) — tests
- [mcp.md](./mcp.md) — MCP handler crate only
