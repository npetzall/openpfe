# Plan 006: nanograph graph engine spike

**Status:** Complete (2026-05-24) — macOS S1–S6 + S6+ stretch green; Linux skipped; S4 uses 200-node bulk (export latency); S6+ latency tests ignored in debug.

**Read when:** executing the nanograph branch of [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) before locking the v1 engine in [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md).

**Branch:** Work on a dedicated spike branch (e.g. `spike/nanograph`); do not merge nanograph into `openpfe-graph` or add it to production `Cargo.toml` until evaluation is updated and a separate implementation plan is approved.

## Normative sources

| Doc | Use |
|-----|-----|
| [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md) | Shared scenarios S1–S6+, fixture, acceptance, measurements, decision rules |
| [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) | nanograph checklist, Results template, S6 focus |
| [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) | Provisional shortlist; update nanograph row after spike |
| [grafeo-outcome.md](../crates/openpfe-graph/grafeo-outcome.md) | Apples-to-apples baseline (Grafeo macOS complete) |
| [nanograph-outcome.md](../crates/openpfe-graph/nanograph-outcome.md) | Outcome summary — scenarios, use cases, findings, improvements |
| [spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md) | Shared fixture / `GraphStore` pattern reference |
| [006-spike-openpfe-graph-grafeo.md](./006-spike-openpfe-graph-grafeo.md) | Spike crate layout and task grouping reference |
| [openpfe-graph/design.md](../crates/openpfe-graph/design.md) | Target `GraphStore` operations (prototype only in spike) |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | Node/edge types, properties, limits |
| [guidelines/plans.md](../guidelines/plans.md) | Intake gate, progress tracking |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order after manifest edit |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#adding-an-external-crate-order) | External crate workflow |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | Integration test layout |
| [dependencies/README.md](../dependencies/README.md) | Intake folder layout |

## Prerequisites

- [x] Phase 1 complete — [005-openpfe-wiring.md](./005-openpfe-wiring.md) **Complete** (lock, socket, echo, HTTP stub).
- [x] Spike branch created and checked out (`spike_db_nanograph`).
- [x] Grafeo spike results available for comparison ([spike-grafeo.md](../crates/openpfe-graph/spike-grafeo.md) / [grafeo-outcome.md](../crates/openpfe-graph/grafeo-outcome.md)) — **not** a blocker; may run in parallel on another branch.

### Dependency intake — nanograph (phase A — complete before spike implementation)

