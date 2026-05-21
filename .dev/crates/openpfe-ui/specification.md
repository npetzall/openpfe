# openpfe-ui — specification

## HTTP API (v1)

Base path: **`/api/v1`**. JSON request/response unless noted.

### Errors (v1)

```json
{ "error": { "code": "snake_case", "message": "human-readable" } }
```

| HTTP | `code` (examples) | When |
|------|-------------------|------|
| 400 | `invalid_request` | Bad JSON or validation |
| 404 | `not_found` | Unknown node/model/job |
| 409 | `conflict` | Graph constraint (e.g. cycle if blocking) |
| 503 | `model_not_loaded` | LLM not loaded |
| 503 | `inference_busy` | Single-flight inference in progress |

Same origin as embedded static UI. **No CORS in v1** — [openpfe-server/design.md](../openpfe-server/design.md).

### Request body limit

**1 MiB** default JSON body (`axum::extract::DefaultBodyLimit`) on mutating routes unless a route documents otherwise.

Export: `pub fn api_router(state: AppState) -> Router` for `openpfe-server` to nest at `/api/v1`.

---

## Graph

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/graph/overview` | Counts by node `type`, list root problem ids |
| `GET` | `/graph/nodes` | List nodes — query: `type`, `cluster_id`, `limit`, `offset` |
| `GET` | `/graph/nodes/:id` | One node + properties |
| `POST` | `/graph/nodes` | Create node (body: `type`, properties) |
| `PATCH` | `/graph/nodes/:id` | Merge property updates |
| `DELETE` | `/graph/nodes/:id` | Delete node (and incident edges) |
| `GET` | `/graph/nodes/:id/edges` | Adjacent edges — query: `direction`, `edge_type` |
| `POST` | `/graph/edges` | Create edge (`from`, `to`, `type`, optional props) |
| `DELETE` | `/graph/edges/:id` | Delete edge by id |
| `GET` | `/graph/clusters/:id/subgraph` | Bounded subgraph — query: `max_depth`, `max_nodes` (defaults in [openpfe-graph/specification.md](../openpfe-graph/specification.md)) |
| `GET` | `/graph/validate` | Cycle check on `depends_on` — `{ "ok": true }` or `{ "cycles": [...] }` |

Schema types: [openpfe-graph/specification.md](../openpfe-graph/specification.md).

**Live updates:** clients poll `GET /graph/overview` or refetch affected nodes — no push channel in v1.

---

## Config

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/config` | Effective merged config (JSON view of TOML) |
| `GET` | `/config/project` | Raw project `./.openpfe/config.toml` as JSON |
| `PUT` | `/config/project` | Replace project config document (writes `./.openpfe/config.toml` only) |

Merge rules: [openpfe-core/specification.md](../openpfe-core/specification.md).

---

## Models

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/models` | Catalog entries + `installed: bool` per id |
| `GET` | `/models/:id` | Manifest + paths if installed |
| `POST` | `/models/:id/download` | Start HTTPS download — returns `{ "job_id" }` |
| `GET` | `/models/downloads/:job_id` | Poll `{ "status", "percent", "error" }` |
| `PUT` | `/llm/active` | Set project `[llm].model` to catalog `id` (body: `{ "model": "<id>" }`) |

Download/install semantics: [openpfe-core/specification.md](../openpfe-core/specification.md#download-v1).

---

## LLM (v1)

Delegated to `openpfe-llm` — [specification.md](../openpfe-llm/specification.md#http-exposure-v1).

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/status` | Engine status + active `model_id` |
| `POST` | `/llm/complete` | Single-shot completion (blocking server-side) |

Not exposed on MCP in v1.

---

## Phasing note

Phase 2 may ship **stub handlers** (501 or empty lists) before graph/UI features are complete; route table above is the **stable v1 contract** for Web UI and TUI.

## Related

- [openpfe-webui/specification.md](../openpfe-webui/specification.md) — static routes
- [openpfe-server/specification.md](../openpfe-server/specification.md) — mount table
