# Plan 006: Grafeo graph engine spike

**Status:** In progress — macOS S1–S6+ green (2026-05-24); Linux skipped; formal ~1k latency optional (`--ignored` test).

**Read when:** executing the Grafeo branch of [spike/program.md](../crates/openpfe-graph/spike/program.md) before phase 2 `openpfe-graph` implementation.

**Branch:** Work on a dedicated spike branch (e.g. `spike/grafeo`); do not merge spike crate or `grafeo` dependency into `openpfe-graph` until evaluation is updated and a separate implementation plan is approved.

## Normative sources

| Doc | Use |
|-----|-----|
| [spike/program.md](../crates/openpfe-graph/spike/program.md) | Shared scenarios S1–S6, fixture, acceptance, measurements |
| [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) | Grafeo checklist, Results template, S6/S6+ detail |
| [spike/grafeo-outcome.md](../crates/openpfe-graph/spike/grafeo-outcome.md) | Outcome summary — scenarios, use cases, findings, improvements |
| [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md) | Provisional IndraDB decision; update after spike |
| [openpfe-graph/design.md](../crates/openpfe-graph/design.md) | Target `GraphStore` operations (prototype only in spike) |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | Node/edge types, properties, limits |
| [spike/indradb.md](../crates/openpfe-graph/spike/indradb.md) | Apples-to-apples comparison baseline (may run in parallel on another branch/plan) |
| [guidelines/plans.md](../guidelines/plans.md) | Intake gate, progress tracking |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order after manifest edit |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#adding-an-external-crate-order) | External crate workflow |
| [dependencies/README.md](../dependencies/README.md) | Intake folder layout |

## Prerequisites

- [x] Phase 1 complete — [005-openpfe-wiring.md](./005-openpfe-wiring.md) **Complete** (lock, socket, echo, HTTP stub).
- [ ] Spike branch created and checked out.

### Dependency intake — `grafeo` (phase A — complete before spike implementation)

Spike depends on **one** new external crate: **`grafeo`** (pinned in spike crate only — not `openpfe-graph` yet).

