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
| 409 | `conflict` | Graph constraint; duplicate catalog id; download in progress |
| 501 | `not_implemented` | Discover source not yet available |
| 503 | `model_not_loaded` | LLM not loaded |
| 503 | `inference_busy` | Single-flight inference |

Same origin as embedded static UI. **No CORS in v1**.

**Body limit:** **1 MiB** default on mutating routes.

Export: `pub fn api_router(state: AppState) -> Router`.

**Orchestration:** graph handlers delegate to **`GraphStore`** (`openpfe-graph`); LLM handlers delegate to **`LlmService`** (`openpfe-llm` / `llm.json`); MCP debug delegates to **`McpHandler`** (`openpfe-mcp`). **`server.json`** is **not** on this API — IPC admin only ([openpfe-ipc/specification.md](../openpfe-ipc/specification.md)). `AppState` is built in **`openpfe-server`** so this crate does not depend on `openpfe-server` — see [design.md](./design.md#appstate-v1).

### `AppState` (v1)

Defined in **`openpfe-ui`**; constructed by **`openpfe-server`** at startup and passed to `api_router`.

| Field | Type (conceptual) | Purpose |
|-------|-------------------|---------|
| `graph` | `Arc<Mutex<dyn GraphStore + Send>>` (or concrete `GrafeoGraphStore` behind same lock) | Sync graph API; lock for `&mut self` writes |
| `llm` | `Arc<dyn LlmService>` | `catalog.json`, `llm.json`, models, inference |
| `mcp` | `Arc<McpHandler>` (`openpfe-mcp`) | MCP JSON-RPC — Debug view; same handler as IPC `type: mcp` |

**Not in `AppState`:** `server.json` / server process config (IPC admin in `openpfe-server`).

Heavy `GraphStore` / `LlmService` work runs on **`spawn_blocking`** from async handlers ([design.md](./design.md)).

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
| `DELETE` | `/graph/edges` | Delete edge — query: `src_id`, `dst_id`, `type` |
| `GET` | `/graph/clusters/:id/subgraph` | Bounded subgraph |
| `GET` | `/graph/validate` | Cycle check on `depends_on` |
| `GET` | `/graph/problems/search` | Lexical search (S6) — “already recorded?” |
| `POST` | `/graph/problems/find-similar` | Merged similarity (S6+) — draft vs existing problems |

Schema: [openpfe-graph/specification.md](../openpfe-graph/specification.md). Live updates: **poll** in v1.

### Edges

v1 edges have **no openpfe UUID**. Identity is the directed triple **`(src_id, dst_id, type)`** — same as [`Edge`](../openpfe-graph/specification.md#identifiers) in `openpfe-graph` and `GraphStore::create_edge` / `delete_edge`. At most one edge per triple is assumed for delete semantics.

Wire shape for create and list responses (properties optional). JSON field **`type`** is the edge type ([openpfe-graph/specification.md](../openpfe-graph/specification.md#edge-types)); maps to `Edge.edge_type` in the graph crate.

```json
{
  "src_id": "<uuid>",
  "dst_id": "<uuid>",
  "type": "depends_on",
  "contract_body": "…"
}
```

Allowed `type` values: `depends_on`, `member_of`, `interfaces`.

#### `POST /graph/edges`

**Request body** — `src_id`, `dst_id`, `type` required; additional keys are edge properties (e.g. `contract_body` on `interfaces`).

Delegates to `GraphStore::create_edge`. Unknown `src_id` or `dst_id` → `404` `not_found`. `member_of` target not a `cluster` → `409` `conflict`.

**201** — response body is the created `Edge` (same shape as above).

#### `DELETE /graph/edges`

Query (all required):

| Param | Notes |
|-------|--------|
| `src_id` | Source node UUID |
| `dst_id` | Target node UUID |
| `type` | Edge type (`depends_on`, `member_of`, `interfaces`) |

Delegates to `GraphStore::delete_edge`. Missing or unknown query param → `400` `invalid_request`. Unknown endpoint node → `404` `not_found`. No matching edge → `404` `not_found`.

**204** — edge removed.

### Search and similarity

Domain types and adapter behavior: [openpfe-graph/specification.md#search-and-similarity-v1](../openpfe-graph/specification.md#search-and-similarity-v1). Handlers call `GraphStore::search_problems` / `GraphStore::find_similar` on the graph handle in `AppState` ([design.md](./design.md)).

**`k` (both routes):** optional; default **5**; valid range **1–50**. Out-of-range → `400` `invalid_request`.

#### `GET /graph/problems/search` — S6

Query:

| Param | Required | Notes |
|-------|----------|--------|
| `q` | yes | Free-text query (BM25 over `problem.title` and `problem.description`) |
| `k` | no | Max hits (see above) |

**200** — JSON array of `SearchHit` (sorted by `score` descending):

```json
[
  { "node_id": "<uuid>", "title": "…", "score": 12.34 }
]
```

Empty or missing `q` → `400` `invalid_request`.

#### `POST /graph/problems/find-similar` — S6+

Query: optional `k` (see above).

**Request body** — `FindSimilarDraft`:

| Field | Type | Required | Notes |
|-------|------|----------|--------|
| `title` | string | yes | Draft problem title |
| `description` | string | yes | May be `""` |
| `cluster_id` | string (UUID) | no | When set, enables **structural** leg within that cluster |
| `embedding` | `f32[]` | no | When set, enables hybrid lexical + vector leg ([openpfe-graph/specification.md](../openpfe-graph/specification.md)) |

Missing `title` → `400` `invalid_request`. Omit `embedding` in v1 when no embedding API is available — lexical and structural legs still apply ([L-2](./gaps_post_007.md#l-2-no-embedding-endpoint) in gaps).

**200** — JSON array of `SimilarHit` (sorted by `score` descending):

```json
[
  {
    "node_id": "<uuid>",
    "title": "…",
    "score": 8.1,
    "match_kinds": ["lexical", "structural"],
    "snippet": "…"
  }
]
```

`match_kinds` values: `lexical`, `semantic`, `structural` — per [openpfe-graph/specification.md#search-and-similarity-v1](../openpfe-graph/specification.md#search-and-similarity-v1). `snippet` may be omitted or `null`.

---

## LLM (catalog, runtime, inference)

Delegated to **`openpfe-llm`** — [specification.md](../openpfe-llm/specification.md). Normative DTOs and reload rules live there; this section defines HTTP routing.

Web UI: [openpfe-webui/assets/configuration/specification.md](../openpfe-webui/assets/configuration/specification.md).

### Catalog (CRUD)

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/catalog` | List `{ "entries": [ CatalogEntry … ] }` |
| `GET` | `/catalog/:id` | One `CatalogEntry` |
| `POST` | `/catalog` | Create **one** entry → **201** + `Location` |
| `PUT` | `/catalog/:id` | Replace entry |
| `PATCH` | `/catalog/:id` | Partial update |
| `DELETE` | `/catalog/:id` | Remove entry → **204** |
| `GET` | `/catalog/discover` | Browse candidates — query: `source`, `q`, `provider`, `curated`, `limit` |

**`POST /catalog`:** single entry only — no bulk body. Clients add multiple models with sequential requests (loopback).

**`GET /catalog/discover`:** read-only. Does not mutate `catalog.json`. See [openpfe-llm/specification.md](../openpfe-llm/specification.md#discoverquery--discoverresponse).

### Install jobs

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/catalog/:id/download` | `{ "job_id", "status_url" }` — start HTTPS download |
| `GET` | `/downloads/:job_id` | `DownloadStatus` |

When status reaches `complete` and active model is installed but not loaded, handler calls **`reload_engine`**.

**Deprecated (remove after migration):** `POST /models/:id/download`, `GET /models/downloads/:job_id`.

### Runtime + inference

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/config` | `LlmSettings` only (`llm.json`) |
| `PUT` | `/llm/config` | Replace `llm.json`; **`reload_engine`** when load-affecting |
| `PUT` | `/llm/active` | `{ "model": "<id>" }` → update `llm.model`; reload if installed |
| `GET` | `/llm/status` | `LlmStatus` |
| `POST` | `/llm/complete` | Completion |

### Read-only aggregate

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/models` | `ModelEntry` = catalog + `installed` + `manifest` |
| `GET` | `/models/:id` | One `ModelEntry`; **404** if not in catalog |

**No LLM init over HTTP** — bootstrap is CLI-only ([openpfe/specification.md](../openpfe/specification.md#llm-init)).

---

## MCP debug (JSON-RPC)

Delegated to **`McpHandler`** (`openpfe-mcp`) — same tools/resources as IPC `type: mcp` ([openpfe-mcp/specification.md](../openpfe-mcp/specification.md)). For Web UI **Debug** integration testing; IDE agents use IPC via `openpfe mcp`.

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/debug/mcp` | MCP JSON-RPC 2.0 request object or batch array |

**Request body:** same JSON as IPC `type: mcp` envelope `payload` — a single MCP JSON-RPC object, or a JSON array of objects (batch).

**Response body:** MCP JSON-RPC response object or batch array for that request — same shape as IPC `type: mcp` response `payload`.

**HTTP status:** **200** when the frame is valid JSON-RPC (including JSON-RPC `error` objects in the body). **400** `invalid_request` for malformed JSON or non-JSON-RPC shape.

MCP-level errors use JSON-RPC `error` inside the response body, not the HTTP error envelope at the top of this document.

**Safety (product):** [openpfe-webui/assets/debug/specification.md](../openpfe-webui/assets/debug/specification.md) — UI defaults to read-only probes; the API exposes the full v1 tool surface.

---

## Related

- [openpfe-webui/specification.md](../openpfe-webui/specification.md)
- [openpfe-server/specification.md](../openpfe-server/specification.md)
