# openpfe-ui — design

**HTTP API for humans** — Web UI (`fetch`) and TUI (HTTP client). Parallel to **`openpfe-mcp`** (agents over IPC).

Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Decisions

| Topic | Decision |
|-------|----------|
| **HTTP framework** | **axum** — export `Router` (or factory) for `openpfe-server` to mount on `127.0.0.1:0`. Cross-cutting HTTP middleware (**tower-http**) lives in `openpfe-server`, not this crate. |
| **Human data API** | REST `/api/v1/…` only for graph, config, models (Web UI + TUI). No duplicate IPC graph API. |
| **Runtime** | Handlers are `async` on **tokio**; call sync `openpfe-core` / `openpfe-graph` from handlers or `spawn_blocking` as needed. |
| **Live updates (v1)** | **REST + client poll** — no WebSocket or SSE in v1. UI may poll `GET /api/v1/graph/overview` (or ETag) on an interval. |
| **API auth** | **Deferred** — no tokens in v1 ([openpfe-server/design.md](../openpfe-server/design.md)). |

## Scope

- Routes under `/api/v1/…` — graph, config, models
- Handlers call `openpfe-core`, `openpfe-graph`, `openpfe-llm`
- Export router / handler factory for `openpfe-server` to mount
- Browser UI is **same-origin** with static assets (served together from `openpfe-server`). **No CORS** in v1 ([openpfe-server/design.md](../openpfe-server/design.md)).

## API surface (design intent)

- REST under `/api/v1/…` for graph CRUD, config read/write, model list/download triggers
- Normative routes: [specification.md](./specification.md)

## Out of scope

- Static HTML/CSS/JS → **`openpfe-webui`**
- IPC, MCP, CLI
- Socket bind → **`openpfe-server`**

## Consumers

| Consumer | How |
|----------|-----|
| Browser | `fetch('/api/v1/…')` |
| TUI | HTTP to `http_base_url` from IPC echo — all data paths; no graph-over-IPC |
| CLI | IPC for control; optional HTTP later for rich CLI |

## Related

- [openpfe-webui/design.md](../openpfe-webui/design.md)
- [openpfe-mcp/design.md](../openpfe-mcp/design.md)
