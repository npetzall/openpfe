# Coding — JavaScript

**Read when:** editing production files under `crates/openpfe-webui/assets/` (HTML/CSS/JS). For tests, see [testing-javascript.md](./testing-javascript.md) (`crates/openpfe-webui/tests/`).

**Normative refs:** [openpfe-webui/design.md](../crates/openpfe-webui/design.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md).

## Role of the Web UI code

- **Browser-only** presentation and interaction; all persistence and graph rules live on the server (`openpfe-ui` + `openpfe-graph`).
- Talk to the backend **only** via HTTP `fetch` (or equivalent) to **`/api/v1/…`** on the same origin — no IPC. **v1:** graph updates via **polling**, not WebSockets ([openpfe-ui/specification.md](../crates/openpfe-ui/specification.md)).

## Module layout

Follow the asset tree in [openpfe-webui/design.md](../crates/openpfe-webui/design.md):

```
crates/openpfe-webui/
  assets/           # production — embedded and served
    index.html
    css/
    js/
      app.js
      api.js
      graph/
        view.js
  tests/            # unit tests — sibling of assets; see testing-javascript.md
    js/
      api.test.js
      graph/
        view.test.js
```

- **`api.js`**: single place for base path, JSON headers, error parsing, and request IDs if needed.
- **View modules**: drill-down, architecture, contract editor — separate files under `js/` or `js/graph/`.
- No inline business logic in `index.html` beyond bootstrapping one script entry.

## Language and style

- **Vanilla JavaScript** for v1 — browser APIs and DOM only; no UI framework (React, Vue, Svelte, etc.) unless [openpfe-webui/design.md](../crates/openpfe-webui/design.md) records an explicit choice.
- **ES modules** (`import`/`export`); **no bundler in v1** — ship native modules from embed; see [openpfe-webui/specification.md](../crates/openpfe-webui/specification.md).
- Prefer `const`/`let`; no `var`.
- `async`/`await` for API calls; handle non-OK HTTP status and network errors in `api.js`.
- **Tests** live in `tests/` beside `assets/`, not inside `assets/` — [testing-javascript.md](./testing-javascript.md).

## Configuration

- Do not hard-code host/port; same-origin relative URLs (`/api/v1/...`) are default.
- If absolute base URL is ever needed (tests, dev proxy), inject via a single config object set at startup — not scattered literals.

## Security

- Treat all graph/API data as untrusted for XSS: use `textContent` / safe DOM APIs; avoid `innerHTML` with server-provided strings unless sanitized.
- No secrets in front-end code; localhost-only product, but still no API keys in `assets/`.

## Out of scope for JS

- MCP, stdio, IPC envelopes — agents use `openpfe mcp`, not the Web UI.
- Config merge rules, graph storage format — server-side only.

## Related

- [webdesign.md](./webdesign.md) — HTML/CSS and UX
- [protocols.md](./protocols.md) — HTTP API conventions
- [testing-javascript.md](./testing-javascript.md) — front-end tests
