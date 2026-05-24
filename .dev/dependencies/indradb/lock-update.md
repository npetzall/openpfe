# Lock update — indradb-lib@5 (openpfe-graph-spike)

Resolution after adding `indradb-lib = { version = "5", features = ["rocksdb-datastore"] }` to `crates/openpfe-graph-spike/Cargo.toml`.

Preview script reported no additional delta once lockfile was resolved (`dependency-lock-diff.sh indradb-lib@5 --package openpfe-graph-spike`).

## Direct addition

| Crate | Version |
|-------|---------|
| indradb-lib | 5.0.0 |

## Notable transitive (from `cargo tree -p openpfe-graph-spike -i indradb-lib`)

- rocksdb (via `rocksdb-datastore`)
- librocksdb-sys
- bincode 1.3.3 (RUSTSEC-2025-0141 unmaintained — transitive only)

Full tree: run `cargo tree -p openpfe-graph-spike -i indradb-lib` at spike commit.
