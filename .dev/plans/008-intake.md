# Plan 008: dependency intake (LLM slice)

**Status:** Complete (2026-06-06) — intake approved; [008-openpfe-llm](./008-openpfe-llm.md) may proceed.

**Read when:** adding external crates required by plans [008-openpfe-llm](./008-openpfe-llm.md), [008-openpfe-ui](./008-openpfe-ui.md), and [008-openpfe-server](./008-openpfe-server.md). **Single intake batch** for the whole 008 series — one human pause before any 008 implementation `src/` work that needs these deps.

**Assumes:** [007-openpfe-graph.md](./007-openpfe-graph.md) **Complete**; plans [001](./001-scaffolding.md)–[005](./005-openpfe-wiring.md) **Complete**.

## Normative sources

| Doc | Use |
|-----|-----|
| [dependencies/README.md](../dependencies/README.md) | Intake folder layout and workflow |
| [guidelines/plans.md](../guidelines/plans.md) | Intake → pause → implement gate |
| [guidelines/security-rust.md](../guidelines/security-rust.md) | `cargo audit` order after manifest edit |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md#adding-an-external-crate-order) | External crate order |
| [openpfe-llm/specification.md](../crates/openpfe-llm/specification.md) | Native engine (`llama-cpp-2`) |

## Goal

Record, evaluate, and merge **all new external dependencies** for the minimal LLM path (config, download, inference, HTTP exposure) in **one intake batch**. After human approval, downstream crate plans may implement product code without further intake pauses (unless a plan discovers an additional external crate).

## Crates affected (manifest edits)

| Workspace member | New / promoted externals |
|------------------|--------------------------|
| **`openpfe-llm`** | `llama-cpp-2`, `reqwest`, `sha2`, `uuid` |
| **`openpfe-ui`** | `axum`, `tokio` (workspace — no new intake folders if already accepted) |
| **`openpfe-server`** | path deps only (`openpfe-ui`, `openpfe-llm`, `openpfe-graph`, `openpfe-mcp`); optional `tower-http` if not already workspace-level |

**Reuse without new intake:** workspace `serde`, `serde_json`, `thiserror`; `tempfile` dev-dep; `axum` / `tokio` if verdicts already **accept** for `openpfe-server`.

**Not in this batch:** `rmcp` (full MCP tools — later plan); `openpfe-webui` static assets.

## Dependency intake (phase A — complete before 008 implementation)

### `llama-cpp-2` (+ transitive `llama-cpp-sys-2`)

- [x] `.dev/dependencies/llama-cpp-2/rational.md` — need: local GGUF inference (FR-8.4); scope: **`openpfe-llm`**; note native C++/CMake build, CI compile time, `unsafe` FFI
- [x] `dependency-lock-diff.sh llama-cpp-2@0.1.146 --package openpfe-llm` → `lock-update.md`
- [x] `crates/openpfe-llm/Cargo.toml` — pin **`llama-cpp-2 = "0.1.146"`** (default features unless rational documents `cuda` etc.)
- [x] Document build prerequisites (CMake, C++ toolchain) in `rational.md`
- [x] **`cargo audit`** immediately after manifest edit → `scan.md`
- [x] `verdict.md` — accept / reject / defer

### `reqwest`

- [x] `.dev/dependencies/reqwest/rational.md` — need: HTTPS model download (FR-8.5); scope: **`openpfe-llm`**; prefer `rustls-tls`, `stream` (or `blocking` if rational chooses sync download thread)
- [x] `dependency-lock-diff.sh reqwest@0.12 --package openpfe-llm` → `lock-update.md`
- [x] `crates/openpfe-llm/Cargo.toml` — add pinned version + features per rational
- [x] **`cargo audit`** → append `scan.md`
- [x] `verdict.md`

### `sha2`

- [x] `.dev/dependencies/sha2/rational.md` — need: sha256 verify after download; scope: **`openpfe-llm`**
- [x] `dependency-lock-diff.sh sha2@0.10 --package openpfe-llm` → `lock-update.md`
- [x] `crates/openpfe-llm/Cargo.toml` — add pin
- [x] **`cargo audit`** → append `scan.md`
- [x] `verdict.md`

### `uuid`

- [x] `.dev/dependencies/uuid/rational.md` — need: opaque `JobId` for download jobs; scope: **`openpfe-llm`**; feature `v4`
- [x] `dependency-lock-diff.sh uuid@1 --package openpfe-llm` → `lock-update.md`
- [x] `crates/openpfe-llm/Cargo.toml` — add pin
- [x] **`cargo audit`** → append `scan.md`
- [x] `verdict.md`

### `openpfe-llm` — workspace reuse (no new intake folder)

- [x] `serde`, `serde_json`, `thiserror` from workspace in `crates/openpfe-llm/Cargo.toml`
- [x] `tempfile` dev-dep for integration tests

### Batch close-out

- [ ] Single commit: intake docs + `Cargo.toml` / `Cargo.lock` for **`openpfe-llm`** only (no `src/` product code)
- [x] Confirm workspace **MSRV** compatible with `llama-cpp-sys-2` build (document in `llama-cpp-2/rational.md`)
- [x] **Human intake approval** ([guidelines/plans.md](../guidelines/plans.md#pause-checkpoint-manual-inspection)) — 2026-06-06

After phase A, set **Status** to `Blocked — dependency intake complete; awaiting human review (YYYY-MM-DD).` and stop until approval.

When satisfied, human sets:

```markdown
**Status:** Complete (YYYY-MM-DD) — intake approved; [008-openpfe-llm](./008-openpfe-llm.md) may proceed.
```

## Acceptance criteria

- [x] Every new direct dependency has `.dev/dependencies/<name>/` with `rational.md`, `lock-update.md`, `scan.md`, `verdict.md`
- [x] `lock-update.md` matches resolution-only preview (not post-build lock)
- [x] `scan.md` contains **`cargo audit` only** output from immediately after manifest edits
- [x] All `verdict.md` files record **accept** for product scope (or documented defer/reject blocks 008)
- [x] No `openpfe-llm` `src/` implementation in intake-only commit
- [x] Human intake approval checkbox checked
- [x] Plan **Status** → `Complete (2026-06-06)`

## Out of scope

- `openpfe-llm` / `openpfe-ui` / `openpfe-server` product `src/` — crate plans below
- `rmcp`, HuggingFace CLI, CUDA feature enablement (unless explicitly added to rational)
- Promoting `reqwest` from `openpfe-server` dev-dep to workspace `[workspace.dependencies]` (optional; per-crate pin is fine)

## Next

1. [008-openpfe-llm](./008-openpfe-llm.md) — domain crate (after intake approval)
2. [008-openpfe-mcp](./008-openpfe-mcp.md) — stub handler (no new externals; may run in parallel with `openpfe-llm` after intake)
3. [008-openpfe-ui](./008-openpfe-ui.md) — LLM HTTP routes
4. [008-openpfe-server](./008-openpfe-server.md) — mount API + `AppState` wiring
