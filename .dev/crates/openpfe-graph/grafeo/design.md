# Grafeo adapter — design

**Read when:** implementing or reviewing `GrafeoGraphStore` in `crates/openpfe-graph`.

Normative mapping: [specification.md](./specification.md). Product schema and search API: [../specification.md](../specification.md). Engine lock: [../decision.md](../decision.md).

---

## Role

`GrafeoGraphStore` implements `openpfe_graph::GraphStore` over Grafeo’s Rust API. HTTP/MCP depend on the trait only — Grafeo query languages (GQL/Cypher) are **not** exposed on product surfaces; the adapter may use them internally later for traversals.

Code: `crates/openpfe-graph/src/grafeo_store.rs`, constants in `schema.rs`.

---

## Storage

| Path | Contents |
|------|----------|
| `./.openpfe/graph/store/` | Grafeo persistent directory (e.g. `store.grafeo` + WAL siblings) |

Open via `GrafeoGraphStore::open("./.openpfe/graph/store/")` or `GraphStore::open`.

---

## Search indexes

| Grafeo feature | openpfe use |
|----------------|-------------|
| `text-index` | BM25 on `problem.title`, `problem.description` — S6 / lexical `find_similar` |
| `vector-index` | HNSW on `problem.embedding` — semantic leg when embeddings present |
| `hybrid-search` | RRF merge of BM25 + vector in `find_similar` when draft has `embedding` |
| `parallel` | Rayon-backed batch search / engine parallelism |

Indexes are created lazily when the first `problem` node exists. After bulk import, call `rebuild_text_indexes()` / `rebuild_vector_index()`.

**Not used:** `embed` (ONNX) — embeddings come from `openpfe-llm`.

---

## `search_problems` (S6)

1. Merge BM25 scores from `text_search` on `problem.title` and `problem.description`.
2. Sort descending, truncate to `k`.
3. Return `SearchHit { node_id, title, score }`.

---

## `find_similar` (S6+) — merged ranking

Fetches up to `max(k × 4, 10)` candidates per leg, merges by `node_id` (keeps max score), tags `match_kinds`, fills `snippet` from matched node `description`, sorts, truncates to `k`.

### Lexical leg

- BM25 on combined `title + description` (and per-field merges).
- Always tags `lexical` when this leg contributes.

### Semantic leg

| Path | Behavior |
|------|----------|
| **Embedding present** | Grafeo `hybrid_search` on draft text + vector; tags `lexical` and `semantic`. Additional description BM25 may add `lexical`. |
| **No embedding (proxy)** | When description BM25 score exceeds title BM25 by **1.5×** and passes `MIN_BM25_SCORE`, tag `semantic` (paraphrase without vectors). |

### Structural leg

Requires `draft.cluster_id`:

1. List `problem` nodes in that cluster.
2. For each candidate with non-empty `depends_on` in+out neighbor set, compute mean Jaccard vs other problems in the cluster that also have deps.
3. Scale by `STRUCTURAL_SCORE_SCALE` (4.0); tag `structural` when score &gt; 0.

Candidates with **no** `depends_on` neighbors are skipped (avoids boosting unrelated cluster peers).

---

## Other adapter operations

| openpfe operation | Grafeo mechanism |
|-------------------|------------------|
| CRUD nodes/edges | `GrafeoDB` Rust API (`create_node_with_props`, `set_node_property`, …) |
| `subgraph` | BFS over pre-built adjacency (bounded caps) |
| `validate_acyclic_deps` | Cycle detection on `depends_on` in adapter |
| `backup_full` | Grafeo `backup_full` + directory copy |

---

## Accepted caveats (v1)

See [../spike/grafeo-outcome.md](../spike/grafeo-outcome.md):

- Transitive **`bincode` unmaintained** (RUSTSEC-2025-0141).
- **Search latency** — multiple BM25 calls per `find_similar`; remeasure in release builds.
- **False positives** — weak BM25 / structural scores on small corpora; score floors and realistic corpus size in product.
- **macOS-only spike evidence** — Linux validation deferred.

---

## Related

- [requirements.md](./requirements.md) — version pin, features, intake
- [specification.md](./specification.md) — labels, properties, indexes
- [../design.md](../design.md) — `GraphStore` trait and crate boundaries
