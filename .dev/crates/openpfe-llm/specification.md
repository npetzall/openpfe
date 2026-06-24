# openpfe-llm — specification

Owns **LLM config**, **model catalog**, registry, downloads, and inference. **Catalog** is per project (`catalog.json`, committed). **Runtime settings** are per machine (`llm.json`, local, not committed). **Weights** are shared (`USER_HOME/.openpfe/models/`).

**Serialization:** **JSON** on disk and on the HTTP API (`serde_json`) — no TOML in this crate.

## Paths

| Path | Scope | Git | Purpose |
|------|--------|-----|---------|
| `./.openpfe/catalog.json` | Project | **Commit** | `catalog[]` — model menu + download metadata |
| `./.openpfe/llm.json` | Project | **Ignore** | `llm` object — active model + inference settings |
| `$HOME/.openpfe/models/<id>/` | Shared | — | Weights + `manifest.json` |

Resolved from **process cwd** (project root) for project paths; `USER_HOME` for shared weights. No user-home LLM config file. Implementations use these literals (no shared path-helper crate).

**Project template:** commit `catalog.json`; add `llm.json` to `.gitignore`. Other `.openpfe/` paths (graph, server runtime) follow their owning crates.

## `catalog.json` (committed)

```json
{
  "entries": [
    {
      "id": "llama-3.2-3b-instruct",
      "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
      "url": "https://example.com/models/llama-3.2-3b-instruct-q4_k_m.gguf",
      "sha256": "hex-digest-required-before-load",
      "recommended": true,
      "requirements": {
        "min_ram_gb": 8,
        "recommended_n_ctx": 4096
      }
    }
  ]
}
```

Missing file → empty `entries: []`.

### `catalog[]` entry (`CatalogEntry`)

| Field | Required | Purpose |
|-------|----------|---------|
| `id` | yes | Stable key; shared weights directory name |
| `filename` | yes* | Basename under shared `models/<id>/` (*unless `path` set) |
| `url` | yes* | HTTPS download URL (*unless `path` set) |
| `sha256` | yes | Verify after download / before load |
| `path` | no | Absolute path to existing `.gguf` — skip download |
| `recommended` | no | `bool` — project hint; **multiple** entries may be `true` |
| `requirements` | no | Team-tuned hardware hints (see below) |

No `default`, `tier`, or single-winner flag — use `recommended` only.

#### `requirements` object

Team opinion on what works well for this project (may differ from manifest measurements):

| Field | Type | Purpose |
|-------|------|---------|
| `min_ram_gb` | number | Minimum system RAM hint for this model |
| `recommended_n_ctx` | u32 | Suggested context size when this model is active |

Additional fields may be added in later revisions; clients ignore unknown keys.

### Catalog validation

- `id` non-empty; `sha256` required (64 hex digits).
- Without `path`: `filename` and `url` required.
- With `path`: `url` / `filename` not required; download rejected for that id.
- Entries missing `url` are non-portable — reject on create when not using `path`.

**Catalog mutations do not call `reload_engine`** unless `llm.model` is changed via `/llm/*` routes.

## `llm.json` (local, gitignored)

```json
{
  "model": "llama-3.2-3b-instruct",
  "n_ctx": 4096,
  "n_threads": 0,
  "temperature": 0.7,
  "max_tokens": 1024
}
```

Missing file → defaults (`model`: null / omitted, other fields per table below).

| Field | Default | Purpose |
|-------|---------|---------|
| `model` | — | Active catalog **id** |
| `n_ctx` | `4096` | Context size (load-time) |
| `n_threads` | `0` | llama.cpp threads (`0` = auto) |
| `temperature` | `0.7` | Default completion temperature |
| `max_tokens` | `1024` | Default cap per request |

Written by **`openpfe llm init`** (CLI) or HTTP `/llm/*` routes — not by catalog CRUD.

## Model artifact (shared registry)

### On disk (per model id)

```
$HOME/.openpfe/models/<id>/
  manifest.json
  <filename>.gguf
```

Only the HTTPS download flow writes here programmatically. Manual drop-in without manifest is unsupported for discover; validation errors apply.

### `manifest.json` (after successful install)

Factual install receipt (not team opinion):

```json
{
  "id": "llama-3.2-3b-instruct",
  "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
  "sha256": "<hex>",
  "url": "<source url>",
  "installed_at": "<RFC3339>",
  "file_size_bytes": 2013265920,
  "estimated_load_ram_gb": 4.2
}
```

| Field | Required | Purpose |
|-------|----------|---------|
| `id`, `filename`, `sha256`, `installed_at` | yes | Identity + verification |
| `url` | yes* | Source URL when downloaded (*required for portable installs) |
| `file_size_bytes` | yes | Measured artifact size on disk |
| `estimated_load_ram_gb` | no | Rough load estimate at install time |

