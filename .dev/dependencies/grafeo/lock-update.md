# grafeo — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh grafeo@0.5.42 --package openpfe-graph-spike` (dry-run reported 0 new packages before member was in lock).

Applied intake manifest:

```toml
grafeo = { version = "0.5.42", default-features = false, features = ["lpg"] }
```

`cargo add grafeo@0.5.42 -p openpfe-graph-spike --no-default-features -F lpg` — **38 packages** added to `Cargo.lock`, including:

| Crate | Role |
|-------|------|
| `grafeo` 0.5.42 | Direct |
| `grafeo-engine`, `grafeo-core`, `grafeo-adapters`, `grafeo-storage`, `grafeo-common` | Grafeo workspace |
| `bincode` 2.0.1 | Transitive (Grafeo stack) |
| `regex`, `aho-corasick`, `memmap2`, `parking_lot`, `dashmap`, `crossbeam-*`, `crc32fast`, … | Transitive |

Workspace dependency count for audit: **260** crate dependencies (post-add, pre-build).
