# Implementation plans

**Read when:** starting or resuming workspace implementation; pick the next plan in order below.

Actionable checklists for scaffolding and each Cargo member. **Normative contracts** remain in [crates/](../crates/) (`design.md`, `requirements.md`, `specification.md`). **How to implement** is in [guidelines.md](../guidelines.md) and [guidelines/](../guidelines/).

## Execution order

Follow [workspace-crates.md#phasing](../workspace-crates.md#phasing).

| # | Plan | Phase | Scope |
|---|------|-------|-------|
| 001 | [001-scaffolding.md](./001-scaffolding.md) | — | Workspace root, empty members |
| — | *(per-crate plans TBD)* | 1 | `openpfe-core`, `openpfe-ipc`, `openpfe-server`, `openpfe` |
| — | *(per-crate plans TBD)* | 2 | `openpfe-graph`, `openpfe-webui`, `openpfe-ui` |
| — | *(per-crate plans TBD)* | 3 | `openpfe-mcp`, `openpfe-llm` |

Within **Phase 1**, build **`openpfe-core`** and **`openpfe-ipc`** before **`openpfe-server`** and the **`openpfe`** binary.

**Phase goals** (from [workspace-crates.md](../workspace-crates.md)):

| Phase | Goal |
|-------|------|
| **1** | Lock, socket, echo, HTTP stub |
| **2** | Graph + static shell + human API |
| **3** | MCP graph tools (read-only) + llama-cpp-2 inference over HTTP |

## Per-plan structure

Each plan links normative crate docs and relevant guidelines, lists prerequisites, tasks (checkboxes), acceptance criteria, and v1 out-of-scope items.

## Related

- [workspace-crates.md](../workspace-crates.md) — members, dependency rules, phasing
- [guidelines.md](../guidelines.md) — strict rules for agents
- [architcture.md](../architcture.md) — system overview
- [cross-cutting.md](../cross-cutting.md) — multi-crate contracts
