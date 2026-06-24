# Plan 007: `openpfe-graph` (phase 2)

**Status:** Complete (2026-06-06).

**Read when:** implementing `crates/openpfe-graph/` — embedded problem graph backed by **[Grafeo](https://github.com/GrafeoDB/grafeo)** per [decision.md](../crates/openpfe-graph/decision.md). **Assumes** plans [001](./001-scaffolding.md)–[005](./005-openpfe-wiring.md) are **complete** (workspace, phase 1 server/IPC/CLI). Spike evidence: [006-spike-openpfe-graph-grafeo.md](./006-spike-openpfe-graph-grafeo.md).

**Trait pattern:** **Exception** — `GraphStore` in this crate **is** the product abstraction. `openpfe-ui` and `openpfe-mcp` depend on `GraphStore` / `GrafeoGraphStore` directly; do **not** add duplicate wrapper traits in consumers ([coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports)).

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-graph/decision.md](../crates/openpfe-graph/decision.md) | Locked engine (Grafeo `0.5.42`) |
| [openpfe-graph/requirements.md](../crates/openpfe-graph/requirements.md) | FR-9.0–9.7 |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | Schema, paths, S6/S6+, traversal limits, validation |
| [openpfe-graph/design.md](../crates/openpfe-graph/design.md) | `GraphStore` surface, concurrency, backup |
| [openpfe-graph/grafeo/requirements.md](../crates/openpfe-graph/grafeo/requirements.md) | Engine pin, features, product intake |
| [openpfe-graph/grafeo/specification.md](../crates/openpfe-graph/grafeo/specification.md) | Label/property mapping |
| [openpfe-graph/grafeo/design.md](../crates/openpfe-graph/grafeo/design.md) | Adapter algorithms (`find_similar` legs, indexes) |
| [openpfe-graph/spike/program.md](../crates/openpfe-graph/spike/program.md) | Scenarios S1–S6+, shared fixture, acceptance bar |
| [openpfe-graph/spike/grafeo-outcome.md](../crates/openpfe-graph/spike/grafeo-outcome.md) | Spike evidence and caveats |
| [cross-cutting.md](../cross-cutting.md) | cwd = project root; `./.openpfe/graph/store/` |
| [guidelines/plans.md](../guidelines/plans.md) | Intake gate, progress tracking |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md) | Sync API, no HTTP in graph crate |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | `tempfile`, integration test layout |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order |

## Prerequisites

**No workspace scaffolding in this plan** — [001-scaffolding.md](./001-scaffolding.md) already created `crates/openpfe-graph/` as a workspace member; [002](./002-openpfe-impl.md)–[005](./005-openpfe-wiring.md) delivered the phase 1 binary, IPC, server, and wiring.

- [x] [001-scaffolding.md](./001-scaffolding.md) — workspace root, `crates/openpfe-graph/` member, path deps declared.
- [x] [002-openpfe-impl.md](./002-openpfe-impl.md) — CLI, ports, mocks.
- [x] [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) — IPC traits and wire API.
- [x] [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) — lock, socket, echo, HTTP stub.
- [x] [005-openpfe-wiring.md](./005-openpfe-wiring.md) — real adapters, e2e phase 1.
- [x] Engine spikes complete; v1 engine **locked to Grafeo** — [decision.md](../crates/openpfe-graph/decision.md), [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md).
- [x] Spike intake for `grafeo` exists — [.dev/dependencies/grafeo/](../dependencies/grafeo/) (spike scope).

### Dependency intake — `grafeo` on product crate (phase A — complete before implementation)

