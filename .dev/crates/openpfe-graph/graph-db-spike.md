# Graph database — v1 spike program

**Read when:** proving an embedded engine can back `openpfe-graph` before phase 2 implementation merges.

**Status:** Grafeo, nanograph, and SparrowDB **macOS complete** ([grafeo-outcome.md](./grafeo-outcome.md), [nanograph-outcome.md](./nanograph-outcome.md), [sparrowdb-outcome.md](./sparrowdb-outcome.md)); engine choice in [graph-db-evaluation.md](./graph-db-evaluation.md) is **provisional** until shortlist spikes are compared against this bar.

## Purpose

The evaluation doc records constraints and an engine **shortlist**. Each spike validates candidates against **real PFE workloads**, not only “embedded + thousands of nodes.”

Each engine spike is a **time-boxed experiment** (throwaway crate or short-lived branch). Outcome: update [graph-db-evaluation.md](./graph-db-evaluation.md) with pass/fail, measurements, and the engine decision for v1.

| Engine spike | Document |
|--------------|----------|
| Grafeo (embedded LPG) | [spike-grafeo.md](./spike-grafeo.md) — [outcome](./grafeo-outcome.md) |
| nanograph (on-device LPG) | [spike-nanograph.md](./spike-nanograph.md) — [outcome](./nanograph-outcome.md) |
| SparrowDB (embedded LPG + WAL) | [spike-sparrowdb.md](./spike-sparrowdb.md) — [outcome](./sparrowdb-outcome.md) |
| ~~IndraDB 5.x + RocksDB~~ | [spike-indradb.md](./spike-indradb.md) — **closed (Fail)** |

Run **each shortlist** spike on **macOS and Linux** before locking `specification.md` engine dependency lines.

---

## v1 workload scenarios (what we test)

Normative schema and limits: [specification.md](./specification.md). MCP/UI surfaces: [openpfe-mcp/specification.md](../openpfe-mcp/specification.md), [openpfe-ui/specification.md](../openpfe-ui/specification.md).

| ID | Scenario | v1 spike? | What “pass” means |
|----|----------|-----------|-------------------|
| **S1** | **Project problem space** — decomposition creates `problem` nodes; graph is SoT under `./.openpfe/graph/` | **Required** | CRUD nodes/edges; properties as JSON; UUID ids; reopen same path and read back |
| **S2** | **User curation** — reorganize (`member_of`), add `depends_on`, update `title`/`description`/`status` | **Required** | Updates visible after reopen; delete node removes incident edges per policy |
| **S3** | **Architecture lens** — clusters, optional `component`, `interfaces` edges with contract fields | **Required** | Can list/filter by `type`; bounded slice includes contract props on `interfaces` edges |
| **S4** | **MCP work area** — bounded `subgraph(cluster_id)` for context shield | **Required** | Default `max_depth=3`, `max_nodes=200`; hard stop before 500 nodes; no full-graph scan API used |
| **S5** | **DAG validation** — `depends_on` must be acyclic | **Required** | Detect injected cycle; return cycle path(s) suitable for UI/MCP |
| **S6** | **Similar / existing problem** — “is this already recorded?” (lexical) | **Required** for engines with text index; document workaround if absent | BM25 or documented text index returns relevant hits on fixture; otherwise document acceptable v1 workaround (e.g. scan `list_nodes` on small fixture) |
| **S6+** | **Search & compare (stretch)** — ranked candidates, semantic/structural signals | **Stretch** — see below | Not required to pass engine spike; results inform v1 vs phase 2 and Grafeo vs sidecar index |

**Out of spike scope (v1 product, later work):** exposing Cypher/GQL on HTTP/MCP; LLM drill-down write tools (`openpfe_suggest_subproblems`); multi-process writers; export/import; full-text on entire repo (non-graph sources).

---

## Stretch goals — search & compare (S6+)

**Product intent (from exploration):** During drill-down or manual entry, the agent or user should discover whether a **problem is already in the project graph** before creating a duplicate — same title, same intent (different wording), or same structural role (cluster + dependencies).

Stretch work is **optional** in the engine spike time box. Run when S1–S5 (and baseline S6) are done; record outcomes in engine spike **Results** under an **S6+** subsection.

### Agent / MCP workflow (target)

