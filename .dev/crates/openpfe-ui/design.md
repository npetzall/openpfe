# openpfe-ui — design

**HTTP API for humans** — Web UI (`fetch`) and TUI (HTTP client). Graph and LLM are REST; MCP is exposed on a **debug JSON-RPC route** that delegates to the same **`McpHandler`** as IPC (agents use `openpfe mcp` stdio bridge).

Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Decisions

| Topic | Decision |
|-------|----------|
| **HTTP framework** | **axum** — export `Router` for `openpfe-server` to mount on `127.0.0.1:0`. **tower-http** on composed router in server. |
| **Wire format** | **JSON** only for API bodies and on-disk config (no TOML in product). |
| **Human data API** | REST `/api/v1/…` for graph, `llm.json`, models, inference. **`server.json`** is **out of scope** — IPC admin ([openpfe-ipc/specification.md](../openpfe-ipc/specification.md)). |
| **Orchestration** | Handlers call **`AppState`**: **`GraphStore`** (`openpfe-graph`), **`LlmService`** (`openpfe-llm`). After LLM mutations, invoke **`reload_engine`**. |
| **Graph search (v1)** | **`search_problems`** (S6) and **`find_similar`** (S6+) exposed on human API — [specification.md#search-and-similarity](./specification.md#search-and-similarity). Sync `GraphStore` search runs on **`spawn_blocking`** from async handlers. **`embedding`** in `find_similar` is optional; without `/llm/embed` (v1), clients omit it and rely on lexical + structural legs. |
| **Edge identity (v1)** | **No edge UUID** — identity is **`(src_id, dst_id, type)`**, matching `Edge` and `GraphStore`. **`DELETE /graph/edges`** takes the triple as query params ([specification.md#edges](./specification.md#edges)). Stable edge ids deferred (would require schema + store changes). Aligns with MCP `openpfe_contract_get` `from` / `to` / `type` lookup. |
| **`AppState` wiring** | **`openpfe-server`** constructs `AppState` (`Arc<Mutex<dyn GraphStore + Send>>`, `Arc<dyn LlmService>`, `Arc<McpHandler>`) and passes to `openpfe_ui::api_router` — keeps **`openpfe-ui` ↛ `openpfe-server`** dependency. Server process config stays in server IPC dispatch, not in `AppState`. |
| **MCP transport** | **`openpfe-mcp`** owns tool semantics; **`openpfe-ui`** exposes `POST /debug/mcp` as the browser transport; IPC `type: mcp` is the agent transport — one shared `McpHandler` in the server process. |
| **Runtime** | Async handlers on **tokio**; sync domain on thread pool where needed. |
| **Live updates (v1)** | REST + **poll** — no WebSocket/SSE. |
| **API auth** | **Deferred** — no tokens in v1. |

## Scope

- Route table: [specification.md](./specification.md)
- Product view needs: [openpfe-webui/assets/README.md](../openpfe-webui/assets/README.md)

## `AppState` (v1)

See [specification.md#appstate-v1](./specification.md#appstate-v1). Graph + LLM + shared `McpHandler` (`openpfe-mcp` dep for handler type only); no server-config handle.

## Out of scope

- Static assets → **`openpfe-webui`**
- **`server.json`** — IPC admin in **`openpfe-server`** / **`openpfe-ipc`**; not human HTTP
- Owning `llm.json` schema → **`openpfe-llm`**
- Owning MCP tool/resource semantics → **`openpfe-mcp`**
- Project path literals → owning crates’ `specification.md` (server, graph, llm); **cwd** convention in [cross-cutting.md](../../cross-cutting.md)
- IPC framing, socket bind, stdio MCP bridge

## Related

- [openpfe-webui/design.md](../openpfe-webui/design.md)
- [openpfe-mcp/design.md](../openpfe-mcp/design.md)
