# IndraDB evaluation outcome — rejected

**Status:** **Rejected** for openpfe v1 (2026-05-24).

**Read when:** understanding why IndraDB is not the graph engine; onboarding on engine selection history.

---

## Summary

[IndraDB](https://github.com/indradb/indradb) 5.x with the **`rocksdb-datastore`** feature was the **provisional** embedded graph engine for `openpfe-graph`. A spike on branch `spike_db_indradb` ([plan 006](../../plans/006-openpfe-graph-indradb-spike.md)) did not complete: the workspace **could not compile** the RocksDB backend, and no other IndraDB persistence option met openpfe’s durability, backup, and “prefer pure Rust build” requirements.

**Verdict: No** — IndraDB will not be used in `openpfe-graph`. Engine selection continues with [Grafeo](./spike-grafeo.md), [nanograph](./spike-nanograph.md), and [SparrowDB](./spike-sparrowdb.md). See [graph-db-evaluation.md](./graph-db-evaluation.md).

---

## What we wanted from IndraDB

openpfe needs an **embedded labeled property graph** under `./.openpfe/graph/`:

| Requirement | How IndraDB was expected to help |
|-------------|----------------------------------|
| No separate DB server | In-process `indradb-lib` |
| Project-local persistence | RocksDB files under `./.openpfe/graph/store/` |
| PFE schema | Vertices/edges with JSON properties (`problem`, `cluster`, `depends_on`, `member_of`, `interfaces`) |
| Bounded traversals | Rust API + custom `subgraph` / cycle checks via [GraphStore](./design.md) |
| Backup | Copy `store/` directory while server stopped |
| Rust workspace fit | “Rust-native” graph library with a maintained disk backend |

Early docs assumed **Apache-2.0** and a straightforward RocksDB story. The spike was meant to confirm S1–S5 (and document S6 search gaps) per [graph-db-spike.md](./graph-db-spike.md) and [spike-indradb.md](./spike-indradb.md).

---

## What we did

| Step | Artifact |
|------|----------|
| Dependency intake | [.dev/dependencies/indradb/](../../dependencies/indradb/) — `rational.md`, `lock-update.md`, `scan.md`, `verdict.md` |
| Crate pin | `indradb-lib = { version = "5", features = ["rocksdb-datastore"] }` on `openpfe-graph-spike` |
| Security scan | `cargo audit` immediately after manifest edit ([scan.md](../../dependencies/indradb/scan.md)) |
| Spike implementation | Started `GraphStore`-shaped adapter; **not finished** — blocked by compile failure |
| Plan | [006-openpfe-graph-indradb-spike.md](../../plans/006-openpfe-graph-indradb-spike.md) — **cancelled** |

Intake and audit completed; **platform spike scenarios (S1–S5) were not validated** because `cargo build` / `cargo test` for the RocksDB feature never succeeded on the spike machine.

---

## Problems in detail

### 1. RocksDB requires C++ — build failed

The only **supported durable** backend in IndraDB 5.x for embedded use is **`rocksdb-datastore`**, which pulls **`librocksdb-sys`** and compiles RocksDB with **C++17**.

On the spike environment (macOS, Command Line Tools):

- Compilation failed with **`fatal error: 'cstdint' file not found`** when building `librocksdb-sys`.
- A minimal test (`#include <cstdint>` with the system `c++`) failed the same way — indicating an **incomplete or misconfigured C++ toolchain**, not an IndraDB-specific bug.
- Practical impact: **CI and developers cannot rely on `cargo build --workspace`** unless every machine has a working C++ SDK (typically full Xcode on macOS). That conflicts with openpfe’s goal of a **Rust-first** workspace and predictable builds.

Even where RocksDB does compile, the dependency tree is heavy (compression libs, long compile times) and is **not pure Rust**.

### 2. No acceptable Rust-native disk backend

IndraDB exposes other datastores, but none satisfy openpfe’s v1 bar:

| Backend | Issue |
|---------|--------|
| **`MemoryDatastore`** | In-process only by default. Optional **msgpack** snapshot to a **single file** via `create_msgpack_db` / `sync` — different from a **directory** under `./.openpfe/graph/store/`, and not the operator backup story in [specification.md](./specification.md). |
| **`indradb-sled` ([crates.io](https://crates.io/crates/indradb-sled) 0.1.0)** | Separate, **stale** crate (0.1.0) for a sled-backed store — **not aligned with IndraDB 5.x**, not maintained as a production path alongside current `indradb-lib`. Treat as **not production-ready** for openpfe. |
| **Custom `Datastore` impl** | Out of scope for v1 — same cost as choosing another engine. |

So: **durable embedded storage for IndraDB 5.x effectively means RocksDB**, which we could not ship reliably in the spike.

### 3. License mismatch with early assumptions

| Source | License |
|--------|---------|
| Early openpfe docs | Assumed **Apache-2.0** |
| `indradb-lib` 5.0.0 on crates.io | **MPL-2.0** |

MPL-2.0 is compatible with many products but imposes **file-level** obligations different from Apache-2.0. This was not a blocker by itself but adds friction versus MIT/Apache-2.0 candidates (e.g. Grafeo, nanograph, SparrowDB).

### 4. Supply chain — transitive `bincode`

`cargo audit` after adding `indradb-lib` (RocksDB feature) reported:

- **RUSTSEC-2025-0141** — `bincode` 1.3.3 **unmaintained**, via `indradb-lib` → `rocksdb-datastore`.

No critical CVE was reported in that run, but carrying an unmaintained serializer on the persistence path is undesirable for a long-lived product crate.

### 5. Product gaps (even if RocksDB had built)

These were known before rejection and remain true:

| Gap | Detail |
|-----|--------|
| **S6 — similar problem search** | No first-class full-text search on node properties. v1 would need `list_nodes` + in-memory filter or a **sidecar index** (e.g. Tantivy), weakening the “single engine” story. |
| **Query surface** | Low-level IndraDB query API — `subgraph`, filters, and DAG validation require a non-trivial adapter (spike started this work). |
| **Server vs library** | The `indradb` binary crate is server-oriented; embedding uses `indradb-lib` directly — fine, but easy to confuse with “run indradb-server”. |

These alone would not have been fatal if RocksDB had passed the spike; they weighed against IndraDB when combined with build and persistence issues.

---

## Rejection decision

| Field | Value |
|-------|--------|
| **Decision** | **Reject** — do not use IndraDB in `openpfe-graph` |
| **Version evaluated** | `indradb-lib` **5.0.0** (`rocksdb-datastore`) |
| **Spike result** | **Fail** (compile blocked; scenarios not run) |
| **Intake verdict** | [.dev/dependencies/indradb/verdict.md](../../dependencies/indradb/verdict.md) |
| **Spike checklist** | [spike-indradb.md](./spike-indradb.md) — closed |
| **Implementation plan** | [006-openpfe-graph-indradb-spike.md](../../plans/006-openpfe-graph-indradb-spike.md) — cancelled |

Normative crate docs (`design.md`, `specification.md`, `requirements.md`) now mark the engine as **TBD** pending spikes on the replacement shortlist.

---

## What we did not conclude

- IndraDB’s **graph model** or Rust API may be fine for other projects with RocksDB already in the stack and MPL-2.0 accepted.
- Fixing macOS CLT/Xcode might unblock **local** RocksDB builds; that does not remove C++ from CI, compile time, or the sled/memory backend gaps above.
- Rejection is **not** a judgment on Grafeo/nanograph/SparrowDB — those are evaluated separately.

---

## Next steps (openpfe)

1. Run [graph-db-spike.md](./graph-db-spike.md) on **Grafeo**, **nanograph**, and **SparrowDB** (see [graph-db-evaluation.md](./graph-db-evaluation.md)).
2. Lock engine lines in [specification.md](./specification.md) and [design.md](./design.md) only after a spike **Pass** (or Pass with documented caveats).
3. Add a new implementation plan (e.g. `007-openpfe-graph-<engine>-spike` or impl plan) for the winner — do not revive plan 006 for IndraDB.

---

## Related documents

| Document | Role |
|----------|------|
| [graph-db-evaluation.md](./graph-db-evaluation.md) | Current candidate table and shortlist |
| [spike-indradb.md](./spike-indradb.md) | Original spike checklist + Fail metadata |
| [graph-db-spike.md](./graph-db-spike.md) | Shared S1–S6 scenarios |
| [dependencies/indradb/](../../dependencies/indradb/) | Intake and audit artifacts |
| [006-openpfe-graph-indradb-spike.md](../../plans/006-openpfe-graph-indradb-spike.md) | Cancelled plan |
