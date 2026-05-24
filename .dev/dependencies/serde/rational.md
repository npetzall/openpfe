# serde

## Need

Serialize and deserialize IPC envelope structs (`v`, `type`, `id`, `payload`) per [openpfe-ipc/specification.md](../../crates/openpfe-ipc/specification.md).

## Scope

`openpfe-ipc` library crate (framing body only; MCP JSON stays opaque `serde_json::Value` in `payload`).

## Trade-off

De-facto Rust JSON/struct mapping; manual parsing is error-prone for envelope fields and versioning.

## Alternatives considered

- `serde_json::Value` only — loses typed `type` and `v` validation at the boundary.
- `miniserde` — not workspace-aligned.
