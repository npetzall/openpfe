# openpfe-llm — design

**Required v1** — project **`llm.json`**, shared weights under **`$HOME/.openpfe/models/`**, HTTPS downloads, llama.cpp inference.

## Decisions

| Topic | Decision |
|-------|----------|
| **Config file** | **`./.openpfe/llm.json`** (JSON) — `llm` + `catalog`; owned and parsed in this crate (`serde_json`). |
| **Weights** | **`$HOME/.openpfe/models/<id>/`** — shared across projects. |
| **No TOML** | Same JSON shapes as HTTP API — one serialization stack. |
| **Paths** | Resolve `llm.json` from **process cwd**; weights from `USER_HOME`. |
| **HTTP** | Routes in **`openpfe-ui`**; persistence + engine in **`LlmService`**. |
| **Reload** | `reload_engine` after load-affecting `llm.json` changes or download of active id. |

## `LlmService`

Config I/O, registry, downloads, `LlmEngine` — see [specification.md](./specification.md).

## Integration

- `openpfe-server` builds `AppState` with `Arc<dyn LlmService>` (or concrete type) for `openpfe-ui`.
- Startup: load `llm.json`; `try_load` when active model installed.

## Related

- [openpfe-server/design.md](../openpfe-server/design.md) — `server.json`
- [openpfe-ui/design.md](../openpfe-ui/design.md) — orchestration
