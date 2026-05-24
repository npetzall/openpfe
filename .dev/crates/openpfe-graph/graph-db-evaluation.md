# Graph database — evaluation (draft)

Track research for an **embedded**, **Rust-native** graph store for the problem graph (nodes, dependency edges, component clusters).

**Decided:** `openpfe-graph` is a **separate workspace crate** — see [design.md](./design.md).

## Decision (2026-05-24 update)

| Question | Decision |
|----------|----------|
| **Property graph vs RDF** | **Labeled property graph** (nodes, typed edges, JSON properties). RDF/SPARQL **not** in v1 — contracts live as node/edge properties, not arbitrary triples. |
| **Engine** | **No default yet.** [IndraDB](https://github.com/indradb/indradb) + RocksDB **rejected** (see below). Run spikes: [Grafeo](./spike-grafeo.md), [nanograph](./spike-nanograph.md), [SparrowDB](./spike-sparrowdb.md). |
| **Query style (v1)** | **Rust API only** via `GraphStore` trait + engine adapter; no Cypher/GQL/Datalog exposed to HTTP/MCP in v1 (engines may use query languages **inside** the spike adapter only). |
| **Concurrency** | **Single writer** (server process); in-process readers only. No multi-process writers. |
| **Backup** | **Directory copy** of `./.openpfe/graph/` while server stopped (or after graceful shutdown). Exact files depend on winning engine (folder-based store preferred). |

**Rejected for v1:** Oxigraph (RDF), CozoDB (Datalog-first), SurrealDB (ops/embedded complexity), raw **petgraph** persistence (build cost), SQLite adjacency-only (weak traversals), **IndraDB** (RocksDB/C++ + no viable Rust disk backend — [spike-indradb.md](./spike-indradb.md)).

**Spike before merge:** [graph-db-spike.md](./graph-db-spike.md) scenarios S1–S6 on macOS and Linux for each shortlist engine.

There is **no** legacy markdown problem-tree format; the graph is created and stored in the embedded DB from the start.

---

## IndraDB — rejected (2026-05-24)

Full write-up: **[indradb-outcome.md](./indradb-outcome.md)**.

| Issue | Detail |
|-------|--------|
| **Persistence** | Production path is `rocksdb-datastore` → `librocksdb-sys` (C++). Failed to build in spike environment; not “Rust-only” for disk. |
| **Alternatives in ecosystem** | `MemoryDatastore` (msgpack file) does not match `./.openpfe/graph/store/` directory backup story. `indradb-sled` 0.1.0 is stale vs IndraDB 5.x — not production-ready. |
| **License** | MPL-2.0 on `indradb-lib` 5.x. |
| **Verdict** | **No** — [.dev/dependencies/indradb/verdict.md](../../dependencies/indradb/verdict.md), plan [006](../plans/006-openpfe-graph-indradb-spike.md) cancelled. |

---

## Requirements (from product)

| Criterion | Target |
|-----------|--------|
| Embedding | No external server process; opens with project server |
| Location | Under `./.openpfe/` (project-scoped), not `USER_HOME` |
| Durability | Survive server crash without corrupting graph |
| Graph model | Directed edges, node properties, cluster/group membership |
| Rust integration | Prefer **pure Rust** build (no required C++ toolchain for default backend) |
| Scale | Thousands of nodes per project (not internet-scale) |
| Licensing | Compatible with openpfe distribution (Apache-2.0 or MIT preferred) |
| **S6 (lexical search)** | “Problem already exists?” — engine-native FTS/BM25 strongly preferred |

---

## Candidates (evaluation record)

| Candidate | Verdict | Notes |
|-----------|---------|--------|
| **[Grafeo](https://github.com/GrafeoDB/grafeo)** | **Spike (macOS pass w/ caveats)** | LPG, embedded, BM25/text for S6, Apache-2.0 — [grafeo-outcome.md](./grafeo-outcome.md), [spike-grafeo.md](./spike-grafeo.md) |
| **[nanograph](https://github.com/nanograph/nanograph)** | **Spike (macOS pass w/ caveats)** | Folder store, `.pg` schema, S6 BM25 + S6+ `find_similar`; export-read latency + protoc/heavy deps — [nanograph-outcome.md](./nanograph-outcome.md), [spike-nanograph.md](./spike-nanograph.md) |
| **[SparrowDB](https://github.com/ryaker/SparrowDB)** | **Spike (parallel)** | Embedded, WAL + crash recovery, Cypher internally, MIT, pure-Rust storage — [spike-sparrowdb.md](./spike-sparrowdb.md) |
| **[indradb](https://github.com/indradb/indradb)** + RocksDB | **Rejected** | C++ RocksDB, sled path not viable — [spike-indradb.md](./spike-indradb.md) |
| [LoraDB](https://github.com/lora-db/lora) | **Watch** | New embedded LPG + vectors; license/community immature — defer unless shortlist spikes fail |
| [BikoDB](https://github.com/DioCrafts/BikoDB) | **Watch** | Multi-model graph+vector; confirm embeddable API vs server-first before spike |
| [CozoDB](https://github.com/cozodb/cozo) | Rejected v1 — Datalog-first |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Rejected v1 — RDF |
| [surrealdb](https://surrealdb.com/) | Rejected v1 — embedded/ops complexity |
| **petgraph** + custom persistence | Rejected v1 — implementation cost |
| **SQLite** adjacency + `GraphStore` | **Fallback** if all engine spikes fail — always builds; weaker traversals, separate FTS for S6 |

### Recommended spike order

1. **Grafeo** — already documented; best known fit for LPG + S6.
2. **nanograph** — strongest alternative for **S6** and “one folder under `.openpfe/graph/`”; validate schema model vs PFE JSON properties.
3. **SparrowDB** — strongest alternative for **durability/WAL** narrative without RocksDB; keep Cypher out of product API.

Compare measurements and audit across all three before locking [specification.md](./specification.md).

### Pre-merge spike docs

| Engine | Document |
|--------|----------|
| Grafeo | [spike-grafeo.md](./spike-grafeo.md) |
| nanograph | [spike-nanograph.md](./spike-nanograph.md) |
| SparrowDB | [spike-sparrowdb.md](./spike-sparrowdb.md) |
| IndraDB (closed) | [spike-indradb.md](./spike-indradb.md) |

---

## Storage placement (decided)

Graph data is **project-local**: under `./.openpfe/graph/` (exact subdirectory name depends on winning engine — e.g. `store/` or engine-native folder). Not shared in `USER_HOME`.

---

## Related documents

- [graph-db-spike.md](./graph-db-spike.md) — v1 spike program
- [spike-grafeo.md](./spike-grafeo.md), [spike-nanograph.md](./spike-nanograph.md), [spike-sparrowdb.md](./spike-sparrowdb.md)
- [indradb-outcome.md](./indradb-outcome.md) — rejection rationale (detailed)
- [spike-indradb.md](./spike-indradb.md) — closed (Fail)
- [architcture.md](../../architcture.md)
- [design.md](./design.md)
- [requirements.md](./requirements.md)
- [cross-cutting.md](../../cross-cutting.md) — data placement index
