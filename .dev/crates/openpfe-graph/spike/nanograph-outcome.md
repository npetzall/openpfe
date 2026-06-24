# nanograph spike — outcome summary

**Read when:** reviewing nanograph spike evidence. **Not selected for v1** — engine locked to **Grafeo** ([decision.md](../decision.md)).

**Implementation:** `crates/openpfe-graph-spike/` on branch `spike_db_nanograph` (throwaway; not product `openpfe-graph`).  
**Detailed checklist / measurements:** [nanograph.md](./nanograph.md).  
**Intake:** [.dev/dependencies/nanograph/](../../dependencies/nanograph/).

| Field | Value |
|-------|--------|
| **Date** | 2026-05-24 |
| **Platform tested** | macOS (Darwin 25.3, arm64) |
| **Linux** | Skipped (team decision, same as Grafeo spike) |
| **nanograph version** | `1.3.0` |
| **Companion crate** | `arrow-array` **58** (parse `RecordBatch` from search queries) |
| **Features enabled** | Default `nanograph` only — no `@embed` / ML stacks |
| **Build requirement** | **`protoc`** on PATH (`brew install protobuf`) |
| **Recommendation** | **Pass with caveats** — viable for v1 folder-backed graph + in-query BM25; heavy deps and read-path cost need product mitigation |
| **vs Grafeo** | Stronger **S6 in query language** and **directory backup**; weaker on **dynamic JSON properties** and **export-based reads** in spike adapter |

---

## Executive summary

nanograph meets the **required** spike bar (S1–S6) and **S6+ stretch** on macOS as an **embedded, folder-backed LPG** with engine-native BM25 on indexed `title` / `description`, `GraphStore`-shaped operations, durability, and directory backup.

**Caveats:** schema-as-code (`.pg`) vs PFE JSON; spike adapter reads via `build_export_rows_at_path` (very slow at hundreds of nodes); **~600** lockfile packages + `protoc`; `cargo audit` RUSTSEC-2023-0071 (`rsa` via Lance/opendal); async `Database`; semantic stretch uses **description BM25 proxy**, not `@embed` / `rrf`; Linux not re-run.

---

## Dependency & supply chain

| Item | Detail |
|------|--------|
| **Scope** | `openpfe-graph-spike` only — `openpfe-graph` product crate unchanged |
| **Intake** | Accepted 2026-05-24 — [verdict.md](../../dependencies/nanograph/verdict.md) |
| **`cargo audit`** | RUSTSEC-2023-0071 (`rsa`, no fix) via `opendal` → `lance-io` — accepted for spike; warnings: unmaintained `paste`, `rustls-pemfile` |
| **Transitive graph** | Lance, Arrow, DataFusion, object_store/opendal — see [lock-update.md](../../dependencies/nanograph/lock-update.md) |
| **License** | MIT |

---

## Schema & storage mapping

| PFE concept | nanograph mapping |
|-------------|-------------------|
| Node `type` (`problem`, `cluster`, …) | Node types `Problem`, `Cluster` (adapter returns lowercase) |
| Stable UUID | Property **`pfe_id: String @key`** — not `id` (clashes with internal row id) |
| `title`, `description`, `status`, `cluster_id` | Fixed columns on `Problem` in embedded `.pg` |
| Edge `type` | `MemberOf`, `DependsOn`, `Interfaces` (PascalCase in schema; snake_case in API) |
| `interfaces` contract fields | Edge properties on `Interfaces` |
| Ad-hoc extra JSON keys | **Not supported** without schema edit + migration |
| Target product path | `./.openpfe/graph/` → spike used `<temp>/openpfe-graph-spike/nanograph/` |
| On-disk | Directory: `schema.pg`, `schema.ir.json`, `nodes/`, `edges/`, Lance datasets + manifest |

**Verdict:** Acceptable for v1 if product node/edge fields are stabilized; poor fit for fully dynamic per-node JSON without a blob/JSON column in `.pg`.

---

## Scenarios tested

Normative definitions: [program.md](./program.md). Tests: `crates/openpfe-graph-spike/tests/nanograph_spike.rs`, `nanograph_spike_s6plus.rs`, `nanograph_smoke_load.rs`.

