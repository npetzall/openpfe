# openpfe-webui — design

**Embedded browser UI only** — HTML/CSS/JS in binary. Calls **`openpfe-ui`** over HTTP (`/api/v1/…`), not IPC.

Renamed from `openpfe-embed`. Workspace role: [workspace-crates.md](../../workspace-crates.md).

## Source tree

```
crates/openpfe-webui/
  assets/              # embed root — production only
    index.html
    css/
    js/
      app.js
      api.js
      graph/
        …
  tests/               # sibling of assets — never embedded
    js/
      api.test.js      # mirrors assets/js/ layout
      graph/
        …
      __fixtures__/
```

## Build (v1)

- **No bundler** — embed and serve files from **`assets/`** only, as native ES modules.
- **Tests** in **`tests/`** (next to `assets/`); **Vitest** + **jsdom** — [testing-javascript.md](../../guidelines/testing-javascript.md).
- **Embed root** is `assets/` — the `tests/` folder is outside it, so test files are not served and need no exclude globs.

## Embedded static serving (v1)

| Topic | Decision |
|-------|----------|
| **Embed crate** | **[`rust-embed`](https://docs.rs/rust-embed)** — `#[derive(RustEmbed)]` on `assets/` + thin **axum** handler in this crate. |
| **Rejected v1** | `tower-http` `ServeDir` (disk only); **static-serve** / **tower-embed** deferred (spike if rust-embed MIME/caching is insufficient). |
| **Export** | `pub fn router() -> Router` (or `static_router()`) for `openpfe-server` to nest at `/` **after** `/api/v1`. |
| **Routes** | `GET /` → `index.html`; `GET /assets/*` → embedded files — [specification.md](./specification.md). |
| **MIME** | Map `.js` → `text/javascript; charset=utf-8`, `.css` → `text/css`, `.html` → `text/html`, `.svg` → `image/svg+xml`; default `application/octet-stream`. |
| **Caching (v1)** | `Cache-Control: no-cache` on all static responses (localhost dev simplicity). |
| **Compression** | **Deferred** — no precompressed assets in v1 ([architcture.md](../../architcture.md)). |
| **404** | Plain text `not found` for unknown asset paths. |

## JavaScript tests (v1)

| Topic | Decision |
|-------|----------|
| **Runner** | **[Vitest](https://vitest.dev/)** + **jsdom** environment |
| **Rejected v1** | `node --test` only (less ergonomic ESM alias setup) |
| **Layout** | `tests/js/**` mirrors `assets/js/**`; fixtures in `tests/js/**/__fixtures__/` |
| **Imports** | Vitest `resolve.alias`: `@assets` → `assets/js` (or relative `../../assets/js/…`) — pick one in `vitest.config.js` at crate root |
| **HTTP mock** | Mock **`fetch`** in `api.js` tests (or MSW if needed); no Rust server in unit tests |
| **E2E** | **Optional** — Playwright smoke against local server; not required in default PR CI |

`package.json` lives at `crates/openpfe-webui/` when implementation starts.

## Out of scope

- REST handlers → **`openpfe-ui`**
- Domain logic, graph, config merge

## Related

- [openpfe-ui/design.md](../openpfe-ui/design.md) — API the JS calls
- [specification.md](./specification.md) — route table for static paths