```
New problem draft (title + description)
        │
        ▼
  find_similar / search_problems  ──► ranked existing `problem` nodes
        │                              (score, match kind, snippet)
        ▼
  Agent or user: link to existing | refine draft | create new node
```

v1 MCP does not define this tool yet; stretch spikes prototype the **query behavior** the store must support (later e.g. `openpfe_graph_find_similar` in [openpfe-mcp/specification.md](../openpfe-mcp/specification.md)).

### Search dimensions (explore all three)

| Kind | Question | Example signal | Stretch spike idea |
|------|----------|----------------|------------------|
| **Lexical** | Same words in title/description? | Exact title match; BM25 / FTS on `title`, `description` | Duplicate title in fixture; near-duplicate wording; unrelated control query |
| **Semantic** | Same intent, different wording? | Embedding cosine similarity on title+description | 3–5 paraphrase pairs in fixture; optional `openpfe-llm` embeddings vs engine vector index (Grafeo HNSW) — document which path |
| **Structural** | Same place in the problem graph? | Shared `member_of` cluster; similar `depends_on` in/out neighbors | Two problems in same cluster with overlapping deps vs same text in different clusters |

**Combined result (stretch):** Return a **merged ranked list** (e.g. top 10) with per-hit metadata: `node_id`, `title`, `score`, `match_kinds: ["lexical", "structural"]` — so MCP/UI can explain *why* something matched.

### Stretch fixture (add to shared fixture)

