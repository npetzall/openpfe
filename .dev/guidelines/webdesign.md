# Web design (embedded UI)

**Read when:** changing HTML, CSS, or visual/interaction design of the browser UI in `openpfe-webui` assets.

**Normative refs:** [openpfe-webui/design.md](../crates/openpfe-webui/design.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md), [openpfe_tooling.md](../../openpfe_tooling.md) (product views).

## Product views (v1 direction)

Align UI structure with tooling goals:

| View | Purpose |
|------|---------|
| **Drill-down** | Expand problem nodes (fishbone / tree) |
| **Architecture** | Clusters, component boundaries |
| **Contract editor** | Consumer-driven contracts tied to graph nodes |

Each view should be reachable without full page reload where possible (JS view switching); deep links are optional until specified.

## Asset and markup

- Semantic HTML5 (`main`, `nav`, `section`, headings in order).
- Styles in `assets/css/` — avoid large `<style>` blocks in HTML.
- Class naming: consistent prefix (e.g. `opfe-`) to avoid clashes with future embeddable widgets.
- Responsive layout for desktop-first localhost use; readable minimum width ~1024px unless TUI covers narrow cases.

## Visual design

- Calm, documentation-like UI — graph and text are primary content; chrome stays minimal.
- Color: sufficient contrast (WCAG AA target for text); do not rely on color alone for state (use icons/labels).
- Typography: system font stack for v1; one monospace stack for contracts/code snippets.

## Interaction

- Loading and error states for every API-backed panel (skeleton or message — no silent failure).
- Destructive actions (delete node, overwrite contract) require explicit confirmation.
- Keyboard: focusable controls, visible focus ring, logical tab order in modals and forms.

## API integration

- All dynamic data from **`/api/v1/…`** — see [coding-javascript.md](./coding-javascript.md).
- Same-origin default; CORS is server concern ([architcture.md](../architcture.md)), not something the embed fixes in JS.

## Performance (localhost)

- Prefer few HTTP requests for v1 (modular JS is fine; avoid dozens of tiny assets).
- Large graph renders: virtualize or paginate in JS when node count grows — spike before shipping huge canvases.

## Out of scope

- TUI rendering (future crate/client — HTTP + optional IPC only per architecture).
- Branding/marketing pages — this embed is the working canvas only.

## Related

- [coding-javascript.md](./coding-javascript.md)
- [openpfe-webui/specification.md](../crates/openpfe-webui/specification.md) — static routes
