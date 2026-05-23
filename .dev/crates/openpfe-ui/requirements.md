# openpfe-ui — requirements

## FR-6 HTTP / human API

- **FR-6.2** Problem graph interaction via [specification.md](./specification.md#graph); **polling** in v1.
- **FR-6.3** Expose **`GET`/`PUT` `/server/config`** — JSON same shape as `./.openpfe/server.json` ([openpfe-server/specification.md](../openpfe-server/specification.md)).
- **FR-6.4** Expose LLM routes — **`llm.json`**, models, download, active model, inference — via **`LlmService`**; orchestrate **`reload_engine`** ([openpfe-llm/specification.md](../openpfe-llm/specification.md)).

## Non-goals

- MCP, static embed, TOML config, persisting config files (delegate to server/llm crates).

## Related

- [openpfe-webui/requirements.md](../openpfe-webui/requirements.md)
- [openpfe-server/requirements.md](../openpfe-server/requirements.md) — FR-7 `server.json`
