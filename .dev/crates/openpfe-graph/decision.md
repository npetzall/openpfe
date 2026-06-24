# openpfe-graph — engine decision

**Status:** **Locked** — v1 embedded engine is **[Grafeo](https://github.com/GrafeoDB/grafeo)** `0.5.42`.

**Date:** 2026-05-25

**Read when:** answering “which graph engine?” or wiring `openpfe-graph` dependencies.

---

## Decision

| Item | Value |
|------|--------|
| **Engine** | **Grafeo** — see [grafeo/requirements.md](./grafeo/requirements.md) for version, features, intake |
| **Mapping** | [grafeo/specification.md](./grafeo/specification.md) |
| **Adapter** | `GrafeoGraphStore` — [grafeo/design.md](./grafeo/design.md) |
| **Query surface (product)** | **`GraphStore` Rust trait only** — no Cypher/GQL on HTTP/MCP |
| **Persistence** | `./.openpfe/graph/store/` |

---

## Why Grafeo

Shortlist spikes on macOS (2026-05-24): Grafeo, nanograph, and SparrowDB **passed with caveats**; IndraDB **failed**.

**Grafeo selected because:**

1. **Best fit for v1 LPG + S6/S6+** — in-engine BM25 (`text-index`) and hybrid/vector search for lexical + semantic `find_similar` without a separate FTS crate.
2. **Rust CRUD API** maps cleanly to `GraphStore`; spike did not need product-facing Cypher/GQL.
3. **Dynamic JSON properties** on nodes/edges — lower schema friction than nanograph’s `.pg` model.
4. **Acceptable trade-offs** — `bincode` unmaintained advisory accepted; young crate / MSRV 1.91.1 documented; Linux spike deferred.

---

## Why not the others

| Engine | Verdict | Why not v1 |
|--------|---------|------------|
| **nanograph** | Not selected | Heavy Lance/Arrow/`protoc` stack; fixed `.pg` schema vs PFE JSON — [spike/nanograph-outcome.md](./spike/nanograph-outcome.md) |
| **SparrowDB** | Not selected | Weaker S6 (`CONTAINS` vs BM25); slow bounded `subgraph` in spike — [spike/sparrowdb-outcome.md](./spike/sparrowdb-outcome.md) |
| **IndraDB** | Rejected | RocksDB/C++ build failed; not “Rust-only” for disk — [spike/indradb-outcome.md](./spike/indradb-outcome.md) |

Full evaluation and spike evidence: [spike/evaluation.md](./spike/evaluation.md), [spike/program.md](./spike/program.md).

---

## Related

- [design.md](./design.md) — crate architecture
- [grafeo/](./grafeo/) — engine adapter details
- [spike/](./spike/) — spike program and outcomes
