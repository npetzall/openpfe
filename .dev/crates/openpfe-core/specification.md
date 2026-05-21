# openpfe-core — specification

## Configuration paths

| Path | Scope |
|------|--------|
| `$HOME/.openpfe/config.toml` | User defaults |
| `./.openpfe/config.toml` | Project overrides (wins on conflict) |
| `$HOME/.openpfe/models/` | Downloaded model artifacts (immutable) |
| `./.openpfe/graph/store/` | IndraDB RocksDB files ([openpfe-graph/specification.md](../openpfe-graph/specification.md)) |

## Merge semantics (v1)

```
effective_config = merge(user_config, project_config)
```

| Value kind | Rule |
|------------|------|
| **Boolean, integer, float, string** | Project value **replaces** user at the same key path. |
| **Table / `[section]`** | **Deep-merge** keys; apply scalar rule at leaves. |
| **Array of scalars** | If project defines the key → **replace entire array**. |
| **`[[models.catalog]]`** | Merge by **`id`**: matching `id` → project row replaces user row; new project ids appended; sort merged result by `id` (UTF-8 ascending). |

Missing user or project file → treat as empty table for that layer.

## Config file layout (v1)

Single TOML file per scope — **no** `models.toml`. Normative sections:

| Section | Scope | Notes |
|---------|--------|--------|
| `[server]` | User + project | `log_level`, `shutdown_timeout_secs` |
| `[http]` | User + project | `host` default `127.0.0.1`; port remains ephemeral unless future key added |
| `[llm]` | User + project | `model` (catalog **id**), `n_ctx`, `n_threads` |
| `[[models.catalog]]` | **User file typical** | Project may add/override rows by `id` |

### Example (user `~/.openpfe/config.toml`)

```toml
[server]
log_level = "info"
shutdown_timeout_secs = 5

[http]
host = "127.0.0.1"

[llm]
model = "llama-3.2-3b-instruct"
n_ctx = 4096
n_threads = 0   # 0 = auto

[[models.catalog]]
id = "llama-3.2-3b-instruct"
filename = "llama-3.2-3b-instruct-q4_k_m.gguf"
url = "https://example.com/models/llama-3.2-3b-instruct-q4_k_m.gguf"
sha256 = "hex-digest-required-before-load"
```

### Example (project `./.openpfe/config.toml`)

```toml
[llm]
model = "llama-3.2-3b-instruct"
n_ctx = 8192

[server]
log_level = "debug"
```

Per-project tuning uses `[llm]` / `[server]`; `[[models.catalog]]` in the project file only when overriding or adding catalog **artifact** rows by `id`.

## Path resolution

| Helper | Resolved path |
|--------|----------------|
| `project_openpfe_dir()` | `./.openpfe/` |
| `project_server_dir()` | `./.openpfe/server/` |
| `project_graph_dir()` | `./.openpfe/graph/` (parent) |
| `project_graph_store_dir()` | `./.openpfe/graph/store/` |
| `user_openpfe_dir()` | `$HOME/.openpfe/` |
| `model_dir(id)` | `$HOME/.openpfe/models/<id>/` |

## Model artifact (registry)

### On disk (per model id)

```
$HOME/.openpfe/models/<id>/
  manifest.json
  <filename>.gguf          # or path symlink / recorded in manifest
```

### `manifest.json` (written after successful install)

```json
{
  "id": "llama-3.2-3b-instruct",
  "filename": "llama-3.2-3b-instruct-q4_k_m.gguf",
  "sha256": "<hex>",
  "url": "<source url if downloaded>",
  "installed_at": "<RFC3339>"
}
```

### Catalog row fields

| Field | Required | Purpose |
|-------|----------|---------|
| `id` | yes | Stable key; merge key for `[[models.catalog]]` |
| `filename` | yes* | Basename under `models/<id>/` (*unless `path` set) |
| `url` | yes* | HTTPS download URL (*unless `path` set) |
| `sha256` | yes | Verify after download |
| `path` | no | Absolute path to existing `.gguf` — skip download; register manifest |

### Download (v1)

1. Stream URL to `<dir>/<filename>.partial` (HTTPS only; follow redirects; **max download size 32 GiB** per artifact — abort and delete partial if exceeded).
2. Verify `sha256`; on mismatch delete partial and error.
3. Rename to `<filename>`; write `manifest.json`.
4. **No** `huggingface-cli` dependency in v1.

Manual install: user copies weights into `models/<id>/` and calls register API (or places file + manifest) — same layout.

Load/inference: [openpfe-llm/specification.md](../openpfe-llm/specification.md).

---

## Model setup (example)

End-to-end flow for v1 (config + HTTP; Web UI uses the same API).

### 1. Declare catalog (user config)

Edit `~/.openpfe/config.toml`:

```toml
[[models.catalog]]
id = "llama-3.2-3b-instruct"
filename = "llama-3.2-3b-instruct-q4_k_m.gguf"
url = "https://example.com/path/to/llama-3.2-3b-instruct-q4_k_m.gguf"
sha256 = "abc123…full-hex-digest…"

[llm]
model = "llama-3.2-3b-instruct"
n_ctx = 4096
n_threads = 0
```

`[[models.catalog]]` describes **where to get** the file; `[llm].model` names which catalog **`id`** this machine/project uses for inference.

### 2. Project override (optional)

`./.openpfe/config.toml` in the repo can override only selection:

```toml
[llm]
model = "llama-3.2-3b-instruct"
```

Catalog rows usually stay in the user file; project file wins on merge for the same `id` if you add one.

### 3. Download via HTTP (Web UI or curl)

With server running (`openpfe` → browser or known `http_base_url` from IPC echo):

```bash
# List catalog + whether weights are on disk
curl -s http://127.0.0.1:PORT/api/v1/models

# Start download (returns job id)
curl -s -X POST http://127.0.0.1:PORT/api/v1/models/llama-3.2-3b-instruct/download

# Poll until status is completed (or failed)
curl -s http://127.0.0.1:PORT/api/v1/models/downloads/JOB_ID
```

On success, files exist under `~/.openpfe/models/llama-3.2-3b-instruct/` (`manifest.json` + `.gguf`).

### 4. Select active model (project)

If not already set in config:

```bash
curl -s -X PUT http://127.0.0.1:PORT/api/v1/llm/active \
  -H 'Content-Type: application/json' \
  -d '{"model":"llama-3.2-3b-instruct"}'
```

Writes/updates `./.openpfe/config.toml` `[llm].model`. Server reloads `LlmEngine` when the model is installed.

### 5. Run inference (HTTP only in v1)

```bash
curl -s http://127.0.0.1:PORT/api/v1/llm/status
curl -s -X POST http://127.0.0.1:PORT/api/v1/llm/complete \
  -H 'Content-Type: application/json' \
  -d '{"prompt":"Summarize the login component problems."}'
```

### Alternative: skip download (local GGUF already on disk)

```toml
[[models.catalog]]
id = "my-local-llama"
filename = "model.gguf"
path = "/Users/me/Models/model.gguf"
sha256 = "…"   # still required to register/verify
```

Or copy into `~/.openpfe/models/my-local-llama/` and register via API.

### What is *not* in v1 docs as implemented yet

- No bundled default model in the binary — user (or example config) supplies catalog + download.
- No MCP tools for download or completion — agents use IDE LLM + MCP graph tools only.
- Register-without-download API shape for manual drops: implied by layout; exact `POST /models/:id/register` may be added at implementation time if needed.
