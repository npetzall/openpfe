/# Testing — JavaScript

**Read when:** adding automated tests for `openpfe-webui` assets or JS that calls `/api/v1`.

**Normative refs:** [openpfe-webui/design.md](../crates/openpfe-webui/design.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md).

## Goals

- Test **UI logic and API client** without a running Rust server for unit tests.
- Reserve a small set of **browser or integration** tests (if any) for critical flows — optional; e2e not in default PR gate.

## v1 decisions

- **No bundler** — production code under `assets/`; the Node test runner imports those modules from the sibling `tests/` tree.
- **Tests beside `assets/`** — all JS tests under `crates/openpfe-webui/tests/` (not inside `assets/`), mirroring `assets/js/` layout.

## Crate layout

```
crates/openpfe-webui/
  assets/          # embedded and served — production only
    js/
      api.js
      graph/
        view.js
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
| Unit | **Vitest** + **jsdom** | `api.js`, graph view state, parsers/formatters |
| API mock | `fetch` mock / MSW | Returns fixture JSON for `/api/v1/...` |
| E2E (optional) | Playwright against local server | Smoke: load embed, one API round-trip |

Normative detail: [openpfe-webui/design.md](../crates/openpfe-webui/design.md#javascript-tests-v1).

## Embed

Embed **only** `assets/`. The `tests/` directory is outside the embed root — no exclude globs needed for test files.

## Conventions

- **Naming:** `tests/js/<path>/<module>.test.js` mirrors `assets/js/<path>/<module>.js`.
- **Fixtures:** `tests/js/__fixtures__/` or `tests/js/<path>/__fixtures__/` — small JSON only.
- **Imports:** via Vitest `resolve.alias` (`@assets` → `assets/js`) or relative `../../assets/js/…` — [openpfe-webui/design.md](../crates/openpfe-webui/design.md#javascript-tests-v1).

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

- JS test job independent of `cargo test` but run in same pipeline when repo has both.
- Fast unit suite only in default PR gate; e2e marked optional or nightly if slow.

## Related

- [coding-javascript.md](./coding-javascript.md)
- [webdesign.md](./webdesign.md)
- [testing-rust.md](./testing-rust.md) — server/API contracts to mock
