# indradb-lib — verdict

- **Decision:** reject
- **Version evaluated:** 5.0.0 (`rocksdb-datastore` feature)
- **Workspace placement:** was scoped to `openpfe-graph-spike` only (not merged to product)

## Rationale (2026-05-24)

1. **RocksDB backend** — `librocksdb-sys` requires a working C++ standard library toolchain. Spike branch could not compile on macOS (missing `cstdint` / incomplete CLT). This blocks reliable CI and developer machines without full Xcode.
2. **No acceptable Rust-native persistence path for v1** — `MemoryDatastore` + msgpack snapshot is not the directory-based durability/backup model in [specification.md](../../crates/openpfe-graph/specification.md). **`indradb-sled`** (0.1.0) is unmaintained relative to IndraDB 5.x and not production-ready.
3. **License** — crates.io lists **MPL-2.0** (not Apache-2.0 as assumed in early docs).
4. **Transitive risk** — `bincode` 1.x via `rocksdb-datastore` (RUSTSEC-2025-0141 unmaintained advisory on audit).

## Follow-ups

- Record **Fail** in [spike-indradb.md](../../crates/openpfe-graph/spike-indradb.md) and [graph-db-evaluation.md](../../crates/openpfe-graph/graph-db-evaluation.md).
- Run engine spikes for **[Grafeo](https://github.com/GrafeoDB/grafeo)**, **[nanograph](https://github.com/nanograph/nanograph)**, and **[SparrowDB](https://github.com/ryaker/SparrowDB)** per updated evaluation doc.
- Cancel [006-openpfe-graph-indradb-spike.md](../../plans/006-openpfe-graph-indradb-spike.md); add replacement plan(s) when a candidate is chosen.
