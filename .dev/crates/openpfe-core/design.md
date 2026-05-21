# openpfe-core — design

Config merge, path resolution, domain types, model registry (id → path under user home). **No** sockets or protocol handlers.

## Decisions

| Topic | Decision |
|-------|----------|
| **Config layout (v1)** | Nested TOML: `[server]`, `[http]`, `[llm]`, `[[models.catalog]]` in **`config.toml` only** — no separate `models.toml` in v1. |
| **Merge — tables** | **Deep-merge** inline tables / `[section]` maps; project leaf scalars **replace** user. |
| **Merge — scalar arrays** | If project sets the key, **replace the entire array** (no element-wise merge). |
| **Merge — `[[models.catalog]]`** | Merge array-of-tables by **`id`**: same `id` → project row wins; project-only ids added; stable order by `id` ascending. |
| **Missing files** | Not errors — typed `Config` with `#[serde(default)]` ([requirements.md](./requirements.md) FR-7.3). |
| **Config writes** | HTTP/UI write **`./.openpfe/config.toml` only** (project overrides); user file is read-only via API unless a future “edit user defaults” flow is added. |
| **`USER_HOME`** | `dirs::home_dir()` (fallback: `$HOME` env); panic/error only if neither resolves. |
| **Model download (v1)** | **HTTPS + checksum** (`reqwest`, verify `sha256` from catalog/manifest). **No** Hugging Face CLI in v1. **Max size 32 GiB** per file. **Also:** user-placed files or catalog `path` override (absolute path to `.gguf`). |
| **Install layout** | `$HOME/.openpfe/models/<id>/` with `manifest.json` + weight file; download via `*.partial` then atomic rename. |

## Configuration merge

Load order:

1. `USER_HOME/.openpfe/config.toml` (optional)
2. `./.openpfe/config.toml` (optional; overrides)

Implementation:

- Deserialize each file to `toml::Value` or typed partial structs.
- Apply merge rules in [specification.md](./specification.md#merge-semantics-v1).
- Produce `EffectiveConfig` (typed) for server, UI, MCP, LLM.

Expose merged config to HTTP handlers and MCP tools.

## Models (registry)

| Aspect | Design |
|--------|--------|
| **Catalog** | `[[models.catalog]]` in user `config.toml` (mergeable from project by `id`) |
| **Storage** | `USER_HOME/.openpfe/models/<id>/` — `manifest.json` + weight file |
| **Sharing** | All projects read same user-level dirs; atomic install (`*.partial` → verify → rename) |
| **Immutability** | After verify, immutable until explicit remove/replace |
| **Selection** | `[llm].model` → catalog `id` → `model_dir(id)`; validate before `openpfe-llm` load |
| **User-provided** | Catalog `path` absolute to `.gguf`, or manual files under `models/<id>/` |

Download progress/API: **`openpfe-ui`** + server; install/checksum/registry: **`openpfe-core`**.

## Path helpers

- Resolve `.openpfe/`, `./.openpfe/server/`, `./.openpfe/graph/store/`
- Resolve `USER_HOME/.openpfe/models/<model_id>/`

## Related

- [openpfe-ui/design.md](../openpfe-ui/design.md) — config/model HTTP
- [openpfe-llm/design.md](../openpfe-llm/design.md) — inference load
- [cross-cutting.md](../../cross-cutting.md) — multi-crate data placement index