### Download

1. Stream catalog `url` to `<dir>/<filename>.partial` (HTTPS only; **max 32 GiB** per artifact).
2. Verify `sha256`; on mismatch delete partial and error.
3. Rename to `<filename>`; write `manifest.json` (including size / load estimates).
4. **No** `huggingface-cli` in v1.
5. Jobs run in the **server process** (`DownloadManager` in-memory).

## CLI init (`openpfe llm init`)

**CLI only** — no HTTP init endpoint; Web UI nudges user to run init when `llm.json` is missing.

Normative detail: [openpfe/specification.md](../openpfe/specification.md#llm-init).

Library support in this crate:

- `probe_hardware()` — RAM (GPU later).
- `resolve_initial_config(catalog, hardware, installed)` — pick best **recommended ∩ capable** model; set `n_ctx` / `n_threads` from `requirements` when present.
- `write_llm_config` — persist `llm.json` only.

Init does **not** use `GET /catalog/discover`; it reads committed `catalog.json` directly.

## Native dependency (v1)

| Crate | Role |
|-------|------|
| `llama-cpp-2` | Rust API over llama.cpp |
| `llama-cpp-sys-2` | FFI + build |

## Internal Rust API

```rust
pub struct LlmSettings { /* llm.json document */ }
pub struct CatalogEntry { /* catalog[] row */ }
pub struct CatalogFile { pub entries: Vec<CatalogEntry> }

pub trait LlmService: Send + Sync {
    // catalog.json
    fn load_catalog(&self) -> Result<CatalogFile, LlmError>;
    fn get_catalog_entry(&self, id: &str) -> Result<CatalogEntry, LlmError>;
    fn create_catalog_entry(&self, entry: &CatalogEntry) -> Result<CatalogEntry, LlmError>;
    fn replace_catalog_entry(&self, id: &str, entry: &CatalogEntry) -> Result<CatalogEntry, LlmError>;
    fn patch_catalog_entry(&self, id: &str, patch: CatalogEntryPatch) -> Result<CatalogEntry, LlmError>;
    fn delete_catalog_entry(&self, id: &str) -> Result<(), LlmError>;
    fn discover_catalog_candidates(&self, query: DiscoverQuery) -> Result<DiscoverResponse, LlmError>;

    // llm.json
    fn load_llm_config(&self) -> Result<LlmSettings, LlmError>;
    fn write_llm_config(&self, settings: &LlmSettings) -> Result<(), LlmError>;

    // aggregate + install + inference
    fn list_models(&self) -> Result<Vec<ModelEntry>, LlmError>;
    fn start_download(&self, id: &str) -> Result<JobId, LlmError>;
    fn download_status(&self, job_id: &JobId) -> Result<DownloadStatus, LlmError>;
    fn set_active_model(&self, id: &str) -> Result<(), LlmError>;
    fn reload_engine(&self) -> Result<(), LlmError>;
    fn status(&self) -> LlmStatus;
    fn complete(&self, prompt: &str, opts: CompleteOptions) -> Result<String, LlmError>;
}
```

- **`complete`** on `spawn_blocking`; **single-flight** busy semantics.
- **Catalog create is single-entry only** — no bulk import endpoint; clients loop `POST /catalog` (loopback latency is negligible).

### `DiscoverQuery` / `DiscoverResponse`

`GET /catalog/discover` parameters:

| Param | Required | Values |
|-------|----------|--------|
| `source` | yes | `installed` \| `curated` \| `search` |
| `q` | for `search` | Provider search text |
| `provider` | for `search` | `huggingface` (default when `source=search`) |
| `curated` | for `curated` | List id, e.g. `openpfe-defaults` |
| `limit` | no | Default **20**, max **50** |

**200** — normalized candidates (read-only; does not mutate catalog):

```json
{
  "source": "installed",
  "candidates": [
    {
      "id": "llama-3.2-3b-instruct",
      "filename": "…",
      "url": "https://…",
      "sha256": "…",
      "provider": null,
      "in_catalog": true,
      "installed": true,
      "addable": false,
      "recommended": null,
      "requirements": null,
      "metadata": { "installed_at": "…", "file_size_bytes": 2013265920 },
      "error": null
    }
  ]
}
```

| Field | Meaning |
|-------|---------|
| `addable` | Valid for `POST /catalog` (has `id`, `url`, `sha256`, `filename`; manifest valid when `source=installed`) |
| `in_catalog` | Already in `catalog.json` |
| `installed` | Weights under `~/.openpfe/models/<id>/` |
| `error` | Per-candidate failure, e.g. `url_missing`, `sha256_mismatch`, `manifest_invalid` |

**Discover sources (phased):**

| `source` | Behavior | Network |
|----------|----------|---------|
| `installed` | Scan `~/.openpfe/models/*/manifest.json`; validate `.gguf` + sha256; require `url` for `addable` | No |
| `curated` | Load pinned / project curated list; cross-check `in_catalog` / `installed` | Optional |
| `search` | Query external provider (HuggingFace first) | Yes |

`search` and remote `curated` may return **`501`** `not_implemented` until implemented.

Adding to catalog: client picks candidate → **`POST /catalog`** with full entry (team sets `recommended` / `requirements`). **One request per entry** by design.

## HTTP exposure

Mounted by **`openpfe-ui`**; handlers call **`LlmService`**. Base path **`/api/v1`**. Normative route detail: [openpfe-ui/specification.md](../openpfe-ui/specification.md#llm-catalog-runtime-and-inference).

### Catalog (CRUD)

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/catalog` | `{ "entries": [ CatalogEntry … ] }` |
| `GET` | `/catalog/:id` | One `CatalogEntry` |
| `POST` | `/catalog` | Create one entry → **201** |
| `PUT` | `/catalog/:id` | Replace entry |
| `PATCH` | `/catalog/:id` | Partial update (`recommended`, `requirements`, …); `id` immutable |
| `DELETE` | `/catalog/:id` | Remove entry → **204**; does not delete shared weights |
| `GET` | `/catalog/discover` | Browse/search candidates — [discover](#discoverquery--discoverresponse) |

### Install jobs

| Method | Path | Purpose |
|--------|------|---------|
| `POST` | `/catalog/:id/download` | Start download → `{ "job_id", "status_url" }` |
| `GET` | `/downloads/:job_id` | `DownloadStatus` |

On `DownloadStatus::Complete`, if downloaded id is active model and engine not loaded, HTTP layer calls `reload_engine`.

**Deprecated aliases (remove after migration):** `POST /models/:id/download`, `GET /models/downloads/:job_id`.

### Runtime + inference

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/config` | `LlmSettings` only |
| `PUT` | `/llm/config` | Replace `llm.json`; `reload_engine` when [reload triggers](#reload_engine-triggers) apply |
| `PUT` | `/llm/active` | `{ "model": "<id>" }` → update `llm.model`; reload when installed |
| `GET` | `/llm/status` | `LlmStatus` |
| `POST` | `/llm/complete` | See [completion](#post-llmcomplete) |

### Read-only aggregate

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/models` | Catalog entries + `installed` + `manifest` (`ModelEntry`) |
| `GET` | `/models/:id` | One `ModelEntry`; **404** if id not in catalog |

### `POST /llm/complete`

Request:

```json
{
  "prompt": "string (required)",
  "temperature": 0.7,
  "max_tokens": 1024
}
```

`temperature` / `max_tokens` optional — default from `llm.json` when omitted.

Response:

```json
{ "text": "generated string" }
```

Errors: `503` `inference_busy` when `LlmError::Busy` (single-flight).

### `GET /llm/status` — `LlmStatus`

```json
{
  "loaded": true,
  "model_id": "llama-3.2-3b-instruct",
  "busy": false,
  "error": null
}
```

### `GET /models` — `ModelEntry`

```json
{
  "models": [
    {
      "id": "llama-3.2-3b-instruct",
      "filename": "…",
      "url": "https://…",
      "sha256": "<hex>",
      "path": null,
      "recommended": true,
      "requirements": { "min_ram_gb": 8, "recommended_n_ctx": 4096 },
      "installed": true,
      "manifest": {
        "id": "…",
        "filename": "…",
        "sha256": "…",
        "url": "…",
        "installed_at": "…",
        "file_size_bytes": 2013265920,
        "estimated_load_ram_gb": 4.2
      }
    }
  ]
}
```

### `GET /downloads/:job_id` — `DownloadStatus`

Tagged union (`state` field):

| `state` | Body fields |
|---------|-------------|
| `queued` | — |
| `running` | `bytes_received` (u64) |
| `complete` | — |
| `failed` | `message` (string) |

### `reload_engine` triggers

| Change | `reload_engine`? |
|--------|------------------|
| `llm.model` (active id) | **Yes** — when target weights installed |
| `llm.n_ctx` | **Yes** |
| `llm.n_threads` | **Yes** |
| `llm.temperature` | No — persist only; used on next `complete` |
| `llm.max_tokens` | No — persist only |
| Download of active id completes | **Yes** — HTTP layer polls `complete` then calls `reload_engine` |
| Catalog CRUD (`/catalog/*`) | No |

Web UI: [openpfe-webui/assets/configuration/specification.md](../openpfe-webui/assets/configuration/specification.md).

## Related

- [design.md](./design.md)
- [openpfe-ui/specification.md](../openpfe-ui/specification.md)
- [openpfe/specification.md](../openpfe/specification.md)
