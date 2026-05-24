# sparrowdb

## Need

Embedded labeled property graph with **WAL-backed durability** for [plan 006](../../plans/006-spike-openpfe-graph-sparrowdb.md) — prove SparrowDB can back openpfe’s v1 problem graph (scenarios S1–S6) without RocksDB/C++. Compare against Grafeo and nanograph per [graph-db-evaluation.md](../../crates/openpfe-graph/graph-db-evaluation.md).

## Scope

**`openpfe-graph-spike` only** (throwaway spike crate on branch `spike_db_sparrowdb`). **Not** `openpfe-graph`, `openpfe-server`, or other members until spike outcome and a separate implementation plan.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.1.16` (latest on crates.io at intake; upstream repo tags `0.1.22` — not published yet) |
| **License** | **MIT** |
| **MSRV** | Workspace **edition 2024** / stable toolchain; confirm against crate `rust-version` on crates.io before production intake |
| **Direct deps** | `sparrowdb`, `sparrowdb-execution` (query result / `Value` types in adapter) |

```toml
sparrowdb = "0.1.16"
sparrowdb-execution = "0.1.16"
```

**Explicitly not used in spike:** `sparrowdb-server`, `sparrowdb-cli`, `sparrowdb-mcp`, `sparrowdb-python`, `sparrowdb-node`, Bolt/HTTP server crates.

**Cypher:** Used **only** inside the spike adapter (not exposed on HTTP/MCP). Product v1 remains Rust `GraphStore` API.

## Trade-off

- **Adopt for spike:** Pure-Rust storage narrative, directory-backed store under `./.openpfe/graph/`, native property text index (`CONTAINS`) and optional `CALL db.index.fulltext.*` for S6.
- **Cost:** Young crate, fast version churn, Cypher surface area vs narrow Rust API; some mutations need labeled `DELETE` / per-rel-type edge removal (documented in spike adapter).
- **Re-implement:** Out of scope (petgraph + custom persistence rejected in evaluation).

## Transitive deps (notable)

Workspace crates pulled via `sparrowdb` (no separate intake unless policy expands):

| Crate | Role |
|-------|------|
| `sparrowdb-storage` | WAL, node/edge store, CSR, fulltext index files |
| `sparrowdb-catalog` | Labels / rel types |
| `sparrowdb-cypher` | Parser (used by `GraphDb::execute`) |
| `sparrowdb-execution` | Query engine |
| `sparrowdb-common` | `NodeId`, errors |
| `bincode` 1.3.3 | Transitive via `sparrowdb-execution` (RUSTSEC-2025-0141 unmaintained) |
| `clap`, `tracing`, `roaring`, `memmap2`, `chacha20poly1305`, … | CLI/crypto/index support in default `sparrowdb` crate graph |

**C++ toolchain:** Not required for default `sparrowdb` build (acceptance criterion for spike).

## Alternatives considered

- **Grafeo** — macOS spike pass with in-engine BM25; [spike-grafeo.md](../../crates/openpfe-graph/spike-grafeo.md).
- **nanograph** — parallel shortlist; stronger schema/search story, heavier Arrow/Lance stack.
- **IndraDB + RocksDB** — rejected; [indradb-outcome.md](../../crates/openpfe-graph/indradb-outcome.md).
- **SQLite adjacency** — evaluation fallback if all engine spikes fail.
