# fd-lock

## Need

Exclusive non-blocking advisory flock on `./.openpfe/server/pid` per [openpfe-server/design.md](../../crates/openpfe-server/design.md).

## Scope

`openpfe-server` singleton lock only.

## Trade-off

Cross-platform flock on an open file descriptor; avoids hand-rolled `libc::flock` in product code.

## Alternatives considered

- **libc::flock** — already in workspace but duplicates error handling across platforms.
- **fs2** — less common in this repo’s documented stack.
