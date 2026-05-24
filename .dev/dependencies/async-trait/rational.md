# async-trait

## Need

Object-safe async traits for `ProjectControl` port (`dyn ProjectControl` in handlers).

## Scope

`openpfe` binary crate.

## Trade-off

Standard approach until native async traits in dyn context are sufficient for our MSRV.

## Alternatives considered

- Generic-only handlers — harder to wire mocks in `main` and tests.
- `async_fn_in_trait` without dyn — rejected for mock injection ergonomics.
