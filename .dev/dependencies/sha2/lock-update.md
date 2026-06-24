# sha2 — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh sha2@0.10 --package openpfe-llm` (2026-06-06).

Applied manifest:

```toml
sha2 = "0.10"
```

**Direct:** `sha2` 0.10.9.

**Transitive added (if not already present via other crates):**

| Crate | Role |
|-------|------|
| `sha2` | SHA-256 |
| `digest` | Trait layer |
| `block-buffer`, `crypto-common` | Digest plumbing |
| `generic-array`, `typenum` | Const generics for block size |

Some of these may already exist in the lock from other workspace members; net new at intake time: **`sha2`** plus small crypto-common stack.
