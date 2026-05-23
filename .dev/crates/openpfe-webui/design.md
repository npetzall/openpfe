# openpfe-webui — design

**Embedded browser UI only** — HTML/CSS/JS in binary. Calls **`openpfe-ui`** over HTTP (`/api/v1/…`), not IPC.

Renamed from `openpfe-embed`. Workspace role: [workspace-crates.md](../../workspace-crates.md).

**Product UI (views, workflows, PFE canvas):** [assets/README.md](./assets/README.md) — one folder per view under `assets/`. This file covers **toolchain and embed** only.

## Source tree

```
crates/openpfe-webui/
  index.html           # Vite HTML entry (dev + build input)
  src/                 # authoring — vanilla JS/CSS (not embedded directly)
    css/
    js/
      app.js
      api.js
      graph/
        …
  assets/              # `vite build` output (release profile) — rust-embed
  assets-dev/          # `vite build` output (debug / test / bench) — rust-embed
    # each: index.html, css/, js/, …
  tests/               # sibling of src — never embedded
    js/
      api.test.js      # mirrors src/js/ layout
      graph/
        …
      __fixtures__/
  vite.config.js
  package.json
  build.rs             # runs npm/Vite during `cargo build`
```

## Build (v1)

- **[Vite](https://vite.dev/)** — dev server (`npm run dev`) and production build (`npm run build`) for **vanilla** ES modules (no UI framework).
- **Authoring** under **`src/`** + root **`index.html`**; embed trees under **`assets/`** / **`assets-dev/`** are **generated** (see [Cargo integration](#cargo-integration-v1)).
- **Tests** in **`tests/`** (next to `src/`); **[Vitest](https://vitest.dev/)** + **jsdom** — shares Vite resolve/alias via `vite.config.js` — [testing-javascript.md](../../guidelines/testing-javascript.md).
- **`src/`** and **`tests/`** are never embedded; no exclude globs on the embed roots.

## Cargo integration (v1)

**Normative:** [specification.md](./specification.md#cargo--npm-build). **`openpfe-webui/build.rs`** runs the front-end build **before** `rustc` compiles this crate so **`rust-embed`** sees up-to-date bytes.

| Topic | Decision |
|-------|----------|
| **Trigger** | Every `cargo build` / `cargo test` that compiles `openpfe-webui` (via `build.rs`) |
| **Command** | `npm ci` when `package-lock.json` exists, else `npm install`; then `npm run build` with env **`OPENPFE_CARGO_PROFILE`** set from Cargo’s **`PROFILE`** (`debug`, `release`, `test`, `bench`) |
| **Profile mapping** | **`release`** → Vite **`production`** mode → output **`assets/`**; all other profiles → Vite **`development`** mode → output **`assets-dev/`** (source maps, no minify in v1) |
| **`rust-embed`** | **`#[folder = "assets-dev"]`** when `cfg(debug_assertions)`; **`#[folder = "assets"]`** otherwise — matches Cargo debug vs release binaries |
| **Rerun** | `cargo:rerun-if-changed` on `package.json`, `package-lock.json`, `index.html`, `vite.config.js`, and `src/`; `cargo:rerun-if-env-changed=PROFILE`, `OPENPFE_SKIP_WEBUI_BUILD` |
| **Skip (optional)** | If **`OPENPFE_SKIP_WEBUI_BUILD=1`** *or* **`npm` is not on `PATH`**: skip npm/Vite, emit **`cargo:warning`**, use existing embed dirs if present |
| **Skip + missing output** | Warn; crate may still compile with empty/placeholder embed (scaffold only) — full UI requires a successful npm build or committed generated trees |
| **Committed output** | **`assets/`** and **`assets-dev/`** are **not** committed in v1 (gitignore); produced locally and in CI by `cargo build` |
| **Rejected v1** | Manual-only “run `npm run build` before `cargo build`” as the primary workflow; separate npm step in CI when `cargo build` already builds the crate |

**Local workflow:** `cargo build` / `cargo build --release` is enough; no separate npm step unless iterating on JS without recompiling Rust (`npm run dev` / `npm test` still useful).

## Embedded static serving (v1)

| Topic | Decision |
|-------|----------|
| **Embed crate** | **[`rust-embed`](https://docs.rs/rust-embed)** — `#[derive(RustEmbed)]` on **`assets-dev/`** (debug) or **`assets/`** (release) per [Cargo integration](#cargo-integration-v1) + thin **axum** handler in this crate. |
| **Rejected v1** | `tower-http` `ServeDir` (disk only); **static-serve** / **tower-embed** deferred (spike if rust-embed MIME/caching is insufficient). |
| **Export** | `pub fn router() -> Router` (or `static_router()`) for `openpfe-server` to nest at `/` **after** `/api/v1`. |
| **Routes** | `GET /` → `index.html`; `GET /assets/*` → embedded files — [specification.md](./specification.md). |
| **MIME** | Map `.js` → `text/javascript; charset=utf-8`, `.css` → `text/css`, `.html` → `text/html`, `.svg` → `image/svg+xml`; default `application/octet-stream`. |
| **Caching (v1)** | `Cache-Control: no-cache` on all static responses (localhost dev simplicity). |
| **Compression** | **Deferred** — no precompressed assets in v1 ([architcture.md](../../architcture.md)). |
| **404** | Plain text `not found` for unknown asset paths. |

## Front-end toolchain (v1)

| Topic | Decision |
|-------|----------|
| **UI** | **Vanilla JavaScript** — browser APIs and DOM only; no React, Vue, Svelte, etc. |
| **Bundler / dev** | **Vite** — ESM `import`/`export` in `src/js/`; `index.html` bootstraps one module entry |
| **Production output** | `vite build` → **`assets/`** or **`assets-dev/`** (`build.outDir` from `OPENPFE_CARGO_PROFILE`); paths must match [specification.md](./specification.md) (`/`, `/assets/…`) |
| **Rejected v1** | Hand-maintained embed tree with no build step; UI frameworks |

## JavaScript tests (v1)

| Topic | Decision |
|-------|----------|
| **Runner** | **[Vitest](https://vitest.dev/)** + **jsdom** — config merged with or imported from **`vite.config.js`** |
| **Rejected v1** | `node --test` only; Vitest without Vite config (lose shared `resolve.alias`) |
| **Layout** | `tests/js/**` mirrors **`src/js/**`**; fixtures in `tests/js/**/__fixtures__/` |
| **Imports** | Vitest `resolve.alias`: `@` → `src` (or `@src/js`); tests import **source** modules, not built `assets/` |
| **HTTP mock** | Mock **`fetch`** in `api.js` tests (or MSW if needed); no Rust server in unit tests |
| **E2E** | **Optional** — Playwright smoke against local server; not required in default PR CI |

`package.json` lives at `crates/openpfe-webui/` when implementation starts (`vite`, `vitest`, `jsdom` devDependencies).

## Out of scope

- REST handlers → **`openpfe-ui`**
- Domain logic, graph, JSON config parsing (`openpfe-server`, `openpfe-llm`)

## Related

- [assets/README.md](./assets/README.md) — product UI index (per-view folders)
- [openpfe-ui/design.md](../openpfe-ui/design.md) — API the JS calls (informed by assets docs)
- [specification.md](./specification.md) — route table for static paths
