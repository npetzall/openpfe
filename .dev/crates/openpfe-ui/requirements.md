# openpfe-ui — requirements

## FR-6 HTTP / human API

- **FR-6.2** Problem graph interaction via [specification.md](./specification.md#graph); **polling** in v1.
- **FR-6.6** **Edge CRUD identity** — create/delete edges via [specification.md#edges](./specification.md#edges); **`DELETE /graph/edges`** must identify edges by **`src_id`**, **`dst_id`**, and **`type`** (query), delegating to `GraphStore::delete_edge` — no edge UUID in v1.
- **FR-6.5** **Graph search (S6 / S6+)** — expose `GET /graph/problems/search` and `POST /graph/problems/find-similar`; delegate to `GraphStore::search_problems` / `GraphStore::find_similar` with HTTP JSON shapes in [specification.md#search-and-similarity](./specification.md#search-and-similarity). Satisfies human surface for [openpfe-graph FR-9.6](../openpfe-graph/requirements.md) / [FR-9.7](../openpfe-graph/requirements.md).
- **FR-6.4** Expose LLM routes — **catalog CRUD**, **`/catalog/discover`**, **`llm.json`**, models aggregate, download jobs, active model, inference — via **`LlmService`**; orchestrate **`reload_engine`** ([openpfe-llm/specification.md](../openpfe-llm/specification.md)). **No HTTP LLM init.**
- **FR-6.9** **Catalog REST** — `GET`/`POST`/`PUT`/`PATCH`/`DELETE /catalog` and `GET /catalog/discover`; **`POST /catalog`** creates one entry per request; download via **`POST /catalog/:id/download`**; poll **`GET /downloads/:job_id`**.
- **FR-6.7** **`AppState`** — graph + LLM + shared **`McpHandler`** ([specification.md#appstate-v1](./specification.md#appstate-v1)); built by **`openpfe-server`**.
- **FR-6.8** **MCP debug transport** — expose `POST /debug/mcp` delegating to **`McpHandler`** ([specification.md#mcp-debug-json-rpc](./specification.md#mcp-debug-json-rpc)); same JSON-RPC as IPC `type: mcp`; satisfies Web UI Debug ([openpfe-webui/assets/debug/requirements.md](../openpfe-webui/assets/debug/requirements.md) FR-UI-7.2).

## Non-goals

- MCP tool/resource definitions (delegate to **`openpfe-mcp`**), static embed, TOML config, **`server.json`** (IPC admin — [openpfe-server/requirements.md](../openpfe-server/requirements.md) FR-7.4), persisting **`catalog.json`** / **`llm.json`** (delegate to **`openpfe-llm`**), CLI **`openpfe llm init`** (delegate to **`openpfe`**).

## Related

- [openpfe-webui/requirements.md](../openpfe-webui/requirements.md)
- [openpfe-server/requirements.md](../openpfe-server/requirements.md) — FR-7 `server.json` (IPC admin, not UI)
