# serde_json

## Need

Opaque JSON-RPC payloads on `ProjectControl::send_mcp` (port contract in plan 002).

## Scope

`openpfe` binary crate (MCP bridge); aligns with workspace `serde_json` usage elsewhere.

## Trade-off

`Value` type for opaque MCP forwarding without defining protocol types in the binary.

## Alternatives considered

- `String` payloads — loses structure validation at boundaries.
- Full MCP types in binary — rejected; semantics live in `openpfe-mcp`.
