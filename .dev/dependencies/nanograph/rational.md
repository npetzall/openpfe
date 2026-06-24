# nanograph

## Need

Embedded typed property graph for [plan 006](../../plans/006-spike-openpfe-graph-nanograph.md) — validate nanograph as the primary **Grafeo alternative**, especially **S6** engine-native BM25/FTS and folder-based storage under `./.openpfe/graph/`.

## Scope

**`openpfe-graph-spike` only** (throwaway spike crate on `spike_db_nanograph`). **Not** `openpfe-graph` or server crates until evaluation locks an engine.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `1.3.0` (crates.io; `1.3.1` yanked) |
| **License** | MIT |
| **MSRV** | Upstream cites **1.91+** (README); crates.io build notes **1.94.1** + **`protoc`** — workspace uses **stable** |
| **Features** | Default crate features only — no optional ML/embed stacks for baseline spike |

**Build:** `protoc` required for transitive gRPC/protobuf crates (document in spike Results).

## Trade-off

- **Adopt for spike:** Strong S6 story (Lance text indexes, `bm25()` in query language); one-folder persistence; pure Rust library API (`nanograph::store::database::Database`).
- **Cost:** Heavy transitive stack (Lance, Arrow, DataFusion); **schema-as-code** (`.pg`) vs PFE ad-hoc JSON — spike must document mapping friction.
- **Re-implement:** Out of scope.

## Alternatives considered

- **Grafeo** — macOS spike complete ([spike/grafeo-outcome.md](../../crates/openpfe-graph/spike/grafeo-outcome.md)); nanograph compares on S6 + schema model.
- **SparrowDB** — parallel shortlist spike.
- **IndraDB** — rejected.
