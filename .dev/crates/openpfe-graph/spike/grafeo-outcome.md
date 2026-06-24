# Grafeo spike — outcome summary

**Read when:** implementing Grafeo in `openpfe-graph` or reviewing spike evidence. **v1 engine locked (2026-05-25)** — [decision.md](../decision.md).

**Implementation:** `crates/openpfe-graph/` (`GrafeoGraphStore`); spike history in branch `spike_db_grafeao`.  
**Detailed checklist / measurements:** [grafeo.md](./grafeo.md).  
**Intake:** [.dev/dependencies/grafeo/](../../dependencies/grafeo/).

| Field | Value |
|-------|--------|
| **Date** | 2026-05-24 |
| **Platform tested** | macOS (Darwin 25.3, arm64) |
| **Linux** | Skipped (team decision) |
| **Grafeo version** | `0.5.42` |
| **Features enabled (v1 product)** | `lpg`, `text-index`, `vector-index`, `hybrid-search`, `parallel` |
| **Features explicitly not enabled** | `embedded`, `ai`, `embed`, `rdf`, `enterprise`, `server`, `grafeo-mcp` |
| **Recommendation** | **Pass with caveats** — **selected** for v1 ([decision.md](../decision.md)) |
| **vs shortlist** | **Selected** over nanograph and SparrowDB; IndraDB **rejected** |

---

## Executive summary

Grafeo meets the **required** spike bar (S1–S6) on macOS as an **embedded LPG** with persistent storage under a project-local path, `GraphStore`-shaped operations, BM25 text search, durability, and backup. **S6+ stretch** (merged lexical / semantic-proxy / structural `find_similar`) was also implemented and tested.

**Caveats:** transitive `bincode` unmaintained (RUSTSEC-2025-0141); young crate / MSRV 1.91.1; search latency high in debug on hundreds of nodes; semantic stretch uses **description BM25 proxy**, not vectors; Linux not re-run.

---

## Dependency & supply chain

| Item | Detail |
|------|--------|
| **Scope** | `openpfe-graph-spike` only — `openpfe-graph` product crate unchanged |
| **Intake** | Accepted 2026-05-24 — [verdict.md](../../dependencies/grafeo/verdict.md) |
| **`cargo audit`** | Exit 0; allowed warning: `bincode` 2.0.1 unmaintained via `grafeo-*` |
| **Transitive graph** | ~38 packages added at intake (`grafeo-engine`, `grafeo-core`, `grafeo-storage`, …) |
| **License** | Apache-2.0 |

---

## Schema & storage mapping

| PFE concept | Grafeo mapping |
|-------------|----------------|
| Node `type` (`problem`, `cluster`, …) | Grafeo **label** |
| Stable id | Property `id` (UUID string); internal `NodeId` mapped in adapter |
| Edge `type` (`depends_on`, `member_of`, `interfaces`) | Relationship **name** |
| JSON properties | Grafeo `Value` properties on nodes/edges |
| Target product path | `./.openpfe/graph/store/` → spike used `…/grafeo-store/store.grafeo` |
| On-disk | `.grafeo` file + WAL/sibling files (`lpg` + `wal` + `grafeo-file` via feature chain) |

---

## Scenarios tested

