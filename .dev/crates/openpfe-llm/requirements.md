# openpfe-llm — requirements

**Required for v1** — not deferred.

## FR-8 LLM (config, registry, inference)

- **FR-8.1** Load and write project **`./.openpfe/llm.json`** (JSON: `llm`, `catalog`).
- **FR-8.2** Store installed models under **`USER_HOME/.openpfe/models/<id>/`** (shared).
- **FR-8.3** Resolve active model by catalog **`id`**; validate before load.
- **FR-8.4** Local inference via **llama.cpp** (`llama-cpp-2`).
- **FR-8.5** Download via **HTTPS** + **sha256**; atomic install.
- **FR-8.6** User-provided weights via catalog **`path`** or files under `models/<id>/`.
- **FR-8.7** Server **may start** without a loaded model.
- **FR-8.8** **Single-flight** inference.
- **FR-8.9** **`reload_engine`** after load-affecting `llm.json` changes or active-model download complete.

## Non-functional

- On-disk config uses **`serde_json`** only (no TOML).
- Inference on **`spawn_blocking`**.

## Related

- [specification.md](./specification.md)
- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md)
