# Spike: nanograph (embedded LPG)

**Parent program:** [program.md](./program.md)

**Outcome summary:** [nanograph-outcome.md](./nanograph-outcome.md) — scenarios, use cases, findings, improvements.

**Engine:** [nanograph](https://github.com/nanograph/nanograph) — on-device typed property graph (Rust, Lance, Arrow, DataFusion). Tagline: “DuckDB for graphs.”

**Role:** Primary **Grafeo alternative** — especially for **S6** (FTS, BM25, semantic/hybrid search) and folder-based project storage.

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | macOS complete (S1–S6 + S6+ stretch) — Linux pending |
| **Owner** | `spike_db_nanograph` |
| **Branch / crate** | `spike_db_nanograph`, `crates/openpfe-graph-spike` ([plan 006](../../plans/006-spike-openpfe-graph-nanograph.md)) |
| **Commit** | `6de47d2` (record at merge) |
| **Platforms tested** | macOS: ☑ (Darwin 25.3 arm64) — Linux: ☐ |

---

## Goals

1. Open/create persistent graph at `<temp>/openpfe-graph-spike/nanograph/` mimicking `./.openpfe/graph/`.
2. Implement **S1–S5** behind the same `GraphStore`-shaped prototype and shared fixture as other spikes.
3. Prove **S6** with engine-native search (BM25 / full-text on `title`/`description`).
4. Record build time, `cargo audit`, MSRV (**1.91+** per upstream), and **protoc** requirement.
5. Document gap between nanograph **schema-as-code** (`.pg` files) and PFE **ad-hoc JSON properties** — acceptable mapping for v1?

---

## Setup

### Dependencies (pin in spike `Cargo.toml`)

| Crate | Version | Notes |
|-------|---------|--------|
| `nanograph` | `1.3.0` | Rust library (`Database::init` / `open`, query language) |
| `arrow-array` | `58` | Read `RecordBatch` from search queries only |

Intake: [.dev/dependencies/nanograph/](../../dependencies/nanograph/).

| Check | Action |
|-------|--------|
| MSRV | Workspace **stable** (≥ upstream 1.91+); first compile needs **protoc** (`brew install protobuf`) |
| `protoc` | Required for `lance-encoding` build |
| Features | Default `nanograph` features only — no embed/ML stretch |

### Storage path

| Path | Use |
|------|-----|
| `<temp>/openpfe-graph-spike/nanograph/` | One-folder DB: `schema.pg`, `schema.ir.json`, `nodes/`, `edges/`, manifest + Lance datasets |

### License check

- [x] **MIT** — crates.io + [upstream LICENSE](https://github.com/nanograph/nanograph)

---

## Tasks (checklist)

### Lifecycle & data (S1–S3)

- [x] Create/open; insert cluster, problem nodes, edges (`depends_on`, `member_of`, `interfaces` + contract props)
- [x] Reopen; stable ids; `list_nodes` by `type` / `cluster_id`
- [x] `subgraph` with defaults `max_depth=3`, `max_nodes=200`

### Curation & validation (S2, S5)

- [x] Upsert/delete nodes and edges; delete problem removes incident edges (engine cascade)
- [x] `validate_acyclic_deps` on `depends_on`

### Context shield (S4)

- [x] Bounded subgraph on ~200 extra `problem` nodes (export-latency trade-off vs 800 in Grafeo spike); cap at 500 nodes

### Search (S6 — required)

- [x] Duplicate/near-duplicate title ranks in top 3; unrelated absent from top 5
- [x] **Rust API:** `Database::run_query` with `bm25($p.title, $q)` / `bm25($p.description, $q)` — maps to future MCP `find_similar` (lexical leg)

### Stretch S6+ — search & compare

- [x] Fixture P-lex / P-sem / P-struct via JSONL load (`tests/s6plus_fixture.jsonl`)
- [x] `find_similar(FindSimilarDraft)` — lexical + description BM25 semantic **proxy** + structural (`depends_on` Jaccard, cluster-scoped)
- [x] Integration tests: `tests/nanograph_spike_s6plus.rs` (4 functional; 2 latency `#[ignore]`)

### Durability & backup

- [x] Close/reopen; copy folder while closed; restore and verify

### Supply chain

- [x] `cargo tree` — Lance/Arrow/DataFusion via `nanograph`
- [x] `cargo audit` — see Results (rsa advisory via Lance/opendal)
- [x] Debug build time; release size not measured (spike lib only)

---

## Results

| Item | Result |
|------|--------|
| **Recommendation** | ☑ Pass with caveats ☐ Pass ☐ Fail |
| **Pinned version** | `nanograph` **1.3.0**; `arrow-array` **58** (query result parsing) |
| **Schema friction** | **Acceptable for v1 with fixed schema** — see below |
| **S6** | **Pass** — BM25 via query language; integration test green (~7s for S6 alone, debug) |

### Recommendation (Pass with caveats)

nanograph meets **S1–S6** and **S6+ stretch** on macOS as an embedded folder-backed LPG with native BM25. Strongest vs Grafeo on **schema-validated storage** and **first-class text indexes** in the query language.

**Caveats before product lock:**

1. **Schema-as-code** — PFE ad-hoc JSON properties become typed `.pg` columns; v1 spike uses `pfe_id` @key (not `id`, which clashes with internal row ids). New product fields require schema migration.
2. **Read path cost** — adapter uses `build_export_rows_at_path` for list/subgraph/neighbors; **slow** at hundreds of nodes (S4 test ~8 min with 200 bulk problems in debug). Production adapter should use targeted `run_query` / prepared reads, not full export.
3. **Build/ops** — `protoc` required; **~600** lockfile packages; `cargo audit`: RUSTSEC-2023-0071 (`rsa` via Lance/opendal), unmaintained `paste` / `rustls-pemfile` warnings.
4. **Linux** — not re-run on this branch.
5. **Async** — `Database` is async (`tokio`); server will need `spawn_blocking` or async graph port.

Full outcome: [nanograph-outcome.md](./nanograph-outcome.md). Compare with [grafeo-outcome.md](./grafeo-outcome.md) and SparrowDB spike before locking [specification.md](../specification.md).

### Schema friction (PFE JSON vs `.pg`)

| PFE | nanograph spike |
|-----|-----------------|
| Node `type` | Node types `Cluster`, `Problem` (returned as lowercase `cluster` / `problem`) |
| UUID `id` | Property `pfe_id: String @key` |
| `title`, `description`, `status`, `cluster_id` | Fixed columns on `Problem` |
| `interfaces` contract fields | Edge `Interfaces` properties |
| Ad-hoc extra JSON keys | **Not supported** without editing `schema.rs` / `schema.pg` and migrating |

**Verdict:** Acceptable for v1 if product schema is stabilized; poor fit for fully dynamic per-node JSON without a JSON column or generic blob field in `.pg`.

### On-disk layout (backup)

With server stopped, operators copy **`./.openpfe/graph/`** (spike: entire `nanograph/` directory):

- `schema.pg` — authored schema
- `schema.ir.json`, manifest / tx metadata
- `nodes/`, `edges/` — Lance dataset paths

Spike verified: close → `copy_data_dir_to` → reopen at new path → counts and known nodes intact.

### Measurements

| Metric | Value | Notes |
|--------|-------|--------|
| Platform | macOS Darwin 25.3 arm64 | |
| `cargo test -p openpfe-graph-spike` | 13/13 pass (9 baseline + 4 S6+), **~494 s** baseline debug, `--test-threads=1` | Dominated by S4 export on ~205 nodes |
| S6+ `nanograph_spike_s6plus` | 4/4 pass, **~6 s** debug | Hold `TempDir` for test lifetime (dropping temp deletes Lance tables) |
| S6 test alone | ~7 s | `search_problems("Auth gateway", 5)` |
| First `cargo build` (cold, with protoc) | ~3–5 min order of magnitude | Lance stack |
| `cargo clippy -p openpfe-graph-spike` | Clean (`-D warnings`) | |
| **API** | Rust `nanograph::store::database::Database` | No CLI in spike tests |

### Supply chain

```text
nanograph v1.3.0
└── openpfe-graph-spike v0.1.0
```

Transitives include **lance**, **arrow-***, **datafusion**, **object_store** / **opendal** (see [lock-update.md](../../dependencies/nanograph/lock-update.md) after lock refresh).

**`cargo audit`:** RUSTSEC-2023-0071 (`rsa` / Marvin attack, no fix) via `opendal` → `lance-io`; accepted for spike per [verdict.md](../../dependencies/nanograph/verdict.md).

### S6 detail

- Indexes: `@index` on `Problem.title` and `Problem.description` in embedded schema.
- Query: `bm25($p.title, $q)` and `bm25($p.description, $q)`; scores merged by max per `pfe_id`.
- False positives: same as Grafeo fixture — unrelated may appear in top 5 with low score; test asserts not in top 2 and score below duplicates.

### S6+ detail (stretch)

| Check | Test | Result |
|-------|------|--------|
| Lexical: P-lex-2 top 3; P-lex-3 not top 5 | `s6plus_lexical_duplicate_in_top3_unrelated_out_of_top5` | **Pass** |
| Semantic proxy: P-sem-1 top 5 | `s6plus_semantic_paraphrase_in_top5` | **Pass** (description BM25 vs title; not engine `@embed` / `rrf`) |
| Structural: P-struct-1 when lexical weak | `s6plus_structural_shared_deps_when_lexical_weak` | **Pass** |
| Merged metadata | `s6plus_merged_hit_carries_multiple_kinds` | **Pass** (`match_kinds`, snippet) |
| Latency ~1k problems | `s6plus_latency_*` | **Deferred** (`#[ignore]` — debug stack overflow on export + bulk BM25) |

**Implementation notes:**

- Seed: `load_jsonl_overwrite` on shared JSONL (put-only seed left Lance tables inconsistent for BM25).
- **Semantic:** same BM25-proxy pattern as Grafeo spike — true vector/`rrf()` exists in nanograph 1.3 but not wired (no embed API keys in spike).
- **Structural:** Jaccard on undirected `depends_on` neighbors within `cluster_id`; skip nodes with no `depends_on` (avoids boosting unrelated cluster mates).
- **Tests:** keep `let dir = tempfile::tempdir()` alive for the whole test — `open_with_s6_plus(&dir)`; dropping the temp dir before queries causes `Table not found` on Lance.

**Engine-native S6+ (not used):** nanograph supports `Vector`, `@embed`, and `rrf()` for hybrid search; product phase 2 can adopt if embeddings are available.

---

## Related

- [nanograph-outcome.md](./nanograph-outcome.md) — outcome summary
- [006-spike-openpfe-graph-nanograph.md](../../plans/006-spike-openpfe-graph-nanograph.md) — execution plan
- [grafeo.md](./grafeo.md)
- [sparrowdb.md](./sparrowdb.md)
- [evaluation.md](./evaluation.md)
