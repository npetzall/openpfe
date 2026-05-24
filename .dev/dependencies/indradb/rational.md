# indradb-lib

## Need

Embedded labeled property graph for the PFE problem graph (nodes, typed edges, JSON properties) per [openpfe-graph/graph-db-evaluation.md](../../crates/openpfe-graph/graph-db-evaluation.md).

## Scope

**Spike only:** `openpfe-graph-spike` (plan [006-openpfe-graph-indradb-spike.md](../../plans/006-openpfe-graph-indradb-spike.md)). Production `openpfe-graph` intake is deferred until spike passes.

```toml
indradb-lib = { version = "5", features = ["rocksdb-datastore"] }
```

## Trade-off

- **Adopt:** Rust-native API, RocksDB persistence under `./.openpfe/graph/store/`, matches provisional v1 choice.
- **Re-implement:** Custom RocksDB/LMDB adjacency layer — high cost; rejected for v1.
- **Grafeo:** Parallel spike on another branch; not in this intake batch.

## License

**MPL-2.0** on crates.io (not Apache-2.0 as assumed in early docs). Spike records SPDX in Results; product distribution must account for MPL-2.0 file-level obligations if accepted.

## Transitive deps (notable)

Pulled via `rocksdb-datastore`: `rocksdb`, `librocksdb-sys`, `bincode`. No separate intake folders unless policy expands.

## Alternatives considered

- `indradb` meta-crate — server-oriented; spike uses `indradb-lib` directly.
- In-memory only — insufficient for durability/backup scenarios.
- Grafeo — [spike-grafeo.md](../../crates/openpfe-graph/spike-grafeo.md).
