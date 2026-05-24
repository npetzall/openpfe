# Plan 006: IndraDB graph engine spike

**Status:** Cancelled — IndraDB rejected (2026-05-24). See [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md); run Grafeo / nanograph / SparrowDB spikes instead.

**Branch:** `spike_db_indradb` — time-boxed experiment; outcomes feed [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) before phase 2 `openpfe-graph` implementation.

**Read when:** proving IndraDB 5.x + RocksDB can back `openpfe-graph` per [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) and [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md).

## Normative sources

| Doc | Use |
|-----|-----|
| [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) | Shared scenarios S1–S6+, acceptance criteria, measurements, decision rules |
| [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md) | IndraDB checklist, Results template, recommendation |
| [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) | Provisional engine choice — update after spike |
| [openpfe-graph/design.md](../crates/openpfe-graph/design.md) | `GraphStore` trait target |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | Schema, paths, traversal limits |
| [guidelines/plans.md](../guidelines/plans.md) | Progress tracking, dependency intake gate |
| [dependencies/README.md](../dependencies/README.md) | Intake artifacts for `indradb` |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order after manifest edit |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Integration test layout |

## Prerequisites

- [x] [001-scaffolding.md](./001-scaffolding.md) complete — workspace compiles; `openpfe-graph` stub present.
- [x] Phase 1 plans ([002](./002-openpfe-impl.md)–[005](./005-openpfe-wiring.md)) complete — spike does **not** depend on server/graph wiring.
- [ ] Grafeo spike ([spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md)) may run in parallel on a separate branch; **not** a blocker for this plan.

### Dependency intake (phase A — complete before spike implementation)

Spike adds **`indradb`** (with **`rocksdb-datastore`**) to a **throwaway** workspace member only — not to production `openpfe-graph` until evaluation locks the engine.

- [ ] `.dev/dependencies/indradb/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`)
- [ ] Note transitive crates in rational (e.g. `rocksdb`, `librocksdb-sys`) — no separate intake unless policy requires
- [ ] Spike crate `Cargo.toml` updated; **`cargo audit`** run immediately after manifest edit (no build/test between)
- [ ] **Human intake approval** (verdicts accepted; see [guidelines/plans.md](../guidelines/plans.md))

**Do not start tasks in “Spike implementation” until the intake approval box is checked.**

## Goal

On branch `spike_db_indradb`, build a **throwaway** spike crate that implements a `GraphStore`-shaped prototype on **IndraDB 5.x + RocksDB**, runs shared workload scenarios **S1–S5** (plus S6 documentation), captures measurements on **macOS and Linux**, and records a **Pass / Pass with caveats / Fail** recommendation in [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md) and [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md).

**Time box:** 1–2 days including both platforms ([graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md)).

## Spike crate layout

| Item | Choice |
|------|--------|
| **Path** | `crates/openpfe-graph-spike/` (workspace member; **not** promoted to product without a follow-on plan) |
| **Storage** | `<temp>/openpfe-graph-spike/store/` mimicking `./.openpfe/graph/store/` |
| **Entry** | Library + integration tests (preferred over binary-only) |
| **Async** | Sync spike OK; document if `spawn_blocking` needed from future HTTP handlers |

Add workspace member in root `Cargo.toml` during implementation (after intake approval).

## Tasks

### Phase A — Dependency intake

- [ ] Write `.dev/dependencies/indradb/rational.md` — need (embedded LPG), scope (`openpfe-graph-spike` only), trade-offs vs Grafeo/custom, Apache-2.0 license note
- [ ] Run `.dev/scripts/dependency-lock-diff.sh indradb@5 --package openpfe-graph-spike` → `lock-update.md`
- [ ] Add `crates/openpfe-graph-spike/` stub + `indradb = { version = "5", features = ["rocksdb-datastore"] }` in spike `Cargo.toml`; register workspace member
- [ ] **`cargo audit`** immediately after manifest edit → `.dev/dependencies/indradb/scan.md`
- [ ] Set **Status** → `Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD)` and **stop** (no spike `src/` beyond empty stub if needed for manifest)

### Phase B — Human review

- [ ] Human checklist per [guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection)
- [ ] After approval: **Status** → `In progress — intake approved (YYYY-MM-DD)`; check intake boxes above

### Phase C — Spike scaffold

- [ ] `crates/openpfe-graph-spike/Cargo.toml` — pin resolved `indradb` version in comments or spike doc
- [ ] `src/lib.rs` — module layout: `store` (IndraDB adapter), `fixture`, `graph_store` (prototype trait)
- [ ] `tests/` — integration tests per scenario; temp dir via `tempfile` (add intake batch if external) or std-only temp under `std::env::temp_dir()`
- [ ] License check: read IndraDB `LICENSE` / crates.io metadata; record SPDX in spike Results

### Phase D — Shared PFE fixture ([graph-db-spike.md § C](../crates/openpfe-graph/graph-db-spike.md#c-pfe-seed-fixture-all-spikes))

- [ ] `fixture::seed_pfe_graph(store)` builds:
  - [ ] One `cluster` node (`title`, `status`)
  - [ ] Three `problem` nodes with `member_of` → cluster
  - [ ] `depends_on` chain `p1 → p2 → p3` (DAG)
  - [ ] One `interfaces` edge with `contract_body`, `version`, `consumer_id`, `provider_id`
  - [ ] Optional ~500–1000 synthetic `problem` nodes for S4 latency smoke
- [ ] Stable UUID ids; document mapping (string property vs IndraDB id strategy)

### Phase E — `GraphStore`-shaped operations

