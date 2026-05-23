# openpfe-ui — design

**HTTP API for humans** — Web UI (`fetch`) and TUI (HTTP client). Parallel to **`openpfe-mcp`** (agents over IPC).

Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Decisions

| Topic | Decision |
|-------|----------|
| **HTTP framework** | **axum** — export `Router` for `openpfe-server` to mount on `127.0.0.1:0`. **tower-http** on composed router in server. |
| **Wire format** | **JSON** only for API bodies and on-disk config (no TOML in product). |
| **Human data API** | REST `/api/v1/…` for graph, `server.json`, `llm.json`, models, inference. |
| **Orchestration** | Handlers call traits/services in **`AppState`**: server settings (`openpfe-server`), **`LlmService`** (`openpfe-llm`). After LLM mutations, invoke **`reload_engine`**. |
| **`AppState` wiring** | **`openpfe-server`** constructs `AppState` (loaded `server.json`, `Arc<dyn LlmService>`, graph) and passes to `openpfe_ui::api_router` — keeps **`openpfe-ui` ↛ `openpfe-server`** dependency. |
| **Runtime** | Async handlers on **tokio**; sync domain on thread pool where needed. |
| **Live updates (v1)** | REST + **poll** — no WebSocket/SSE. |
| **API auth** | **Deferred** — no tokens in v1. |

## Scope

- Route table: [specification.md](./specification.md)
- Product view needs: [openpfe-webui/assets/README.md](../openpfe-webui/assets/README.md)

## Out of scope

- Static assets → **`openpfe-webui`**
- Owning `server.json` / `llm.json` schemas → **`openpfe-server`** / **`openpfe-llm`**
- Project path literals → owning crates’ `specification.md` (server, graph, llm); **cwd** convention in [cross-cutting.md](../../cross-cutting.md)
- IPC, MCP, socket bind

## Related

- [openpfe-webui/design.md](../openpfe-webui/design.md)
- [openpfe-mcp/design.md](../openpfe-mcp/design.md)
