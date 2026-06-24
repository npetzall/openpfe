# Web design (embedded UI)

**Read when:** changing HTML, CSS, or visual/interaction design of the browser UI in `openpfe-webui` (`index.html`, `src/`).

**Normative refs:** [openpfe-webui/assets/README.md](../crates/openpfe-webui/assets/README.md), [openpfe-ui/specification.md](../crates/openpfe-ui/specification.md), [openpfe_tooling.md](../../openpfe_tooling.md).

## Product views

Normative index and per-view behaviour: [openpfe-webui/assets/README.md](../crates/openpfe-webui/assets/README.md).

| View | Purpose |
|------|---------|
| **Dashboard** | Summarise problems, specifications, tasks (open vs handled/completed) |
| **Problem** | Unhandled problems, graph, embedded LLM, MCP handoff |
| **Architecture** | Problem domains, dependencies, contracts; create specification per domain |
| **Specification** | Unhandled specs; approve → creates tasks |
| **Task** | Unhandled tasks; complete |
| **Configuration** | `catalog.json`, discover/add, download, active model, local `llm.json` settings; init via CLI |
| **Debug** | Local LLM and MCP protocol diagnostics |

Reachable without full page reload (JS view switching); deep links optional until specified.

## Asset and markup

- Semantic HTML5 (`main`, `nav`, `section`, headings in order).
- Styles in `src/css/` (built to `assets/css/`) — avoid large `<style>` blocks in HTML.
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
- [openpfe-webui/assets/README.md](../crates/openpfe-webui/assets/README.md) — product views
- [openpfe-webui/specification.md](../crates/openpfe-webui/specification.md) — static routes
