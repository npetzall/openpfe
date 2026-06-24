# openpfe-llm — specification

Owns **LLM JSON config**, model registry, downloads, and inference. **Config** is per project (`llm.json`); **weights** are shared (`USER_HOME/.openpfe/models/`).

**Serialization:** **JSON** on disk and on the HTTP API (`serde_json`) — no TOML in this crate.

## Paths

| Path | Scope | Purpose |
|------|--------|---------|
| `./.openpfe/llm.json` | Project | `llm` + `catalog` |
| `$HOME/.openpfe/models/<id>/` | Shared | Weights + `manifest.json` |

Resolved from **process cwd** (project root) for project paths; `USER_HOME` for shared weights. No user-home LLM config file. Implementations use these literals (no shared path-helper crate).

## `llm.json` document (v1)

```json
{
  "llm": {
    "model": "llama-3.2-3b-instruct",
    "n_ctx": 4096,
    "n_threads": 0,
    "temperature": 0.7,
    "max_tokens": 1024
  },
  "catalog": [
    {
      "id": "llama-3.2-3b-instruct",
      "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
      "url": "https://example.com/models/llama-3.2-3b-instruct-q4_k_m.gguf",
      "sha256": "hex-digest-required-before-load"
    }
  ]
}
```

Missing file → defaults; `catalog` may be `[]`.

### `llm` object

| Field | Default | Purpose |
|-------|---------|---------|
| `model` | — | Active catalog **id** |
| `n_ctx` | `4096` | Context size (load-time) |
| `n_threads` | `0` | llama.cpp threads (`0` = auto) |
| `temperature` | `0.7` | Default completion temperature |
| `max_tokens` | `1024` | Default cap per request |

### `catalog[]` entry

| Field | Required | Purpose |
|-------|----------|---------|
| `id` | yes | Stable key |
| `filename` | yes* | Basename under shared `models/<id>/` (*unless `path` set) |
| `url` | yes* | HTTPS download URL (*unless `path` set) |
| `sha256` | yes | Verify after download / before load |
| `path` | no | Absolute path to existing `.gguf` — skip download |

## Model artifact (registry)

### On disk (per model id)

```
$HOME/.openpfe/models/<id>/
  manifest.json
  <filename>.gguf
```

### `manifest.json` (after successful install)

```json
{
  "id": "llama-3.2-3b-instruct",
  "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
  "sha256": "<hex>",
  "url": "<source url if downloaded>",
  "installed_at": "<RFC3339>"
}
```

### Download (v1)

1. Stream URL to `<dir>/<filename>.partial` (HTTPS only; **max 32 GiB** per artifact).
2. Verify `sha256`; on mismatch delete partial and error.
3. Rename to `<filename>`; write `manifest.json`.
4. **No** `huggingface-cli` in v1.

## Native dependency (v1)

| Crate | Role |
|-------|------|
| `llama-cpp-2` | Rust API over llama.cpp |
| `llama-cpp-sys-2` | FFI + build |

## Internal Rust API

```rust
pub struct LlmSettings { /* llm.json "llm" object */ }
pub struct LlmFile { pub llm: LlmSettings, pub catalog: Vec<CatalogEntry> }

pub trait LlmService: Send + Sync {
    fn load_config(&self) -> Result<LlmFile, LlmError>;
    fn write_config(&self, file: &LlmFile) -> Result<(), LlmError>;
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
- **`reload_engine`** after load-affecting `llm.json` changes or active-model download complete.

## HTTP exposure (v1)

Mounted by **`openpfe-ui`**; handlers call **`LlmService`**. JSON bodies below are normative DTO mirrors of crate types (`serde_json`).

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/config` | Full `llm.json` document (`LlmFile`) |
| `PUT` | `/llm/config` | Replace `LlmFile`; call `reload_engine` when [reload triggers](#reload_engine-triggers) apply |
| `GET` | `/models` | `{ "models": [ ModelEntry … ] }` |
| `GET` | `/models/:id` | `ModelEntry` (manifest + `installed` when present) |
| `POST` | `/models/:id/download` | `{ "job_id": "<uuid>" }` |
| `GET` | `/models/downloads/:job_id` | `DownloadStatus` |
| `PUT` | `/llm/active` | Request `{ "model": "<id>" }` → response `{ "model": "<id>" }`; `set_active_model` + `reload_engine` when installed |
| `GET` | `/llm/status` | `LlmStatus` |
| `POST` | `/llm/complete` | See [completion](#post-llmcomplete) |

### `POST /llm/complete`

Request:

```json
{
  "prompt": "string (required)",
  "temperature": 0.7,
  "max_tokens": 1024
}
```

`temperature` / `max_tokens` optional — default from `llm.json` `llm` object when omitted.

Response:

```json
{ "text": "generated string" }
```

Errors: `409` or mapped client error when `LlmError::Busy` (single-flight).

### `GET /llm/status` — `LlmStatus`

```json
{
  "loaded": true,
  "model_id": "llama-3.2-3b-instruct",
  "busy": false,
  "error": null
}
```

| Field | Type | Meaning |
|-------|------|---------|
| `loaded` | bool | Active model GGUF loaded in engine |
| `model_id` | string? | Active catalog id from `llm.model` |
| `busy` | bool | Inference in progress |
| `error` | string? | Last load/engine error (process still runs — FR-8.7) |

### `GET /models` — `ModelEntry`

```json
{
  "models": [
    {
      "id": "llama-3.2-3b-instruct",
      "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
      "url": "https://example.com/…",
      "sha256": "<hex>",
      "path": null,
      "installed": true,
      "manifest": { "id": "…", "filename": "…", "sha256": "…", "url": "…", "installed_at": "…" }
    }
  ]
}
```

### `GET /models/downloads/:job_id` — `DownloadStatus`

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
| `catalog[]` edits (no load-field change) | No |

Web UI: [openpfe-webui/assets/configuration/specification.md](../openpfe-webui/assets/configuration/specification.md).

## Related

- [design.md](./design.md)
- [openpfe-ui/specification.md](../openpfe-ui/specification.md)
