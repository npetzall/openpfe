# Implementation plans

**Read when:** starting or resuming workspace implementation; pick the next plan in order below.

Actionable checklists for scaffolding and each Cargo member. **Normative contracts** remain in [crates/](../crates/) (`design.md`, `requirements.md`, `specification.md`). **How to implement** is in [guidelines.md](../guidelines.md) and [guidelines/](../guidelines/).

## Execution order

Follow [workspace-crates.md#phasing](../workspace-crates.md#phasing).

| # | Plan | Phase | Scope |
|---|------|-------|-------|
| 001 | [001-scaffolding.md](./001-scaffolding.md) | — | Workspace root, empty members |
| **002** | [002-openpfe-impl.md](./002-openpfe-impl.md) | 1 | **`openpfe` binary (primary)** — ports, CLI, mocks |
| **003** | [003-openpfe-ipc-impl.md](./003-openpfe-ipc-impl.md) | 1 | `openpfe-ipc` — traits + wire API (aligns with 002 ports) |
| **004** | [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) | 1 | `openpfe-server` — phase 1 minimal server (no port pattern) |
| **005** | [005-openpfe-wiring.md](./005-openpfe-wiring.md) | 1 | Path deps, real adapters, integration tests, e2e |
| — | *(per-crate plans TBD)* | 2 | `openpfe-graph`, `openpfe-webui`, `openpfe-ui` |
| — | *(per-crate plans TBD)* | 3 | `openpfe-mcp`, `openpfe-llm` |

**Phase 1 flow:** **002** and **003** in parallel (ports contract in 002 drives 003); **004** after **003**; finish with **005** wiring (no new external deps).

**Ports:** [coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports) — `openpfe` uses crate-local traits; `openpfe-ipc` uses internal traits; **`openpfe-server` is excluded** (plain functions + direct ipc in composer code).

**Phase goals** (from [workspace-crates.md](../workspace-crates.md)):

| Phase | Goal |
|-------|------|
| **1** | Lock, socket, echo, HTTP stub |
| **2** | Graph + static shell + human API |
| **3** | MCP graph tools (read-only) + llama-cpp-2 inference over HTTP |

## Per-plan structure

Each plan links normative crate docs and relevant guidelines, lists prerequisites, tasks (checkboxes), acceptance criteria, and v1 out-of-scope items.

Plans that add **external** crates must use the **intake → pause → implement** phases in [guidelines/plans.md](../guidelines/plans.md) (dependency gate, human review, example checklist).

During implementation, **track progress in the plan file** (checkboxes and **Status**) as tasks complete — see [guidelines/plans.md#tracking-progress-during-implementation](../guidelines/plans.md#tracking-progress-during-implementation).

## Related

- [workspace-crates.md](../workspace-crates.md) — members, dependency rules, phasing
- [guidelines.md](../guidelines.md) — strict rules for agents
- [architcture.md](../architcture.md) — system overview
- [cross-cutting.md](../cross-cutting.md) — multi-crate contracts
