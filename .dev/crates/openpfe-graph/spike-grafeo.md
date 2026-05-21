# Spike: Grafeo (embedded LPG)

**Parent program:** [graph-db-spike.md](./graph-db-spike.md)

**Engine:** [Grafeo](https://github.com/GrafeoDB/grafeo) — pure-Rust embeddable graph DB ([grafeo.dev](https://grafeo.dev)).

**Role:** Alternative to IndraDB if it meets the same v1 bar **and** justifies extra dependency weight (notably **S6 text search**).

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | Not started |
| **Owner** | _unassigned_ |
| **Branch / crate** | _e.g. `spike/grafeo` or `crates/openpfe-graph-spike`_ |
| **Commit** | _SHA when complete_ |
| **Platforms tested** | macOS: ☐ — Linux: ☐ |

---

## Goals

1. Prove persistent embedded open at a project-local path under `./.openpfe/graph/`.
2. Implement [graph-db-spike.md](./graph-db-spike.md) scenarios **S1–S5** behind a `GraphStore`-shaped prototype (same fixture as IndraDB spike).
3. Prove **S6** — BM25 (or documented full-text) search on `title`/`description` for “similar / existing problem” on fixture.
4. Compare build time, binary size, and `cargo audit` against [spike-indradb.md](./spike-indradb.md) results.
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
- [ ] Insert `problem` and `cluster` nodes with properties matching [specification.md](./specification.md)
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

Optional — [graph-db-spike.md](./graph-db-spike.md#stretch-goals--search--compare-s6). If run:

- [ ] Lexical + **semantic** (paraphrase fixture) + **structural** (cluster/deps) per parent checklist
- [ ] Merged ranked results with `match_kinds` (or equivalent) documented
- [ ] Compare S6+ latency/quality vs [spike-indradb.md](./spike-indradb.md) stretch notes

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

- Prefer a thin adapter: Grafeo behind same `GraphStore` shapes as [spike-indradb.md](./spike-indradb.md) for apples-to-apples timing.
- PFE schema: node label(s) + `type` property, or one label per `type` — document mapping.
- Context shield limits must be enforced in **adapter**, not only by query discipline.
- RDF, SPARQL, Gremlin, GraphQL: **out of scope** for openpfe v1 — do not score Grafeo on them.
- Vector search: spike S6 with **BM25 only** unless product explicitly wants embeddings in v1.

**Reference:** [README](https://github.com/GrafeoDB/grafeo/blob/main/README.md), [grafeo crate](https://crates.io/crates/grafeo).

---

## Results

_Fill when spike completes._

### Summary

| Item | Result |
|------|--------|
| **Recommendation** | ☐ Pass ☐ Pass with caveats ☐ Fail |
| **vs IndraDB** | ☐ Prefer Grafeo ☐ Prefer IndraDB ☐ Inconclusive |
| **Caveats** | _e.g. MSRV, churn, binary size_ |
| **Pinned version** | `grafeo = _._._` |
| **Features enabled** | _list_ |

### Measurements

| Metric | macOS | Linux |
|--------|-------|-------|
| `open` cold (ms) | | |
| `subgraph` default (ms, nodes/edges) | | |
| `validate_acyclic_deps` (ms) | | |
| `list_nodes` limit 200 (ms) | | |
| S6 search query (ms) | | |
| Debug `cargo build` (s) | | |
| Release size delta | | |
| `cargo audit` | | |

### S6 results

| Query | Expected hit | Actual |
|-------|----------------|--------|
| _example duplicate title_ | | |
| _example unrelated_ | no hit | |

### S6+ stretch results (optional)

| Kind | Pass? | Notes |
|------|-------|-------|
| Lexical (P-lex-2 / P-lex-3) | | |
| Semantic (P-sem-1) | | |
| Structural (P-struct-1) | | |
| Merged ranking | | |

### Transitive deps (notable)

```
(paste cargo tree -i grafeo excerpt)
```

### Features used in v1 (proposal)

| Feature | Use? |
|---------|------|
| GQL/Cypher internal only | |
| BM25 / text index | |
| Vector / HNSW | |
| RDF | No |
| grafeo-mcp | No |

### Backup procedure

_Operator steps for Grafeo files under `./.openpfe/graph/`._

---

## Recommendation template

**Pass:** S1–S6 required items green on both platforms; audit acceptable; build/size acceptable; clear backup path; adapter can enforce MCP context shield.

**Pass with caveats:** e.g. young crate / version churn, large binary, but S6 and traversals strong — team accepts risk.

**Fail:** Cannot meet S4/S5, persistence unreliable, audit blocker, or build cost >> IndraDB without commensurate S6 benefit.

**Prefer Grafeo over IndraDB only if:** Pass (or pass with caveats) **and** S6 is committed for near-term product **and** measurements/audit are not worse than IndraDB by team thresholds.

---

## Related

- [graph-db-spike.md](./graph-db-spike.md)
- [spike-indradb.md](./spike-indradb.md)
- [graph-db-evaluation.md](./graph-db-evaluation.md)
