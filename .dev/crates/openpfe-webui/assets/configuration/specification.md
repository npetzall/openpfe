# Configuration view — specification

**API:** catalog CRUD, `/catalog/discover`, `/llm/config`, `/models`, `/llm/active`, `/llm/status` — [openpfe-ui/specification.md](../../../openpfe-ui/specification.md). **Files:** [openpfe-llm/specification.md](../../../openpfe-llm/specification.md) (`catalog.json` committed, `llm.json` local).

## Layout

1. Engine status (`GET /llm/status`)
2. Setup nudge when `llm.json` missing — **run `openpfe llm init`** (CLI only; no in-browser init)
3. Active model (`PUT /llm/active`)
4. Catalog table (`GET /models`, discover, add, download, poll)
5. LLM settings (`GET`/`PUT /llm/config` — runtime fields only)

## Missing local config

When **`llm.json`** is absent, show clear copy: run **`openpfe llm init`** in the project root. Do **not** implement hardware probe or init over HTTP.

## Active model

| Step | API |
|------|-----|
| Load | `GET /llm/config` → `model` |
| Change | `PUT /llm/active` |

Updates **`./.openpfe/llm.json`**. Weights in **`~/.openpfe/models/<id>/`**.

## Catalog

| Step | API |
|------|-----|
| List (with install state) | `GET /models` |
| Browse candidates | `GET /catalog/discover?source=installed` (or `curated`, `search`) |
| Add one entry | `POST /catalog` — **one request per model** |
| Edit | `PATCH /catalog/:id` (`recommended`, `requirements`, …) |
| Remove | `DELETE /catalog/:id` |

Populate form from discover candidate + let user set `recommended` / `requirements` before `POST /catalog`.

## Download

`POST /catalog/:id/download` → poll `GET /downloads/:job_id` (or `status_url` from response) → refresh `GET /models`.

## LLM settings

Edit `n_ctx`, `n_threads`, `temperature`, `max_tokens` via `GET`/`PUT /llm/config` (**`LlmSettings` only** — not catalog).

Implements FR-UI-8 in [requirements.md](./requirements.md).
