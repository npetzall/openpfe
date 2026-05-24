# libc

## Need

Unix `setsid()` in `pre_exec` when spawning detached `openpfe --server` per [openpfe/design.md](../../crates/openpfe/design.md).

## Scope

`openpfe` binary crate, `cfg(unix)` spawn adapter only.

## Trade-off

Minimal `unsafe` for one syscall vs heavier `nix` for a single call.

## Alternatives considered

- `nix` — broader API than needed for v1 spawn.
- No session detach — rejected; server must not hold parent tty.
