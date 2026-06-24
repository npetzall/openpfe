# SparrowDB spike — outcome summary

**Read when:** reviewing SparrowDB spike evidence. **Not selected for v1** — engine locked to **Grafeo** ([decision.md](../decision.md)).

**Implementation:** `crates/openpfe-graph-spike/` (throwaway; not product `openpfe-graph`).  
**Detailed checklist / measurements:** [sparrowdb.md](./sparrowdb.md).  
**Intake:** [.dev/dependencies/sparrowdb/](../../dependencies/sparrowdb/).

| Field | Value |
|-------|--------|
| **Date** | 2026-05-24 |
| **Platform tested** | macOS (Darwin 25.3, arm64) |
| **Linux** | Skipped (team decision, same as Grafeo spike) |
| **SparrowDB version** | `0.1.16` (crates.io; GitHub at 0.1.22 not published at intake) |
| **Direct deps** | `sparrowdb`, `sparrowdb-execution` |
| **Features used** | Default `sparrowdb` crate graph; Cypher via `GraphDb::execute`; `WriteTx` for fulltext index maintenance |
| **Features explicitly not used** | `sparrowdb-server`, `sparrowdb-cli`, `sparrowdb-mcp`, Bolt/HTTP, encrypted open |
| **Recommendation** | **Pass with caveats** — viable for v1 graph + WAL durability; weaker S6 and adapter ergonomics than Grafeo |
| **vs Grafeo** | **Not selected** — Grafeo locked for in-engine BM25 and adapter ergonomics |

---

## Executive summary

SparrowDB meets the **required** spike bar (S1–S6) on macOS as an **embedded LPG** with directory-backed persistence, WAL, `GraphStore`-shaped operations, property-level lexical search (`CONTAINS`), and directory backup. **S6+ stretch** (`find_similar` with lexical, semantic-proxy, and structural legs) was implemented and tested.

**Caveats:** adapter is **Cypher-string-heavy** (reserved property name `id` → `node_id`; per-property `SET`; labeled `DELETE`; explicit per-rel-type edge removal before node delete); **very slow** bounded `subgraph` on ~800-node fixture in debug (~3+ min); S6 is not Grafeo-grade BM25; transitive `bincode` unmaintained; crates.io lags upstream; Linux not re-run.

**Verdict:** Strongest differentiator is **pure-Rust + WAL** without RocksDB/C++. Weakest vs shortlist is **query adapter cost** and **S6/search latency at scale**.

---

## Dependency & supply chain

| Item | Detail |
|------|--------|
| **Scope** | `openpfe-graph-spike` only — `openpfe-graph` product crate unchanged |
| **Intake** | Accepted 2026-05-24 — [verdict.md](../../dependencies/sparrowdb/verdict.md) |
| **`cargo audit`** | Exit 0; allowed warning: `bincode` 1.3.3 unmaintained (RUSTSEC-2025-0141) via `sparrowdb-execution` |
| **Transitive graph** | ~140 normal deps from spike crate (`sparrowdb-storage`, `sparrowdb-cypher`, `sparrowdb-execution`, `clap`, …) |
| **License** | MIT |
| **C++ toolchain** | **Not required** for default build (spike acceptance criterion met) |

---

## Schema & storage mapping

| PFE concept | SparrowDB mapping |
|-------------|-------------------|
| Node `type` (`problem`, `cluster`, …) | Cypher **label** |
| Stable UUID | Property **`node_id`** (`id` is reserved in Cypher) |
| Type for reads | Property **`pfe_type`** (mirrors PFE `type`) |
| Edge `type` (`depends_on`, `member_of`, `interfaces`) | Relationship **name** |
| JSON properties | Node/edge properties via `CREATE` / sequential `SET` |
| Target product path | `./.openpfe/graph/` → spike used `<temp>/…/openpfe-graph-spike/sparrowdb/` |
| On-disk | Directory: `wal/`, catalog/CSR/column files, optional fulltext index files |

See [README](../../../crates/openpfe-graph-spike/README.md).

