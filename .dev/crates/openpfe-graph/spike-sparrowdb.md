# Spike: SparrowDB (embedded LPG)

**Parent program:** [graph-db-spike.md](./graph-db-spike.md)

**Engine:** [SparrowDB](https://github.com/ryaker/SparrowDB) — embedded Rust graph DB with WAL-backed durability and Cypher execution (use **Rust API / library** in spike, not product-facing Cypher).

**Role:** Grafeo alternative focused on **pure-Rust storage** and **crash-safe durability** without RocksDB.

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | Not started |
| **Owner** | _unassigned_ |
| **Branch / crate** | _e.g. `spike/sparrowdb`_ |
| **Commit** | _SHA when complete_ |
| **Platforms tested** | macOS: ☐ — Linux: ☐ |

---

## Goals

1. Embed SparrowDB in-process at a project-local directory under `<temp>/openpfe-graph-spike/sparrowdb/`.
2. Implement **S1–S5** via `GraphStore`-shaped adapter (Cypher allowed **only** inside spike if faster than low-level API).
3. Document **S6** — native FTS or acceptable v1 workaround (`list` + filter / sidecar).
4. Compare build, audit, and binary size vs Grafeo and nanograph.

---

## Setup

### Dependencies

Pin crate(s) from SparrowDB workspace (e.g. storage + API facade) in spike `Cargo.toml`; record exact git tag or crates.io version in Results.

### Storage path

Document on-disk layout (catalog, WAL, CSR files) for operator backup under `./.openpfe/graph/`.

### License check

- [ ] **MIT** — confirm in repo `LICENSE`

---

## Tasks (checklist)

Same bar as [graph-db-spike.md](./graph-db-spike.md) sections A–F:

- [ ] S1–S5 scenarios with shared fixture
- [ ] S6 — document engine FTS or workaround
- [ ] Durability smoke + directory backup/restore
- [ ] `cargo audit`; `cargo tree`; build time; release size delta
- [ ] Confirm **no C++** required for default build

---

## Results

_Fill when spike completes._

| Item | Result |
|------|--------|
| **Recommendation** | ☐ Pass ☐ Pass with caveats ☐ Fail |
| **S6** | |
| **vs Grafeo** | ☐ Prefer SparrowDB ☐ Prefer Grafeo ☐ Inconclusive |

---

## Related

- [spike-grafeo.md](./spike-grafeo.md)
- [spike-nanograph.md](./spike-nanograph.md)
- [graph-db-evaluation.md](./graph-db-evaluation.md)