Implement minimal surface ([graph-db-spike.md § B](../crates/openpfe-graph/graph-db-spike.md#b-graphstore-shaped-operations)); signatures may differ from final crate API.

- [ ] `open` / `create` — empty dir vs reopen
- [ ] `get_node` / `list_nodes` — filter by `type`, `cluster_id`
- [ ] `upsert_node` / `delete_node` — S1, S2
- [ ] `create_edge` / `delete_edge` — S2, S3
- [ ] `neighbors` — adjacency for UI/MCP-shaped payloads
- [ ] `subgraph(cluster_id, limits)` — BFS over `member_of`, `depends_on`, `interfaces`; defaults `max_depth=3`, `max_nodes=200`
- [ ] `validate_acyclic_deps()` — cycle detection on `depends_on`; return cycle path(s)

Document PFE `type` → IndraDB vertex/edge mapping in spike `README.md` or `spike-indradb.md` Implementation notes.

### Phase F — Scenario tests (S1–S5)

| Scenario | Tasks |
|----------|-------|
| **S1** Project problem space | [ ] CRUD nodes/edges; JSON properties; UUID ids; reopen same path and read back |
| **S2** User curation | [ ] Upsert title/status; add/delete `depends_on`; delete problem removes incident edges; cluster remains |
| **S3** Architecture lens | [ ] List/filter by `type`; subgraph/neighbors include contract props on `interfaces` edges |
| **S4** MCP work area | [ ] `subgraph` ≤ 200 nodes, depth 3; cap at 500; record ms on ~1k fixture |
| **S5** DAG validation | [ ] DAG fixture → ok; injected cycle → path; remove edge → ok again |

### Phase G — S6 and optional S6+

- [ ] **S6 (required doc):** note no first-class FTS in IndraDB; spike workaround (`list_nodes` + in-memory title filter on fixture scale)
- [ ] Record v1 recommendation: separate search index in phase 2 vs blocking release
- [ ] **S6+ (stretch, optional):** lexical via sidecar/brute-force; structural via neighbors + `cluster_id`; semantic defer or note `openpfe-llm` path — see [graph-db-spike.md stretch section](../crates/openpfe-graph/graph-db-spike.md#stretch-goals--search--compare-s6)

### Phase H — Durability, backup, supply chain

- [ ] Close/reopen after full fixture — counts + known node id + edge
- [ ] Optional crash mid-write — document behavior (acceptable: last tx lost; fail: store won't reopen)
- [ ] Backup: copy `store/` tree while closed; restore to new path; `open` and verify — document operator steps for `./.openpfe/graph/store/`
- [ ] `cargo tree -i indradb` — paste notable transitive deps in Results
- [ ] Record debug `cargo build` time (cold) and release size delta vs workspace without spike crate
- [ ] **`cargo audit`** summary in Results (should match intake `scan.md` unless lock changed)

### Phase I — Platform verification

- [ ] **macOS** — clean debug build; run `cargo test -p openpfe-graph-spike`
- [ ] **Linux** — same (CI or manual); record version/arch in Results

### Phase J — Documentation and decision

- [ ] Fill [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md) **Results** (measurements table, transitive deps, S6 note, backup procedure, recommendation)
- [ ] Update [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md) metadata (Status, Owner, Commit SHA, platform checkboxes)
- [ ] Update [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) decision table if recommendation is Pass / Fail / caveats change provisional choice
- [ ] If engine unchanged: leave [specification.md](../crates/openpfe-graph/specification.md) engine lines as-is; if Fail: note reopen evaluation in evaluation doc only (spec change deferred to follow-on plan)

## Acceptance criteria

- [ ] Dependency intake complete and human-approved per [guidelines/plans.md](../guidelines/plans.md)
- [ ] `cargo test -p openpfe-graph-spike` passes on **macOS** and **Linux**
- [ ] Shared fixture matches [graph-db-spike.md § C](../crates/openpfe-graph/graph-db-spike.md#c-pfe-seed-fixture-all-spikes)
- [ ] S1–S5 scenario tasks checked; S6 documented with v1 workaround recommendation
- [ ] [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) sections A (embedding), D (durability), E (supply chain), F (backup) satisfied and recorded
- [ ] Measurements table filled in [spike-indradb.md](../crates/openpfe-graph/spike-indradb.md) (reproducible commands + commit SHA)
- [ ] Clear **Recommendation**: Pass | Pass with caveats | Fail in spike doc
- [ ] `cargo fmt --all` and `cargo clippy -p openpfe-graph-spike -- -D warnings` clean (or documented exceptions)
- [ ] No production changes to `crates/openpfe-graph/` beyond what evaluation docs require (spike stays in `openpfe-graph-spike`)

## Out of scope

- Full `openpfe-graph` product crate implementation (phase 2 plan TBD)
- `openpfe-server` / `openpfe-ui` / `openpfe-mcp` integration
- Grafeo spike ([spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md)) — parallel track
- Exposing Cypher/Datalog on HTTP/MCP
- Multi-process writers; export/import; LLM drill-down write tools
- Promoting `openpfe-graph-spike` to workspace product without a new plan

## Next

- If **Pass** or **Pass with caveats:** draft phase 2 plan `007-openpfe-graph-impl` (or equivalent) — real `GraphStore` in `crates/openpfe-graph`, server lifecycle, dependency intake for production `indradb` edge
- If **Fail:** update [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md); prioritize Grafeo spike results or reopen candidates
- Run Grafeo spike on its branch before locking [specification.md](../crates/openpfe-graph/specification.md) engine lines ([graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md))
