# Graph database — evaluation (draft)

Track research for an **embedded**, **Rust-native** graph store for the problem graph (nodes, dependency edges, component clusters).

**Decided:** `openpfe-graph` is a **separate workspace crate** (not merged into `openpfe-core`) — see [design.md](./design.md).

## Decision (2026-05)

| Question | Decision |
|----------|----------|
| **Property graph vs RDF** | **Labeled property graph** (nodes, typed edges, JSON properties). RDF/SPARQL **not** in v1 — contracts live as node/edge properties, not arbitrary triples. |
| **Engine** | **[IndraDB](https://github.com/indradb/indradb)** with **`rocksdb-datastore`** feature — embedded directory under `./.openpfe/graph/store/`. |
| **Query style (v1)** | **Rust API only** via `GraphStore` trait + IndraDB adapter; no Cypher/Datalog exposed to HTTP/MCP in v1. |
| **Concurrency** | **Single writer** (server process); readers via store API on same process (IndraDB/RocksDB snapshot reads). No multi-process writers. |
| **Backup** | **Directory copy** of `./.openpfe/graph/` while server stopped (or after graceful shutdown). RocksDB-backed tree — not a single SQLite file. |

**Rejected for v1:** Oxigraph (RDF overhead), CozoDB (Datalog-first, heavier embedding story), SurrealDB (server/embedded mode complexity), raw **petgraph** persistence (build cost), SQLite adjacency-only (weak traversals).

**Spike before merge:** prove IndraDB 5.x + RocksDB on macOS/Linux, subgraph traversal for context shield, compile time acceptable.

There is **no** legacy markdown problem-tree format; the graph is created and stored in the embedded DB from the start.

---

## Requirements (from product)

| Criterion | Target |
|-----------|--------|
| Embedding | No external server process; opens with project server |
| Location | Under `./.openpfe/` (project-scoped), not `USER_HOME` |
| Durability | Survive server crash without corrupting graph |
| Graph model | Directed edges, node properties, cluster/group membership |
| Rust integration | Prefer pure Rust or stable FFI; fits workspace build |
| Scale | Thousands of nodes per project (not internet-scale) |
| Licensing | Compatible with openpfe distribution |

---

## Candidates (evaluation record)

| Candidate | Verdict |
|-----------|---------|
| **[indradb](https://github.com/indradb/indradb)** + RocksDB | **Selected v1** — property graph, embedded, Rust API |
| [CozoDB](https://github.com/cozodb/cozo) | Rejected v1 — Datalog-first; heavier fit for PFE |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Rejected v1 — RDF; contracts modeled as properties instead |
| [surrealdb](https://surrealdb.com/) | Rejected v1 — embedded story / ops complexity |
| **petgraph** + custom persistence | Rejected v1 — implementation cost |
| **SQLite** adjacency | Rejected v1 — weak traversal ergonomics |

### Pre-merge spike (IndraDB)

- [ ] Embedded open on `./.openpfe/graph/store/` (macOS + Linux)
- [ ] Subgraph traversal within shield limits
- [ ] Acceptable compile time / binary size for workspace
- [ ] License confirmed (Apache-2.0 expected)
- [ ] Seed: problem node + `depends_on` + `member_of` cluster

---

## Storage placement (decided)

Graph data is **project-local**: `./.openpfe/graph/store/` (IndraDB RocksDB files). Not shared in `USER_HOME`.

---

## Related documents

- [architcture.md](../../architcture.md)
- [design.md](./design.md)
- [requirements.md](./requirements.md)
- [cross-cutting.md](../../cross-cutting.md) — data placement index