Spike intake approved **`openpfe-graph-spike` only**. **Product** placement on `openpfe-graph` requires a **new intake batch** and human pause ([grafeo/requirements.md](../crates/openpfe-graph/grafeo/requirements.md#intake)).

- [x] Update `.dev/dependencies/grafeo/rational.md` — scope: **`openpfe-graph`** (product); note accepted `bincode` advisory (RUSTSEC-2025-0141)
- [x] `dependency-lock-diff.sh grafeo@0.5.42 --package openpfe-graph` → refresh `lock-update.md` if manifest changes
- [x] `crates/openpfe-graph/Cargo.toml` — add or confirm `grafeo` **0.5.42** pin only (no new workspace members; manifest from 001 already exists)
- [x] Reuse workspace `serde`, `serde_json`, `thiserror`; add `tempfile` dev-dep for tests if not already present
- [x] **`cargo audit`** immediately after manifest edit → append `scan.md`
- [x] Update `verdict.md` — product scope accepted
- [x] Confirm workspace **MSRV ≥ Grafeo 1.91.1** (`rust-toolchain.toml`)
- [x] **Human intake approval** ([guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection))

**Do not start tasks in “Implementation” until the human intake approval box is checked.**

After phase A, set **Status** to `Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD).` and stop until approval.

## Goal

Ship **`openpfe-graph`** as the embedded, project-local problem graph: a sync **`GraphStore`** trait and **`GrafeoGraphStore`** adapter over Grafeo at **`./.openpfe/graph/store/`**, covering CRUD, bounded traversals, DAG validation, **S6** lexical search, and **S6+** merged `find_similar`. No HTTP, IPC, or MCP in this crate. All spike scenarios S1–S6+ pass in integration tests on the shared PFE fixture.

**In scope:** graph crate `src/` and `tests/` only — not workspace layout, not `openpfe-server` / `openpfe-ui` wiring (those stay out of scope).

If partial implementation already exists under `crates/openpfe-graph/`, treat it as **spike carry-over** — audit and refactor until behavior and tests match this plan.

## Public API (this plan)

```rust
// lib.rs re-exports
pub mod error;
pub mod graph_store;
pub mod grafeo_store;
pub mod schema;
pub mod types;

pub use error::{GraphError, Result};
pub use graph_store::GraphStore;
pub use grafeo_store::GrafeoGraphStore;
pub use types::{
    Edge, FindSimilarDraft, Node, NodeFilter, SearchHit, SimilarHit, SubgraphLimits,
    SubgraphResult,
};
```

| Type / fn | Purpose |
|-----------|---------|
| `GraphStore` | Product port — all graph I/O |
| `GrafeoGraphStore` | Grafeo `0.5.42` adapter |
| `GraphStore::open(store_dir)` | Open or create under **`./.openpfe/graph/store/`** (see [Storage path](#storage-path) below) |
| `GraphStore::close` | Flush and release DB handle |
| `search_problems` / `find_similar` | FR-9.6 / FR-9.7 |
| `SubgraphLimits::default()` | `max_depth = 3`, `max_nodes = 200` per [specification.md](../crates/openpfe-graph/specification.md#traversal-limits-context-shield) |

### Storage path

Normative directory: **`./.openpfe/graph/store/`** (relative to process **cwd**).

| Convention (v1) | Detail |
|-------------------|--------|
| **`GraphStore::open` argument** | **Store directory** (`./.openpfe/graph/store/`), not the raw `.grafeo` filename |
| **Grafeo DB file** | Adapter resolves `{store_dir}/store.grafeo` (+ WAL siblings in same directory) |
| **Server caller** | `openpfe-server` passes `"./.openpfe/graph/store/"` at startup; creates parent dirs |

If existing code opens `…/store/store.grafeo` directly, **refactor** to directory-based open so callers match [specification.md](../crates/openpfe-graph/specification.md).

## Module layout

```
crates/openpfe-graph/
  Cargo.toml
  src/
    lib.rs
    error.rs           # GraphError (thiserror)
    types.rs           # Node, Edge, filters, search DTOs
    schema.rs          # label/rel/prop constants; tuning (MIN_BM25_SCORE, …)
    graph_store.rs     # GraphStore trait
    grafeo_store.rs    # GrafeoGraphStore impl
  tests/
    fixture.rs         # Shared PFE + S6+ seed (spike/program.md § C)
    grafeo_integration.rs   # S1–S6, S4, S5, durability, backup, filters
    grafeo_s6plus.rs        # S6+ acceptance bar
```

Keep Grafeo types (`GrafeoDB`, `NodeId`, `Value`) **inside** `grafeo_store.rs` only.

## Known drift (refactor checklist)

Use when auditing existing `crates/openpfe-graph/` work:

| Area | Normative target | Likely drift |
|------|------------------|--------------|
| Open path | Directory `./.openpfe/graph/store/` | Tests/callers pass `store.grafeo` file path |
| `create` vs `open` | `open` creates store dir + DB if missing ([design.md](../crates/openpfe-graph/design.md)) | No separate `create`; document in trait rustdoc |
| Integration tests | Full spike bar ([spike/program.md](../crates/openpfe-graph/spike/program.md)) | S4, S5, durability, backup, `list_nodes` filters may be missing |
| Bulk fixture | ~800 synthetic `problem` nodes for S4 smoke | `seed_bulk` helper may be absent |
| `member_of` validation | Target must be `type = cluster` ([specification.md](../crates/openpfe-graph/specification.md#validation)) | May be unchecked on `create_edge` |
| `cluster_id` consistency | Denormalized `cluster_id` must match `member_of` | Document; optional upsert guard |
| Hybrid / vector leg | `find_similar` with `draft.embedding` → `hybrid_search` ([grafeo/design.md](../crates/openpfe-graph/grafeo/design.md)) | May be untested |
| Public helpers | `node_count`, `db_path` | OK for tests; keep `#[doc(hidden)]` or test-only if not in spec |
| Intake | Product `grafeo` on `openpfe-graph` | Manifest may predate human product approval |

## Tasks

### Implementation (phase C — after intake approval)

#### 1. Errors and public surface

Crate directory and `Cargo.toml` already exist from 001 — implement or align modules only.

- [x] `error.rs` — `GraphError` with `Grafeo(#[from] grafeo::Error)` and `Message(String)`; `Result<T>`
- [x] `lib.rs` — module graph and public re-exports per [Public API](#public-api-this-plan)
- [x] Crate-level rustdoc links to `.dev/crates/openpfe-graph/specification.md`

#### 2. Types and schema

- [x] `types.rs` — `Node` (`id`, `type`, flattened JSON props), `Edge`, `NodeFilter`, `SubgraphLimits` (defaults 3 / 200), `SubgraphResult`, `SearchHit`, `FindSimilarDraft`, `SimilarHit`
- [x] `schema.rs` — `label::{PROBLEM, CLUSTER, COMPONENT, CONTRACT}`, `rel::{DEPENDS_ON, MEMBER_OF, INTERFACES}`, `prop::*`, `SUBGRAPH_EDGE_TYPES`, `MIN_BM25_SCORE`, `DEFAULT_EMBEDDING_DIMENSIONS`, `STRUCTURAL_SCORE_SCALE` (or equivalent per [grafeo/design.md](../crates/openpfe-graph/grafeo/design.md))

#### 3. `GraphStore` trait

- [x] `graph_store.rs` — full v1 surface ([design.md](../crates/openpfe-graph/design.md#graphstore-trait-v1)):
  - `open` / `close`
  - `upsert_node`, `get_node`, `list_nodes`, `delete_node`
  - `create_edge`, `delete_edge`
  - `neighbors(id, edge_types?, outgoing)`
  - `subgraph(cluster_id, limits)`
  - `validate_acyclic_deps() -> Vec<Vec<String>>` (cycle paths as UUID lists)
  - `search_problems(query, k)`
  - `find_similar(draft, k)`
  - `backup_full(backup_dir)`
  - `rebuild_text_indexes`, `rebuild_vector_index`
- [x] Trait methods sync; document that HTTP layers use `spawn_blocking` ([design.md](../crates/openpfe-graph/design.md))

#### 4. `GrafeoGraphStore` — open, indexes, id map

- [x] `GrafeoGraphStore` fields: `GrafeoDB`, store path, `id_to_node` / `node_to_id`, index-ready flag
- [x] `open(store_dir)` — `create_dir_all`, open `{store_dir}/store.grafeo`, `rebuild_id_index` from property `id`
- [x] `close` — `GrafeoDB::close`
- [x] `ensure_problem_indexes` — lazy BM25 on `problem.title` / `problem.description`; HNSW on `problem.embedding` when first problem exists ([grafeo/design.md](../crates/openpfe-graph/grafeo/design.md#search-indexes))
- [x] JSON ↔ Grafeo `Value` conversion (scalars, strings; `embedding` as `Value::Vector`; nested JSON as string in v1)

#### 5. CRUD and edges

- [x] `upsert_node` — inject `id` property; label = `node_type`; update vs `create_node_with_props`
- [x] `get_node` / `list_nodes` — filter by label (`type`), `cluster_id`, `limit`
- [x] `delete_node` — remove from id map; Grafeo deletes incident edges per engine policy
- [x] `create_edge` / `delete_edge` — typed directed edges; edge JSON properties
- [x] **Validation:** `member_of` target must exist and have label `cluster`; return `GraphError::msg` on violation
- [x] **Optional guard:** when upserting `problem` with `cluster_id`, warn or reject if no matching `member_of` edge (document v1 behavior in rustdoc)

#### 6. Traversal

- [x] `neighbors` — filter by edge types and direction (outgoing vs incoming)
- [x] `subgraph` — BFS from cluster id over `SUBGRAPH_EDGE_TYPES` (undirected adjacency for traversal); enforce `max_depth` and `max_nodes`; hard stop before unbounded scan
- [x] Return `SubgraphResult { nodes, edges }` with openpfe UUIDs and edge properties (incl. `interfaces` contract fields)

#### 7. DAG validation

- [x] `validate_acyclic_deps` — DFS on `depends_on` only; return each detected cycle as ordered UUID path
- [x] Empty graph / clean DAG → empty `Vec`

#### 8. S6 — `search_problems`

- [x] Merge BM25 from `text_search` on `problem.title` and `problem.description`; max score per node; apply `MIN_BM25_SCORE`
- [x] Sort descending; truncate to `k`; return `SearchHit { node_id, title, score }`

#### 9. S6+ — `find_similar`

Implement per [grafeo/design.md](../crates/openpfe-graph/grafeo/design.md#find_similar-s6--merged-ranking):

- [x] Fetch up to `max(k × 4, 10)` candidates per leg
- [x] **Lexical leg** — BM25 on combined title + description; tag `lexical`
- [x] **Semantic leg (no embedding)** — description BM25 &gt; 1.5× title BM25 and ≥ `MIN_BM25_SCORE` → tag `semantic`
- [x] **Semantic leg (embedding)** — Grafeo `hybrid_search` on draft text + vector; tags `lexical` + `semantic`
- [x] **Structural leg** — when `draft.cluster_id` set: Jaccard on `depends_on` in+out neighbor sets vs cluster peers; scale × `STRUCTURAL_SCORE_SCALE`; skip candidates with empty deps; tag `structural`
- [x] Merge by `node_id` (max score); populate `snippet` from `description`; sort; truncate to `k`

#### 10. Backup and index maintenance

- [x] `backup_full` — `GrafeoDB::backup_full` into operator-chosen directory
- [x] `rebuild_text_indexes` / `rebuild_vector_index` — delegate to Grafeo; document post–bulk-import use

#### 11. Shared test fixture

- [x] `tests/fixture.rs` — stable UUIDs; `seed_core` (cluster, 3 problems, `depends_on` chain, `interfaces` edge); `seed_s6_lexical`; `seed_s6_plus` (P-lex-1…P-struct-1 per [spike/program.md](../crates/openpfe-graph/spike/program.md#stretch-fixture-add-to-shared-fixture))
- [x] `seed_bulk(store, n)` — synthetic `problem` nodes (~800) for S4 caps test

#### 12. Integration tests — scenarios S1–S6

| Test | Scenario | Pass criterion |
|------|----------|----------------|
| `s1_lifecycle_and_reopen_roundtrip` | S1 | Seed, close, reopen; known node props + count |
| `s2_curation_update_and_delete` | S2 | Upsert merges props; delete removes node |
| `s3_interfaces_contract_in_subgraph` | S3 | `contract_body` visible in subgraph edges |
| `s4_subgraph_respects_caps` | S4 | Defaults on bulk fixture: ≤ 200 nodes, &lt; 500, depth respected |
| `s5_acyclic_validation` | S5 | Clean DAG → no cycles; injected cycle returns path |
| `s6_bm25_search_duplicate_title` | S6 | Duplicate in top 3; unrelated not in top 2 |
| `durability_close_reopen_counts` | Durability | Node count stable after close + reopen |
| `backup_full_and_directory_copy` | Backup | Backup + copy store dir; reopen and read |
| `list_nodes_by_type_and_cluster` | Filters | `list_nodes` by `type` + `cluster_id` |
| `neighbors_outgoing_depends_on` | Adjacency | `neighbors` with type filter |
| `member_of_rejects_non_cluster_target` | Validation | Error when target is not `cluster` |

- [x] All tests use `tempfile`; store under `…/.openpfe/graph/store/` layout mimicking production
- [x] `#[ignore]` optional: `s6plus_latency_bulk_1k` — manual release-build timing ([spike/grafeo-outcome.md](../crates/openpfe-graph/spike/grafeo-outcome.md)) — deferred (optional)

#### 13. Integration tests — S6+ ([specification.md](../crates/openpfe-graph/specification.md#acceptance-bar-integration-tests))

File: `tests/grafeo_s6plus.rs`

- [x] `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` — P-lex-2 top 3; P-lex-3 absent top 5
- [x] `s6plus_semantic_paraphrase_in_top5` — P-sem-1 top 5 (proxy path)
- [x] `s6plus_structural_shared_deps_when_lexical_weak` — P-struct-1 top 5; `match_kinds` contains `structural`
- [x] `s6plus_merged_hit_carries_multiple_kinds` — ≥2 `match_kinds` and `snippet` on at least one hit
- [x] `s6plus_hybrid_when_embedding_present` — synthetic `f32` embedding; `semantic` + `lexical` kinds (smoke; dimension matches `DEFAULT_EMBEDDING_DIMENSIONS`)

#### 14. Docs and spike cross-links

- [x] Ensure [spike/grafeo-outcome.md](../crates/openpfe-graph/spike/grafeo-outcome.md) **Implementation** line points at product crate tests (not only spike crate)
- [x] No change to locked [decision.md](../crates/openpfe-graph/decision.md) unless engine version bump (separate intake)

## Acceptance criteria

- [x] All implementation task boxes above are `[x]`
- [x] Dependency intake (product `grafeo`) approved; `verdict.md` scope includes `openpfe-graph`
- [x] `cargo test -p openpfe-graph` passes (all non-ignored tests)
- [x] `cargo clippy -p openpfe-graph -- -D warnings` clean (or documented in `verdict.md`)
- [x] S6+ acceptance bar met on shared fixture ([specification.md](../crates/openpfe-graph/specification.md#acceptance-bar-integration-tests))
- [x] `GraphStore::open("./.openpfe/graph/store/")` works from temp cwd with directory convention documented
- [x] Grafeo types not leaked in public API (`lib.rs` exports)
- [x] `cargo audit` clean or documented allowed advisories in `verdict.md`
- [x] Plan **Status** → `Complete (2026-06-06)`

## Out of scope

- Workspace / member **scaffolding** — covered by [001-scaffolding.md](./001-scaffolding.md) (complete)
- Phase 1 **CLI, IPC, server, wiring** — [002](./002-openpfe-impl.md)–[005](./005-openpfe-wiring.md) (complete)
- **`openpfe-server`** graph wiring — open at startup, `Arc<GrafeoGraphStore>` in `AppState` (separate plan)
- **`openpfe-ui`** HTTP routes (`/api/v1/graph/*`) — separate plan
- **`openpfe-mcp`** graph tools — phase 3
- **`openpfe-llm`** embeddings in production path — optional; S6+ proxy path suffices for v1 graph tests
- Cypher/GQL on HTTP/MCP; Grafeo `server` / `grafeo-mcp` features
- Export/import, multi-writer, markdown legacy problem-tree
- Linux-only CI matrix (macOS spike evidence accepted; Linux re-run deferred per [decision.md](../crates/openpfe-graph/decision.md))

## Next

- Parallel phase 2: **`openpfe-webui`** and **`openpfe-ui`** implementation plans (TBD — mount graph HTTP on `GraphStore`)
- Server graph lifecycle plan (open on start, graceful close, backup on shutdown) — after this crate is **Complete**
- Phase 3: `openpfe-mcp` read-only graph tools consuming `GraphStore`
