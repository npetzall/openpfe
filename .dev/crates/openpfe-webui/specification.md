# openpfe-webui — specification

## Static HTTP routes

| Route | Content |
|-------|---------|
| **GET /** | `index.html` |
| **GET /assets/…** | CSS, JS modules |

Mounted by `openpfe-server` via **`openpfe-webui::router()`** (`rust-embed` + axum) — [design.md](./design.md#embedded-static-serving-v1).

## Front-end structure (normative layout)

**`assets/`** (embedded and served):

- `index.html` — shell, `type=module` scripts
- `css/` — layered styles (base, layout, components)
- `js/` — ES modules: `api.js`, `graph/`, `config/`, `models/`

**`tests/`** (sibling of `assets/`, not embedded):

- `js/**/*.test.js` — unit tests mirroring `assets/js/` layout
- `js/**/__fixtures__/**` — test JSON

**No bundler** in v1; native ES modules served from `assets/` embed only.

API calls: `/api/v1/…` — [openpfe-ui/specification.md](../openpfe-ui/specification.md).