Extend [§ C. PFE seed fixture](#c-pfe-seed-fixture-all-spikes) when running S6+:

1. **P-lex-1** — `problem` with `title: "Authenticate API users"`, rich `description`
2. **P-lex-2** — duplicate title or >90% similar title (should rank #1 lexically)
3. **P-lex-3** — unrelated title (should not appear in top 5)
4. **P-sem-1** — paraphrase of P-lex-1 description, different title (e.g. “API user authentication”) — should rank high if semantic enabled
5. **P-struct-1** — same `member_of` cluster and overlapping `depends_on` as P-lex-1, different text — should rank when structural leg enabled

### Stretch checklist (per engine)

- [ ] **Lexical:** query draft text → P-lex-2 in top 3; P-lex-3 absent from top 5
- [ ] **Semantic (if attempted):** P-sem-1 in top 5 without identical title; note model/index used
- [ ] **Structural (if attempted):** P-struct-1 in top 5 when lexical score low; document query (neighbors, cluster filter, etc.)
- [ ] **Latency:** search query on fixture ~1k `problem` nodes — record ms (stretch metric)
- [ ] **False positives:** one-sentence note on confusing matches (acceptable for stretch, informs product)
- [ ] **Implementation path:** engine-native (Grafeo BM25/vector) vs graph store + **sidecar index** (document recommendation)

### How stretch affects the engine decision

| Stretch outcome | Implication |
|-----------------|-------------|
| Grafeo S6+ strong (lexical + optional semantic) with acceptable build/audit | Stronger case for Grafeo as single store |
| Shortlist engine S1–S5 pass; S6+ only via sidecar/Tantivy/LLM embeddings | Engine + separate search module in phase 2 |
| Semantic only viable with heavy deps | Defer semantic to phase 2; ship lexical dedup first |
| Structural requires custom Rust over neighbors | Expected for any engine; not a differentiator |

Stretch goals do **not** block v1 if baseline **S6** (lexical only) or documented workaround passes — they guide whether “find existing problem” ships in v1 or phase 2.

---

## Shared acceptance criteria (all engines)

Every engine spike must report results for:

### A. Embedding and placement

- [ ] Opens under a temp dir mimicking `./.openpfe/graph/store/` (or engine-documented path under `./.openpfe/graph/`)
- [ ] No separate database server process
- [ ] **macOS** and **Linux** both tested on a clean debug build

### B. `GraphStore`-shaped operations

Implement the same minimal surface in the spike (trait prototype or inline functions):

| Operation | Maps to |
|-----------|---------|
| `open` / `create` | First run vs reopen |
| `get_node` / `list_nodes` | Filter by `type`, `cluster_id` |
| `upsert_node` / `delete_node` | S1, S2 |
| `create_edge` / `delete_edge` | S2, S3 |
| `neighbors` | Adjacency for UI/MCP |
| `subgraph(cluster_id, limits)` | S4 |
| `validate_acyclic_deps()` | S5 |

Spike code does **not** need to match final public API signatures; behavior must match [design.md](./design.md) intent.

### C. PFE seed fixture (all spikes)

Build the same graph in each engine (script or test helper):

1. One `cluster` node (title, `status`)
2. Three `problem` nodes with `member_of` → cluster
3. `depends_on` chain: `p1 → p2 → p3` (DAG)
4. One `interfaces` edge between two clusters (or cluster→cluster) with `contract_body`, `version`, `consumer_id`, `provider_id`
5. Optional: ~500–1000 extra `problem` nodes (synthetic titles) for S4 performance smoke test
6. **S6+ only:** lexical / semantic / structural pairs per [Stretch goals — search & compare (S6+)](#stretch-goals--search--compare-s6)

### D. Durability smoke

- [ ] Write fixture, close store, reopen, assert counts and one known node id + edge
- [ ] Kill process mid-write (optional): document behavior — acceptable if last transaction lost, not if store fails to reopen

### E. Supply chain and build

Per [security-rust.md](../../guidelines/security-rust.md):

- [ ] Record **direct** crate version(s) and **license** (file path or SPDX)
- [ ] `cargo tree -i <engine-crate>` — note notable transitive deps (RocksDB, etc.)
- [ ] `cargo audit` clean for workspace with spike dependency, or list advisories + remediation
- [ ] Record **clean debug** `cargo build` time (cold, one number) and **release** binary size delta vs workspace without engine (or spike crate only)

### F. Backup story

- [ ] Document how operators copy/back up project graph data with server stopped
- [ ] Perform one manual copy/restore on fixture dir; reopen and verify

---

## Measurements (report in each engine doc)

| Metric | How to capture |
|--------|----------------|
| Platforms | macOS version/arch, Linux distro/arch |
| Engine version | crates.io version pinned in spike |
| Open/create latency | ms, cold dir |
| `subgraph` (defaults, fixture ~1k nodes) | ms, node/edge count returned |
| `validate_acyclic_deps` | ms on fixture |
| `list_nodes` with `type=problem` | ms, limit 200 |
| **S6+** `find_similar` / search (stretch) | ms; top-k; which kinds (lexical/semantic/structural) |
| Debug build time | `cargo build` seconds (note if sccache) |
| Release size | `target/release/openpfe` or spike bin — KB/MB delta |
| Audit | `cargo audit` summary |

Numbers need not be benchmark-grade; they must be **reproducible** (command + commit noted in spike doc).

---

## Spike execution (suggested)

1. Create `crates/openpfe-graph-spike/` (or `examples/graph-spike-<engine>/`) — **not** merged as product crate unless promoted.
2. Pin engine per [spike-indradb.md](./spike-indradb.md) / [spike-grafeo.md](./spike-grafeo.md).
3. Implement shared fixture + operation checklist.
4. Fill engine doc **Results** section; set **Recommendation**: Pass / Fail / Pass with caveats.
5. Update [graph-db-evaluation.md](./graph-db-evaluation.md) decision table and [specification.md](./specification.md) engine lines if the winner changes.

**Time box:** 1–2 days per engine including both platforms.

---

## Decision after spikes

| Outcome | Action |
|---------|--------|
| One shortlist engine pass, others fail or “caveats too heavy” | Lock winner; record S6/S6+ caveats in evaluation |
| Multiple pass | Compare S6, build/audit, latency, ops per [graph-db-evaluation.md](./graph-db-evaluation.md); prefer engine-native search if S6 is committed for v1 |
| All fail | Reopen evaluation — SQLite + FTS, or custom `redb`/LMDB layer per [graph-db-evaluation.md](./graph-db-evaluation.md) |
| Pass with caveats | Record caveats in evaluation; default to simpler engine unless S6 is committed for first release |
| **S6+ stretch** favors one engine | Note in evaluation; product may still ship lexical-only in v1 if stretch semantic deps too heavy |

---

## Related

- [graph-db-evaluation.md](./graph-db-evaluation.md) — candidates and provisional decision
- [design.md](./design.md) — `GraphStore` trait target
- [plans/openpfe-graph.md](../../plans/openpfe-graph.md) — phase 2 implementation plan
