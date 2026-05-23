# openpfe-ui — specification

## HTTP API (v1)

Base path: **`/api/v1`**. **JSON** request/response.

### Errors (v1)

```json
{ "error": { "code": "snake_case", "message": "human-readable" } }
```

| HTTP | `code` (examples) | When |
|------|-------------------|------|
| 400 | `invalid_request` | Bad JSON or validation |
| 404 | `not_found` | Unknown node/model/job |
| 409 | `conflict` | Graph constraint |
| 503 | `model_not_loaded` | LLM not loaded |
| 503 | `inference_busy` | Single-flight inference |

Same origin as embedded static UI. **No CORS in v1**.

**Body limit:** **1 MiB** default on mutating routes.

Export: `pub fn api_router(state: AppState) -> Router`.

**Orchestration:** handlers delegate persistence to **`openpfe-server`** (`server.json`) and **`openpfe-llm`** (`LlmService` / `llm.json`). `AppState` is built in **`openpfe-server`** so this crate does not depend on `openpfe-server` — see [design.md](./design.md).

---

## Graph

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/graph/overview` | Counts by node `type`, root problem ids |
| `GET` | `/graph/nodes` | List — query: `type`, `cluster_id`, `limit`, `offset` |
| `GET` | `/graph/nodes/:id` | One node + properties |
| `POST` | `/graph/nodes` | Create node |
| `PATCH` | `/graph/nodes/:id` | Merge property updates |
| `DELETE` | `/graph/nodes/:id` | Delete node |
| `GET` | `/graph/nodes/:id/edges` | Adjacent edges |
| `POST` | `/graph/edges` | Create edge |
| `DELETE` | `/graph/edges/:id` | Delete edge |
| `GET` | `/graph/clusters/:id/subgraph` | Bounded subgraph |
| `GET` | `/graph/validate` | Cycle check on `depends_on` |

Schema: [openpfe-graph/specification.md](../openpfe-graph/specification.md). Live updates: **poll** in v1.

---

## Server config (`server.json`)

Delegated to **`openpfe-server`** — [specification.md](../openpfe-server/specification.md#serverjson-project-config).

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/server/config` | Full `server.json` document |
| `PUT` | `/server/config` | Replace `server.json` |

---

## LLM (`llm.json`, models, inference)

Delegated to **`openpfe-llm`** — [specification.md](../openpfe-llm/specification.md).

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/config` | Full `llm.json` document |
| `PUT` | `/llm/config` | Replace `llm.json`; **`reload_engine`** when load-affecting |
| `GET` | `/models` | Catalog + `installed` |
| `GET` | `/models/:id` | Detail if installed |
| `POST` | `/models/:id/download` | `{ "job_id" }` |
| `GET` | `/models/downloads/:job_id` | Poll progress |
| `PUT` | `/llm/active` | `{ "model": "<id>" }` → update `llm.model`; reload if installed |
| `GET` | `/llm/status` | Engine status |
| `POST` | `/llm/complete` | Completion |

Web UI: [openpfe-webui/assets/configuration/specification.md](../openpfe-webui/assets/configuration/specification.md).

---

## Related

- [openpfe-webui/specification.md](../openpfe-webui/specification.md)
- [openpfe-server/specification.md](../openpfe-server/specification.md)
