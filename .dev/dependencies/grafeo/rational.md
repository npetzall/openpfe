# grafeo

## Need

Embedded labeled property graph engine for [plan 006](../../plans/006-spike-openpfe-graph-grafeo.md). Spike proved S1–S6; **v1 engine locked to Grafeo** (2026-05-25) — [decision.md](../../crates/openpfe-graph/decision.md). Phase 2 **`openpfe-graph`** product intake completed per [plan 007](../../plans/007-openpfe-graph.md) (2026-06-06).

## Scope

**`openpfe-graph`** (product adapter). Spike crate `openpfe-graph-spike` was throwaway evidence only — not a workspace member in v1.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.5.42` (crates.io latest in spike window) |
| **License** | Apache-2.0 |
| **MSRV** | Crate docs cite **1.91.1**; workspace `rust-toolchain.toml` uses **stable** (currently ≥ MSRV) |
| **Features (product v1)** | `default-features = false`, `features = ["lpg", "text-index", "vector-index", "hybrid-search", "parallel"]` |

**Explicitly not enabled:** `embedded`, `ai`, `embed` (ONNX), `rdf`, `enterprise`, `server`, `grafeo-mcp`.

## Trade-off

- **Adopt** for spike: validates S6 lexical search in-engine vs IndraDB + sidecar FTS.
- **Cost:** Large transitive graph even with `lpg` only; young crate / fast version churn; default `embedded` feature set is heavier than openpfe needs for v1.
- **Re-implement:** Out of scope for spike (petgraph + custom persistence rejected in evaluation).

## Alternatives considered

- **IndraDB + RocksDB** — provisional v1; parallel [spike/indradb.md](../../crates/openpfe-graph/spike/indradb.md).
- **IndraDB only, defer search** — acceptable if Grafeo spike fails S6 or audit/build bar too high.
- **SQLite adjacency / custom store** — evaluation rejects for v1 ergonomics.
