# Plan 008: `openpfe-llm`

**Status:** Complete (2026-06-06).

**Read when:** implementing `crates/openpfe-llm/` — project **`llm.json`**, shared model registry, HTTPS downloads, and **llama.cpp** inference per [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md).

**Trait pattern:** `LlmService` in this crate **is** the product abstraction. `openpfe-ui` depends on `LlmService` / `LlamaLlmService` directly — no duplicate wrapper in consumers ([coding-rust.md](../guidelines/coding-rust.md#workspace-crate-boundaries-ports)).

**Assumes:** [008-intake.md](./008-intake.md) **Complete** (intake approved). Plans [001](./001-scaffolding.md)–[005](./005-openpfe-wiring.md) and [007-openpfe-graph.md](./007-openpfe-graph.md) **Complete**.

## Normative sources

| Doc | Use |
|-----|-----|
| [openpfe-llm/requirements.md](../crates/openpfe-llm/requirements.md) | FR-8.1–8.9 |
| [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md) | Paths, `llm.json`, `LlmService`, download flow |
| [openpfe-llm/design.md](../crates/openpfe-llm/design.md) | Config vs weights, reload policy |
| [cross-cutting.md](../cross-cutting.md) | cwd = project root; `llm.json` + `USER_HOME/.openpfe/models/` |
| [guidelines/testing-rust.md](../guidelines/testing-rust.md) | `tempfile`, mock inference in CI, `#[ignore]` for real GGUF |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md) | Sync domain API; no HTTP in this crate |

## Prerequisites

- [x] [008-intake.md](./008-intake.md) — **Complete**; human intake approval checked
- [x] [001-scaffolding.md](./001-scaffolding.md) — `crates/openpfe-llm/` member exists
- [x] [007-openpfe-graph.md](./007-openpfe-graph.md) — graph crate complete (embeddings deferred; not required for this plan)

## Goal

Ship **`openpfe-llm`** as the sync **`LlmService`** implementation: load/write **`./.openpfe/llm.json`**, registry under **`$HOME/.openpfe/models/<id>/`**, HTTPS download with sha256 verification, and local completion via **`llama-cpp-2`**. Server may start without a loaded model (FR-8.7). **No HTTP, IPC, or MCP** in this crate.

## Public API (this plan)

```rust
// lib.rs re-exports
pub mod error;
pub mod types;
pub mod service;

pub use error::{LlmError, Result};
pub use service::LlamaLlmService;
pub use types::{
    CatalogEntry, CompleteOptions, DownloadStatus, JobId, LlmFile, LlmService,
    LlmSettings, LlmStatus, ModelEntry,
};
```

| Type / fn | Purpose |
|-----------|---------|
| `LlmService` | Product port — config, registry, download, inference |
| `LlamaLlmService` | Default impl backed by `llama-cpp-2` |
| `LlamaLlmService::new()` | Resolve paths from cwd + `USER_HOME`; load `llm.json`; `try_load` active model if installed |
| `reload_engine` | Unload + reload when load-affecting settings change |

## Spec doc updates (before or with implementation)

Close [gaps_post_007.md](../crates/openpfe-ui/gaps_post_007.md) **L-1** and **L-3** in [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md):

- [x] **HTTP wire shapes** (normative for handlers in `openpfe-ui`; defined here as DTO mirrors):
  - `POST /llm/complete` — request `{ "prompt", "temperature"?, "max_tokens"? }` → `{ "text" }`
  - `GET /llm/status` — `{ "loaded", "model_id", "busy", "error" }`
  - `GET /models` — `{ "models": [ ModelEntry… ] }`
  - `GET /models/:id` — manifest + `installed` when present
  - `GET /models/downloads/:job_id` — `DownloadStatus`
  - `PUT /llm/active` — `{ "model" }` → `{ "model" }`
- [x] **`reload_engine` trigger table** — `model`, `n_ctx`, `n_threads` → reload; `temperature`, `max_tokens` → persist only; reload after download when `id == llm.model`

## Module layout

```
crates/openpfe-llm/
  Cargo.toml
  src/
    lib.rs
    error.rs           # LlmError (thiserror)
    types.rs           # DTOs + LlmService trait
    paths.rs           # llm.json + models dir literals
    config.rs          # load defaults, atomic write
    registry.rs        # list_models, manifest, resolve .gguf path
    download.rs        # HTTPS stream, sha256, job progress
    engine.rs          # llama-cpp-2 load / complete / unload
    service.rs         # LlamaLlmService
  tests/
    common/mod.rs
    config_roundtrip.rs
    download_mock_https.rs
    busy_and_path.rs
    inference_manual.rs   # #[ignore]
```

## Tasks

### Implementation (phase C — after intake approval only)

#### 1. Errors and public surface

- [x] `error.rs` — `LlmError` variants: `Config`, `Io`, `NotFound`, `Download`, `Verify`, `Busy`, `Engine`, `InvalidRequest`; `Result<T>`
- [x] `lib.rs` — modules and re-exports per [Public API](#public-api-this-plan)
- [x] Crate-level rustdoc links to `.dev/crates/openpfe-llm/specification.md`

#### 2. Types

- [x] `types.rs` — `LlmSettings`, `CatalogEntry`, `LlmFile`, `ModelEntry`, `LlmStatus`, `CompleteOptions`, `DownloadStatus`, `JobId` (newtype over `uuid::Uuid` or string), `LlmService` trait per spec

#### 3. Paths and config

- [x] `paths.rs` — `./.openpfe/llm.json`, `$HOME/.openpfe/models/<id>/`
- [x] `config.rs` — missing file → defaults (`catalog: []`); `serde_json` roundtrip; atomic write (`write temp + rename`)

#### 4. Registry

- [x] `registry.rs` — merge catalog + disk: `installed`, read `manifest.json`, validate sha256 before load
- [x] Honor catalog **`path`** (absolute `.gguf`) — skip download (FR-8.6)
- [x] `set_active_model` — update `llm.model` in config file

#### 5. Download

- [x] `download.rs` — HTTPS only; stream to `<filename>.partial`; max **32 GiB**; verify sha256; rename; write `manifest.json` with `installed_at` (RFC3339)
- [x] `start_download` / `download_status` — in-memory job map keyed by `JobId`; one active download per model id (document behavior)
- [x] On successful download of active model id → caller invokes `reload_engine` (service method documents contract; HTTP layer in `openpfe-ui`)

#### 6. Engine

- [x] `engine.rs` — wrap `llama-cpp-2`: load GGUF with `n_ctx`, `n_threads`; `complete(prompt, temperature, max_tokens)` → `String`
- [x] `try_load` on `new()` when active model installed; errors stored in `LlmStatus` without failing process construction

#### 7. Service

- [x] `service.rs` — `LlamaLlmService`: internal mutex(es) for engine + jobs
- [x] **Single-flight** `complete` — concurrent call → `LlmError::Busy`
- [x] `reload_engine` — unload + reload per trigger table
- [x] Document: HTTP/async layers call `complete` / `reload_engine` from **`spawn_blocking`** (FR inference NFR)

#### 8. Integration tests

| Test | Pass criterion |
|------|----------------|
| `config_defaults_when_missing` | Empty cwd → defaults; write + reload |
| `download_mock_https` | Tiny payload via local mock server; manifest + sha256 |
| `busy_rejects_second_complete` | Two overlapping completes → `Busy` |
| `path_catalog_skips_download` | `path` entry loads without URL |
| `inference_manual` **#[ignore]** | Local tiny `.gguf` via `path`; one completion |

- [x] All tests use `tempfile`; model dir under temp `HOME` or override hook for tests
- [x] **No** multi-GB download in default `cargo test`

## Acceptance criteria

- [x] All implementation task boxes above are `[x]`
- [x] [008-intake.md](./008-intake.md) **Complete**
- [x] Spec updates for L-1 / L-3 merged in `openpfe-llm/specification.md`
- [x] `cargo test -p openpfe-llm` passes (non-ignored)
- [x] `cargo clippy -p openpfe-llm -- -D warnings` clean (or documented in `llama-cpp-2/verdict.md`)
- [x] `LlmService` surface matches specification trait
- [x] Plan **Status** → `Complete (2026-06-06)`

## Out of scope

- HTTP handlers — [008-openpfe-ui](./008-openpfe-ui.md)
- `AppState` / server mount — [008-openpfe-server](./008-openpfe-server.md)
- `/llm/embed`, streaming tokens, chat message arrays (v1: single `prompt` in / `text` out)
- Default catalog with live HuggingFace URL (dev uses `path` or manual catalog); placeholder URLs in spec stay examples until product pins a model
- HuggingFace CLI (FR-8.5 explicit no)

## Next

[008-openpfe-mcp](./008-openpfe-mcp.md) (stub, parallel OK) → [008-openpfe-ui](./008-openpfe-ui.md) → [008-openpfe-server](./008-openpfe-server.md).
