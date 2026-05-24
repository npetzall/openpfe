# Implementation plans

**Read when:** authoring or executing a file under [`.dev/plans/`](../plans/); resuming agent work on a crate after a break.

**Normative refs:** [plans/README.md](../plans/README.md) (order and phasing), [dependencies/README.md](../dependencies/README.md), [security-rust.md](./security-rust.md), [coding-rust.md](./coding-rust.md#adding-an-external-crate-order).

## Role of plans

Plans are **actionable checklists**, not specs. Crate `specification.md` / `requirements.md` stay authoritative for behavior. Plans split work into phases agents and humans can track with checkboxes and **Status** lines.

**Progress is tracked in the plan file itself** — not only in chat or commit messages. Anyone resuming work should read the plan and see what is done, in progress, or blocked.

## Tracking progress during implementation

Update the **same plan file** (e.g. `.dev/plans/002-openpfe-impl.md`) as work proceeds. Do not wait until the end of a large batch to mark tasks complete.

| When | Update in the plan |
|------|---------------------|
| Starting a plan (no external deps, or after intake approved) | **Status** → `In progress — …` with date if helpful |
| Finishing a task (or a small, coherent group) | Check the matching **Tasks** box `[x]` |
| Finishing a prerequisite or intake item | Check the **Prerequisites** / intake box |
| Verifying acceptance (tests, audit, etc.) | Check **Acceptance criteria** boxes |
| Blocked on human review or a dependency | **Status** → `Blocked — …` and leave relevant boxes unchecked |
| All acceptance criteria met | **Status** → `Complete (YYYY-MM-DD)`; ensure every task and criterion box is `[x]` |

**Agents:** after each meaningful chunk of implementation in a session, update the plan in **that same turn** before stopping. If a task is only partly done, leave its box unchecked and optionally add a one-line note under the task (sparingly — prefer clear task titles over long notes).

**Humans:** use the plan as the handoff surface between reviews; adjust **Status** when approving intake or closing a plan.

**Commits:** prefer including plan updates in the same commit as the code they describe so `git log` and the checklist stay aligned. Intake-only commits may update only `.dev/dependencies/` and plan intake boxes.

## Dependency intake gate (required)

If a plan’s prerequisites list **external** crates (crates.io / git), the plan **must** separate intake from implementation. Do not implement product code that needs those crates until intake is complete and a human has cleared the pause (below).

| Phase | Who | Work |
|-------|-----|------|
| **A — Intake** | Agent | `rational.md` per crate; `dependency-lock-diff.sh` → `lock-update.md`; edit `Cargo.toml` / `Cargo.lock`; **`cargo audit` only** as the first Cargo command after the manifest edit ([security-rust.md](./security-rust.md)) |
| **B — Pause** | Human | Review intake folder(s), lock delta, audit output; accept or reject in `verdict.md` |
| **C — Implement** | Agent | Source, tests, clippy; no new external deps without returning to phase A |

**Forbidden between manifest edit and `cargo audit`:** `cargo build`, `cargo check`, `cargo test`, `cargo update`, `cargo fetch` (unless a documented scan explicitly requires one of these).

**One intake batch per pause.** If implementation discovers another external crate, stop, add phase A for that crate, and pause again before using it in code.

## Pause checkpoint (manual inspection)

After phase A, set plan **Status** to a blocked form and stop agent work until a human updates the plan.

Recommended status line:

```markdown
**Status:** Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD).
```

Human review checklist:

- [ ] Every new direct dependency has `.dev/dependencies/<name>/rational.md`
- [ ] `lock-update.md` matches the resolution-only preview (not a post-build lock)
- [ ] `scan.md` contains **`cargo audit` only** output from immediately after the manifest edit
- [ ] `verdict.md` records accept / reject / defer
- [ ] No application `src/` changes mixed into the intake-only commit (preferred)

When satisfied, the human sets:

```markdown
**Status:** In progress — intake approved (YYYY-MM-DD); implementation may continue.
```

and checks the intake prerequisite boxes in the plan.

## Per-plan structure

Each plan should include:

1. **Status** — current phase (including blocked / approved intake).
2. **Normative sources** — links to crate docs and guidelines.
3. **Prerequisites** — prior plans; **external crate intake** (separate checkbox group).
4. **Goal** — one paragraph.
5. **Tasks** — checkboxes grouped by phase (intake vs implement).
6. **Acceptance criteria** — verifiable before “Complete”.
7. **Out of scope** — defers to other plans.
8. **Next** — following plan link.

## Example: external deps in a crate plan

Below is the pattern plan **002** should use when adding `tokio`, `clap`, etc. (illustrative; adjust crate names and versions).

```markdown
# Plan 0NN: `openpfe` binary (primary)

**Status:** Blocked — dependency intake complete; awaiting human review (2026-05-23).

## Prerequisites

- [x] [001-scaffolding.md](./001-scaffolding.md) complete.

### Dependency intake (phase A — complete before implementation)

- [x] `.dev/dependencies/tokio/rational.md` (+ `lock-update.md`, `scan.md`, `verdict.md`)
- [x] `.dev/dependencies/clap/rational.md` (+ …)
- [x] `Cargo.toml` / `Cargo.lock` updated; **`cargo audit`** run immediately after manifest edit
- [ ] **Human intake approval** (verdicts accepted; see [guidelines/plans.md](../guidelines/plans.md))

**Do not start tasks in “Implementation” until the intake approval box is checked.**

## Goal

Implement `openpfe` behind crate-local ports with mock adapters.

## Tasks

### Implementation (phase C — after intake approval only)

- [ ] `ports.rs` — traits + `EchoInfo`.
- [ ] `adapters/mock.rs` — …
- [ ] …

## Acceptance criteria

- [ ] `cargo test -p openpfe` passes with mock adapters only.
- [ ] Intake artifacts present and audit clean (or documented in `verdict.md`).
```

Agent workflow for that plan:

1. Complete **only** intake checkboxes and files; run `cargo audit`; check intake boxes; set **Status** to **Blocked**.
2. **Stop.** Do not edit `src/` for product behavior in the same turn.
3. After human approval, set **Status** to **In progress — intake approved**; check the human approval box.
4. Implement task groups; **check each task box as it is completed**; run `cargo test` / `cargo clippy` when relevant.
5. When acceptance criteria pass, check those boxes and set **Status** to **Complete (YYYY-MM-DD)**.

Plans with **no** new external crates (e.g. [001-scaffolding.md](../plans/001-scaffolding.md)) skip phases A–B; say so explicitly under prerequisites.

## Status vocabulary

| Status | Meaning |
|--------|---------|
| **Not started** | No work yet |
| **Blocked — … awaiting human review** | Intake done; waiting on checklist |
| **In progress — intake approved** | Implementation allowed |
| **Complete (YYYY-MM-DD)** | Acceptance criteria met |

## Related

- [plans/README.md](../plans/README.md) — execution order
- [dependencies/README.md](../dependencies/README.md) — intake file templates
- [security-rust.md](./security-rust.md) — `cargo audit` order
- [guidelines.md](../guidelines.md) — strict rules for agents
