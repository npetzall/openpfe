# Configuration view — specification

**API:** `GET`/`PUT /llm/config`, `/models`, `/llm/active`, `/llm/status` — [openpfe-ui/specification.md](../../../openpfe-ui/specification.md). **`llm.json`:** [openpfe-llm/specification.md](../../../openpfe-llm/specification.md).

## Layout

1. Engine status (`GET /llm/status`)
2. Active model (`PUT /llm/active`)
3. Catalog table (`GET /models`, download, poll)
4. LLM settings (`GET`/`PUT /llm/config` — `llm` object fields)

## Active model

| Step | API |
|------|-----|
| Load | `GET /llm/config` → `llm.model` |
| Change | `PUT /llm/active` |

Updates **`./.openpfe/llm.json`**. Weights in **`~/.openpfe/models/<id>/`**.

## Catalog / download

`POST /models/:id/download` → poll job → refresh list.

## LLM settings

Edit `n_ctx`, `n_threads` in full document via `PUT /llm/config`.

Implements FR-UI-8 in [requirements.md](./requirements.md).
