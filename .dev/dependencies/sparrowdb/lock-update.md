# sparrowdb — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh sparrowdb@0.1.16 --package openpfe-graph-spike` reported **0 new packages** when run after `sparrowdb` was already in `Cargo.lock` (dry-run only).

Applied intake manifest (`crates/openpfe-graph-spike/Cargo.toml`):

```toml
sparrowdb = "0.1.16"
sparrowdb-execution = "0.1.16"
```

`cargo add sparrowdb@0.1.16 -p openpfe-graph-spike` updated the workspace lockfile. Direct SparrowDB workspace crates in `Cargo.lock`:

| Crate | Version |
|-------|---------|
| `sparrowdb` | 0.1.16 |
| `sparrowdb-catalog` | 0.1.16 |
| `sparrowdb-common` | 0.1.16 |
| `sparrowdb-cypher` | 0.1.16 |
| `sparrowdb-execution` | 0.1.16 |
| `sparrowdb-storage` | 0.1.16 |

Approximate normal dependency tree depth from `openpfe-graph-spike`: **~140** packages (includes `serde`, `clap`, `tracing`, crypto, `memmap2`, etc.). Re-run `dependency-lock-diff.sh` on a clean branch without `sparrowdb` in lock for a full added/removed diff if needed for audit.

Workspace dependency count for audit at intake: **260** crate dependencies (post-add, per `cargo audit` output).
