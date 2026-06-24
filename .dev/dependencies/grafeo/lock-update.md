# grafeo — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh grafeo@0.5.42 --package openpfe-graph` (product crate; features expanded from spike `lpg` only).

Applied product manifest (`crates/openpfe-graph/Cargo.toml`):

```toml
grafeo = { version = "0.5.42", default-features = false, features = [
    "lpg", "text-index", "vector-index", "hybrid-search", "parallel",
] }
```

Spike intake (`openpfe-graph-spike`, `lpg` only) — **38 packages** added to `Cargo.lock`, including:

| Crate | Role |
|-------|------|
| `grafeo` 0.5.42 | Direct |
| `grafeo-engine`, `grafeo-core`, `grafeo-adapters`, `grafeo-storage`, `grafeo-common` | Grafeo workspace |
| `bincode` 2.0.1 | Transitive (Grafeo stack) |
| `regex`, `aho-corasick`, `memmap2`, `parking_lot`, `dashmap`, `crossbeam-*`, `crc32fast`, … | Transitive |

Workspace dependency count for audit: **260** crate dependencies (post-add, pre-build).
