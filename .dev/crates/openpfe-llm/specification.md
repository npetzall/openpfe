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

Mounted by **`openpfe-ui`**; handlers call **`LlmService`**.

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/config` | Full `llm.json` document |
| `PUT` | `/llm/config` | Replace document; reload when needed |
| `GET` | `/models` | Catalog + `installed` per id |
| `GET` | `/models/:id` | Manifest + paths if installed |
| `POST` | `/models/:id/download` | `{ "job_id" }` |
| `GET` | `/models/downloads/:job_id` | Poll progress |
| `PUT` | `/llm/active` | `{ "model": "<id>" }` → update `llm.model`; reload if installed |
| `GET` | `/llm/status` | Engine status |
| `POST` | `/llm/complete` | Completion |

Web UI: [openpfe-webui/assets/configuration/specification.md](../openpfe-webui/assets/configuration/specification.md).

## Related

- [design.md](./design.md)
- [openpfe-ui/specification.md](../openpfe-ui/specification.md)
