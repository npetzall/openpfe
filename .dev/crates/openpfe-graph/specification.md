# openpfe-graph — specification

## Storage location

| Path | Contents |
|------|----------|
| `./.openpfe/graph/` | Project graph root — backup copies this tree when server is stopped |
| `./.openpfe/graph/store/` | Persistent database directory (Grafeo: e.g. `store.grafeo` + WAL siblings) |

Open with `GraphStore::open` at **`./.openpfe/graph/store/`** (relative to **process cwd** = project root). No shared path-helper API.

## Schema (v1)

**Engine mapping (Grafeo):** node `type` → label; edge `type` → relationship name; fields below → properties. Canonical id is property **`id`** (UUID). Full mapping: [grafeo/specification.md](./grafeo/specification.md).

### Node types

| `type` | Purpose |
|--------|---------|
| `problem` | Atomic problem node (PFE drill-down leaf or branch) |
| `cluster` | Logical grouping → component boundary candidate |
| `component` | Approved architecture unit (optional v1; may merge with `cluster` until UI needs split) |
| `contract` | Consumer-driven contract artifact (optional node; may be edge-only metadata in early v1) |

### Edge types

| `type` | Direction | Meaning |
|--------|-----------|---------|
| `depends_on` | directed | Source problem/cluster **depends on** target (prerequisite) |
| `member_of` | directed | Problem **`member_of`** cluster |
| `interfaces` | directed | Cluster/component **interfaces** with target (contract boundary) |

### Common JSON properties (nodes)

| Property | Used on | Notes |
|----------|---------|--------|
| `title` | all | Short label |
| `description` | problem, cluster | Markdown/plain text |
| `status` | problem, cluster | e.g. `open`, `refined`, `done` |
| `cluster_id` | problem | Denormalized helper for filters (must match `member_of` edge) |
| `embedding` | problem | Optional `f32[]` from `openpfe-llm`; vector index for hybrid search |

Edges may carry `contract_body`, `version`, `consumer_id`, `provider_id` when `type = interfaces`.

### Identifiers

- **UUID** strings (v4) for **node** ids in v1 — stable across export/import later.
- **Edges** have no openpfe id in v1 — identity is **`(src_id, dst_id, type)`** plus optional JSON properties (`Edge` in `openpfe-graph`). Human API: [openpfe-ui/specification.md#edges](../openpfe-ui/specification.md#edges).

## Search and similarity (v1)

Normative adapter behavior: [grafeo/design.md](./grafeo/design.md). Spike fixture and acceptance bar: [spike/program.md](./spike/program.md).

### `search_problems(query, k)` — S6

Lexical BM25 over `problem.title` and `problem.description`. Returns `SearchHit`.

HTTP: `GET /api/v1/graph/problems/search` — [openpfe-ui/specification.md#search-and-similarity](../openpfe-ui/specification.md#search-and-similarity).

| Field | Type | Notes |
|-------|------|--------|
| `node_id` | string (UUID) | Canonical problem id |
| `title` | string | Snapshot at query time |
| `score` | f64 | Higher = better match |

**Default `k`:** callers pass explicitly; integration tests use **5** unless noted.

### `find_similar(draft, k)` — S6+

Merged ranked search across three signal kinds. Input `FindSimilarDraft`.

HTTP: `POST /api/v1/graph/problems/find-similar` — [openpfe-ui/specification.md#search-and-similarity](../openpfe-ui/specification.md#search-and-similarity).

| Field | Type | Required | Notes |
|-------|------|----------|--------|
| `title` | string | yes | Draft problem title |
| `description` | string | yes | Draft problem description (may be empty) |
| `cluster_id` | string (UUID) | no | When set, enables **structural** leg within that cluster |
| `embedding` | `f32[]` | no | When set, enables Grafeo **hybrid_search** (lexical + vector) |

Returns `SimilarHit`:

| Field | Type | Notes |
|-------|------|--------|
| `node_id` | string (UUID) | Canonical problem id |
| `title` | string | Snapshot at query time |
| `score` | f64 | Merged score across legs (higher = better) |
| `match_kinds` | string[] | Subset of `lexical`, `semantic`, `structural` — explains *why* the node matched |
| `snippet` | string? | Usually `description` of the matched problem |

**`match_kinds` values (v1):**

| Kind | When set |
|------|----------|
| `lexical` | BM25 on draft `title` + `description`, or hybrid text leg |
| `semantic` | Draft has `embedding` (hybrid/vector path), **or** description BM25 proxy when title BM25 is weak |
| `structural` | `cluster_id` set; candidate shares `depends_on` neighborhood (Jaccard) with peers in cluster |

Results are sorted by `score` descending and truncated to `k`.

### Acceptance bar (integration tests)

On the shared PFE + S6+ fixture ([spike/program.md](./spike/program.md#stretch-goals--search--compare-s6)):

| Check | Pass criterion |
|-------|----------------|
| Lexical | P-lex-2 in top **3**; P-lex-3 absent from top **5** |
| Semantic | P-sem-1 in top **5** for paraphrase draft (proxy or embedding path) |
| Structural | P-struct-1 in top **5** when draft text is lexical-noise but `cluster_id` matches |
| Merged metadata | At least one hit carries multiple `match_kinds` and optional `snippet` |

## Traversal limits (context shield)

`subgraph(cluster_id, { max_nodes, max_depth })` defaults (implementation constants, overridable later):

| Limit | Default |
|-------|---------|
| `max_depth` | **3** hops from cluster boundary |
| `max_nodes` | **200** nodes |

Used by `openpfe-mcp` tools and `GET /api/v1/graph/clusters/:id/subgraph` — [openpfe-ui/specification.md](../openpfe-ui/specification.md).

## Validation

- `depends_on` edges among problems/clusters must form a **DAG** — `validate_acyclic_deps()` returns cycles for UI/MCP.
- `member_of` target must be `type = cluster`.

## Engine (v1)

**Grafeo** — locked per [decision.md](./decision.md). Version, features, intake: [grafeo/requirements.md](./grafeo/requirements.md).

IndraDB, nanograph, and SparrowDB are **not** v1 engines — see [decision.md](./decision.md) and [spike/evaluation.md](./spike/evaluation.md).

## Related

- [grafeo/specification.md](./grafeo/specification.md) — openpfe ↔ Grafeo labels and properties
- [decision.md](./decision.md) — locked engine
- [spike/](./spike/) — spike program and outcomes
- [openpfe-mcp/specification.md](../openpfe-mcp/specification.md) — agent query surfaces
