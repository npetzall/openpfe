# openpfe-llm — requirements

**Required for v1** — not deferred.

## FR-8 Models (inference)

- **FR-8.4** Local inference via **llama.cpp** (`llama-cpp-2`) for configured model(s).
- **FR-8.7** Server **may start without** a loaded model; graph and HTTP non-LLM routes remain available.
- **FR-8.8** **Single-flight** inference in v1 — reject or 503 concurrent second request.

## Non-functional

- Inference runs on **`spawn_blocking`** — must not block tokio accept loops ([openpfe-server/design.md](../openpfe-server/design.md)).

## Related

- [openpfe-core/requirements.md](../openpfe-core/requirements.md) — FR-8.1–8.3, FR-8.5–8.6
- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md) — HTTP UX
- [specification.md](./specification.md)
