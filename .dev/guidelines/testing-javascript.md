# Testing — JavaScript

**Read when:** adding automated tests for `openpfe-webui` sources or JS that calls `/api/v1`.

**Normative refs:** [openpfe-webui/design.md](../crates/openpfe-webui/design.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md).

## Goals

- Test **UI logic and API client** without a running Rust server for unit tests.
- Reserve a small set of **browser or integration** tests (if any) for critical flows — optional; e2e not in default PR gate.

## v1 decisions

- **[Vitest](https://vitest.dev/)** + **jsdom** — shares **`vite.config.js`** resolve/alias with the app; no separate test bundler.
- **Import authoring sources** from **`src/js/`** (via `@` alias or relative paths), not built files under **`assets/`**.
- **Tests beside `src/`** — all JS tests under `crates/openpfe-webui/tests/` (not inside `src/` or `assets/`), mirroring **`src/js/`** layout.
- **Production build** — **[Vite](https://vite.dev/)** `npm run build` → **`assets/`**; unit tests do not require a successful build unless testing integration with built artifacts (unusual in v1).

## Crate layout

```
crates/openpfe-webui/
  src/             # authoring — Vitest imports from here
    js/
      api.js
      graph/
        view.js
  assets/          # vite build output — not used by default unit tests
  tests/           # never embedded
    js/
      api.test.js
      graph/
        view.test.js
      __fixtures__/   # JSON fixtures for unit tests
```

## Approach (v1)

| Layer | Tooling | Scope |
|-------|---------|--------|
| Unit | **Vitest** + **jsdom** (via Vite config) | `api.js`, graph view state, parsers/formatters |
| API mock | `fetch` mock / MSW | Returns fixture JSON for `/api/v1/...` |
| E2E (optional) | Playwright against local server | Smoke: load embed, one API round-trip |

Normative detail: [openpfe-webui/design.md](../crates/openpfe-webui/design.md#javascript-tests-v1).

## Embed

Embed **only** **`assets/`** (Vite build output). **`src/`** and **`tests/`** are outside the embed root — no exclude globs needed for source or test files.

## Conventions

- **Naming:** `tests/js/<path>/<module>.test.js` mirrors **`src/js/<path>/<module>.js`**.
- **Fixtures:** `tests/js/__fixtures__/` or `tests/js/<path>/__fixtures__/` — small JSON only.
- **Imports:** Vitest `resolve.alias` in **`vite.config.js`** (e.g. `@` → `src`) — [openpfe-webui/design.md](../crates/openpfe-webui/design.md#javascript-tests-v1). Prefer alias over long relative paths into `src/`.

## Mocking HTTP

- Mock at **`fetch`** boundary in `api.js` tests so view modules stay free of transport stubs.
- Use relative URLs in tests (`/api/v1/graph`) with a global mock base or MSW handler.
- Test error paths: 4xx/5xx, malformed JSON, network failure.

## What not to test in JS

- IPC, MCP, llama inference, on-disk graph format — Rust integration tests own these.
- Embedded static MIME/compression — Rust embed crate tests or manual smoke unless shared harness exists.

## DOM and a11y

- Assert visible text and ARIA roles for critical panels (loading/error/confirm dialogs).
- Prefer `userEvent` / simulated clicks over brittle CSS-selector-only checks.

## CI

- JS job: `npm ci`, `npm test` (Vitest); **`cargo build`** / **`cargo test`** already run the web UI npm build via **`openpfe-webui/build.rs`** unless **`OPENPFE_SKIP_WEBUI_BUILD=1`**.
- JS test job independent of `cargo test` but in the same pipeline when the repo has both.
- Fast unit suite only in default PR gate; e2e marked optional or nightly if slow.

## Related

- [coding-javascript.md](./coding-javascript.md)
- [webdesign.md](./webdesign.md)
- [testing-rust.md](./testing-rust.md) — server/API contracts to mock
