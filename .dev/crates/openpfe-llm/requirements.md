# openpfe-llm — requirements

**Required for v1** — not deferred.

## FR-8 LLM (config, registry, inference)

- **FR-8.1** Load and write project **`./.openpfe/catalog.json`** (committed catalog) and **`./.openpfe/llm.json`** (local runtime settings).
- **FR-8.2** Store installed models under **`USER_HOME/.openpfe/models/<id>/`** (shared).
- **FR-8.3** Resolve active model by catalog **`id`**; validate before load.
- **FR-8.4** Local inference via **llama.cpp** (`llama-cpp-2`).
- **FR-8.5** Download via **HTTPS** + **sha256**; atomic install; write **`manifest.json`** including **`file_size_bytes`** (and optional load estimates).
- **FR-8.6** User-provided weights via catalog **`path`** or files under `models/<id>/`.
- **FR-8.7** Server **may start** without a loaded model.
- **FR-8.8** **Single-flight** inference.
- **FR-8.9** **`reload_engine`** after load-affecting **`llm.json`** changes or active-model download complete — **not** after catalog-only CRUD.
- **FR-8.10** **Catalog CRUD** — create, read, update, delete single catalog entries; **one entry per create request** (no bulk import).
- **FR-8.11** **`GET /catalog/discover`** — read-only candidates: **`installed`** (scan shared manifests), **`curated`**, **`search`** (provider query; phased).
- **FR-8.12** Catalog entries support **`recommended`** (multiple allowed) and team-tuned **`requirements`** (`min_ram_gb`, `recommended_n_ctx`, …).
- **FR-8.13** Reject discover/import candidates whose manifest lacks **`url`** (non-portable) or fails validation.
- **FR-8.14** Library support for CLI init: **`probe_hardware`**, **`resolve_initial_config`**, **`write_llm_config`** — reads **`catalog.json`** only (not discover).

## Non-functional

- On-disk config uses **`serde_json`** only (no TOML).
- Inference on **`spawn_blocking`**.
- Download jobs live in the **server process**; CLI init starts download via HTTP after IPC echo.

## Related

- [specification.md](./specification.md)
- [openpfe/requirements.md](../openpfe/requirements.md) — FR-5 LLM init (CLI)
- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md)
