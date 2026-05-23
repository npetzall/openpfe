# Configuration view — requirements

## FR-UI-8 Configuration view

- **FR-UI-8.1** Show **`llm.json`** `llm` settings and active catalog **`id`**.
- **FR-UI-8.2** List catalog with **installed** state (`GET /models`).
- **FR-UI-8.3** Download + progress polling.
- **FR-UI-8.4** **Select active model** (`PUT /llm/active` → updates `llm.json`; reload engine).
- **FR-UI-8.5** Edit **`n_ctx`**, **`n_threads`** via `GET`/`PUT /llm/config`.
- **FR-UI-8.6** Show **`GET /llm/status`**.
- **FR-UI-8.7** No graph/spec/task mutation.
- **FR-UI-8.8** Clear path when model not installed.

API: [openpfe-ui/specification.md](../../../openpfe-ui/specification.md). File format: [openpfe-llm/specification.md](../../../openpfe-llm/specification.md).
