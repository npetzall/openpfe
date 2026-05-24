# thiserror

## Need

Crate-local `IpcError` with protocol vs I/O distinction per plan 003 and [coding-rust.md](../../guidelines/coding-rust.md).

## Scope

`openpfe-ipc` library crate only.

## Trade-off

Standard `#[derive(Error)]` ergonomics; manual `Display`/`Error` impls duplicate `openpfe` `ClientError` style without benefit here.

## Alternatives considered

- Manual error enum — acceptable but inconsistent with workspace direction for library crates.
- `anyhow` — too opaque for a wire library consumed by adapters mapping to `ClientError`.
