# Plan 008: `openpfe-ui` (LLM HTTP slice)

**Status:** Complete (2026-06-06).

**Read when:** implementing LLM-related HTTP handlers in `crates/openpfe-ui/` — config, models, download, active model, status, completion — per [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md) and [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md).

**Assumes:** [008-openpfe-llm](./008-openpfe-llm.md) **Complete**; [008-openpfe-mcp](./008-openpfe-mcp.md) **Complete** (stub `McpHandler`). [008-intake](./008-intake.md) **Complete**.

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md) | `/api/v1` base, errors, `AppState`, LLM route table |
| [openpfe-ui/design.md](../crates/openpfe-ui/design.md) | axum router, `spawn_blocking`, orchestration |
| [openpfe-ui/requirements.md](../crates/openpfe-ui/requirements.md) | FR-6.4 |
| [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md) | HTTP wire shapes (L-1), `LlmService`, reload table (L-3) |
| [openpfe-webui/assets/configuration/specification.md](../crates/openpfe-webui/assets/configuration/specification.md) | Configuration view API |
| [openpfe-webui/assets/debug/specification.md](../crates/openpfe-webui/assets/debug/specification.md) | Debug chat: `POST /llm/complete`, `GET /llm/status` |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | `tower::ServiceExt::oneshot` |

## Prerequisites

- [x] [008-intake.md](./008-intake.md) — **Complete**
- [x] [008-openpfe-llm.md](./008-openpfe-llm.md) — **Complete** (`LlmService` / `LlamaLlmService`)
- [x] [008-openpfe-mcp.md](./008-openpfe-mcp.md) — **Complete** (`McpHandler` stub)
- [x] [007-openpfe-graph.md](./007-openpfe-graph.md) — `GraphStore` for `AppState` field type

## Goal

Export **`api_router(state: AppState) -> Router`** with **LLM + models routes only** under **`/api/v1`**, delegating to **`LlmService`**. Define **`AppState`** for the 008 slice so [008-openpfe-server](./008-openpfe-server.md) can mount the router. Enable **curl / HTTP client** testing of config, download, and chat (`POST /llm/complete`) without WebUI assets or graph HTTP routes.

## Public API (this plan)

```rust
pub struct AppState {
    pub graph: Arc<Mutex<dyn GraphStore + Send>>,
    pub llm: Arc<dyn LlmService>,
    pub mcp: Arc<McpHandler>,
}

pub fn api_router(state: AppState) -> Router;
```

| Export | Purpose |
|--------|---------|
| `AppState` | Shared state for handlers (constructed in `openpfe-server`) |
| `api_router` | Nest LLM routes at `/api/v1` |

## Routes in scope (this plan)

| Method | Path | Handler behavior |
|--------|------|------------------|
| `GET` | `/llm/config` | `load_config` → full `llm.json` document |
| `PUT` | `/llm/config` | `write_config`; `reload_engine` when load-affecting |
| `GET` | `/models` | `list_models` → `{ "models": … }` |
| `GET` | `/models/:id` | Detail / 404 |
| `POST` | `/models/:id/download` | `start_download` → `{ "job_id" }` |
| `GET` | `/models/downloads/:job_id` | `download_status` |
| `PUT` | `/llm/active` | `set_active_model` + `reload_engine` if installed |
| `GET` | `/llm/status` | `status()` |
| `POST` | `/llm/complete` | `complete` on `spawn_blocking` |

**Optional (same plan if trivial):** `POST /debug/mcp` → `mcp.handle_jsonrpc` (stub errors OK).

## Module layout

```
crates/openpfe-ui/
  Cargo.toml
  src/
    lib.rs
    state.rs           # AppState
    error.rs           # map LlmError → HTTP envelope
    router.rs          # api_router
    routes/
      llm.rs           # /llm/*
      models.rs        # /models/*
      debug.rs         # optional /debug/mcp
  tests/
    llm_routes.rs      # MockLlmService + oneshot
```

## Tasks

### Implementation

#### 1. Dependencies

- [x] `Cargo.toml` — `openpfe-graph`, `openpfe-llm`, `openpfe-mcp` (path); workspace `axum`, `tokio`, `serde`, `serde_json`, `thiserror`
- [x] Remove `stub()` from `lib.rs`

#### 2. `AppState` and errors

- [x] `state.rs` — `AppState` per [specification.md#appstate-v1](../crates/openpfe-ui/specification.md#appstate-v1)
- [x] `error.rs` — HTTP JSON `{ "error": { "code", "message" } }`; map `503 model_not_loaded`, `503 inference_busy`, `404 not_found`, `400 invalid_request`

#### 3. LLM handlers

- [x] `routes/llm.rs` — config get/put, active, status, complete
- [x] `PUT /llm/config` — compare old/new for reload triggers ([L-3](../crates/openpfe-llm/specification.md))
- [x] `POST /llm/complete` — body per L-1; `spawn_blocking` for `LlmService::complete`
- [x] Poll download completion in handler or document client polls then calls `reload_engine` via separate endpoint — prefer: **server-side** hook after download job `done` for active id (call `reload_engine` in `GET /models/downloads/:job_id` when state transitions to done, or in download worker callback in `openpfe-llm` — document chosen approach in rustdoc)

#### 4. Models handlers

- [x] `routes/models.rs` — list, get by id, start download, poll job

#### 5. Router

- [x] `router.rs` — `Router::new().nest("/api/v1", …)` with shared `State<AppState>`
- [x] **1 MiB** body limit on mutating routes (spec)
- [x] `pub fn api_router(state: AppState) -> Router`

#### 6. Mock for tests

- [x] `#[cfg(test)]` `MockLlmService` implementing `LlmService` (or test-only module)

#### 7. Integration tests

| Test | Pass criterion |
|------|----------------|
| `get_llm_config_defaults` | 200 + JSON document |
| `put_llm_active` | 200; mock records set_active |
| `complete_returns_text` | Mock returns fixed string |
| `complete_busy` | 503 `inference_busy` |
| `model_not_loaded` | 503 when engine unloaded |

- [x] Use `tower::ServiceExt::oneshot` — no TCP bind required

## Manual test script (acceptance aid)

After [008-openpfe-server](./008-openpfe-server.md):

```bash
BASE="$(openpfe echo | jq -r .http_base_url)/api/v1"
curl -s "$BASE/llm/status"
curl -s -X POST "$BASE/llm/complete" -H 'Content-Type: application/json' \
  -d '{"prompt":"Say hi in one sentence."}'
```

## Acceptance criteria

- [x] All task boxes `[x]`
- [x] `cargo test -p openpfe-ui` passes
- [x] `cargo clippy -p openpfe-ui -- -D warnings` clean
- [x] `api_router` exposes all LLM routes in scope
- [x] Wire JSON matches [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md) HTTP section (L-1)
- [x] `reload_engine` orchestration matches L-3 table
- [x] Plan **Status** → `Complete (YYYY-MM-DD)`

## Out of scope

- **`/graph/*` routes** — separate `openpfe-ui` graph plan (phase 2 remainder)
- **`openpfe-webui`** static embed — later plan
- CORS, auth, WebSocket/SSE streaming
- Client-side chat history (Debug view concatenates prompts; server is stateless per request)
- `server.json` HTTP

## Next

[008-openpfe-server](./008-openpfe-server.md) — replace HTTP stub, construct `AppState`, mount `api_router`.
