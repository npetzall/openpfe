# Per-crate documentation

One folder per workspace member. **Authoritative** for that crate’s design, requirements, and specification.

Convention: [workspace-crates.md#documentation-convention](../workspace-crates.md#documentation-convention).

## Folders

| Folder | Crate | Role |
|--------|--------|------|
| [openpfe/](./openpfe/) | `openpfe` | Binary — CLI, IPC client, MCP stdio bridge |
| [openpfe-server/](./openpfe-server/) | `openpfe-server` | Listeners, lock, mount UI + webui + MCP |
| [openpfe-ipc/](./openpfe-ipc/) | `openpfe-ipc` | Framing, UDS, control messages |
| [openpfe-ui/](./openpfe-ui/) | `openpfe-ui` | HTTP API for humans (Web UI, TUI) |
| [openpfe-webui/](./openpfe-webui/) | `openpfe-webui` | Embedded browser assets |
| [openpfe-mcp/](./openpfe-mcp/) | `openpfe-mcp` | MCP for agents |
| [openpfe-graph/](./openpfe-graph/) | `openpfe-graph` | Embedded graph store (+ [graph-db-evaluation.md](./openpfe-graph/graph-db-evaluation.md)) |
| [openpfe-llm/](./openpfe-llm/) | `openpfe-llm` | `llm.json`, shared model store, llama.cpp (v1) |

## Each folder

- `design.md` — how the crate is built
- `requirements.md` — what it must do
- `specification.md` — APIs and formats it owns
