# openpfe-webui — specification

**Product UI (views, entity states, workflows):** [assets/README.md](./assets/README.md). This file is **static routes and build** only.

## Static HTTP routes

| Route | Content |
|-------|---------|
| **GET /** | `index.html` |
| **GET /assets/…** | CSS, JS (built bundles or chunks from Vite) |

Mounted by `openpfe-server` via **`openpfe-webui::router()`** (`rust-embed` + axum) — [design.md](./design.md#embedded-static-serving-v1).

## Front-end structure (normative layout)

**Authoring** (not embedded):

- **`index.html`** (crate root) — Vite entry; `type=module` script points at `src/js/…`
- **`src/css/`** — layered styles (base, layout, components)
- **`src/js/`** — vanilla ES modules: `api.js`, `graph/`, `llm/` (Configuration view → `/llm/*`, `/models/*`)

**Generated embed trees** (Vite output — not committed in v1):

| Directory | When produced | Embedded when |
|-----------|---------------|---------------|
| **`assets-dev/`** | `PROFILE` ≠ `release` (debug, test, bench) | `cfg(debug_assertions)` |
| **`assets/`** | `PROFILE` = `release` | `not(debug_assertions)` |

Each tree contains `index.html`, `css/`, `js/` — paths and filenames must satisfy the route table above after build.

## Cargo / npm build

**Implement in `crates/openpfe-webui/build.rs`.** Invoked automatically when this crate is built; normative detail in [design.md](./design.md#cargo-integration-v1).

1. If **`OPENPFE_SKIP_WEBUI_BUILD`** is set to a non-empty value **or** `npm` is not found on `PATH`: print **`cargo:warning`**, do not run npm, exit `build.rs` successfully.
2. Else, from `crates/openpfe-webui/`:
   - Run **`npm ci`** if `package-lock.json` exists, else **`npm install`**.
   - Set **`OPENPFE_CARGO_PROFILE`** to Cargo’s **`PROFILE`** env (`debug`, `release`, `test`, `bench`).
   - Run **`npm run build`** (must invoke Vite with mode/outDir per profile — see design).
3. On npm/Vite failure: **`build.rs` exits non-zero** (fail the Cargo build).
4. Declare **`cargo:rerun-if-changed`** for `package.json`, `package-lock.json`, `index.html`, `vite.config.js`, and the `src/` tree; **`cargo:rerun-if-env-changed=PROFILE`** and **`OPENPFE_SKIP_WEBUI_BUILD`**.

**`package.json` scripts (normative when present):**

- **`build`** — runs `vite build`; reads **`OPENPFE_CARGO_PROFILE`**: `release` → `--mode production`, outDir **`assets/`**; otherwise → `--mode development`, outDir **`assets-dev/`**.

**Environment:**

| Variable | Set by | Purpose |
|----------|--------|---------|
| `PROFILE` | Cargo | Debug vs release (and test/bench); forwarded as `OPENPFE_CARGO_PROFILE` for Vite |
| `OPENPFE_SKIP_WEBUI_BUILD` | Developer / CI | Skip npm/Vite; use existing generated trees if any |

**CI:** `cargo build --workspace` (or `cargo test`) is sufficient for the embed step; run **`npm test`** separately for Vitest. Do not require a duplicate standalone `npm run build` unless skipping via `OPENPFE_SKIP_WEBUI_BUILD`.

**`tests/`** (sibling of `src/`, not embedded):

- `js/**/*.test.js` — unit tests mirroring **`src/js/`** layout
- `js/**/__fixtures__/**` — test JSON

**Toolchain (v1):** **[Vite](https://vite.dev/)** for dev/build; **[Vitest](https://vitest.dev/)** + **jsdom** for unit tests — [design.md](./design.md#front-end-toolchain-v1). **Vanilla JavaScript** only — no UI framework.

API calls: `/api/v1/…` — [openpfe-ui/specification.md](../openpfe-ui/specification.md) (to be extended using [assets/README.md](./assets/README.md) and per-view specs).