---

## Scenarios tested

Normative definitions: [program.md](./program.md). Tests: `crates/openpfe-graph-spike/tests/sparrowdb_spike.rs`, `sparrowdb_spike_s6plus.rs`.

| ID | Product scenario | Spike test(s) | Result (macOS) |
|----|------------------|---------------|----------------|
| **S1** | Project problem space — CRUD, UUID, JSON props, persistence | `s1_lifecycle_and_reopen_roundtrip` | **Pass** — open, seed, close, reopen; properties round-trip |
| **S2** | User curation — update props, edges; delete node | `s2_curation_update_and_delete` | **Pass** — per-property `SET`; delete removes incident edges then labeled `DELETE n` |
| **S3** | Architecture lens — `interfaces` contract on bounded read | `s3_interfaces_contract_in_subgraph` | **Pass** — `contract_body`, `version`, etc. on `interfaces` edges in subgraph |
| **S4** | MCP context shield — bounded `subgraph(cluster_id)` | `s4_subgraph_respects_caps` | **Pass** — `max_depth=3`, `max_nodes=200` on ~805-node fixture; &lt; 500 nodes; **slow** (~213 s debug) |
| **S5** | DAG validation — acyclic `depends_on` | `s5_acyclic_validation` | **Pass** — clean DAG empty; injected cycle returns path containing `p1` |
| **S6** | Similar / existing problem (lexical) | `s6_search_duplicate_title` | **Pass** — `CONTAINS` on title/description; duplicate in top 3; unrelated not in top 2 |
| **S6+** | Search & compare (stretch) | `sparrowdb_spike_s6plus.rs` (5 tests + 1 ignored) | **Pass** — see [S6+ subsection](#s6-stretch-search--compare) |

### Additional integration tests (supporting scenarios)

| Test | Covers |
|------|--------|
| `durability_close_reopen_counts` | Durability — node count stable after close + reopen |
| `backup_directory_copy` | Backup — copy `sparrowdb/` tree; reopen and read |
| `list_nodes_by_type_and_cluster` | Filters — `list_nodes` by `type` + `cluster_id` |

### S6+ stretch (search & compare)

| Stretch check | Test | Result |
|---------------|------|--------|
| Lexical: P-lex-2 top 3; P-lex-3 not top 5 | `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` | **Pass** |
| Semantic: P-sem-1 in top 5 (paraphrase) | `s6plus_semantic_paraphrase_in_top5` | **Pass** (description `CONTAINS` + token-overlap proxy, not vectors) |
| Structural: P-struct-1 when lexical weak | `s6plus_structural_shared_deps_when_lexical_weak` | **Pass** (Jaccard on `depends_on` in adapter) |
| Merged ranking + `match_kinds` + snippet | `s6plus_merged_hit_carries_multiple_kinds` | **Pass** |
| Latency record (~200 problems) | `s6plus_latency_on_bulk_fixture` | **Recorded** (~3.4 s debug `find_similar`; seed dominates) |
| Latency ~1k problems | `s6plus_latency_bulk_1k` | **Ignored** — run manually |

### Not tested / deferred

| Item | Notes |
|------|--------|
| Linux platform | Skipped per team |
| Kill mid-write durability | Optional spike item — not documented |
| Release binary size delta | Not measured |
| Product-facing Cypher / `sparrowdb-mcp` | Out of scope |
| crates.io **0.1.22** features | Not available at pinned 0.1.16 |
| Formal per-op latency suite | Informal only (except S6+ eprintln) |

---

## Use cases enabled by SparrowDB (via spike adapter)

Prototype API: `PfeGraphStore` in `crates/openpfe-graph-spike/src/store.rs`. Maps to target [design.md](../design.md) `GraphStore` and product surfaces in [openpfe-ui/specification.md](../openpfe-ui/specification.md), [openpfe-mcp/specification.md](../openpfe-mcp/specification.md).

| Use case | Product need | Spike API / SparrowDB capability |
|----------|--------------|----------------------------------|
| **Open project graph** | Server starts; graph under `./.openpfe/graph/` | `PfeGraphStore::open` → `GraphDb::open(directory)` |
| **Problem decomposition (write)** | Create nodes with title, description, status | `upsert_node`, `create_edge` (Cypher `CREATE` / `MERGE` + `SET`) |
| **Read problem by id** | UI/MCP fetch one node | `get_node` |
| **List / filter problems** | By type, cluster, limits | `list_nodes` + `NodeFilter` (filter in Rust; no `WHERE` in Cypher) |
| **User edits graph** | Update fields; add/remove edges; delete node | `upsert_node`, `create_edge`, `delete_edge`, `delete_node` |
| **Architecture contracts** | `interfaces` edges with contract metadata | `create_edge` with inline rel properties |
| **MCP context shield** | Bounded cluster subgraph | `subgraph` — BFS in adapter with many Cypher round-trips |
| **Dependency DAG check** | Block invalid `depends_on` cycles | `validate_acyclic_deps` via `MATCH` on `depends_on` edges |
| **“Already recorded?” (lexical)** | Dedup by title/description (S6) | `search_problems` → `CONTAINS` + optional fulltext index |
| **“Find similar” (stretch)** | Ranked candidates (future MCP tool) | `find_similar(FindSimilarDraft)` → `SimilarHit` |
| **Durability** | Survive restart | `checkpoint()` on close; WAL under `wal/` |
| **Backup / restore** | Operator copies project graph dir | Directory copy of engine folder; `GraphDb::open` on restore |
| **Adjacency / neighbors** | Traversal helpers | `neighbors` — one Cypher query per direction × rel union |

### Adapter quirks (product phase 2 must hide)

| Quirk | Mitigation in product adapter |
|-------|------------------------------|
| Cypher reserves `id` | Store UUID as `node_id` |
| `MATCH…SET` cannot comma-separate multiple props | One `SET` per property |
| `MATCH…SET` on relationships not supported | `CREATE` rel with inline `{props}` |
| Node delete fails if edges remain | Enumerate incoming/outgoing per rel type; `DELETE r` then `DELETE n` with label |
| Incoming edge direction in Cypher | Use `MATCH (m)-[:rel]->(n)` not `n<-[:rel]-(m)` for “incoming” |

### `find_similar` legs (S6+ prototype)

| `match_kinds` | Implementation |
|---------------|----------------|
| `lexical` | `CONTAINS` on `title` + `description`; merged scores |
| `semantic` | **Proxy:** description `CONTAINS` when title match weak — not embeddings |
| `structural` | Jaccard overlap of `depends_on` neighbors within `draft.cluster_id` |

---

## Measurements (informal)

| Metric | macOS (debug) |
|--------|----------------|
| Commit | `6de47d2` |
| Warm `cargo build -p openpfe-graph-spike` | ~0.7 s |
| S4 `subgraph` ~805 nodes | ~213 s (`s4_subgraph_respects_caps`) |
| S6 `search_problems` (fixture scale) | &lt; 1 s |
| `find_similar` ~206 problems | ~3.4 s (S6+ test eprintln) |
| `find_similar` ~1005 problems | Run `cargo test -p openpfe-graph-spike s6plus_latency_bulk_1k -- --ignored --nocapture` |

Reproduce baseline: `cargo test -p openpfe-graph-spike --test sparrowdb_spike`  
Fast subset: `cargo test -p openpfe-graph-spike --test sparrowdb_spike -- --skip s4_subgraph_respects_caps`

---

## Findings

### Strengths

1. **Pure-Rust embedded** store with **WAL** — no RocksDB/C++ narrative for v1 evaluation.
2. **MIT** license — simple distribution story vs MPL IndraDB.
3. **S1–S6** achievable with Cypher inside a thin `PfeGraphStore` wrapper.
4. **Property text index** (`CONTAINS`, SPA-251) covers baseline S6 on small fixtures without a separate FTS crate.
5. **Directory backup** — copy whole engine path after `checkpoint()`; verified in tests.
6. **S6+ stretch** parity with Grafeo adapter shape (`find_similar`, `match_kinds`).

### Weaknesses / risks

1. **Adapter complexity** — string-built Cypher, engine parser limitations, easy to get edge direction wrong.
2. **`subgraph` performance** — BFS with per-step `neighbors` Cypher calls; unacceptable at ~1k nodes in debug without redesign (var-length Cypher or Rust `ReadTx` API).
3. **S6 quality** — `CONTAINS` + heuristic scores vs Grafeo BM25; fulltext index path immature vs dedicated search engines.
4. **`bincode` unmaintained** — transitive via `sparrowdb-execution`.
5. **Version lag** — crates.io 0.1.16 vs newer GitHub releases.
6. **Default crate pulls `clap`** — CLI surface in dependency graph even for library embed.
7. **Linux** — not validated on second platform.
8. **Young project** — API and Cypher subset still evolving (e.g. relationship `SET` restrictions).

### False positives / negatives (documented)

- Unrelated problems may appear in `CONTAINS` results on tiny corpora with weak token overlap.
- Structural leg can rank cluster peers when draft text is unrelated but shares `depends_on` hub.
- Semantic tag overlaps lexical when paraphrase still matches `CONTAINS` on description.

---

## Comparison vs Grafeo (same spike program)

| Dimension | SparrowDB | Grafeo |
|-----------|-----------|--------|
| **S1–S5** | Pass | Pass with caveats |
| **S6 lexical** | `CONTAINS` / partial FTS | BM25 (`text-index`) |
| **S6+ stretch** | Pass (adapter proxy) | Pass (BM25 + proxy) |
| **Durability story** | WAL directory (strong) | `.grafeo` + WAL siblings |
| **Adapter** | Cypher-heavy | Rust `GrafeoDB` API |
| **Subgraph ~1k (debug)** | ~213 s | Acceptable in spike |
| **License** | MIT | Apache-2.0 |
| **Audit** | `bincode` 1.3.3 | `bincode` 2.0.1 |

**nanograph:** not compared on this branch.

---

## Possible improvements (SparrowDB features & usage)

Prioritized for follow-up spike or product phase 2 — not required to close plan 006.

| Priority | Change | Expected benefit | Cost |
|----------|--------|------------------|------|
| **High** | Replace BFS `subgraph` with **var-length Cypher** or **`ReadTx`/`WriteTx`** low-level API | Orders-of-magnitude faster bounded reads | Adapter rewrite |
| **High** | Pin **newer SparrowDB** when on crates.io; re-run spike | Bugfixes, parser features (relationship SET, etc.) | Re-intake if lock changes |
| **Medium** | Product S6 via **Tantivy sidecar** or Grafeo if engine locked | Better lexical ranking than `CONTAINS` alone | Extra crate or dual-store |
| **Medium** | `CALL db.index.fulltext.*` only path after stable index rebuild policy | Scored search without BM25 | Ops complexity |
| **Low** | Trim unused **`clap`** from dependency path if upstream allows feature flags | Smaller graph | Upstream |
| **Skip** | Expose Cypher on HTTP/MCP | — | Out of product scope |

---

## Decision implications

| Outcome | Action |
|---------|--------|
| **Not selected (2026-05-25)** | [decision.md](../decision.md) — Grafeo chosen for v1 |
| Retained value | WAL / pure-Rust durability notes for ops comparison only |
| Product | Structural `find_similar` leg stays custom Rust in Grafeo adapter |

---

## Related documents

- [plan 006](../../plans/006-spike-openpfe-graph-sparrowdb.md)
- [sparrowdb.md](./sparrowdb.md)
- [program.md](./program.md)
- [evaluation.md](./evaluation.md)
- [decision.md](../decision.md) — locked engine (Grafeo)
- [grafeo-outcome.md](./grafeo-outcome.md)
- [nanograph.md](./nanograph.md), [nanograph-outcome.md](./nanograph-outcome.md)
