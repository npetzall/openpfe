# Plan 001: workspace scaffolding

**Status:** Complete (2026-05-23). Workspace stubs, path-only deps, CI (fmt/clippy/test); `cargo audit` deferred until first external crate intake.

**Read when:** bootstrapping the Rust workspace (no root `Cargo.toml` yet). This plan creates **empty, compiling** workspace members only — product behavior belongs in later per-crate plans.

## Normative sources

| Doc | Use |
|-----|-----|
| [workspace-crates.md](../workspace-crates.md) | Members, layout, dependency rules, phasing |
| [architcture.md](../architcture.md) | tokio, axum, tower-http stack |
| [guidelines.md](../guidelines.md) | Strict rules for agents |
| [guidelines/coding-rust.md](../guidelines/coding-rust.md) | Workspace layout, edition, async boundaries |
| [dependencies/README.md](../dependencies/README.md) | External crate intake (not used in this plan) |

## Prerequisites

- [x] `.dev/crates/<name>/` docs present for each member (`design.md`, `requirements.md`, `specification.md`).
- [x] Rust toolchain installed (`rustup`); root `Cargo.toml` and `crates/` tree in place.

## Goal

A **compiling** Cargo workspace with eight members under `crates/` (initial scaffold had nine including `openpfe-core`; **removed** — paths documented per crate, see [workspace-crates.md](../workspace-crates.md)), **path dependencies only** (no external crates.io/git deps yet), wired per [dependency rules](../workspace-crates.md#dependency-rules-normative), and **no product logic** — only empty `lib.rs` / `main.rs` stubs so `cargo check --workspace` and `cargo test --workspace` succeed.

External crates enter later via [.dev/dependencies/](../dependencies/README.md) intake, not in this plan.

**Explicitly deferred:** IPC, HTTP, flock, graph store, MCP, LLM, `server.json` / `llm.json`, embedded assets content, and any handler/router implementation (per-crate plans after this one).

## Workspace members

| Crate | Path | Package type | Entry file |
|-------|------|--------------|------------|
| `openpfe` | `crates/openpfe/` | binary (`[[bin]]`) | `src/main.rs` |
| `openpfe-server` | `crates/openpfe-server/` | library | `src/lib.rs` |
| `openpfe-ipc` | `crates/openpfe-ipc/` | library | `src/lib.rs` |
| `openpfe-ui` | `crates/openpfe-ui/` | library | `src/lib.rs` |
| `openpfe-webui` | `crates/openpfe-webui/` | library | `src/lib.rs` |
| `openpfe-mcp` | `crates/openpfe-mcp/` | library | `src/lib.rs` |
| `openpfe-graph` | `crates/openpfe-graph/` | library | `src/lib.rs` |
| `openpfe-llm` | `crates/openpfe-llm/` | library | `src/lib.rs` |

Each library crate exposes a minimal public surface (e.g. empty `pub fn stub() {}` or crate-level doc comment only) so dependents can link without unused-crate warnings if needed. The binary crate’s `main` may be empty or `Ok(())` only.

## Path dependencies (scaffold time)

Wire **only** the edges allowed by [dependency rules](../workspace-crates.md#dependency-rules-normative). Stubs must not import sibling crates yet unless required to satisfy the dependency graph; prefer declaring path deps in `Cargo.toml` without use in source until the owning crate plan.

| Crate | `path` dependencies |
|-------|---------------------|
| `openpfe` | `openpfe-ipc` |
| `openpfe-server` | `openpfe-ipc`, `openpfe-ui`, `openpfe-webui`, `openpfe-mcp`, `openpfe-llm` |
| `openpfe-ipc` | *(none — domain-free)* |
| `openpfe-ui` | `openpfe-graph`, `openpfe-llm` |
| `openpfe-webui` | *(none — embed-only)* |
| `openpfe-mcp` | `openpfe-graph` |
| `openpfe-graph` | *(none — no I/O crates)* |
| `openpfe-llm` | *(none — self-contained under `./.openpfe/`)* |

**Forbidden at any time** (verify with `cargo tree`): `openpfe-ui` → `openpfe-mcp` / `openpfe-webui` / `openpfe-server`; `openpfe-webui` → `openpfe-ui` / `openpfe-graph`; `openpfe-mcp` → `openpfe-ui`; `openpfe` bin → `openpfe-ui` / `openpfe-mcp` / `openpfe-llm` on client paths.

## Tasks

### Root workspace

- [x] Add root `Cargo.toml` with `[workspace]` `resolver = "2"` and `members` listing all workspace paths under `crates/` (see [physical layout](../workspace-crates.md#physical-workspace-layout)).
- [x] Add `[workspace.package]` defaults: `edition = "2024"`, shared `version`, `license` / `repository` if already decided for the repo. *(No `license` field yet — not decided for the repo.)*
- [x] Do **not** add `[workspace.dependencies]` or any crates.io/git dependency — first external crate follows [dependencies/README.md](../dependencies/README.md).
- [x] Add `rust-toolchain.toml` (stable channel + components: `rustfmt`, `clippy`) or document MSRV in root `Cargo.toml` if org policy requires it.

### Per-crate manifests

- [x] Create `crates/<name>/Cargo.toml` for each member: `name`, `edition` via workspace inheritance, `publish = false` if applicable.
- [x] Set `[lib]` / `[[bin]]` correctly (`openpfe` only as binary; others as `lib`).
- [x] Declare `path` dependencies per table above only (no `{ workspace = true }` external deps).
- [x] Add empty `src/lib.rs` or `src/main.rs` for every member.

### Repository hygiene

- [x] Extend root `.gitignore`: `target/`, `.openpfe/`, editor/OS noise.
- [x] Add placeholder `crates/openpfe-webui/assets/` (empty or `.gitkeep`) — no embedded bundle yet.
- [x] Add minimal CI (e.g. `.github/workflows/rust.yml`) or extend existing pipeline:
  - [x] `cargo fmt --check`
  - [x] `cargo clippy --workspace -- -D warnings` (or project default)
  - [x] `cargo test --workspace`
  - [ ] `cargo audit` once a `Cargo.lock` with external deps exists (see [security-rust.md](../guidelines/security-rust.md))

### Verification

- [x] `cargo check --workspace` exits 0.
- [x] `cargo test --workspace` exits 0 (allow empty test suites).
- [x] `cargo tree` shows no forbidden edges.
- [x] All workspace member directories exist under `crates/` with the expected `Cargo.toml` + `src/` entry file.

## Acceptance criteria

- [x] Workspace builds and tests with **stubs only** — no IPC, HTTP listeners, flock, graph persistence, MCP, or inference.
- [x] Member list and path dependency graph match [workspace-crates.md](../workspace-crates.md).
- [x] CI (if added) runs the checks above on push/PR (except `cargo audit` — pending external deps).

## Out of scope

- Implementing FRs or APIs from any `specification.md`.
- `openpfe-webui` JS toolchain, `package.json`, or real static assets.
- Adding external crates or `[workspace.dependencies]` (use [dependencies/README.md](../dependencies/README.md) instead).
- `Cargo.lock` with crates.io entries until the first dependency intake.

## Next

Per [workspace-crates phasing](../workspace-crates.md#phasing), implement **Phase 1** crates in dependency order:

1. `openpfe-ipc`
2. `openpfe-server`
3. `openpfe` (binary)

Add dedicated per-crate plans under `.dev/plans/` as they are authored.
