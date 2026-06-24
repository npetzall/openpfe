# Spike: Grafeo (embedded LPG)

**Parent program:** [program.md](./program.md)

**Outcome summary:** [grafeo-outcome.md](./grafeo-outcome.md) — scenarios, use cases, findings, improvements.

**Engine:** [Grafeo](https://github.com/GrafeoDB/grafeo) — pure-Rust embeddable graph DB ([grafeo.dev](https://grafeo.dev)).

**Role:** Alternative to IndraDB if it meets the same v1 bar **and** justifies extra dependency weight (notably **S6 text search**).

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | macOS complete — Linux pending |
| **Owner** | spike branch |
| **Branch / crate** | `crates/openpfe-graph-spike` ([plan 006](../../plans/006-spike-openpfe-graph-grafeo.md)) |
| **Commit** | _record at merge_ |
| **Platforms tested** | macOS: ☑ (Darwin 25.3 arm64) — Linux: ☐ |

---

## Goals

1. Prove persistent embedded open at a project-local path under `./.openpfe/graph/`.
2. Implement [program.md](./program.md) scenarios **S1–S5** behind a `GraphStore`-shaped prototype (same fixture as IndraDB spike).
3. Prove **S6** — BM25 (or documented full-text) search on `title`/`description` for “similar / existing problem” on fixture.
4. Compare build time, binary size, and `cargo audit` against [indradb.md](./indradb.md) results.
5. Decide whether Grafeo’s breadth (query languages, RDF, vectors) is **used in v1** or **disabled** via minimal feature set.

---

## Setup

### Dependencies (pin in spike `Cargo.toml`)

Start **minimal** — add features only when a task requires them:

```toml
# Example — pin exact version in Results; adjust features after first compile
grafeo = { version = "0.5", default-features = true }
```

| Feature profile | Use in spike |
|-----------------|--------------|
| Default / `lpg` | S1–S5 CRUD + traversals |
| `ai` (or docs-equivalent for text search) | **S6** BM25 / full-text — enable when running S6 |

**Do not** enable `rdf`, `enterprise`, or `embed` (ONNX) unless a task explicitly needs them.

Record **MSRV** from crate (currently **1.91.1** on crates.io) — confirm workspace toolchain.

### Storage path

Use Grafeo persistent open API (spike doc example: `GrafeoDB::open("./path")`).

| Path | Use |
|------|-----|
| `<temp>/openpfe-graph-spike/grafeo-store/` | Dedicated dir under fake `./.openpfe/graph/` |

Document actual files created (`.grafeo`, WAL, etc.) for backup section.

### License check

- [ ] **Apache-2.0** — confirm in repo `LICENSE` and crates.io

### openpfe integration stance (spike only)

- [ ] v1 product will **not** expose Grafeo query languages on HTTP/MCP — spike may use GQL/Cypher **internally** for speed of experiment; note which queries map to future `GraphStore` Rust API
- [ ] Do **not** adopt [grafeo-mcp](https://github.com/GrafeoDB/grafeo-mcp) as product MCP — openpfe keeps [openpfe-mcp/specification.md](../openpfe-mcp/specification.md) tool names and context shield

---

## Tasks (checklist)

### 1. Lifecycle (S1)

- [ ] Create/open persistent DB at spike path
- [ ] Insert `problem` and `cluster` nodes with properties matching [specification.md](../specification.md)
- [ ] Insert `depends_on`, `member_of`, `interfaces` edges (typed relationships + JSON-like properties)
- [ ] Reopen — stable ids and property round-trip
- [ ] List/filter by label or `type` property equivalent

### 2. Curation (S2)

- [ ] Update node properties (merge/replace policy documented)
- [ ] Add/delete edges
- [ ] Delete node and confirm incident edges removed

### 3. Architecture data (S3)

- [ ] `interfaces` edge carries `contract_body`, `version`, `consumer_id`, `provider_id`
- [ ] Bounded read includes contract fields in subgraph result

### 4. Context shield (S4)

- [ ] Bounded subgraph from `cluster_id`: `max_depth=3`, `max_nodes=200`
- [ ] Enforce cap — no unbounded `MATCH (n) RETURN n` in product path; spike documents query used
- [ ] Latency on ~1k node fixture (Measurements)

### 5. Acyclic validation (S5)

- [ ] Detect cycle on `depends_on` — return structure usable by MCP/UI
- [ ] Clean DAG passes

### 6. Search (S6) — **required for Grafeo spike**

- [ ] Index or ingest text fields on `title` / `description` per Grafeo BM25 docs
- [ ] Query: find existing problem similar to a new title/description (fixture: inject near-duplicate title)
- [ ] Document false-positive/negative on 3–5 manual cases
- [ ] Record whether vector index was used (v1 default: **no** — BM25 only unless team opts in)

### 7. Stretch — search & compare (S6+)

Optional — [program.md#stretch-goals--search--compare-s6](./program.md#stretch-goals--search--compare-s6). If run:

- [x] Lexical + **semantic** (paraphrase fixture) + **structural** (cluster/deps) per parent checklist
- [x] Merged ranked results with `match_kinds` (or equivalent) documented
- [ ] Compare S6+ latency/quality vs [indradb.md](./indradb.md) stretch notes

### 8. Optional: architecture projection

- [ ] If supported: `create_projection` or label-filtered view for `cluster`/`component` only — note ergonomics for “architecture lens” (S3 view); not blocking pass

### 9. Durability and backup

- [ ] Close/reopen persistence
- [ ] Use Grafeo backup API (`backup_full` / copy dir) — restore and verify
- [ ] Document operator backup steps for `./.openpfe/graph/`

### 10. Supply chain

- [ ] `cargo tree -i grafeo` — size of graph
- [ ] `cargo audit`
- [ ] Debug build time vs IndraDB spike
- [ ] Release binary size delta vs IndraDB spike

### 11. Complexity budget

- [ ] List Grafeo features **used** vs **available but unused** in v1
- [ ] Team judgment: worth dependency vs IndraDB + separate FTS later

---

## Implementation notes

- Prefer a thin adapter: Grafeo behind same `GraphStore` shapes as [indradb.md](./indradb.md) for apples-to-apples timing.
- PFE schema: node label(s) + `type` property, or one label per `type` — document mapping.
- Context shield limits must be enforced in **adapter**, not only by query discipline.
- RDF, SPARQL, Gremlin, GraphQL: **out of scope** for openpfe v1 — do not score Grafeo on them.
- Vector search: spike S6 with **BM25 only** unless product explicitly wants embeddings in v1.

**Reference:** [README](https://github.com/GrafeoDB/grafeo/blob/main/README.md), [grafeo crate](https://crates.io/crates/grafeo).

---

## Results

**2026-05-24** — spike implementation in `openpfe-graph-spike`; `cargo test -p openpfe-graph-spike` green on macOS.

### Summary

| Item | Result |
|------|--------|
| **Recommendation** | ☑ Pass with caveats — **selected for v1** ([decision.md](../decision.md)) |
| **vs shortlist** | ☑ Grafeo selected ☐ nanograph ☐ SparrowDB; IndraDB rejected |
| **Caveats** | MSRV 1.91.1; transitive `bincode` unmaintained; young crate; BM25 ranking noisy on tiny corpus; Linux not re-run yet |
| **Pinned version** | `grafeo = 0.5.42` |
| **Features enabled** | `lpg`, `text-index` (not `embedded` / `ai` / `rdf` / `embed`) |

### Measurements

| Metric | macOS | Linux |
|--------|-------|-------|
| `open` cold (ms) | _not timed_ | |
| `subgraph` default (ms, nodes/edges) | _not timed_ (S4 test: ≤200 nodes on ~805-node fixture) | |
| `validate_acyclic_deps` (ms) | _not timed_ | |
| `list_nodes` limit 200 (ms) | _not timed_ | |
| S6 search query (ms) | _not timed_ | |
| Debug `cargo build` (s) | ~18s cold (incl. grafeo compile) | |
| Release size delta | _not measured_ | |
| `cargo audit` | exit 0; RUSTSEC-2025-0141 `bincode` allowed | |

### S6 results

| Query | Expected hit | Actual |
|-------|----------------|--------|
| `Auth gateway` | P1 or P_DUP in top 3 | ☑ top 3 contains duplicate titles |
| unrelated control | lower score than duplicates | ☑ may appear in top 5 on small corpus; not in top 2; score &lt; best duplicate |

**False positives:** On 7-node problem set, unrelated `"Completely unrelated billing export"` can appear in top 5 with weak BM25 score — acceptable for spike; product should set score floors / larger corpus.

### S6+ stretch results (optional)

| Kind | Pass? | Notes |
|------|-------|-------|
| Lexical (P-lex-2 / P-lex-3) | ☑ | `find_similar` → P-lex-2 in top 3; P-lex-3 absent from top 5 |
| Semantic (P-sem-1) | ☑ (proxy) | Description BM25 leg tags `semantic` when title BM25 weak; **not** vector/HNSW |
| Structural (P-struct-1) | ☑ | Jaccard on `depends_on` neighbors within `cluster_id`; shared hub with P-lex-1 |
| Merged ranking | ☑ | `SimilarHit { match_kinds, snippet, score }` — see `PfeGraphStore::find_similar` |
| Latency (~200 problems) | ☑ recorded | ~19s `find_similar` debug (2026-05-24); ~1k: `cargo test … s6plus_latency_bulk_1k -- --ignored` |
| False positives | note | Unrelated cluster peers can pick up weak structural score when draft is lexical-noise |
| Implementation path | ☑ | **Engine-native** lexical (`text-index`); semantic proxy via description BM25; structural in adapter Rust. Vector/`ai` feature not enabled. |

**Semantic path (stretch):** Grafeo `vector-index` / `embed` not used — paraphrase coverage via BM25 on `description` only. Full semantic needs HNSW or `openpfe-llm` embeddings + sidecar (phase 2 option).

### Transitive deps (notable)

```
grafeo v0.5.42
└── openpfe-graph-spike
    (+ grafeo-engine, grafeo-core, grafeo-storage, grafeo-adapters, bincode, regex, …)
```

### Features used in v1 (proposal)

| Feature | Use? |
|---------|------|
| GQL/Cypher internal only | Optional — spike uses Rust CRUD API |
| BM25 / text index | **Yes** (`create_text_index`, `text_search`) |
| Vector / HNSW | No |
| RDF | No |
| grafeo-mcp | No |

### Schema mapping (spike)

- Grafeo **label** = PFE `type` (`problem`, `cluster`, …).
- Canonical **UUID** in property `id` (not Grafeo internal `NodeId`).
- Edge types: `depends_on`, `member_of`, `interfaces` as relationship names.

### On-disk layout (spike path)

Persistent open uses a **`.grafeo` file** (e.g. `./.openpfe/graph/store/store.grafeo`) plus WAL/sibling files in the store directory (Grafeo `lpg` + `wal` + `grafeo-file`).

### Backup procedure

1. **Graceful:** `openpfe stop` (server not in spike scope).
2. **`backup_full`:** Grafeo API to a backup directory (see `openpfe-graph-spike` `backup_full` test), or
3. **Operator copy:** With server stopped, copy entire `./.openpfe/graph/` directory (including `store.grafeo` and WAL files).
4. Restore: copy back to project path and `GrafeoDB::open` (or `restore_to_epoch` from backup manifest for PITR).

---

## Recommendation template

**Pass:** S1–S6 required items green on both platforms; audit acceptable; build/size acceptable; clear backup path; adapter can enforce MCP context shield.

**Pass with caveats:** e.g. young crate / version churn, large binary, but S6 and traversals strong — team accepts risk.

**Fail:** Cannot meet S4/S5, persistence unreliable, audit blocker, or build cost >> IndraDB without commensurate S6 benefit.

**Locked (2026-05-25):** Grafeo selected for v1 — [decision.md](../decision.md). IndraDB rejected.

---

## Related

- [program.md](./program.md)
- [indradb.md](./indradb.md)
- [evaluation.md](./evaluation.md)
