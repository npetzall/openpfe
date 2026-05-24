# External dependency intake

**Read when:** proposing or adding a **crates.io** or **git** dependency to any `Cargo.toml`.

Every new external crate is **recorded and evaluated here before** it appears in the workspace manifest. Path dependencies between workspace members do not use this folder.

## Folder layout

One directory per **external** crate name (crates.io package name):

```
.dev/dependencies/
  README.md
  <crate-name>/
    rational.md      # why needed; re-implement vs use; scope
    scan.md          # audit and other scan outputs (append over time)
    lock-update.md   # transitive lockfile delta from resolution-only preview
    verdict.md       # accept / reject / defer; what was merged
```

Create `<crate-name>/` **before** editing root or member `Cargo.toml`.

Implementation plans that introduce dependencies must follow the **intake → pause → implement** split in [guidelines/plans.md](../guidelines/plans.md) so agents stop after audit for human review.

## Workflow

| Step | Action |
|------|--------|
| 1 | Create `.dev/dependencies/<crate-name>/` and write **`rational.md`** (need, scope, trade-offs). |
| 2 | From repo root: `.dev/scripts/dependency-lock-diff.sh <crate-name>@<version>` (optional `--package <member>`, default `openpfe`; optional `--workspace` for `[workspace.dependencies]` or `{ workspace = true }` edges) — resolution-only lockfile preview (`cargo add`, then `cargo update --workspace --dry-run`; manifests restored on exit); paste or save the output into **`lock-update.md`**. |
| 3 | Add the dependency to the real `Cargo.toml`(s), then **immediately** run `cargo audit` from the repo root; record output in **`scan.md`**. |
| 4 | Run any other scans the project adopts later; append to **`scan.md`**. |
| 5 | Write **`verdict.md`** (accepted version, owning crate, PR link, or reject/defer reason). |
| 6 | Commit intake docs with the `Cargo.toml` / `Cargo.lock` change. |

**Order after real manifest edit:** add to `Cargo.toml` → `cargo audit` (no other command in between). See [security-rust.md](../guidelines/security-rust.md) and [coding-rust.md](../guidelines/coding-rust.md).

The preview script restores manifests when it exits; discard any `Cargo.lock` changes from step 2 before committing.

## File templates

### `rational.md`

- **Need:** what capability is missing?
- **Scope:** which workspace member(s) will depend on it?
- **Trade-off:** re-implement vs adopt this crate (maintenance, `unsafe`, license, binary size).
- **Alternatives considered:** other crates or std-only approach.

### `scan.md`

Dated sections per tool, e.g.:

```markdown
## cargo audit — 2026-05-21

(paste output)
```

### `lock-update.md`

Transitive packages **added**, **removed**, or **version-changed** vs current `Cargo.lock`, from the resolution-only preview (not from `cargo build`). Source: `dependency-lock-diff.sh <crate-name>@<version>` output (`cargo update --workspace --dry-run`).

### `verdict.md`

- **Decision:** accept | reject | defer
- **Version / source:** crates.io version or git rev
- **Workspace placement:** `[workspace.dependencies]` key and member edges
- **Follow-ups:** upgrades, exceptions, links to PR or `design.md`

## Related

- [.dev/scripts/dependency-lock-diff.sh](../scripts/dependency-lock-diff.sh)
- [guidelines/security-rust.md](../guidelines/security-rust.md)
- [guidelines/coding-rust.md](../guidelines/coding-rust.md)
