# Plan 006: SparrowDB graph engine spike

**Status:** Complete (2026-05-25) — macOS S1–S6+ green; Linux skipped (same as Grafeo spike); S4 slow in debug (~3 min).

**Read when:** executing the SparrowDB branch of [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) before phase 2 `openpfe-graph` implementation.

**Branch:** Work on a dedicated spike branch (e.g. `spike/sparrowdb`); do not merge spike crate or `sparrowdb` dependency into `openpfe-graph` until [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) is updated and a separate implementation plan is approved.

## Normative sources

| Doc | Use |
|-----|-----|
| [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) | Shared scenarios S1–S6+, fixture, acceptance, measurements, decision rules |
| [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) | SparrowDB checklist, Results template |
| [sparrowdb-outcome.md](../crates/openpfe-graph/sparrowdb-outcome.md) | Outcome summary — scenarios, findings, vs Grafeo |
| [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) | Provisional engine choice — update after spike |
| [openpfe-graph/design.md](../crates/openpfe-graph/design.md) | Target `GraphStore` operations (prototype only in spike) |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | Node/edge types, properties, limits |
| [spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md) / [grafeo-outcome.md](../crates/openpfe-graph/grafeo-outcome.md) | Comparison baseline for S6, build, audit |
| [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) | Parallel shortlist spike (optional comparison in Results) |
| [guidelines/plans.md](../guidelines/plans.md) | Intake gate, progress tracking |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order after manifest edit |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#adding-an-external-crate-order) | External crate workflow |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Integration test layout |
| [dependencies/README.md](../dependencies/README.md) | Intake folder layout |

## Prerequisites

- [x] Phase 1 complete — [005-openpfe-wiring.md](./005-openpfe-wiring.md) **Complete** (lock, socket, echo, HTTP stub).
- [x] Spike branch created and checked out (`spike_db_sparrowdb`).
- [x] Grafeo spike results available for **vs Grafeo** comparison ([spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md) / [grafeo-outcome.md](../crates/openpfe-graph/grafeo-outcome.md))

### Dependency intake — `sparrowdb` (phase A — complete before spike implementation)

Spike depends on **`sparrowdb`** (crates.io or pinned git tag) in the throwaway spike crate only — not in production `openpfe-graph` until evaluation locks the engine.

Primary crate: [`sparrowdb`](https://crates.io/crates/sparrowdb) (facade over `sparrowdb-storage`, `sparrowdb-catalog`, `sparrowdb-cypher`, etc.). Pin exact version (e.g. **0.1.22** at plan authoring) in spike `Cargo.toml` and Results.

- [x] `.dev/dependencies/sparrowdb/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`)
- [x] `dependency-lock-diff.sh sparrowdb@0.1.16 --package openpfe-graph-spike` → `lock-update.md`
- [x] Note transitive crates in rational (storage, cypher, execution) — no separate intake unless policy requires
- [x] `crates/openpfe-graph-spike/Cargo.toml` lists `sparrowdb` with **minimal** surface (library embed only; no `sparrowdb-server`, `sparrowdb-cli`, `sparrowdb-mcp` path deps)
- [x] Workspace `Cargo.toml` adds member `crates/openpfe-graph-spike` if absent on this branch; `Cargo.lock` updated
- [x] **`cargo audit`** run immediately after manifest edit; output in `scan.md`
- [x] **Human intake approval** (verdicts accepted; see [guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection))

**Do not start “Spike implementation” tasks until the human intake approval box is checked.**

**Forbidden between manifest edit and `cargo audit`:** `cargo build`, `cargo check`, `cargo test`, `cargo update`, `cargo fetch` (unless a documented scan requires it).

Record SparrowDB **MSRV** and **license (MIT)** in intake rational; confirm **no C++ toolchain** required for default build (acceptance criterion).

## Goal