| ID | Product scenario | Spike test(s) | Result (macOS) |
|----|------------------|---------------|----------------|
| **S1** | Project problem space — CRUD, UUID, persistence | `s1_lifecycle_and_reopen_roundtrip` | **Pass** — `Database::init` / `open`; `put` mutations; reopen |
| **S2** | User curation — update props, edges; delete node | `s2_curation_update_and_delete` | **Pass** — upsert via `put`; delete removes incident edges |
| **S3** | Architecture lens — `interfaces` contract on bounded read | `s3_interfaces_contract_in_subgraph` | **Pass** — contract props on `interfaces` in `subgraph` |
| **S4** | MCP context shield — bounded `subgraph(cluster_id)` | `s4_subgraph_respects_caps` | **Pass** — `max_depth=3`, `max_nodes=200` on ~205-node fixture; < 500 nodes (**~8 min** debug — export cost) |
| **S5** | DAG validation — acyclic `depends_on` | `s5_acyclic_validation` | **Pass** — cycle path returned |
| **S6** | Similar / existing problem (lexical) | `s6_bm25_search_duplicate_title` | **Pass** — `bm25($p.title, $q)` / description; duplicate top 3 |
| **S6+** | Search & compare (stretch) | `nanograph_spike_s6plus.rs` | **Pass** — see [S6+ subsection](#s6-stretch-search--compare) |

### Additional integration tests (supporting scenarios)

| Test | Covers |
|------|--------|
| `durability_close_reopen_counts` | S1 / durability — counts stable after close + reopen |
| `backup_directory_copy` | Backup — `copy_data_dir_to` while closed; reopen at new path |
| `list_nodes_by_type_and_cluster` | S3 / filters — export-backed `list_nodes` |
| `smoke_init_load_bm25_*` | S6 seed path — JSONL load + BM25 |
| `smoke_find_similar_after_seed` | S6+ smoke — `find_similar` after `seed_s6_plus` |

### S6+ stretch (search & compare)

| Stretch check | Test | Result |
|---------------|------|--------|
| Lexical: P-lex-2 top 3; P-lex-3 not top 5 | `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` | **Pass** |
| Semantic: P-sem-1 in top 5 (paraphrase) | `s6plus_semantic_paraphrase_in_top5` | **Pass** (BM25 proxy, not vectors) |
| Structural: P-struct-1 when lexical weak | `s6plus_structural_shared_deps_when_lexical_weak` | **Pass** (adapter Jaccard on `depends_on`) |
| Merged ranking + `match_kinds` + snippet | `s6plus_merged_hit_carries_multiple_kinds` | **Pass** |
| Latency ~56 / ~1k problems | `s6plus_latency_on_bulk_fixture`, `s6plus_latency_bulk_1k` | **Ignored** — debug stack overflow on export + bulk BM25; run manually |

### Not tested / deferred

| Item | Notes |
|------|--------|
| Linux platform | Skipped per team |
| Kill mid-write durability | Optional spike item — not documented |
| Release binary size delta | Not measured |
| Engine `@embed` / `Vector` / `rrf()` hybrid | Available in 1.3; not wired (no API keys in spike) |
| Targeted `run_query` for list/subgraph/neighbors | Spike uses full export — production should not |
| Close/reopen between S6+ seed and query in same test | Breaks Lance tables — tests keep DB open |

---

## Use cases enabled by nanograph (via spike adapter)

Prototype API: `PfeGraphStore` in `crates/openpfe-graph-spike/src/store.rs`. Maps to target [design.md](../design.md) `GraphStore` and product surfaces in [openpfe-ui/specification.md](../openpfe-ui/specification.md), [openpfe-mcp/specification.md](../openpfe-mcp/specification.md).

| Use case | Product need | Spike API / nanograph capability |
|----------|--------------|----------------------------------|
| **Open project graph** | Server starts; graph under `./.openpfe/graph/` | `PfeGraphStore::open` → `Database::open` / `init` (folder store) |
| **Problem decomposition (write)** | Create `problem` / `cluster` with title, description, status | `upsert_node` (`put` mutations); bulk `load_jsonl_overwrite` |
| **Read problem by id** | UI/MCP fetch one node | `get_node` (export-backed in spike) |
| **List / filter problems** | By type, cluster, limits | `list_nodes` + `NodeFilter` |
| **User edits graph** | Update fields; add/remove edges; delete node | `upsert_node`, `create_edge`, `delete_edge`, `delete_node` |
| **Architecture contracts** | `interfaces` edges with contract metadata | `create_edge` with properties; read via `subgraph` |
| **MCP context shield** | Bounded cluster subgraph for agents | `subgraph(cluster_id, SubgraphLimits)` — adapter-enforced caps |
| **Dependency DAG check** | Block invalid `depends_on` cycles | `validate_acyclic_deps` → cycle paths as UUID lists |
| **“Already recorded?” (lexical)** | Dedup by title/description (S6) | `search_problems` → `bm25()` in query language |
| **“Find similar” (stretch)** | Ranked candidates with reasons (future MCP tool) | `find_similar(FindSimilarDraft)` → `SimilarHit { match_kinds, snippet, score }` |
| **Durability** | Survive restart | Close + reopen same folder |
| **Backup / restore** | Operator copies project graph dir | Copy entire `nanograph/` directory while process stopped |
| **Adjacency / neighbors** | Traversal helpers | `neighbors` (export-backed in spike) |

### `find_similar` legs (S6+ prototype)

| `match_kinds` | Implementation |
|---------------|----------------|
| `lexical` | BM25 on `title` + `description` (`bm25()` in `.pg` queries) |
| `semantic` | **Proxy:** description BM25 when stronger than title BM25 — not `@embed` / `rrf` |
| `structural` | Jaccard on undirected `depends_on` neighbors within `draft.cluster_id`; skip nodes with no `depends_on` |

**S6+ seed:** `tests/s6plus_fixture.jsonl` via `load_jsonl_overwrite` — put-only seed left Lance/BM25 inconsistent in experiments.

---

## Measurements (informal)

| Metric | macOS (debug) |
|--------|----------------|
| Full `cargo test -p openpfe-graph-spike` (incl. S4) | **~494 s**, `--test-threads=1` (S4 export-dominated) |
| Tests excluding `s4_subgraph` + ignored latency | **~43 s** (8 baseline + 4 S6+ + 3 smoke) |
| S6 alone | ~7 s |
| S6+ suite (`nanograph_spike_s6plus`) | ~6 s |
| First cold `cargo build` (with `protoc`) | ~3–5 min order of magnitude |
| S4 subgraph ~205 nodes | Passes caps; **~8 min** (export) |
| `cargo clippy -p openpfe-graph-spike` | Clean (`-D warnings`) |

---

## Findings

### Strengths

1. **Folder-native storage** — aligns with `./.openpfe/graph/` backup story (copy directory).
2. **BM25 in query language** — `@index` on columns; no separate FTS crate for S6.
3. **Typed schema** — validates node/edge shape at ingest (good for SoT graph integrity).
4. **S1–S5** achievable with `put` / `delete` / parameterized queries.
5. **Future semantic path** — engine supports `Vector`, `@embed`, `rrf()` without leaving nanograph (not exercised in spike).
6. **S6+ stretch** — same merged `find_similar` shape as Grafeo spike for apples-to-apples comparison.

### Weaknesses / risks

1. **Schema friction** — every new PFE property needs `.pg` + migration; no ad-hoc JSON per node.
2. **Read path in spike** — `build_export_rows_at_path` for list/subgraph/neighbors/`get_node` is **O(entire graph)** and very slow; misleading for engine ceiling.
3. **Dependency weight** — Lance/Arrow/DataFusion; `protoc`; large lockfile.
4. **`rsa` advisory** — RUSTSEC-2023-0071 transitive, no fix; security review before product lock.
5. **Async API** — `Database` is async; `openpfe-server` needs async graph port or `spawn_blocking`.
6. **Test footgun** — dropping `TempDir` before queries deletes Lance files (`Table not found`).
7. **Bulk + debug** — S6+ latency tests hit stack overflow on export + many BM25 calls.
8. **Linux** — not validated on second platform.

### False positives / negatives (documented)

- Unrelated problems may appear in BM25 top 5 on tiny fixtures (low scores).
- Structural leg initially boosted unrelated cluster mates without `depends_on` — fixed by skipping empty dependency sets.
- Semantic tag may overlap lexical when paraphrase still matches BM25 on description/title.

---

## Possible improvements (nanograph features & usage)

Prioritized for **follow-up spike** or product phase 2 — not required to close plan 006.

| Priority | Change | Expected benefit | Cost |
|----------|--------|------------------|------|
| **High** | Replace export reads with **targeted `run_query`** for `get_node`, `list_nodes`, `subgraph`, edges | Orders-of-magnitude faster reads at scale | Adapter rewrite |
| **High** | **`@embed` / `rrf()`** with vectors from `openpfe-llm` | Real semantic similarity vs BM25 proxy | API keys, intake, embedding pipeline |
| **Medium** | JSON/blob column in `.pg` for rare extensibility | Reduce schema churn for odd metadata | Schema design |
| **Medium** | Remeasure S6+ latency in **release** + without export in hot path | Honest stretch metrics | Benchmark harness |
| **Low** | `component` node type in schema | Parity with PFE spec | Schema + fixture |
| **Skip** | Exposing nanograph query language on HTTP/MCP | — | Out of product scope |

---

## Comparison snapshot (nanograph vs Grafeo spike)

| Dimension | nanograph | Grafeo |
|-----------|-----------|--------|
| Storage | Directory + Lance | `.grafeo` file + WAL |
| S6 search | `bm25()` in queries | `text-index` / `text_search` |
| Dynamic properties | Fixed `.pg` columns | JSON properties on nodes |
| Spike read path | Full export (slow) | `iter_edges` / CRUD (faster) |
| Transitive audit | `rsa` (Lance) | `bincode` unmaintained |
| S6+ stretch | Pass (same adapter pattern) | Pass |
| Linux | Skipped | Skipped |

Engine lock: [decision.md](../decision.md) (Grafeo). Comparison record: [evaluation.md](./evaluation.md).

---

## Decision implications

| Outcome | Action |
|---------|--------|
| **Not selected (2026-05-25)** | [decision.md](../decision.md) — Grafeo chosen for v1 |
| Retained value | Query-language BM25 and folder backup patterns inform future docs only |
| IndraDB | **Rejected** — [indradb.md](./indradb.md) |

---

## Related documents

- [plan 006 — nanograph](../../plans/006-spike-openpfe-graph-nanograph.md)
- [nanograph.md](./nanograph.md)
- [grafeo-outcome.md](./grafeo-outcome.md)
- [program.md](./program.md)
- [evaluation.md](./evaluation.md)
- [decision.md](../decision.md) — locked engine (Grafeo)
- [sparrowdb.md](./sparrowdb.md), [sparrowdb-outcome.md](./sparrowdb-outcome.md)
