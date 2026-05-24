# Spike: SparrowDB (embedded LPG)

**Parent program:** [graph-db-spike.md](./graph-db-spike.md)

**Engine:** [SparrowDB](https://github.com/ryaker/SparrowDB) — embedded Rust graph DB with WAL-backed durability and Cypher execution (use **Rust API / library** in spike, not product-facing Cypher).

**Role:** Grafeo alternative focused on **pure-Rust storage** and **crash-safe durability** without RocksDB.

**Outcome summary:** [sparrowdb-outcome.md](./sparrowdb-outcome.md) — scenarios, use cases, findings, improvements.

**Implementation:** `crates/openpfe-graph-spike/` — [README](../../../crates/openpfe-graph-spike/README.md). **Intake:** [.dev/dependencies/sparrowdb/](../../dependencies/sparrowdb/).

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | macOS complete — Linux skipped |
| **Owner** | spike branch |
| **Branch / crate** | `spike_db_sparrowdb` — `crates/openpfe-graph-spike` ([plan 006](../../plans/006-spike-openpfe-graph-sparrowdb.md)) |
| **Commit** | `6de47d2` (record at merge) |
| **Platforms tested** | macOS: ☑ (Darwin 25.3, arm64) — Linux: ☐ skipped (team decision, same as Grafeo) |

---

## Goals

1. Embed SparrowDB in-process at a project-local directory under `<temp>/openpfe-graph-spike/sparrowdb/`.
2. Implement **S1–S5** via `GraphStore`-shaped adapter (Cypher allowed **only** inside spike if faster than low-level API).
3. Document **S6** — native FTS or acceptable v1 workaround (`list` + filter / sidecar).
4. **S6+ (stretch)** — `find_similar` with lexical, semantic-proxy, and structural legs ([graph-db-spike.md](./graph-db-spike.md#stretch-goals--search--compare-s6)).
5. Compare build, audit, and binary size vs Grafeo and nanograph.

---

## Setup

### Dependencies

| Crate | Version |
|-------|---------|
| `sparrowdb` | **0.1.16** (crates.io; upstream repo at 0.1.22 not published at intake) |
| `sparrowdb-execution` | **0.1.16** (query `Value` / `QueryResult` in adapter) |

### Storage path

| Path | Use |
|------|-----|
| `<temp>/…/sparrowdb/` | Directory for `GraphDb::open` (WAL, catalog, CSR, property columns) |
| `./.openpfe/graph/` | Product target — copy **entire** engine directory while server stopped after `checkpoint()` |

### License check

- [x] **MIT** — crates.io / [GitHub LICENSE](https://github.com/ryaker/SparrowDB/blob/main/LICENSE)

---

## Tasks (checklist)

- [x] S1–S5 scenarios with shared fixture (`tests/sparrowdb_spike.rs`)
- [x] S6 — property text index (`CONTAINS`) + `create_fulltext_index` / `add_to_fulltext_index`
- [x] S6+ stretch — `find_similar`, `seed_s6_plus`, `tests/sparrowdb_spike_s6plus.rs`
- [x] Durability smoke + directory backup/restore
- [x] `cargo audit`; `cargo tree`; build time; release size (informal)
- [x] Confirm **no C++** required for default build

---

## Results

| Item | Result |
|------|--------|
| **Recommendation** | ☑ Pass with caveats ☐ Pass ☐ Fail |
| **S6** | **Pass (workaround + partial native)** — `WHERE n.title CONTAINS …` (SPA-251 text index); optional `create_fulltext_index` + `add_to_fulltext_index` on problem upsert; no Grafeo-grade BM25 |
| **vs Grafeo** | ☑ Inconclusive ☐ Prefer SparrowDB ☐ Prefer Grafeo |

### Recommendation summary

**Pass with caveats** for v1 graph + durability story on macOS. S1–S6 and **S6+ stretch** integration tests pass (`sparrowdb_spike`, `sparrowdb_spike_s6plus`). Caveats: Cypher-only adapter surface (labeled `DELETE`, per-property `SET`, edge delete direction quirks); **slow** bounded `subgraph` on ~800-node fixture in debug (~3+ min); transitive `bincode` unmaintained; crates.io lags GitHub; Linux not re-run; S6/S6+ weaker than Grafeo BM25.

### Measurements (macOS arm64, debug, commit `6de47d2`)

| Metric | Command / notes | Value |
|--------|-----------------|-------|
| Platforms | `uname -a` | Darwin 25.3 arm64 |
| Engine version | `Cargo.toml` | `sparrowdb` 0.1.16 |
| Open/create | spike tests (tempdir) | &lt; 100 ms per test dir |
| `subgraph` (~805 nodes) | `s4_subgraph_respects_caps` | ~213 s (debug; adapter BFS + many Cypher round-trips) |
| `validate_acyclic_deps` | `s5_acyclic_validation` | &lt; 1 s |
| `list_nodes` filter | `list_nodes_by_type_and_cluster` | &lt; 1 s |
| S6 search | `s6_search_duplicate_title` | &lt; 1 s (fixture scale) |
| S6+ `find_similar` (~206 problems) | `s6plus_latency_on_bulk_fixture` (`--nocapture`) | ~3.4 s query; ~19 s test wall (incl. bulk seed) |
| Debug build (warm) | `cargo build -p openpfe-graph-spike` | ~0.7 s incremental |
| Release size delta | Not measured vs baseline workspace | _defer_ |
| Audit | `cargo audit` | Exit 0; RUSTSEC-2025-0141 `bincode` 1.3.3 via `sparrowdb-execution` (accepted in intake) |

Reproduce: `cargo test -p openpfe-graph-spike --test sparrowdb_spike`  
S6+: `cargo test -p openpfe-graph-spike --test sparrowdb_spike_s6plus`

### S6 (lexical search)

| Approach | Status |
|----------|--------|
| `MATCH (n:problem) WHERE n.title CONTAINS $q OR n.description CONTAINS $q` | **Used** — SPA-251 property text index |
| `GraphDb::create_fulltext_index` + `WriteTx::add_to_fulltext_index` | Used on problem upsert; optional `queryNodes` in `search_problems` (baseline S6) |
| v1 product recommendation | Ship **CONTAINS + title filter** for small graphs, or **sidecar FTS** (Tantivy) in phase 2 if Grafeo not chosen |

### vs Grafeo / nanograph

| Dimension | SparrowDB spike | Grafeo ([grafeo-outcome.md](./grafeo-outcome.md)) |
|-----------|-----------------|--------------------------------------------------|
| S1–S5 | Pass (macOS) | Pass with caveats (macOS) |
| S6 | CONTAINS / partial fulltext | BM25 in-engine |
| S6+ stretch | Pass — token overlap + exact-title boost; structural Jaccard in adapter | Pass — BM25 + description proxy + structural Jaccard |
| Durability / WAL | Strong narrative (WAL dir) | `.grafeo` + WAL siblings |
| C++ build | **No** | **No** |
| Adapter complexity | High (Cypher strings) | Medium (Rust API) |
| `subgraph` perf (debug, ~1k) | Slow | Acceptable in spike |

**nanograph:** not run on this branch — comparison **inconclusive**.

### Supply chain

- **License:** MIT
- **`cargo tree -i sparrowdb`:** direct on `openpfe-graph-spike` only
- **Notable transitive:** `sparrowdb-storage`, `sparrowdb-cypher`, `sparrowdb-execution`, `clap` (CLI in default crate graph), `bincode` 1.3.3

### Backup procedure

1. Stop openpfe server (single writer).
2. `checkpoint()` graph store (spike `PfeGraphStore::close` does this).
3. Copy entire `./.openpfe/graph/` engine directory (same layout as spike `sparrowdb/` folder).
4. Restore: copy tree to new path; `GraphDb::open(restored_path)`; verify known `node_id` via `get_node`.

Verified in test `backup_directory_copy`.

### On-disk layout (observed)

Under `GraphDb::open(path)`: `wal/` directory present; catalog/CSR/column files per SparrowDB storage engine (see upstream docs). Operator copies **whole directory**, not a single file.

### S6+ stretch (search & compare)

**Outcome:** **Pass** — same stretch fixture and checklist as Grafeo ([spike-grafeo.md](./spike-grafeo.md)); details in [sparrowdb-outcome.md](./sparrowdb-outcome.md#s6-stretch-search--compare).

| Kind | Pass? | Notes |
|------|-------|-------|
| Lexical (P-lex-2 / P-lex-3) | ☑ | `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` — P-lex-2 in top 3; P-lex-3 absent from top 5 |
| Semantic (P-sem-1) | ☑ (proxy) | `s6plus_semantic_paraphrase_in_top5` — description token overlap when title leg weak; **not** vector/HNSW |
| Structural (P-struct-1) | ☑ | `s6plus_structural_shared_deps_when_lexical_weak` — Jaccard on cached `depends_on` neighbor sets within `cluster_id` |
| Merged ranking | ☑ | `s6plus_merged_hit_carries_multiple_kinds` — `SimilarHit { match_kinds, snippet, score }` |
| Latency (~200 problems) | ☑ recorded | `s6plus_latency_on_bulk_fixture` — ~3.4 s `find_similar` on ~206 problems (debug, 2026-05-25, after structural cache); bulk seed dominates wall time |
| Latency (~1k problems) | optional | `s6plus_latency_bulk_1k -- --ignored --nocapture` |
| False positives | note | Unrelated cluster peers can pick up weak structural score when draft text is lexical noise |
| Implementation path | ☑ | Adapter-owned ranking; see legs below. Full semantic → `openpfe-llm` or sidecar in phase 2 |

| `match_kinds` | Implementation (`PfeGraphStore::find_similar`) |
|---------------|--------------------------------------------------|
| `lexical` | Token overlap on `title` / `description` + exact-title boost |
| `semantic` | Description token overlap when description score &gt; 1.5× title score |
| `structural` | Mean Jaccard of `depends_on` in/out neighbors vs cluster peers (×4 scale) |

**vs Grafeo S6+:** Same fixture and tests shape; Grafeo uses in-engine BM25 (`text-index`). SparrowDB meets the stretch bar via **adapter heuristics** — comparable for “find existing problem” prototyping, weaker native ranking at scale.

Reproduce: `cargo test -p openpfe-graph-spike --test sparrowdb_spike_s6plus`

---

## Related

- [sparrowdb-outcome.md](./sparrowdb-outcome.md)
- [spike-grafeo.md](./spike-grafeo.md)
- [grafeo-outcome.md](./grafeo-outcome.md)
- [spike-nanograph.md](./spike-nanograph.md)
- [graph-db-evaluation.md](./graph-db-evaluation.md)