Time-boxed proof that **[SparrowDB](https://github.com/ryaker/SparrowDB)** can back openpfe’s v1 problem graph: embedded persistence under a project-local path with **WAL-backed durability**, `GraphStore`-shaped operations for scenarios **S1–S5**, **documented S6** (native FTS or acceptable v1 workaround), plus supply-chain, backup, and **macOS + Linux** evidence. Compare build, audit, and binary size vs **Grafeo** and **nanograph** where data exists. Outcome updates [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) **Results** and [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md); **does not** ship product code in `openpfe-graph`.

**Product API:** Rust `GraphStore` adapter only — **no Cypher/GQL on HTTP/MCP**. Cypher allowed **only** inside the spike adapter if faster than the low-level Rust API.

**Time box:** 1–2 days per [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) (both platforms).

## Spike crate layout

| Item | Choice |
|------|--------|
| **Path** | `crates/openpfe-graph-spike/` (workspace member on spike branch; **not** promoted without follow-on plan) |
| **Storage** | `<temp>/openpfe-graph-spike/sparrowdb/` mimicking `./.openpfe/graph/` |
| **Entry** | Library + integration tests (preferred over binary-only) |
| **Async** | Sync spike OK; document if `spawn_blocking` needed from future HTTP handlers |

Document on-disk layout (catalog, WAL, CSR files) in spike Results for operator backup under `./.openpfe/graph/`.

## Tasks

### Intake (phase A)

- [x] Create `.dev/dependencies/sparrowdb/` — rational (scope: `openpfe-graph-spike` only; pure-Rust durability narrative; vs Grafeo/nanograph; MIT license)
- [x] Run `.dev/scripts/dependency-lock-diff.sh` for pinned version; save `lock-update.md`
- [x] Scaffold `crates/openpfe-graph-spike/` (lib placeholder — scenarios after intake approval)
- [x] Add workspace member; pin `sparrowdb` in spike `Cargo.toml` (record git tag or crates.io version in comments)
- [x] `cargo audit` → `scan.md`; set plan **Status** to **Blocked — dependency intake complete; awaiting human review (2026-05-24)**

**Stop after intake.** No spike scenario code in the same turn as intake-only work (preferred: intake commit without S1–S6 logic).

### Spike implementation (phase C — after intake approval)

Set **Status** to **In progress — intake approved (YYYY-MM-DD)** when human clears intake.

#### 1. Spike crate scaffold

- [x] `crates/openpfe-graph-spike/Cargo.toml` — `sparrowdb` only (no path dep on `openpfe-graph` product crate)
- [x] Thin module: `GraphStore`-shaped prototype (`PfeGraphStore`) matching [graph-db-spike.md § B](../crates/openpfe-graph/graph-db-spike.md#b-graphstore-shaped-operations)
- [x] Open/create at `<temp>/openpfe-graph-spike/sparrowdb/`; document mapping to `./.openpfe/graph/`
- [x] Update [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) metadata (branch, commit SHA, owner, platforms)

#### 2. Shared PFE fixture ([graph-db-spike.md § C](../crates/openpfe-graph/graph-db-spike.md#c-pfe-seed-fixture-all-spikes))

- [x] One `cluster` + three `problem` nodes; `member_of` → cluster; `depends_on` chain `p1 → p2 → p3`
- [x] One `interfaces` edge with `contract_body`, `version`, `consumer_id`, `provider_id`
- [x] Optional ~500–1000 synthetic `problem` nodes for S4 smoke (800 in test)
- [x] Document PFE `type` / JSON properties → SparrowDB labels/properties ([README](../../../crates/openpfe-graph-spike/README.md))

#### 3. Scenarios S1–S5 ([spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) tasks)

- [x] **S1** — create/open, CRUD nodes/edges, UUID ids, JSON properties, reopen round-trip
- [x] **S2** — update properties; add/delete edges; delete node removes incident edges
- [x] **S3** — `interfaces` contract fields; bounded read includes them in subgraph
- [x] **S4** — `subgraph(cluster_id)`: `max_depth=3`, `max_nodes=200`; hard stop before 500; ~213 s debug on ~805-node fixture
- [x] **S5** — `validate_acyclic_deps()` on `depends_on`; cycle path returned; clean DAG passes

#### 4. Scenario S6 — search (document required)

- [x] Determine whether SparrowDB exposes **native FTS** (or property-index search) suitable for `title`/`description`
- [x] If native: duplicate/near-duplicate title in top 3; unrelated absent from top 5; record ms
- [x] If absent: implement and document v1 workaround (`list_nodes` + in-memory filter on fixture scale, or sidecar) per [graph-db-spike.md S6](../crates/openpfe-graph/graph-db-spike.md#v1-workload-scenarios-what-we-test)
- [x] Record recommendation: engine-native S6 vs phase-2 sidecar vs blocking v1

#### 5. Durability, backup, platforms ([graph-db-spike.md § A, D, F](../crates/openpfe-graph/graph-db-spike.md))

- [x] Write fixture → close → reopen → assert counts + known id/edge (WAL durability smoke)
- [ ] Optional: kill mid-write — document behavior in spike-sparrowdb Results
- [x] Backup: copy `sparrowdb/` tree while closed; restore to new path; `open` and verify
- [x] Operator backup steps for `./.openpfe/graph/` documented in Results (catalog, WAL, CSR files)
- [x] **macOS** clean debug build + `cargo test -p openpfe-graph-spike`
- [x] **Linux** — skipped per team (2026-05-24, same as Grafeo spike)

#### 6. Supply chain measurements ([graph-db-spike.md § E](../crates/openpfe-graph/graph-db-spike.md#e-supply-chain-and-build))

- [x] **MIT** confirmed — repo `LICENSE` or crates.io metadata; record SPDX in Results
- [x] Confirm **no C++** required for default `sparrowdb` build (note any optional features that pull native code)
- [x] `cargo tree -i sparrowdb` excerpt in Results
- [x] Debug `cargo build` time (cold, one number); release binary size delta vs workspace without spike (or vs Grafeo spike numbers)
- [x] `cargo audit` summary in Results (should match intake `scan.md` unless lock changed)
- [x] **vs Grafeo** / **vs nanograph** — build time, audit, release size (fill [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) comparison row)

#### 7. Stretch S6+ (optional — time box after S1–S6 baseline)

- [x] Extend fixture with P-lex / P-sem / P-struct pairs (`fixture::seed_s6_plus`)
- [x] Lexical / structural probes; semantic proxy (token overlap + description leg; vectors deferred to `openpfe-llm`)
- [x] `find_similar` + `tests/sparrowdb_spike_s6plus.rs`
- [x] Fill S6+ subsection in [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) Results (does not block v1)

#### 8. Documentation and decision

- [x] Complete [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) **Results** (Recommendation, measurements table, on-disk layout, S6, backup, vs Grafeo)
- [x] Update [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) SparrowDB row per [graph-db-spike.md decision rules](../crates/openpfe-graph/graph-db-spike.md#decision-after-spikes)
- [x] If engine choice changes: note required follow-up for `design.md` / `specification.md` (do **not** edit product engine lines until shortlist spikes complete unless human directs early lock)
- [x] Check corresponding boxes in [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) task list

#### 9. Verification

- [x] `cargo test -p openpfe-graph-spike` passes on **macOS**
- [x] `cargo test -p openpfe-graph-spike` passes on **Linux** — skipped (macOS verified)
- [x] `cargo fmt --all` and `cargo clippy -p openpfe-graph-spike -- -D warnings` clean (or documented exceptions in verdict)
- [x] No `sparrowdb` dependency added to `crates/openpfe-graph/Cargo.toml` in this plan

## Acceptance criteria

- [x] Intake artifacts for `sparrowdb` present; human approval recorded in plan and `verdict.md`
- [x] [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) **Recommendation** filled: Pass / Pass with caveats / Fail
- [x] S1–S5 pass on **macOS** (Linux skipped per team — same as Grafeo spike)
- [x] **S6** documented with pass (native/workaround) or explicit v1 workaround recommendation
- [x] Durability smoke and directory backup/restore documented (WAL layout included)
- [x] **No C++** for default build confirmed and recorded
- [x] `cargo audit` outcome recorded (clean or advisories + remediation in verdict)
- [x] Measurements table filled (reproducible commands + commit SHA)
- [x] **vs Grafeo** (and nanograph if available) comparison recorded in spike Results
- [x] [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) updated with SparrowDB spike outcome
- [x] Product `openpfe-graph` remains empty of engine code; spike isolated to `openpfe-graph-spike`

## Out of scope

- Implementing `openpfe-graph` production adapter or server wiring (phase 2 implementation plan)
- Exposing SparrowDB Cypher on HTTP or MCP; adopting `sparrowdb-mcp` / `sparrowdb-server` in product
- `openpfe-ui`, `openpfe-mcp`, MCP tools (`openpfe_graph_find_similar`, etc.)
- Grafeo or nanograph spike execution (separate branches/plans; consume their Results for comparison only)
- Multi-process writers; export/import; LLM drill-down write tools
- Promoting `openpfe-graph-spike` to a permanent workspace member beyond spike branch (human decides merge vs discard)

## Next

- Outcome recorded in [sparrowdb-outcome.md](../crates/openpfe-graph/sparrowdb-outcome.md).
- Complete **nanograph** spike if not done — compare S6 and folder-store story in [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md).
- After **all** shortlist spikes (Grafeo, nanograph, SparrowDB): lock engine in evaluation, then phase 2 **`openpfe-graph` implementation** plan (TBD in [plans/README.md](./README.md)).
- If **Fail** or caveats favor Grafeo/nanograph: record in evaluation; do not change [specification.md](../crates/openpfe-graph/specification.md) engine lines without follow-on plan.
