# Testing — Rust

**Read when:** adding or running `cargo test`, integration tests against the server/IPC, or CI for workspace crates.

**Normative refs:** crate `requirements.md` files, [openpfe-core/specification.md](../crates/openpfe-core/specification.md) (paths), [openpfe-ipc/specification.md](../crates/openpfe-ipc/specification.md).

## Test types

| Type | Location | Use for |
|------|----------|---------|
| **Unit** | `src/*.rs` with `#[cfg(test)]` mod or `tests/unit/` | Pure functions, framing encode/decode, config merge |
| **Integration** | `tests/*.rs` per crate | IPC round-trip, HTTP routes via `tower::ServiceExt`, graph on temp dir |
| **Workspace e2e** | `tests/` at repo root (if added) | `openpfe` bin spawn, echo, shutdown — few tests, high value |

Prefer testing at the **lowest crate** that owns the behavior; avoid e2e when a library test suffices.

## Fixtures and filesystem

- Use **`tempfile`** (or similar) for project roots; create `.openpfe/` layout per [openpfe-core/specification.md](../crates/openpfe-core/specification.md).
- **Never** run tests against a developer’s real `./.openpfe/` in the repo workspace unless documented as manual.
- Set **cwd** explicitly in integration tests to the temp project root (matches production rule: cwd = project root).

## Async tests

- `#[tokio::test]` for async IPC/HTTP code; current-thread runtime is fine for most integration tests.
- Timeouts on tests that wait for server ready (echo loop) — avoid flaky CI.
- `openpfe-llm`: mock inference or skip heavy model load in CI unless marked `#[ignore]` with feature flag.

## IPC/HTTP testing

- IPC: in-process listener on temp socket path, or test-only socket under temp `.openpfe/server/`.
- HTTP: `axum::Router` + `tower::ServiceExt::oneshot` without binding TCP when possible; use `reqwest` against loopback only when full stack needed.
- Assert JSON envelope `v`, `type`, and required fields (`http_base_url` on echo).

## Singleton and concurrency

- Tests that start a server must respect **pid flock** semantics or use isolated temp dirs so parallel tests do not collide.
- If two tests need a server, use separate temp project directories.

## Quality bar

- Tests should assert **behavior**, not implementation details (avoid testing private fn names).
- One assertion theme per test where possible; descriptive test names (`echo_returns_http_base_url`).
- No `unwrap()` in test helpers without context — prefer `expect("…")` with message.

## CI expectations

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --workspace` when workspace exists.
- Do not download multi-GB models in default CI job.

## Related

- [coding-rust.md](./coding-rust.md)
- [protocols.md](./protocols.md)
- [mcp.md](./mcp.md) — MCP handler tests
