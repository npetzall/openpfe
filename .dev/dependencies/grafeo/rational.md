# grafeo

## Need

Embedded labeled property graph engine for [plan 006](../../plans/006-spike-openpfe-graph-grafeo.md) — prove Grafeo can back openpfe’s v1 problem graph (scenarios S1–S6) before phase 2 `openpfe-graph` implementation. Required for comparison against provisional IndraDB choice in [graph-db-evaluation.md](../../crates/openpfe-graph/graph-db-evaluation.md).

## Scope

**`openpfe-graph-spike` only** (throwaway spike crate). **Not** `openpfe-graph`, `openpfe-server`, or other members until spike outcome and a separate implementation plan.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.5.42` (crates.io latest in spike window) |
| **License** | Apache-2.0 |
| **MSRV** | Crate docs cite **1.91.1**; workspace `rust-toolchain.toml` uses **stable** (currently ≥ MSRV) |
| **Features (intake)** | `default-features = false`, `features = ["lpg", "text-index"]` — S1–S5 + S6 BM25 (same crate; enabled after intake approval) |

**Explicitly not enabled:** `rdf`, `enterprise`, `embed` (ONNX), `server`, `grafeo-mcp`.

## Trade-off

- **Adopt** for spike: validates S6 lexical search in-engine vs IndraDB + sidecar FTS.
- **Cost:** Large transitive graph even with `lpg` only; young crate / fast version churn; default `embedded` feature set is heavier than openpfe needs for v1.
- **Re-implement:** Out of scope for spike (petgraph + custom persistence rejected in evaluation).

## Alternatives considered

- **IndraDB + RocksDB** — provisional v1; parallel [spike-indradb.md](../../crates/openpfe-graph/spike-indradb.md).
- **IndraDB only, defer search** — acceptable if Grafeo spike fails S6 or audit/build bar too high.
- **SQLite adjacency / custom store** — evaluation rejects for v1 ergonomics.
