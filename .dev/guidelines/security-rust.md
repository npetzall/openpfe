# Security — Rust dependencies

**Read when:** adding or upgrading a **crates.io** or **git** dependency, touching root or member `Cargo.toml`, or reviewing supply-chain risk for the workspace.

**Normative refs:** [workspace-crates.md](../workspace-crates.md#dependency-rules-normative) (crate boundaries), [coding-rust.md](./coding-rust.md) (dependency style).

## `cargo audit` (RustSec)

[`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) checks the resolved dependency graph against the [RustSec advisory database](https://github.com/RustSec/advisory-db).

1. **Install** (once per machine): `cargo install cargo-audit` (or your preferred Rust toolchain manager equivalent).
2. **Run** from the repository root (workspace root): `cargo audit`.
3. **When to run**
   - Before opening or updating a PR that changes `Cargo.toml` or `Cargo.lock`.
   - After rebasing or merging branches that touched the lockfile.
   - Periodically on `main` (ideally automated in CI when a pipeline exists).

**If `cargo audit` reports issues:** upgrade the affected crate to a non-vulnerable version, replace the dependency, or (only with team agreement) document a time-bounded exception in the PR and, if the risk is architectural, in the owning crate’s `design.md`.

Keep `cargo-audit` itself reasonably current so the advisory DB format stays supported (`cargo audit --version` vs [releases](https://github.com/rustsec/rustsec/releases)).

## Process: adding a new external crate

Treat every new direct dependency as a small **supply-chain and maintenance** decision, not only a version pin.

1. **Need**
   - Prefer **std** and crates already mandated by [architcture.md](../architcture.md) / crate `design.md` (e.g. one HTTP stack).
   - If the capability exists in the workspace, extend the owning crate instead of pulling a parallel library.

2. **Workspace rules**
   - Obey [dependency rules](../workspace-crates.md#dependency-rules-normative): no forbidden edges (e.g. UI/MCP boundaries).
   - Add the version under root **`[workspace.dependencies]`** when the workspace uses that table; member crates reference it with `{ workspace = true }`.

3. **Crate selection (short checklist)**
   - **License:** compatible with the project’s licensing policy; read `LICENSE` / SPDX on crates.io.
   - **Maintenance:** recent releases, responsive maintainers, no obvious abandonment for security-sensitive code (crypto, IPC, HTTP, parsing untrusted input).
   - **Source:** prefer **crates.io** with reproducible versions; **git** deps need a pinned revision and a clear reason (document in PR or `design.md`).
   - **Scope:** avoid crates that pull large unrelated subsystems or duplicate an existing stack choice.

4. **Graph impact**
   - Run `cargo tree -i <crate>` (after adding) to see **who** brings it in and avoid surprise duplicates.
   - Run `cargo audit` on the updated lockfile.

5. **Documentation**
   - Non-obvious choices (native `build.rs`, network at build time, `unsafe`, or security-relevant behavior) belong in the owning crate’s **`design.md`** (and `specification.md` if behavior is normative).

6. **Merge bar**
   - PR that introduces or materially upgrades an external crate should show **clean `cargo audit`** or explain the remediation plan / accepted risk in the PR description.

## Embedded Web UI (JavaScript)

When `crates/openpfe-webui/` (or another crate) adds a **`package.json`** dependency, apply the same discipline: justify the need, check license and maintenance, run **`npm audit`** (or the repo’s chosen JS audit command) before merge, and keep lockfiles committed when the project adopts them.

## Related

- [coding-rust.md](./coding-rust.md) — workspace deps, `cargo tree` mental model
- [testing-rust.md](./testing-rust.md) — CI expectations when tests cover integration
