# Plan 008: `openpfe-server` (LLM slice wiring)

**Status:** Complete (2026-06-06).

**Read when:** replacing the phase-1 HTTP stub with **`openpfe-ui::api_router`**, constructing **`AppState`**, and opening the graph at startup per [openpfe-server/design.md](../crates/openpfe-server/design.md) step 6.

**Assumes:** [008-openpfe-ui](./008-openpfe-ui.md) **Complete**; [008-openpfe-llm](./008-openpfe-llm.md) **Complete**; [008-openpfe-mcp](./008-openpfe-mcp.md) **Complete**. [007-openpfe-graph](./007-openpfe-graph.md) **Complete**.

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-server/design.md](../crates/openpfe-server/design.md) | Startup sequence, `AppState` construction, HTTP composition |
| [openpfe-server/specification.md](../crates/openpfe-server/specification.md) | Runtime paths, shutdown |
| [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md#appstate-v1) | `AppState` fields |
| [openpfe-graph/specification.md](../crates/openpfe-graph/specification.md) | `./.openpfe/graph/store/` |
| [openpfe-llm/design.md](../crates/openpfe-llm/design.md) | `LlamaLlmService::new`, `try_load` |
| [cross-cutting.md](../cross-cutting.md) | cwd = project root |

## Prerequisites

- [x] [004-openpfe-server-impl.md](./004-openpfe-server-impl.md) — lock, socket, echo, HTTP stub
- [x] [005-openpfe-wiring.md](./005-openpfe-wiring.md) — `openpfe --server` invokes `run_server`
- [x] [007-openpfe-graph.md](./007-openpfe-graph.md) — `GrafeoGraphStore::open`
- [x] [008-intake.md](./008-intake.md) — **Complete**
- [x] [008-openpfe-llm.md](./008-openpfe-llm.md) — **Complete**
- [x] [008-openpfe-mcp.md](./008-openpfe-mcp.md) — **Complete**
- [x] [008-openpfe-ui.md](./008-openpfe-ui.md) — **Complete**

## Goal

Wire the running server process to **human LLM HTTP API**: at startup open an (empty) graph, construct **`LlamaLlmService`** and stub **`McpHandler`**, build **`AppState`**, and serve **`openpfe_ui::api_router`** on the existing loopback listener. IPC **echo** continues to return the live **`http_base_url`**. Closes [gaps_post_007.md](../crates/openpfe-ui/gaps_post_007.md) **S-2** for the LLM slice (graph handle + `AppState`).

## Startup sequence (normative for this plan)

1. Existing steps 1–5 unchanged (runtime dir, pid flock, stale socket, bind IPC + HTTP).
2. **Graph:** `GrafeoGraphStore::open("./.openpfe/graph/store/")` — create dirs if missing; wrap in `Arc<Mutex<_>>`.
3. **LLM:** `Arc::new(LlamaLlmService::new()?)` — loads `llm.json`, `try_load` active model if installed.
4. **MCP:** `Arc::new(McpHandler::new(graph.clone()))`.
5. **UI state:** `AppState { graph, llm, mcp }`.
6. **HTTP router:** `api_router(state)` — LLM routes only until graph HTTP plan lands.
7. Serve merged router (static `openpfe-webui` **deferred** — API-only OK for curl testing).

## Tasks

### Implementation

#### 1. Path dependencies

- [x] `crates/openpfe-server/Cargo.toml` — add `openpfe-ui`, `openpfe-llm`, `openpfe-graph`, `openpfe-mcp` path deps (keep existing deps)

#### 2. Replace `HttpStub` router

- [x] Refactor `http_stub.rs` or new `http.rs` — build `api_router(AppState)` instead of `GET /` only
- [x] Keep `127.0.0.1:0` bind; `base_url` for echo unchanged
- [x] Optional: keep `GET /` → minimal OK or redirect note until `openpfe-webui` plan

#### 3. Service construction

- [x] Extract `build_app_state(cwd: &Path) -> Result<AppState, ServerError>` (or inline in `run_server`) — graph open + LLM + MCP
- [x] Map `GraphError` / `LlmError` into `ServerError` at startup (document: missing `llm.json` is OK; corrupt JSON may error)
- [x] On shutdown: `graph.lock().close()` best-effort before dropping state ([007 storage path](../crates/openpfe-graph/specification.md))

#### 4. `run_server` integration

- [x] Wire `build_app_state` into existing `run_server` / runtime entry used by `openpfe --server`
- [x] Graceful shutdown: drop router state; existing shutdown timeout applies

#### 5. Integration tests

- [x] Extend `tests/echo_integration.rs` — echo still returns `http_base_url`
- [x] New: `GET {base}/api/v1/llm/status` → 200 JSON (`loaded` field present)
- [x] Tests use `tempfile` project root; set **cwd** to temp dir before `run_server`
- [ ] Optional `#[ignore] llm_complete_with_local_gguf` — manual e2e with `path` catalog

## Acceptance criteria

- [x] All task boxes `[x]`
- [x] `cargo test -p openpfe-server` passes (non-ignored)
- [x] `cargo clippy -p openpfe-server -- -D warnings` clean
- [x] `openpfe --server` in temp project serves `/api/v1/llm/status`
- [x] Echo `http_base_url` matches bound listener
- [x] Graph opens under `./.openpfe/graph/store/` relative to cwd
- [x] Plan **Status** → `Complete (YYYY-MM-DD)`

## Out of scope

- **`server.json`** load/save (still deferred from plan 004)
- **`TraceLayer` / tower-http** — optional follow-up
- **`openpfe-webui`** static mount — separate plan
- Full graph HTTP routes
- Full MCP tools (stub handler only)
- IPC `type: mcp` routing to real tools (stub responses acceptable if wired)

## End-to-end verification (008 series)

When this plan is **Complete**, the 008 slice is done:

1. `cd` to temp project with `.openpfe/` layout
2. `openpfe --server` (or test harness)
3. Configure catalog (`path` to tiny GGUF or download flow)
4. `PUT /api/v1/llm/active`
5. `POST /api/v1/llm/complete` — chat smoke test
6. WebUI Debug panel works once an `openpfe-webui` assets plan mounts (optional)

## Next

- **`openpfe-ui` graph HTTP** plan (TBD) — `/graph/*` on same `AppState`
- **`009-openpfe-mcp`** (TBD) — full graph tools + `rmcp` intake
- **`openpfe-webui`** static shell plan (TBD)