- [x] `.dev/dependencies/grafeo/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`)
- [x] `dependency-lock-diff.sh grafeo@0.5.42 --package openpfe-graph-spike` → `lock-update.md`
- [x] `crates/openpfe-graph-spike/Cargo.toml` lists `grafeo` with **minimal** features (`lpg` only; `text-index` / `ai` deferred to S6 — see [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md#dependencies-pin-in-spike-cargotoml))
- [x] Workspace `Cargo.toml` adds member `crates/openpfe-graph-spike`; `Cargo.lock` updated
- [x] **`cargo audit`** run immediately after manifest edit; output in `scan.md`
- [x] **Human intake approval** (verdicts accepted; see [guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection))

**Do not start “Spike implementation” tasks until the human intake approval box is checked.**

**Forbidden between manifest edit and `cargo audit`:** `cargo build`, `cargo check`, `cargo test`, `cargo update`, `cargo fetch` (unless a documented scan requires it).

Record Grafeo **MSRV** (crates.io currently **1.91.1**) in intake rational; confirm workspace toolchain before implementation.

## Goal

Time-boxed proof that **[Grafeo](https://github.com/GrafeoDB/grafeo)** can back openpfe’s v1 problem graph: embedded persistence under a project-local path, `GraphStore`-shaped operations for scenarios **S1–S5**, **required S6** lexical search on `title`/`description`, plus supply-chain and backup evidence on **macOS and Linux**. Outcome updates [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) **Results** and [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md); **does not** ship product code in `openpfe-graph`.

## Tasks

### Intake (phase A)

- [x] Create `.dev/dependencies/grafeo/` — rational (scope: `openpfe-graph-spike` only; BM25 feature deferred in rational until S6 task; no `rdf` / `enterprise` / ONNX `embed`)
- [x] Run `.dev/scripts/dependency-lock-diff.sh` for pinned version; save `lock-update.md`
- [x] Scaffold `crates/openpfe-graph-spike/` (lib placeholder — scenarios after intake approval)
- [x] Add workspace member; pin `grafeo` in spike `Cargo.toml`
- [x] `cargo audit` → `scan.md`; set plan **Status** to **Blocked — dependency intake complete; awaiting human review (2026-05-24)**

**Stop after intake.** No spike scenario code in the same turn as intake-only work (preferred: intake commit without S1–S6 logic).

### Spike implementation (phase C — after intake approval)

Set **Status** to **In progress — intake approved (YYYY-MM-DD)** when human clears intake.

#### 1. Spike crate scaffold

- [x] `crates/openpfe-graph-spike/Cargo.toml` — `grafeo` only (no path dep on `openpfe-graph` product crate)
- [x] Thin module: `GraphStore`-shaped prototype (`PfeGraphStore`) matching [spike/program.md § B](../crates/openpfe-graph/spike/program.md#b-graphstore-shaped-operations)
- [x] Temp storage: `…/grafeo-store/store.grafeo` — documented in [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) Results
- [x] Update [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) metadata (branch, commit SHA, owner)

#### 2. Shared PFE fixture ([spike/program.md § C](../crates/openpfe-graph/spike/program.md#c-pfe-seed-fixture-all-spikes))

- [x] One `cluster` + three `problem` nodes; `member_of` → cluster; `depends_on` chain `p1 → p2 → p3`
- [x] One `interfaces` edge with `contract_body`, `version`, `consumer_id`, `provider_id`
- [x] Optional ~500–1000 synthetic `problem` nodes for S4 smoke (800 in test)
- [x] Document PFE schema mapping (labels vs `type` property)

#### 3. Scenarios S1–S5 ([spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) tasks 1–5)

- [x] **S1** — create/open, CRUD nodes/edges, UUID ids, JSON properties, reopen round-trip
- [x] **S2** — update properties; add/delete edges; delete node removes incident edges (Grafeo deletes incident edges)
- [x] **S3** — `interfaces` contract fields; bounded read includes them in subgraph
- [x] **S4** — `subgraph(cluster_id)`: `max_depth=3`, `max_nodes=200`; hard stop before 500; adapter-enforced caps
- [x] **S5** — `validate_acyclic_deps()` on `depends_on`; cycle path returned; clean DAG passes

#### 4. Scenario S6 — search (**required for Grafeo**)

- [x] Enable Grafeo `text-index` feature (`lpg` + `text-index`)
- [x] Index `title` and `description` via `create_text_index`
- [x] Near-duplicate title → top-3 hit; unrelated scored below duplicates (see Results false-positive note)
- [x] Document false positive/negative cases in [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) Results
- [ ] Record S6 query latency in Results (formal ms)

#### 5. Durability, backup, platforms ([spike/program.md § A, D, F](../crates/openpfe-graph/spike/program.md))

- [x] Write fixture → close → reopen → assert counts + known id/edge
- [ ] Optional: kill mid-write — document behavior in spike-grafeo Results
- [x] Backup: `backup_full` + directory copy; reopen verify
- [x] Operator backup steps for `./.openpfe/graph/` documented in Results
- [x] **macOS** clean debug build + `cargo test -p openpfe-graph-spike`
- [x] **Linux** — skipped per team (2026-05-24)

#### 6. Supply chain measurements ([spike/program.md § E](../crates/openpfe-graph/spike/program.md#e-supply-chain-and-build))

- [x] Apache-2.0 confirmed (crates.io)
- [x] `cargo tree -i grafeo` excerpt in Results
- [x] Debug `cargo build` time noted (~18s cold, informal)
- [ ] Release binary size delta vs workspace without spike (or spike bin only)
- [x] Complexity budget: features **used** vs **available but unused** in Results

#### 7. Stretch S6+ (optional — time box after S1–S6)

Only if baseline S1–S6 are green; see [spike/program.md stretch](../crates/openpfe-graph/spike/program.md#stretch-goals--search--compare-s6):

- [x] Extend fixture with P-lex / P-sem / P-struct pairs (`fixture::seed_s6_plus`)
- [x] Lexical stretch checklist; semantic proxy + structural (`find_similar`, `tests/grafeo_spike_s6plus.rs`)
- [x] Fill S6+ subsection in [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) Results

#### 8. Documentation and decision

- [x] Complete [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) **Results** (Recommendation, measurements table, S6 table, backup procedure)
- [x] Update [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md) decision table / Grafeo row per [spike/program.md decision rules](../crates/openpfe-graph/spike/program.md#decision-after-spikes)
- [x] If engine choice changes: note required follow-up for `design.md` / `specification.md` (do **not** edit product engine lines until both engine spikes complete unless human directs early lock)
- [ ] Check corresponding boxes in [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) task list

#### 9. Verification

- [x] `cargo test -p openpfe-graph-spike` (or documented test binary) passes on dev machine
- [x] `cargo clippy -p openpfe-graph-spike -- -D warnings` (or documented exceptions in verdict)
- [x] No `grafeo` dependency added to `crates/openpfe-graph/Cargo.toml` in this plan

## Acceptance criteria

- [x] Intake artifacts for `grafeo` present; human approval recorded in plan and `verdict.md`
- [x] [spike/grafeo.md](../crates/openpfe-graph/spike/grafeo.md) **Recommendation** filled: Pass / Pass with caveats / Fail
- [x] S1–S5 pass on **macOS** per shared acceptance criteria (Linux skipped)
- [x] **S6** pass (BM25 or documented full-text) with measurements and manual case notes
- [x] Durability smoke and backup restore documented
- [x] `cargo audit` outcome recorded (clean or advisories + remediation in verdict)
- [x] [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md) updated with Grafeo spike outcome
- [x] Product `openpfe-graph` remains empty of engine code; spike isolated to `openpfe-graph-spike`

## Out of scope

- Implementing `openpfe-graph` production adapter or server wiring (phase 2 implementation plan)
- [spike/indradb.md](../crates/openpfe-graph/spike/indradb.md) execution (separate plan/branch; compare results when both exist)
- Exposing Grafeo GQL/Cypher/GraphQL on HTTP or MCP; adopting [grafeo-mcp](https://github.com/GrafeoDB/grafeo-mcp)
- `openpfe-ui`, `openpfe-mcp`, MCP tools (`openpfe_graph_find_similar`, etc.)
- Promoting `openpfe-graph-spike` to a permanent workspace member beyond spike branch (human decides merge vs discard)
- RDF, Gremlin, ONNX embeddings in v1 unless explicitly recorded as stretch-only experiment

## Next

- Run **IndraDB** spike ([spike/indradb.md](../crates/openpfe-graph/spike/indradb.md)) if not already in progress — plan TBD (e.g. `007-spike-openpfe-graph-indradb.md`).
- After **both** engine spikes: lock engine in [spike/evaluation.md](../crates/openpfe-graph/spike/evaluation.md), then phase 2 **`openpfe-graph` implementation** plan (TBD in [plans/README.md](./README.md)).
