# Development guidelines

How we implement **openpfe** consistently across the Rust workspace and embedded Web UI. These guidelines are **operational** — normative product contracts remain in [crates/](./crates/) per [workspace-crates.md#documentation-convention](./workspace-crates.md#documentation-convention).

**Agents:** Start here for strict rules and the path-indexed guideline list below.

## Strict rules (non-negotiable)

1. **Normative contracts live in crate docs.** APIs, envelopes, paths, and FRs are authoritative in `.dev/crates/<crate-name>/` (`design.md`, `requirements.md`, `specification.md`). Do not implement behavior that contradicts those files; if something is missing, extend the crate doc first or flag it explicitly.

2. **Respect crate boundaries and dependency rules.** Follow [workspace-crates.md#dependency-rules-normative](./workspace-crates.md#dependency-rules-normative). Examples: `openpfe-ui` must not depend on `openpfe-mcp`; `openpfe-webui` stays embed-only; `openpfe-server` wires listeners and mounts routes — it does not own domain handlers.

3. **One transport per audience (v1).** Humans (browser, TUI): **HTTP** `/api/v1/…` via `openpfe-ui` only. Agents (IDE): **IPC** `type: mcp` via `openpfe-mcp` only. CLI control: **IPC** `echo` / `shutdown`. Do not duplicate graph/config APIs on IPC for TUI or browser.

4. **Project scope is cwd.** **cwd = project root**; JSON config (`server.json`, `llm.json`), graph, and server runtime under `./.openpfe/`. Model **weights** under **`USER_HOME/.openpfe/models/`** (shared). **JSON** for config and API — no TOML. One server per project. HTTP base URL from IPC echo only.

5. **Minimize scope; match existing docs and code.** Smallest correct change; no drive-by refactors. When adding a feature, update the owning crate’s `.dev/crates/<name>/` docs in the same change when behavior is normative.

6. **Dependency hygiene.** New or upgraded **external** crates are recorded under [`.dev/dependencies/<crate-name>/`](./dependencies/README.md) **before** any `Cargo.toml` edit; after adding to the manifest, run **`cargo audit`** immediately (see [security-rust.md](./guidelines/security-rust.md)).

## Layout

```
.dev/guidelines/
  coding-rust.md
  coding-javascript.md
  webdesign.md
  protocols.md
  mcp.md
  testing-rust.md
  testing-javascript.md
  security-rust.md
```

## Guidelines index

Read a file when your task touches its **scope**. Paths are repo-relative from the repository root.

| Path | Read when | Scope |
|------|-----------|--------|
| [guidelines/coding-rust.md](./guidelines/coding-rust.md) | Editing or adding Rust workspace crates, CLI, server, IPC, graph, LLM | Style, async/sync boundaries, errors, crate layout |
| [guidelines/coding-javascript.md](./guidelines/coding-javascript.md) | Editing embedded Web UI JS under `openpfe-webui` | Modules, API client, no backend logic in static assets |
| [guidelines/webdesign.md](./guidelines/webdesign.md) | HTML/CSS/UI for the embedded browser UI | Layout, a11y, same-origin API usage, asset layout |
| [guidelines/protocols.md](./guidelines/protocols.md) | IPC framing, HTTP API shape, versioning, client/server messages | Envelopes, transports, echo/shutdown — not MCP tool semantics |
| [guidelines/mcp.md](./guidelines/mcp.md) | MCP tools/resources, `openpfe mcp`, agent context | Model Context Protocol over IPC; alignment with PFE graph |
| [guidelines/testing-rust.md](./guidelines/testing-rust.md) | Rust unit/integration tests, CI for crates | Test layout, async tests, fixtures under `./.openpfe/` |
| [guidelines/testing-javascript.md](./guidelines/testing-javascript.md) | Front-end tests for embedded UI | JS test runner, mocking `/api/v1`, no real server required for unit tests |
| [guidelines/security-rust.md](./guidelines/security-rust.md) | New `Cargo.toml` deps, lockfile changes, supply-chain review | `.dev/dependencies/` intake, `cargo audit` right after manifest edit, JS audit when `package.json` exists |
| [dependencies/README.md](./dependencies/README.md) | Proposing or evaluating a new crates.io/git crate | `rational.md`, `scan.md`, `lock-update.md`, `verdict.md`, lock-diff script |

## When to use which guideline

| Guideline | Typical tasks |
|-----------|----------------|
| [coding-rust.md](./guidelines/coding-rust.md) | New crate code, axum handlers, IPC server, graph store, CLI |
| [coding-javascript.md](./guidelines/coding-javascript.md) | `assets/js/`, API client, graph UI logic in the browser |
| [webdesign.md](./guidelines/webdesign.md) | `assets/` HTML/CSS, drill-down / architecture views |
| [protocols.md](./guidelines/protocols.md) | IPC envelopes, HTTP `/api/v1`, versioning, client discovery |
| [mcp.md](./guidelines/mcp.md) | MCP tools/resources, stdio bridge, agent context shield |
| [testing-rust.md](./guidelines/testing-rust.md) | `cargo test`, integration tests with temp project dirs |
| [testing-javascript.md](./guidelines/testing-javascript.md) | UI unit tests, fetch mocks, component behavior |
| [security-rust.md](./guidelines/security-rust.md) | Adding/upgrading crates.io or git deps; auditing `Cargo.lock` |
| [dependencies/README.md](./dependencies/README.md) | Recording and evaluating a candidate external crate before merge |

## System context (when rules above are not enough)

| Path | Read when |
|------|-----------|
| [architcture.md](./architcture.md) | Stack choices, startup flows, tower-http/axum/tokio |
| [cross-cutting.md](./cross-cutting.md) | Multi-crate contracts, data paths, FR traceability |
| [workspace-crates.md](./workspace-crates.md) | Members, phasing, documentation convention |

## Relationship to other `.dev` docs

| Layer | Location |
|-------|----------|
| **Guidelines (this file + folder)** | `.dev/guidelines.md`, `.dev/guidelines/*.md` |
| **Per-crate normative spec** | `.dev/crates/<name>/specification.md` |
| **System architecture** | [architcture.md](./architcture.md), [cross-cutting.md](./cross-cutting.md) |
| **Implementation plans** | [plans/README.md](./plans/README.md) |

If a guideline and a crate `specification.md` disagree, **the crate specification wins** — update the guideline or the spec in the same PR.

## Contributing guidelines

- Keep each file focused; link to crate specs instead of copying normative tables.
- Add a **“Read when”** line at the top of new guideline files for agents.
- Prefer repo-relative paths in links (as in the index above).
