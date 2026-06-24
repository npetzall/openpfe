# Spike: IndraDB 5.x + RocksDB

**Parent program:** [program.md](./program.md)

**Engine:** [IndraDB](https://github.com/indradb/indradb) with **`rocksdb-datastore`** feature.

**Provisional v1 choice:** was documented in [evaluation.md](./evaluation.md) — **rejected**. See [indradb-outcome.md](./indradb-outcome.md).

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | **Closed — Fail** (2026-05-24) |
| **Owner** | _unassigned_ |
| **Branch / crate** | _e.g. `spike/indradb` or `crates/openpfe-graph-spike`_ |
| **Commit** | _SHA when complete_ |
| **Platforms tested** | macOS: ☐ — Linux: ☐ |

---

## Goals

1. Prove embedded open/create at `./.openpfe/graph/store/`-style path on macOS and Linux.
2. Implement [program.md](./program.md) scenarios **S1–S5** behind a `GraphStore`-shaped prototype.
3. Document **S6** (similar problem search) as **out of engine** and acceptable for v1 without blocking.
4. Capture build, audit, backup, and durability evidence for [evaluation.md](./evaluation.md).

---

## Setup

### Dependencies (pin in spike `Cargo.toml`)

```toml
indradb = { version = "5", features = ["rocksdb-datastore"] }
# Record exact resolved version in Results
```

Also note `indradb-lib` if used directly.

### Storage path

| Path | Use |
|------|-----|
| `<temp>/openpfe-graph-spike/store/` | Mimics [specification.md](../specification.md) `./.openpfe/graph/store/` |

Use a fresh directory per run; never point at a real project graph during experiments.

### License check

- [ ] Read `LICENSE` in IndraDB repo and crates.io metadata
- [ ] Expected: **Apache-2.0** — record actual SPDX in Results

---

## Tasks (checklist)

### 1. Lifecycle (S1)

- [ ] `create(store_path)` on empty dir — succeeds
- [ ] Insert nodes: `problem`, `cluster` with JSON properties (`title`, `description`, `status`, `type`)
- [ ] Insert edges: `depends_on`, `member_of`, `interfaces` with contract properties on `interfaces`
- [ ] `open(store_path)` after close — same node count and stable UUID lookup
- [ ] `list_nodes` filter `type = problem` and `cluster_id` (denormalized property) behave as expected

### 2. Curation (S2)

- [ ] `upsert_node` — change `title`, `status`
- [ ] `create_edge` — add extra `depends_on` between problems
- [ ] `delete_edge` — remove one edge, reopen, confirmed gone
- [ ] `delete_node` — remove problem; incident edges gone; cluster remains

### 3. Architecture data (S3)

- [ ] Fixture includes `interfaces` edge with `contract_body`, `version`, `consumer_id`, `provider_id`
- [ ] `subgraph` or `neighbors` returns contract fields on edges in response payload (JSON map)

### 4. Context shield (S4)

- [ ] `subgraph(cluster_id, { max_depth: 3, max_nodes: 200 })` on shared fixture — returns ≤ 200 nodes, depth honored
- [ ] Request with `max_nodes: 500` — stops at cap (no unbounded traversal)
- [ ] With ~1k synthetic problems: record latency (see Measurements)

### 5. Acyclic validation (S5)

- [ ] `validate_acyclic_deps()` on DAG fixture — `ok`
- [ ] Add edge creating cycle on `depends_on` — returns cycle representation (list of node ids or paths)
- [ ] Remove cycle edge — `ok` again

### 6. Search (S6) — optional for IndraDB

- [ ] Document: no first-class FTS in IndraDB
- [ ] v1 workaround for spike: `list_nodes` + in-memory filter on `title` for fixture scale only
- [ ] Note in Results whether v1 product needs separate search index (recommended if S6 is phase 2)

### 7. Stretch — search & compare (S6+)

Optional — [program.md#stretch-goals--search--compare-s6](./program.md#stretch-goals--search--compare-s6). If run:

- [ ] Lexical via sidecar or brute-force on fixture; structural via `neighbors` + `cluster_id`
- [ ] Semantic: document deferral or spike with `openpfe-llm` embeddings + in-memory ranking (not blocking)
- [ ] Record recommendation: IndraDB + separate FTS/index vs engine switch

### 8. Durability and backup

- [ ] Close/reopen after full fixture write
- [ ] Backup: copy `store/` tree while closed; restore to new path; `open` and verify
- [ ] Optional crash test: document outcome

### 9. Supply chain

- [ ] `cargo tree -i indradb`
- [ ] `cargo audit` — paste summary
- [ ] Debug build time; release binary size delta (see parent doc)

---

## Implementation notes

- Map PFE `type` (node/edge) to IndraDB identifiers/properties per [specification.md](../specification.md). Spike may use a single vertex/edge type with a `type` property if simpler.
- UUIDs: store as string property or native id mapping — document choice.
- Subgraph: BFS from cluster via `member_of` + `depends_on` + `interfaces` within limits; implement in spike even if IndraDB query API is low-level.
- Async: spike may be sync only; note if `spawn_blocking` will be required from axum handlers.

**Reference:** IndraDB 5.x docs, `rocksdb-datastore` feature flags, [CHANGELOG](https://github.com/indradb/indradb/blob/master/CHANGELOG.md).

---

## Results

### Summary

| Item | Result |
|------|--------|
| **Recommendation** | **Fail** |
| **Caveats** | RocksDB/`librocksdb-sys` did not compile (C++ `cstdint` / toolchain). No production Rust disk backend. `indradb-sled` 0.1.0 not viable for IndraDB 5.x. MPL-2.0. |
| **Pinned version** | `indradb-lib = 5.0.0` (intake only; spike implementation not completed) |

### Measurements

| Metric | macOS | Linux |
|--------|-------|-------|
| `open` cold (ms) | | |
| `subgraph` default (ms, nodes/edges) | | |
| `validate_acyclic_deps` (ms) | | |
| `list_nodes` limit 200 (ms) | | |
| Debug `cargo build` (s) | | |
| Release size delta | | |
| `cargo audit` | | |

### Transitive deps (notable)

```
(paste cargo tree -i indradb excerpt)
```

### S6 note

_How v1 will handle “problem already exists” without engine FTS._

### S6+ stretch results (optional)

_See [program.md#stretch-goals--search--compare-s6](./program.md#stretch-goals--search--compare-s6) — lexical / semantic / structural, sidecar recommendation._

### Backup procedure

_Operator steps for `./.openpfe/graph/store/` copy._

---

## Recommendation template

**Pass:** All required S1–S5 checks green on both platforms; audit acceptable; build/size within team budget; backup path clear.

**Pass with caveats:** e.g. slow subgraph at 1k nodes but under acceptable ms; S6 explicitly deferred.

**Fail:** Cannot reopen store, corrupts on crash, fails DAG/subgraph, audit blocker, or build cost prohibitive — reopen [evaluation.md](./evaluation.md).

---

## Related

- [program.md](./program.md)
- [grafeo.md](./grafeo.md)
