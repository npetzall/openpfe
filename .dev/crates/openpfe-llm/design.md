# openpfe-llm — design

**Required v1** — project **`catalog.json`** (committed) + local **`llm.json`**, shared weights under **`$HOME/.openpfe/models/`**, HTTPS downloads, llama.cpp inference.

## Decisions

| Topic | Decision |
|-------|----------|
| **Catalog file** | **`./.openpfe/catalog.json`** — committed; project's model menu + download metadata. |
| **Runtime file** | **`./.openpfe/llm.json`** — **gitignored**; per-machine active model + inference tuning. |
| **Weights** | **`$HOME/.openpfe/models/<id>/`** — shared across projects. |
| **Catalog vs manifest** | Catalog = intent + team **`requirements`** + **`recommended`** flags. Manifest = factual install receipt (**`file_size_bytes`**, load estimates). |
| **`recommended`** | Multiple entries may be `true`. No `default` or `tier`. CLI init picks best **recommended ∩ capable** for hardware. |
| **Catalog API** | REST CRUD on **`/catalog`**; **one `POST /catalog` per entry** (no bulk import). |
| **Discover** | **`GET /catalog/discover`** — read-only candidates from **`installed`**, **`curated`**, **`search`** (phased). Client adds via **`POST /catalog`**. |
| **No TOML** | Same JSON shapes as HTTP API — one serialization stack. |
| **Paths** | Resolve project files from **process cwd**; weights from `USER_HOME`. |
| **HTTP** | Routes in **`openpfe-ui`**; persistence + engine in **`LlmService`**. |
| **Init** | **`openpfe llm init`** (CLI only) — not HTTP/Web UI. |
| **Init download** | Ensure server → IPC **`echo`** → HTTP **`POST /catalog/:id/download`**; print **`job_id`** + **`status_url`**. |
| **Reload** | `reload_engine` after load-affecting **`llm.json`** changes or download of active id — **not** after catalog-only edits. |

## Config split rationale

| Concern | File | Git |
|---------|------|-----|
| Clone-and-go model BOM | `catalog.json` | Commit |
| Machine-specific runtime | `llm.json` | Ignore |

Team commits catalog so a cloned project knows which models to download. Each developer runs **`openpfe llm init`** to create local `llm.json` matched to their hardware.

## Discover

Discover does **not** mutate the catalog. It returns **candidates** the UI or CLI can turn into **`POST /catalog`** bodies.

| Source | Use |
|--------|-----|
| `installed` | Scan shared `manifest.json` + validate weights |
| `curated` | OpenPFE or project pinned lists |
| `search` | Provider query (HuggingFace first) |

Reject manifests without **`url`** (`non-portable`). Hand-written / invalid manifests → per-candidate **`error`**; out of scope beyond validation.

## `LlmService`

Catalog I/O, `llm.json` I/O, registry, downloads, discover, `LlmEngine` — see [specification.md](./specification.md).

## Integration

- `openpfe-server` builds `AppState` with `Arc<dyn LlmService>` (or concrete type) for `openpfe-ui`.
- Startup: load `catalog.json` + `llm.json`; `try_load` when active model installed.
- `openpfe` CLI: init + download status via IPC echo + HTTP (see [openpfe/design.md](../openpfe/design.md)).

## Related

- [openpfe-server/design.md](../openpfe-server/design.md) — `server.json`
- [openpfe-ui/design.md](../openpfe-ui/design.md) — orchestration
- [openpfe/design.md](../openpfe/design.md) — `llm init`
