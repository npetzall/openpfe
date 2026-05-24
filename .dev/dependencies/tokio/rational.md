# tokio

## Need

Async runtime for IPC client operations (echo retry loop, MCP stdio bridge) per [openpfe/design.md](../../crates/openpfe/design.md).

## Scope

`openpfe` binary crate (v1); `openpfe-ipc` and `openpfe-server` for UDS (`net` feature).

## Feature note (2026-05-23)

Workspace `tokio` adds **`net`** for `UnixListener` / `UnixStream` in `openpfe-ipc` adapters only.

## Trade-off

Workspace standard per [architcture.md](../../architcture.md). Re-implementing an executor is out of scope.

## Alternatives considered

- `async-std` — not the workspace stack.
- Blocking-only client — conflicts with IPC/async bridge design.
