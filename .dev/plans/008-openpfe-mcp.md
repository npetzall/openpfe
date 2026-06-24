# Plan 008: `openpfe-mcp` (stub handler)

**Status:** Complete (2026-06-06).

**Read when:** implementing the minimal **`McpHandler`** required for `AppState` wiring in the 008 LLM slice. Full graph MCP tools are **out of scope** here — deferred to a later plan.

**Assumes:** [007-openpfe-graph.md](./007-openpfe-graph.md) **Complete**. [008-intake.md](./008-intake.md) **Complete** (no new externals in this plan).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-mcp/specification.md](../crates/openpfe-mcp/specification.md) | Transport, tool table (implement later), JSON-RPC errors |
| [openpfe-mcp/design.md](../crates/openpfe-mcp/design.md) | `McpHandler::handle_jsonrpc`, `Arc<GraphStore>` state |
| [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md#appstate-v1) | `AppState.mcp` field |
| [guidelines/mcp.md](../guidelines/mcp.md) | MCP testing conventions |

## Prerequisites

- [x] [007-openpfe-graph.md](./007-openpfe-graph.md) — `GraphStore` / `GrafeoGraphStore` available
- [x] [008-intake.md](./008-intake.md) — **Complete** (no MCP-specific deps; unblocks server/UI series)

## Goal

Ship a **minimal `McpHandler`** so `openpfe-server` and `openpfe-ui` can construct valid **`AppState`** without implementing the full v1 tool surface. Handler holds `Arc<Mutex<dyn GraphStore + Send>>` (or concrete store behind same lock) and returns well-formed JSON-RPC errors for all methods until the dedicated MCP plan lands.

**No new external crates** in this plan (`rmcp` deferred).

## Public API (this plan)

```rust
pub struct McpHandler {
    // graph: Arc<Mutex<dyn GraphStore + Send>>
}

impl McpHandler {
    pub fn new(graph: Arc<Mutex<dyn GraphStore + Send>>) -> Self;
    /// Single request or batch array in/out — same contract as IPC `type: mcp` payload.
    pub fn handle_jsonrpc(&self, payload: serde_json::Value) -> serde_json::Value;
}
```

## Tasks

### Implementation

#### 1. Crate dependencies

- [x] `crates/openpfe-mcp/Cargo.toml` — `openpfe-graph`, workspace `serde`, `serde_json`
- [x] Remove `stub()` from `lib.rs`

#### 2. Handler stub

- [x] `handler.rs` — `McpHandler` stores graph handle (unused in stub except `Clone`/`Send` wiring proof)
- [x] `handle_jsonrpc`:
  - Valid JSON-RPC shape → response with error **`-32601`** (method not found) for `tools/call`, `resources/read`, etc.
  - `initialize` → minimal compliant result; `tools/list` → empty array (Debug panel can connect)
- [x] Batch: array in → array out, per MCP JSON-RPC batch rules
- [x] Malformed input → JSON-RPC `-32600` (invalid request shape)

#### 3. Tests

- [x] Unit: single `tools/list` or `initialize` roundtrip returns JSON object with `jsonrpc: "2.0"`
- [x] Unit: unknown method → `-32601`
- [x] Unit: batch of two requests → two responses

## Acceptance criteria

- [x] All task boxes `[x]`
- [x] `cargo test -p openpfe-mcp` passes
- [x] `cargo clippy -p openpfe-mcp -- -D warnings` clean
- [x] `McpHandler` constructible with `GrafeoGraphStore` behind `Mutex`
- [x] No `rmcp` / no graph tool implementations yet
- [x] Plan **Status** → `Complete (2026-06-06)`

## Out of scope

- Full tools/resources from [specification.md](../crates/openpfe-mcp/specification.md) — later **`009-openpfe-mcp`** (or phase 3 plan TBD)
- `rmcp` dependency intake
- IPC stdio bridge (`openpfe` binary) — already phase 1 stub path
- HTTP route `POST /debug/mcp` — [008-openpfe-ui](./008-openpfe-ui.md) (optional mount in LLM slice; stub handler must exist first)

## Next

[008-openpfe-ui](./008-openpfe-ui.md) — depends on `McpHandler` type for `AppState`.
