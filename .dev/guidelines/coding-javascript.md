# Coding — JavaScript

**Read when:** editing front-end sources under `crates/openpfe-webui/src/` (HTML entry, CSS, JS). For tests, see [testing-javascript.md](./testing-javascript.md) (`crates/openpfe-webui/tests/`). Built output under `assets/` is generated — do not hand-edit except when debugging a build.

**Normative refs:** [openpfe-webui/design.md](../crates/openpfe-webui/design.md) (toolchain), [openpfe-webui/assets/README.md](../crates/openpfe-webui/assets/README.md) (product views), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md).

## Role of the Web UI code

- **Browser-only** presentation and interaction; all persistence and graph rules live on the server (`openpfe-ui` + `openpfe-graph`).
- Talk to the backend **only** via HTTP `fetch` (or equivalent) to **`/api/v1/…`** on the same origin — no IPC. **v1:** graph updates via **polling**, not WebSockets ([openpfe-ui/specification.md](../crates/openpfe-ui/specification.md)).

## Module layout

Follow the tree in [openpfe-webui/design.md](../crates/openpfe-webui/design.md):

```
crates/openpfe-webui/
  index.html        # Vite entry
  src/              # authoring — vanilla JS/CSS
    css/
    js/
      app.js
      api.js
      graph/
        view.js
  assets/           # vite build output — embedded; do not edit by hand
  tests/            # unit tests — sibling of src; see testing-javascript.md
    js/
      api.test.js
      graph/
        view.test.js
```

- **`api.js`**: single place for base path, JSON headers, error parsing, and request IDs if needed.
- **View modules**: drill-down, architecture, contract editor — separate files under `js/` or `js/graph/`.
- No inline business logic in `index.html` beyond bootstrapping one script entry (`src/js/app.js` or equivalent).

## Language and style

- **Vanilla JavaScript** for v1 — browser APIs and DOM only; no UI framework (React, Vue, Svelte, etc.) unless [openpfe-webui/design.md](../crates/openpfe-webui/design.md) records an explicit choice.
- **ES modules** (`import`/`export`) in **`src/js/`**; **[Vite](https://vite.dev/)** builds into **`assets/`** for embed — [openpfe-webui/specification.md](../crates/openpfe-webui/specification.md).
- Prefer `const`/`let`; no `var`.
- `async`/`await` for API calls; handle non-OK HTTP status and network errors in `api.js`.
- **Tests** live in `tests/` beside `src/`, not inside `src/` or `assets/` — [testing-javascript.md](./testing-javascript.md).

## Local development

- **`npm run dev`** — Vite dev server with HMR; proxy `/api/v1` to the local Rust server when needed (configure in `vite.config.js`).
- **`cargo build`** / **`cargo build --release`** — runs **`openpfe-webui/build.rs`**, which invokes **`npm run build`** and writes **`assets-dev/`** or **`assets/`** per Cargo profile — [openpfe-webui/specification.md](../crates/openpfe-webui/specification.md#cargo--npm-build). Set **`OPENPFE_SKIP_WEBUI_BUILD=1`** to skip when `npm` is not installed.

## Configuration

- Do not hard-code host/port; same-origin relative URLs (`/api/v1/...`) are default.
- If absolute base URL is ever needed (tests, dev proxy), inject via a single config object set at startup — not scattered literals.

## Security

- Treat all graph/API data as untrusted for XSS: use `textContent` / safe DOM APIs; avoid `innerHTML` with server-provided strings unless sanitized.
- No secrets in front-end code; localhost-only product, but still no API keys in `src/` or built `assets/`.

## Out of scope for JS

- MCP, stdio, IPC envelopes — agents use `openpfe mcp`, not the Web UI.
- `server.json` / `llm.json` formats, graph storage — server-side only (`openpfe-server`, `openpfe-llm`).

## Related

- [webdesign.md](./webdesign.md) — HTML/CSS and UX
- [protocols.md](./protocols.md) — HTTP API conventions
- [testing-javascript.md](./testing-javascript.md) — front-end tests
