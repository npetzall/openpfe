# Graph database — evaluation

Track research for an **embedded**, **Rust-native** graph store for the problem graph (nodes, dependency edges, component clusters).

**Decided:** `openpfe-graph` is a **separate workspace crate** — see [design.md](../design.md).

**Engine (locked):** **[Grafeo](https://github.com/GrafeoDB/grafeo)** — see [decision.md](../decision.md) (2026-05-25).

---

## Decision (locked 2026-05-25)

| Question | Decision |
|----------|----------|
| **Property graph vs RDF** | **Labeled property graph** (nodes, typed edges, JSON properties). RDF/SPARQL **not** in v1 — contracts live as node/edge properties, not arbitrary triples. |
| **Engine** | **[Grafeo](https://github.com/GrafeoDB/grafeo)** `0.5.42`, features `lpg`, `text-index`, `vector-index`, `hybrid-search`, `parallel`. Adapter: `crates/openpfe-graph`. Mapping: [grafeo/specification.md](../grafeo/specification.md). |
| **Query style (v1)** | **Rust API only** via `GraphStore` trait + Grafeo adapter; no Cypher/GQL/Datalog exposed to HTTP/MCP in v1 (GQL may be used **inside** the adapter for performance). |
| **Concurrency** | **Single writer** (server process); in-process readers only. No multi-process writers. |
| **Backup** | **Directory copy** of `./.openpfe/graph/` (or `store/`) while server stopped after graceful shutdown; Grafeo `backup_full` as documented in spike. |

**Rejected for v1:** Oxigraph (RDF), CozoDB (Datalog-first), SurrealDB (ops/embedded complexity), raw **petgraph** persistence (build cost), SQLite adjacency-only (weak traversals), **IndraDB** (RocksDB/C++), **nanograph** and **SparrowDB** (not selected after shortlist comparison — see [decision.md](../decision.md)).

**Spike program:** [program.md](./program.md) — Grafeo, nanograph, SparrowDB **macOS complete**; engine **locked** per [decision.md](../decision.md). Linux re-run **deferred** (team).

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
| **[Grafeo](https://github.com/GrafeoDB/grafeo)** | **Selected (v1)** | LPG, embedded, BM25/text for S6, Apache-2.0 — [decision.md](../decision.md), [grafeo-outcome.md](./grafeo-outcome.md), [grafeo.md](./grafeo.md) |
| **[nanograph](https://github.com/nanograph/nanograph)** | **Not selected** | Spike pass w/ caveats — [nanograph-outcome.md](./nanograph-outcome.md), [nanograph.md](./nanograph.md) |
| **[SparrowDB](https://github.com/ryaker/SparrowDB)** | **Not selected** | Spike pass w/ caveats — [sparrowdb-outcome.md](./sparrowdb-outcome.md), [sparrowdb.md](./sparrowdb.md) |
| **[indradb](https://github.com/indradb/indradb)** + RocksDB | **Rejected** | C++ RocksDB, sled path not viable — [indradb.md](./indradb.md) |
| [LoraDB](https://github.com/lora-db/lora) | **Watch** | New embedded LPG + vectors; license/community immature — defer |
| [BikoDB](https://github.com/DioCrafts/BikoDB) | **Watch** | Multi-model graph+vector; confirm embeddable API vs server-first before spike |
| [CozoDB](https://github.com/cozodb/cozo) | Rejected v1 — Datalog-first |
| [Oxigraph](https://github.com/oxigraph/oxigraph) | Rejected v1 — RDF |
| [surrealdb](https://surrealdb.com/) | Rejected v1 — embedded/ops complexity |
| **petgraph** + custom persistence | Rejected v1 — implementation cost |
| **SQLite** adjacency + `GraphStore` | **Fallback** if Grafeo implementation fails — always builds; weaker traversals, separate FTS for S6 |

### Spike docs (historical)

| Engine | Document |
|--------|----------|
| Grafeo (winner) | [grafeo.md](./grafeo.md), [grafeo-outcome.md](./grafeo-outcome.md) |
| nanograph | [nanograph.md](./nanograph.md), [nanograph-outcome.md](./nanograph-outcome.md) |
| SparrowDB | [sparrowdb.md](./sparrowdb.md), [sparrowdb-outcome.md](./sparrowdb-outcome.md) |
| IndraDB (closed) | [indradb.md](./indradb.md), [indradb-outcome.md](./indradb-outcome.md) |

---

## Storage placement (decided)

Graph data is **project-local**: **`./.openpfe/graph/store/`** hosts the Grafeo persistent database (e.g. `store.grafeo` and WAL-related files). Backup copies **`./.openpfe/graph/`** when the server is stopped. Not shared in `USER_HOME`.

---

## Related documents

- [decision.md](../decision.md) — **locked engine** (canonical)
- [program.md](./program.md) — v1 spike program
- [grafeo.md](./grafeo.md), [nanograph.md](./nanograph.md), [sparrowdb.md](./sparrowdb.md)
- [indradb-outcome.md](./indradb-outcome.md) — rejection rationale (detailed)
- [indradb.md](./indradb.md) — closed (Fail)
- [architcture.md](../../architcture.md)
- [design.md](../design.md)
- [requirements.md](../requirements.md)
- [cross-cutting.md](../../cross-cutting.md) — data placement index