Normative definitions: [program.md](./program.md), [specification.md](../specification.md#search-and-similarity-v1). Tests: `crates/openpfe-graph/tests/grafeo_integration.rs`, `grafeo_s6plus.rs` (spike history: `openpfe-graph-spike`).

| ID | Product scenario | Spike test(s) | Result (macOS) |
|----|------------------|---------------|----------------|
| **S1** | Project problem space — CRUD, UUID, JSON props, persistence | `s1_lifecycle_and_reopen_roundtrip` | **Pass** — open, seed, close, reopen; properties round-trip |
| **S2** | User curation — update props, edges; delete node | `s2_curation_update_and_delete` | **Pass** — merge-style upsert; delete removes incident `depends_on` edges (Grafeo policy) |
| **S3** | Architecture lens — `interfaces` contract on bounded read | `s3_interfaces_contract_in_subgraph` | **Pass** — `contract_body`, `version`, etc. in `subgraph` edges |
| **S4** | MCP context shield — bounded `subgraph(cluster_id)` | `s4_subgraph_respects_caps` | **Pass** — `max_depth=3`, `max_nodes=200` on ~805-node fixture; < 500 nodes |
| **S5** | DAG validation — acyclic `depends_on` | `s5_acyclic_validation` | **Pass** — clean DAG empty; injected cycle returns path containing `p1` |
| **S6** | Similar / existing problem (lexical) | `s6_bm25_search_duplicate_title` | **Pass** — duplicate title in top 3; unrelated not in top 2; score below duplicates |
| **S6+** | Search & compare (v1) | `grafeo_s6plus.rs` (4 tests) | **Pass** — see [S6+ subsection](#s6-stretch-search--compare) |

### Additional integration tests (supporting scenarios)

| Test | Covers |
|------|--------|
| `durability_close_reopen_counts` | S1 / durability — node count stable after close + reopen |
| `backup_full_and_directory_copy` | Backup — `backup_full` + filesystem copy; reopen and read |
| `list_nodes_by_type_and_cluster` | S3 / filters — `list_nodes` by `type` + `cluster_id` |

### S6+ stretch (search & compare)

| Stretch check | Test | Result |
|---------------|------|--------|
| Lexical: P-lex-2 top 3; P-lex-3 not top 5 | `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` | **Pass** |
| Semantic: P-sem-1 in top 5 (paraphrase) | `s6plus_semantic_paraphrase_in_top5` | **Pass** (BM25 proxy, not vectors) |
| Structural: P-struct-1 when lexical weak | `s6plus_structural_shared_deps_when_lexical_weak` | **Pass** (adapter Jaccard on `depends_on`) |
| Merged ranking + `match_kinds` + snippet | `s6plus_merged_hit_carries_multiple_kinds` | **Pass** |
| Latency record (~200 problems) | `s6plus_latency_on_bulk_fixture` | **Recorded** (~19s debug `find_similar`) |
| Latency ~1k problems | `s6plus_latency_bulk_1k` | **Ignored** — run manually |

### Not tested / deferred

| Item | Notes |
|------|--------|
| Linux platform | Skipped per team |
| Kill mid-write durability | Optional spike item — not documented |
| Formal cold-open / per-op ms benchmarks | Not timed (except latency eprintln) |
| Release binary size delta | Not measured |
| Grafeo GQL/Cypher for traversals | `lpg` includes GQL/Cypher; spike uses Rust CRUD + `iter_edges` only |
| Vector / HNSW / `embed` / `hybrid-search` | Not enabled |
| `grafeo-mcp`, HTTP exposure of query languages | Out of product scope |

---

## Use cases enabled by Grafeo (via spike adapter)

Prototype API: `PfeGraphStore` in `crates/openpfe-graph-spike/src/store.rs`. Maps to target [design.md](../design.md) `GraphStore` and product surfaces in [openpfe-ui/specification.md](../openpfe-ui/specification.md), [openpfe-mcp/specification.md](../openpfe-mcp/specification.md).

| Use case | Product need | Spike API / Grafeo capability |
|----------|--------------|------------------------------|
| **Open project graph** | Server starts; graph under `./.openpfe/graph/` | `PfeGraphStore::open` → `GrafeoDB::open` (persistent `.grafeo`) |
| **Problem decomposition (write)** | Create `problem` / `cluster` nodes with title, description, status | `upsert_node`, `create_edge` (`member_of`, `depends_on`) |
| **Read problem by id** | UI/MCP fetch one node | `get_node` |
| **List / filter problems** | By type, cluster, limits | `list_nodes` + `NodeFilter` |
| **User edits graph** | Update fields; add/remove edges; delete node | `upsert_node`, `create_edge`, `delete_edge`, `delete_node` |
| **Architecture contracts** | `interfaces` edges with contract metadata | `create_edge_with_props`; read via `subgraph` |
| **MCP context shield** | Bounded cluster subgraph for agents | `subgraph(cluster_id, SubgraphLimits)` — adapter-enforced caps |
| **Dependency DAG check** | Block invalid `depends_on` cycles | `validate_acyclic_deps` → cycle paths as UUID lists |
| **“Already recorded?” (lexical)** | Dedup by title/description (S6) | `ensure_text_indexes`, `search_problems` → Grafeo `text_search` / BM25 |
| **“Find similar” (stretch)** | Ranked candidates with reasons (future MCP tool) | `find_similar(FindSimilarDraft)` → `SimilarHit { match_kinds, snippet, score }` |
| **Durability** | Survive restart | Close + reopen; WAL-backed persistence |
| **Backup / restore** | Operator copies project graph dir | `backup_full`; directory copy of store files |
| **Adjacency / neighbors** | Traversal helpers | `neighbors` (via `iter_edges`; not GQL) |

### `find_similar` legs (S6+ prototype)

| `match_kinds` | Implementation |
|---------------|----------------|
| `lexical` | BM25 on `title` + `description` (`text_search`) |
| `semantic` | **Proxy:** description BM25 when title BM25 weak — not true embeddings |
| `structural` | Jaccard overlap of `depends_on` in/out neighbors within `draft.cluster_id` |

---

## Measurements (informal)

| Metric | macOS (debug) |
|--------|----------------|
| Cold `cargo build` (incl. grafeo) | ~18s (spike note) |
| `find_similar` ~206 problems | ~19s (test eprintln) |
| `find_similar` ~1005 problems | Run `cargo test -p openpfe-graph-spike s6plus_latency_bulk_1k -- --ignored --nocapture` |
| S4 subgraph ~805 nodes | Passes caps; not timed |

---

## Findings

### Strengths

1. **Single embedded store** for graph + lexical search (no separate FTS crate for S6).
2. **Rust CRUD API** fits `GraphStore` adapter pattern; no need to expose GQL on HTTP/MCP.
3. **S1–S5** straightforward with labels, typed edges, persistence, delete-cascade edges.
4. **BM25** (`text-index`) satisfies baseline and stretch lexical requirements.
5. **Backup API** (`backup_full`) plus directory copy documented.

### Weaknesses / risks

1. **`bincode` unmaintained** — transitive; upstream risk.
2. **Search performance** — multiple BM25 calls per `find_similar`; slow on hundreds of nodes in debug; `rebuild_text_index` costly after bulk load.
3. **Small-corpus ranking noise** — unrelated problems can appear with low BM25 scores; needs score floors and realistic corpus size in product.
4. **Semantic gap** — paraphrase handling via BM25 proxy; real intent-matching needs `vector-index` + embeddings or `openpfe-llm` sidecar.
5. **Structural similarity** — custom Rust, not engine-native; expected for any engine.
6. **Young crate / MSRV 1.91.1** — version churn risk.
7. **Subgraph implementation** — full `iter_edges` scans; may not scale vs planner/GQL on large graphs.
8. **Linux** — not validated on second platform.

### False positives / negatives (documented)

- Unrelated problem with weak token overlap may appear in BM25 top 5 on tiny fixtures.
- S6+ structural leg can boost unrelated peers in the same cluster when draft text is lexical noise.
- Semantic tag may overlap with lexical when paraphrase still matches BM25 on title/description.

---

## Possible improvements (Grafeo features & usage)

Prioritized for a **follow-up spike** or product phase 2 — not required to close plan 006.

| Priority | Change | Expected benefit | Cost |
|----------|--------|------------------|------|
| **High** | Enable **`vector-index`** + **`hybrid-search`** (not `embed`); supply vectors from `openpfe-llm` | Real semantic similarity; native RRF vs hand-rolled merge | New intake; embedding pipeline; larger deps |
| **Medium** | Enable **`parallel`**; remeasure bulk index + search | Faster rebuild / possibly faster queries | Feature + audit review |
| **Medium** | Prototype **bounded GQL** for `subgraph` / neighbors (already in `lpg`) | Better traversal performance at scale | Query discipline for context caps |
| **Medium** | Adapter optimizations without new features — fewer `text_search` calls, cache indexes | Lower latency | Engineering only |
| **Low** | **`algos`** if cycle API fits `validate_acyclic_deps` | Less custom graph code | Small |
| **Skip** | `embedded`, `rdf`, `embed`, `grafeo-mcp`, `server` | — | Too heavy or wrong model |

**Note:** `lpg` already enables GQL/Cypher internally; improvement is **usage**, not an extra feature flag.

---

## Decision implications

| Outcome | Action |
|---------|--------|
| **Grafeo locked (2026-05-25)** | [decision.md](../decision.md); `design.md` / `specification.md` updated |
| Phase 2 | Promote spike adapter to `openpfe-graph`; Grafeo intake on product crate; optional `vector-index` follow-up |
| Product API | `find_similar` / MCP tool remains adapter-owned; structural leg stays custom Rust |

---

## Related documents

- [plan 006](../../plans/006-spike-openpfe-graph-grafeo.md)
- [grafeo.md](./grafeo.md)
- [program.md](./program.md)
- [evaluation.md](./evaluation.md)
- [decision.md](../decision.md) — locked engine
- [nanograph-outcome.md](./nanograph-outcome.md), [sparrowdb-outcome.md](./sparrowdb-outcome.md) — not selected
- [indradb.md](./indradb.md) — rejected
