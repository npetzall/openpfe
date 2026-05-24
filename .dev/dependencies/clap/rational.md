# clap

## Need

CLI parsing for `openpfe` commands and global flags per [openpfe/specification.md](../../crates/openpfe/specification.md).

## Scope

`openpfe` binary crate only.

## Trade-off

Mature derive API; manual argv parsing is error-prone for subcommands and env overrides.

## Alternatives considered

- `argh` — smaller but less ergonomic for subcommands.
- Manual parsing — rejected for maintainability.
