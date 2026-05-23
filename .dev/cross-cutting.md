# Cross-cutting concerns

Details that **no single crate owns** — implemented jointly. Crate docs are authoritative for their scope; this file records **system-wide contracts** and **coordination points**.

Unresolved decisions live in each crate’s `design.md` (and crate-local research docs such as [graph-db-evaluation.md](./crates/openpfe-graph/graph-db-evaluation.md)).

## Project root convention

- **cwd = project root** for CLI and project-scoped MCP (no walk-up, no `--cwd` in v1).
- Client behavior: [openpfe/design.md](./crates/openpfe/design.md).
- Each crate resolves **its own** paths relative to **process cwd** — documented in that crate’s `specification.md` (no shared path-helper crate).

## Data placement

Index only; normative path tables live in owning crate specs:

| Data | Location | Normative doc |
|------|----------|----------------|
| Server / HTTP config | `./.openpfe/server.json` | [openpfe-server/specification.md](./crates/openpfe-server/specification.md) |
| LLM config + catalog | `./.openpfe/llm.json` | [openpfe-llm/specification.md](./crates/openpfe-llm/specification.md) |
| Downloaded models | `USER_HOME/.openpfe/models/<id>/` (shared) | [openpfe-llm/specification.md](./crates/openpfe-llm/specification.md) |
| Problem graph | `./.openpfe/graph/store/` (IndraDB RocksDB) | [openpfe-graph/specification.md](./crates/openpfe-graph/specification.md) |
| Server runtime | `./.openpfe/server/` (`pid`, `socket`) | [openpfe-server/specification.md](./crates/openpfe-server/specification.md), [openpfe/specification.md](./crates/openpfe/specification.md) (client) |
| HTTP base URL | **Not on disk** — IPC echo only | [openpfe-ipc/specification.md](./crates/openpfe-ipc/specification.md) |

**Config:** JSON per crate under `./.openpfe/` (`server.json`, `llm.json`). **Weights** shared under `USER_HOME/.openpfe/models/`. **One server per project.**

## Client transports

Authoritative matrix: [workspace-crates.md#client-transports-decided](./workspace-crates.md#client-transports-decided).

## Multi-crate flows

Startup scenarios: [architcture.md](./architcture.md#startup-scenarios).

| Flow | Crates | Detail |
|------|--------|--------|
| Default `openpfe` | `openpfe` → `openpfe-server` → `openpfe-ipc`; UI via `openpfe-webui` + `openpfe-ui` | [openpfe/design.md](./crates/openpfe/design.md) |
| `openpfe mcp` | `openpfe` (stdio bridge) → server → `openpfe-mcp` → graph | [openpfe/design.md](./crates/openpfe/design.md#mcp-over-ipc-openpfe-mcp) |
| Singleton / races | `openpfe-server` (`pid` flock), `openpfe-ipc` (echo), `openpfe` (client wait) | [openpfe-server/design.md](./crates/openpfe-server/design.md), [openpfe/design.md](./crates/openpfe/design.md) |

## Cross-cutting NFRs (index)

| ID | Concern | Crate detail |
|----|---------|----------------|
| NFR-1 | Performance | [openpfe](./crates/openpfe/), [openpfe-server](./crates/openpfe-server/), [openpfe-ipc/requirements.md](./crates/openpfe-ipc/requirements.md) |
| NFR-2 | Reliability | [openpfe-graph](./crates/openpfe-graph/), [openpfe-server](./crates/openpfe-server/), [openpfe-llm](./crates/openpfe-llm/) |
| NFR-3 | Security | [openpfe-server](./crates/openpfe-server/), [openpfe-ipc](./crates/openpfe-ipc/) |
| NFR-4 | Maintainability | [workspace-crates.md](./workspace-crates.md) |
| NFR-5 | Packaging | [openpfe](./crates/openpfe/), [openpfe-webui](./crates/openpfe-webui/) |
| NFR-6 | Platform | [openpfe-ipc](./crates/openpfe-ipc/) |

## Out of scope (product)

- Remote / multi-user hosted deployment.
- Cloud models as default (local first).
- Mobile clients.

## FR traceability

Product functional requirements are **owned by crate** `requirements.md` files. Split reference:

| FR | Crate doc |
|----|-----------|
| FR-1 (singleton) | [openpfe-server/requirements.md](./crates/openpfe-server/requirements.md) (server); [openpfe/requirements.md](./crates/openpfe/requirements.md) (client) |
| FR-2, FR-3, FR-4 | [openpfe/requirements.md](./crates/openpfe/requirements.md) |
| FR-3.3 (MCP semantics) | [openpfe-mcp/requirements.md](./crates/openpfe-mcp/requirements.md) |
| FR-5 | [openpfe-ipc/requirements.md](./crates/openpfe-ipc/requirements.md) |
| FR-6.1 | [openpfe-webui/requirements.md](./crates/openpfe-webui/requirements.md) |
| FR-6.2–6.4 | [openpfe-ui/requirements.md](./crates/openpfe-ui/requirements.md) |
| FR-6.5 | [openpfe-server/requirements.md](./crates/openpfe-server/requirements.md) |
| FR-7 (project root / cwd) | [openpfe/requirements.md](./crates/openpfe/requirements.md); convention above |
| FR-7 (server paths + `server.json`) | [openpfe-server/requirements.md](./crates/openpfe-server/requirements.md) |
| FR-7 (graph paths) | [openpfe-graph/requirements.md](./crates/openpfe-graph/requirements.md) |
| FR-7 (LLM paths) | [openpfe-llm/requirements.md](./crates/openpfe-llm/requirements.md) |
| FR-8 (LLM config, registry, inference) | [openpfe-llm/requirements.md](./crates/openpfe-llm/requirements.md) |
| FR-9 | [openpfe-graph/requirements.md](./crates/openpfe-graph/requirements.md), [openpfe-mcp/requirements.md](./crates/openpfe-mcp/requirements.md) |

| Source | Maps to |
|--------|---------|
| User briefing (2026-05) | Crate `requirements.md` above |
| [openpfe_tooling.md](../openpfe_tooling.md) | FR-6, FR-9, MCP crates |
| [pfe_methodology.md](../pfe_methodology.md) | FR-9, graph crate |

## Related

| Layer | Files |
|-------|--------|
| Overview | [README.md](./README.md), [architcture.md](./architcture.md) |
| Workspace | [workspace-crates.md](./workspace-crates.md#documentation-convention) |
| Per crate | [crates/](./crates/) — `design.md`, `requirements.md`, `specification.md` |