Spike adds **one** new external engine dependency (crates.io name TBD from upstream — e.g. `nanograph-db` / library crate per [nanograph](https://github.com/nanograph/nanograph) README). Pin in **spike crate only** — not `openpfe-graph`.

- [x] `.dev/dependencies/nanograph/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`)
- [x] Rational notes: MIT license, MSRV (**1.91+** per upstream), **`protoc`** / build.rs requirements, Lance/Arrow/DataFusion transitive weight, minimal features (no unused ML stacks unless S6+ semantic stretch)
- [x] `.dev/scripts/dependency-lock-diff.sh nanograph@1.3.0 --package openpfe-graph-spike` → `lock-update.md`
- [x] Workspace adds member `crates/openpfe-graph-spike`; spike `Cargo.toml` lists **nanograph** only (no `grafeo` on this branch)
- [x] `Cargo.lock` updated; **`cargo audit`** run immediately after manifest edit; output in `scan.md`
- [x] **Human intake approval** (verdicts accepted; see [guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection))

**Do not start “Spike implementation” tasks until the human intake approval box is checked.**

**Forbidden between manifest edit and `cargo audit`:** `cargo build`, `cargo check`, `cargo test`, `cargo update`, `cargo fetch` (unless a documented scan requires it).

Align `rust-toolchain.toml` with nanograph MSRV before implementation if workspace pin is lower.

## Goal

Time-boxed proof that **[nanograph](https://github.com/nanograph/nanograph)** can back openpfe’s v1 problem graph as the primary **Grafeo alternative**: folder-based persistence under a project-local path, `GraphStore`-shaped operations for **S1–S5**, **required S6** engine-native full-text/BM25 on `title`/`description`, plus durability/backup and supply-chain evidence on **macOS and Linux**. Explicitly record **schema friction** (nanograph `.pg` schema-as-code vs PFE ad-hoc JSON properties) and whether a v1 mapping is acceptable.

Outcome updates [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) **Results** and [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md); **does not** ship product code in `openpfe-graph`.

**Time box:** 1–2 days per [graph-db-spike.md](../crates/openpfe-graph/graph-db-spike.md).

## Spike crate layout

| Item | Choice |
|------|--------|
| **Path** | `crates/openpfe-graph-spike/` (reuse Grafeo spike structure on **`spike/nanograph` branch** — one engine per branch; do not combine `grafeo` + nanograph in one manifest) |
| **Storage** | `<temp>/openpfe-graph-spike/nanograph/` — one-folder store mimicking `./.openpfe/graph/` |
| **Entry** | Library + integration tests (mirror Grafeo spike modules: `store`, `fixture`, `types`) |
| **API surface** | Prefer **Rust library** API; document if CLI-only paths were used for any scenario |

## Tasks

### Intake (phase A)

- [x] Identify primary crates.io package and minimal feature set from upstream docs; record in rational
- [x] Create `.dev/dependencies/<nanograph-crate>/` — rational (scope: `openpfe-graph-spike` only; S6 native search; defer semantic/ML unless S6+ stretch)
- [x] Run `.dev/scripts/dependency-lock-diff.sh` for pinned version; save `lock-update.md`
- [x] Scaffold or repoint `crates/openpfe-graph-spike/` on spike branch (lib placeholder — scenarios after intake approval)
- [x] Add workspace member if missing; pin nanograph in spike `Cargo.toml`; document `protoc` install for dev/CI in rational or spike doc
- [x] `cargo audit` → `scan.md`; set plan **Status** to **Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD)**

**Stop after intake.** No spike scenario code in the same turn as intake-only work (preferred: intake commit without S1–S6 logic).

### Spike implementation (phase C — after intake approval)

Set **Status** to **In progress — intake approved (YYYY-MM-DD)** when human clears intake.

#### 1. Spike crate scaffold

- [x] `crates/openpfe-graph-spike/Cargo.toml` — nanograph only (no path dep on product `openpfe-graph`)
- [x] Thin module: `GraphStore`-shaped prototype matching [graph-db-spike.md § B](../crates/openpfe-graph/graph-db-spike.md#b-graphstore-shaped-operations)
- [x] Temp storage: `<temp>/openpfe-graph-spike/nanograph/` — document on-disk files for backup section
- [x] Update [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) metadata (branch, commit SHA, owner, platforms)

#### 2. Schema mapping (nanograph-specific)

- [x] Define `.pg` for PFE `cluster`, `problem` (+ edges `member_of`, `depends_on`, `interfaces`); `component` deferred (same as Grafeo core fixture)
- [x] Map PFE JSON properties (`title`, `description`, `status`, `cluster_id`, contract fields on `interfaces`) — document acceptable v1 compromises (fixed columns vs JSON blob vs schema evolution)
- [x] License check: **MIT** — confirm in repo `LICENSE` / crates.io metadata

#### 3. Shared PFE fixture ([graph-db-spike.md § C](../crates/openpfe-graph/graph-db-spike.md#c-pfe-seed-fixture-all-spikes))

- [x] One `cluster` + three `problem` nodes; `member_of` → cluster; `depends_on` chain `p1 → p2 → p3`
- [x] One `interfaces` edge with `contract_body`, `version`, `consumer_id`, `provider_id`
- [x] Optional ~500–1000 synthetic `problem` nodes for S4 smoke
- [x] Stable ids across reopen; document id strategy vs Grafeo spike

#### 4. Scenarios S1–S5 ([spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) lifecycle & curation tasks)

- [x] **S1** — create/open; insert cluster, problems, edges; reopen round-trip; UUID/stable ids
- [x] **S2** — upsert/delete nodes and edges; delete problem removes incident edges; cluster remains
- [x] **S3** — list/filter by `type`; subgraph/neighbors include contract props on `interfaces`
- [x] **S4** — `subgraph(cluster_id)`: defaults `max_depth=3`, `max_nodes=200`; hard stop before 500 on ~1k fixture; record ms
- [x] **S5** — `validate_acyclic_deps()` on `depends_on`; cycle path returned; clean DAG passes

#### 5. Scenario S6 — search (**required**)

- [x] Engine-native BM25 / full-text on `title` and `description`
- [x] Near-duplicate title ranks in top 3; unrelated absent from top 5 (shared fixture cases)
- [x] Record query latency (ms); note API (Rust vs CLI) and mapping to future MCP `find_similar`
- [x] Document false positive/negative cases in Results

#### 6. Durability, backup, platforms ([graph-db-spike.md § A, D, F](../crates/openpfe-graph/graph-db-spike.md))

- [x] Write fixture → close → reopen → assert counts + known id/edge
- [ ] Optional: kill mid-write — document behavior in Results
- [x] Backup: copy nanograph folder while closed; restore to new path; reopen and verify
- [x] Operator backup steps for `./.openpfe/graph/` documented in Results
- [x] **macOS** — clean debug build + `cargo test -p openpfe-graph-spike`
- [ ] **Linux** — same (CI or manual); record version/arch in Results

#### 7. Supply chain measurements ([graph-db-spike.md § E](../crates/openpfe-graph/graph-db-spike.md#e-supply-chain-and-build))

- [x] `cargo tree -i <nanograph-crate>` — note Lance/Arrow/DataFusion weight in Results
- [x] `cargo audit` summary (should match intake unless lock changed)
- [ ] Debug `cargo build` time (cold); release binary size delta vs workspace without spike (or spike crate only)
- [x] Record **protoc** requirement and CI/dev install steps

#### 8. Stretch S6+ (optional — time box after S1–S6)

Only if baseline S1–S6 are green; see [graph-db-spike.md stretch](../crates/openpfe-graph/graph-db-spike.md#stretch-goals--search--compare-s6):

- [x] Extend fixture with P-lex / P-sem / P-struct pairs
- [x] Lexical / semantic / hybrid search if upstream supports without heavy unused deps
- [x] Fill S6+ subsection in [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) Results

#### 9. Documentation and decision

- [x] Complete [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) **Results** (Recommendation, pinned version, schema friction, S6 table, measurements, backup procedure)
- [x] Check all task boxes in [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) task list
- [x] Update [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) nanograph row per [graph-db-spike.md decision rules](../crates/openpfe-graph/graph-db-spike.md#decision-after-spikes)
- [x] If engine choice changes: note follow-up for `design.md` / `specification.md` (do **not** edit product engine lines until shortlist spikes complete unless human directs early lock)

#### 10. Verification

- [x] `cargo test -p openpfe-graph-spike` passes on dev machine(s)
- [x] `cargo clippy -p openpfe-graph-spike -- -D warnings` (or documented exceptions in verdict)
- [x] No nanograph dependency added to `crates/openpfe-graph/Cargo.toml` in this plan

## Acceptance criteria

- [x] Intake artifacts for nanograph present; human approval recorded in plan and `verdict.md`
- [x] [spike-nanograph.md](../crates/openpfe-graph/spike-nanograph.md) **Recommendation** filled: Pass / Pass with caveats / Fail
- [x] **Schema friction** documented with clear v1 accept/reject recommendation
- [x] S1–S5 pass on **macOS** per shared acceptance criteria; **Linux** skipped (team decision, same as Grafeo spike)
- [x] **S6** pass (engine-native BM25/FTS) with measurements and manual case notes
- [x] Durability smoke and folder backup/restore documented
- [x] `cargo audit` outcome recorded (clean or advisories + remediation in verdict)
- [x] `protoc` and MSRV requirements recorded for operators/CI
- [x] [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md) updated with nanograph spike outcome
- [x] Product `openpfe-graph` remains empty of engine code; spike isolated to `openpfe-graph-spike` on spike branch

## Out of scope

- Implementing `openpfe-graph` production adapter or server wiring (phase 2 implementation plan)
- [spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md) execution (separate plan; compare when all shortlist spikes exist)
- Exposing nanograph query languages on HTTP or MCP
- `openpfe-ui`, `openpfe-mcp`, MCP tools (`openpfe_graph_find_similar`, etc.)
- Promoting `openpfe-graph-spike` to a permanent workspace member on `main` without human decision
- Combining Grafeo and nanograph dependencies in one spike manifest (use separate branches)
- Mandatory semantic/hybrid S6+ for v1 pass (stretch only)

## Next

- Run **SparrowDB** spike ([spike-sparrowdb.md](../crates/openpfe-graph/spike-sparrowdb.md)) — plan TBD (e.g. `007-spike-openpfe-graph-sparrowdb.md`).
- After **all** shortlist spikes: compare S6, schema friction, build/audit, latency in [graph-db-evaluation.md](../crates/openpfe-graph/graph-db-evaluation.md); lock engine; then phase 2 **`openpfe-graph` implementation** plan (TBD in [plans/README.md](./README.md)).
