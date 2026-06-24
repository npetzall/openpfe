# Configuration view — requirements

## FR-UI-8 Configuration view

- **FR-UI-8.1** Show **`llm.json`** runtime settings and active catalog **`id`**.
- **FR-UI-8.2** List catalog with **installed** state (`GET /models`).
- **FR-UI-8.3** Download + progress polling (`POST /catalog/:id/download`, `GET /downloads/:job_id`).
- **FR-UI-8.4** **Select active model** (`PUT /llm/active` → updates `llm.json`; reload engine).
- **FR-UI-8.5** Edit runtime fields via `GET`/`PUT /llm/config` (`n_ctx`, `n_threads`, …).
- **FR-UI-8.6** Show **`GET /llm/status`**.
- **FR-UI-8.7** No graph/spec/task mutation.
- **FR-UI-8.8** Clear path when model not installed.
- **FR-UI-8.9** **Discover + add catalog** — `GET /catalog/discover`; add via **`POST /catalog`** (one entry per request); edit via **`PATCH /catalog/:id`**. **No bulk import.**
- **FR-UI-8.10** When **`llm.json`** missing, prompt **`openpfe llm init`** — **no HTTP/Web UI init.**

API: [openpfe-ui/specification.md](../../../openpfe-ui/specification.md). File format: [openpfe-llm/specification.md](../../../openpfe-llm/specification.md).
