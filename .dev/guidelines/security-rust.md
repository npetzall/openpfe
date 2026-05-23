# Security — Rust dependencies

**Read when:** adding or upgrading a **crates.io** or **git** dependency, touching root or member `Cargo.toml`, or reviewing supply-chain risk for the workspace.

**Normative refs:** [workspace-crates.md](../workspace-crates.md#dependency-rules-normative) (crate boundaries), [coding-rust.md](./coding-rust.md) (dependency style), [dependencies/README.md](../dependencies/README.md) (intake folder and workflow).

## Intake before `Cargo.toml`

No external crate is added to root or member `Cargo.toml` until a folder exists at **`.dev/dependencies/<crate-name>/`** with at least **`rational.md`** started. Before merge, that folder must also contain **`scan.md`**, **`lock-update.md`**, and **`verdict.md`** (see [dependencies/README.md](../dependencies/README.md)).

| File | Purpose |
|------|---------|
| `rational.md` | Why the crate is needed; scope; re-implement vs use trade-off |
| `lock-update.md` | Transitive lock delta from **resolution-only** preview (not from `cargo build`) |
| `scan.md` | `cargo audit` and other scan outputs (append as tools are added) |
| `verdict.md` | Accept / reject / defer; version; what landed in the workspace |

**Lock preview:** [.dev/scripts/dependency-lock-diff.sh](../scripts/dependency-lock-diff.sh) copies `Cargo.toml` / `Cargo.lock` to gitignored `Cargo-with-<crate-name>.*`, resolves against the trial manifest, and diffs lockfiles without leaving the workspace lock changed.

## `cargo audit` (RustSec)

[`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) checks the resolved dependency graph against the [RustSec advisory database](https://github.com/RustSec/advisory-db).

1. **Install** (once per machine): `cargo install cargo-audit` (or your preferred Rust toolchain manager equivalent).
2. **Mandatory order when adding a dependency:** edit `Cargo.toml` → run **`cargo audit`** from the repository root → record output in `.dev/dependencies/<crate-name>/scan.md`. Do not run `cargo build`, `cargo check`, or `cargo update` before that audit unless a documented scan requires it.
3. **When to run again**
   - Before opening or updating a PR that changes `Cargo.toml` or `Cargo.lock`.
   - After rebasing or merging branches that touched the lockfile.
   - Periodically on `main` (automate in CI when a pipeline exists).

**If `cargo audit` reports issues:** upgrade the affected crate to a non-vulnerable version, replace the dependency, or (only with team agreement) document a time-bounded exception in **`verdict.md`** and the PR; architectural risk may also go in the owning crate’s `design.md`.

Keep `cargo-audit` current ([releases](https://github.com/rustsec/rustsec/releases)).

## Process: adding a new external crate

Treat every new direct dependency as a **supply-chain and maintenance** decision.

1. **Record** — `.dev/dependencies/<crate-name>/rational.md` (need, scope, re-implement vs adopt, alternatives).
2. **Preview graph** — `dependency-lock-diff.sh <crate-name>@<version>` → capture diff in **`lock-update.md`**.
3. **Workspace rules** — obey [dependency rules](../workspace-crates.md#dependency-rules-normative); add version under `[workspace.dependencies]` when the workspace uses that table; members reference `{ workspace = true }`.
4. **Manifest** — apply the same dependency shape to the real `Cargo.toml`(s).
5. **Audit** — **`cargo audit`** immediately; paste into **`scan.md`**.
6. **Other scans** — append to **`scan.md`** as the project adds tools.
7. **Decide** — **`verdict.md`**; non-obvious security behavior also in owning crate **`design.md`**.
8. **Merge bar** — clean `cargo audit` or documented remediation in **`verdict.md`** and the PR.

### Crate selection (checklist)

- **License:** compatible with project policy; read `LICENSE` / SPDX on crates.io.
- **Maintenance:** recent releases; no obvious abandonment for security-sensitive areas (crypto, IPC, HTTP, untrusted parsing).
- **Source:** prefer **crates.io** with pinned versions; **git** deps need a pinned revision and reason in **`rational.md`**.
- **Scope:** avoid large unrelated subgraphs or duplicate stack choices from [architcture.md](../architcture.md).

After merge, run `cargo tree -i <crate>` when reviewing unexpected transitive deps.

## Embedded Web UI (JavaScript)

When `crates/openpfe-webui/` (or another crate) adds a **`package.json`** dependency, apply the same discipline: justify the need, check license and maintenance, run **`npm audit`** (or the repo’s chosen JS audit command) before merge, and keep lockfiles committed when the project adopts them.

## Related

- [dependencies/README.md](../dependencies/README.md) — folder layout and workflow
- [coding-rust.md](./coding-rust.md) — manifest edit order, workspace deps
- [testing-rust.md](./testing-rust.md) — CI expectations when tests cover integration
