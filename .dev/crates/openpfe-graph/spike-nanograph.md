# Spike: nanograph (embedded LPG)

**Parent program:** [graph-db-spike.md](./graph-db-spike.md)

**Engine:** [nanograph](https://github.com/nanograph/nanograph) — on-device typed property graph (Rust, Lance, Arrow, DataFusion). Tagline: “DuckDB for graphs.”

**Role:** Primary **Grafeo alternative** — especially for **S6** (FTS, BM25, semantic/hybrid search) and folder-based project storage.

---

## Spike metadata

| Field | Value |
|-------|--------|
| **Status** | Not started |
| **Owner** | _unassigned_ |
| **Branch / crate** | _e.g. `spike/nanograph` or `crates/openpfe-graph-spike`_ |
| **Commit** | _SHA when complete_ |
| **Platforms tested** | macOS: ☐ — Linux: ☐ |

---

## Goals

1. Open/create persistent graph at `<temp>/openpfe-graph-spike/nanograph/` mimicking `./.openpfe/graph/`.
2. Implement **S1–S5** behind the same `GraphStore`-shaped prototype and shared fixture as other spikes.
3. Prove **S6** with engine-native search (BM25 / full-text on `title`/`description`).
4. Record build time, `cargo audit`, MSRV (**1.91+** per upstream), and **protoc** requirement.
5. Document gap between nanograph **schema-as-code** (`.pg` files) and PFE **ad-hoc JSON properties** — acceptable mapping for v1?

---

## Setup

### Dependencies (pin in spike `Cargo.toml`)

Prefer **Rust library crate** from workspace (e.g. `nanograph-db` / FFI crate per upstream docs) — pin exact version in Results.

| Check | Action |
|-------|--------|
| MSRV | Align `rust-toolchain.toml` if needed |
| `protoc` | Document CI/dev install if required |
| Features | Minimal — avoid pulling unused ML stacks unless S6+ semantic stretch |

### Storage path

| Path | Use |
|------|-----|
| `<temp>/openpfe-graph-spike/nanograph/` | One-folder store; document files for backup section |

### License check

- [ ] **MIT** — confirm in repo `LICENSE`

---

## Tasks (checklist)

### Lifecycle & data (S1–S3)

- [ ] Create/open; insert cluster, problem nodes, edges (`depends_on`, `member_of`, `interfaces` + contract props)
- [ ] Reopen; stable ids; `list_nodes` by `type` / `cluster_id`
- [ ] `subgraph` with defaults `max_depth=3`, `max_nodes=200`

### Curation & validation (S2, S5)

- [ ] Upsert/delete nodes and edges; delete problem removes incident edges
- [ ] `validate_acyclic_deps` on `depends_on`

### Context shield (S4)

- [ ] Bounded subgraph on ~1k node fixture; cap at 500 nodes

### Search (S6 — required)

- [ ] Duplicate/near-duplicate title ranks in top 3; unrelated absent from top 5
- [ ] Note API used (CLI vs Rust) and mapping to future MCP `find_similar`

### Durability & backup

- [ ] Close/reopen; copy folder while closed; restore and verify

### Supply chain

- [ ] `cargo tree` — note Lance/Arrow/DataFusion weight
- [ ] `cargo audit`
- [ ] Debug build time; release size delta

---

## Results

_Fill when spike completes._

| Item | Result |
|------|--------|
| **Recommendation** | ☐ Pass ☐ Pass with caveats ☐ Fail |
| **Pinned version** | |
| **Schema friction** | _PFE JSON vs `.pg` schema_ |
| **S6** | _pass/fail + ms_ |

---

## Related

- [spike-grafeo.md](./spike-grafeo.md)
- [spike-sparrowdb.md](./spike-sparrowdb.md)
- [graph-db-evaluation.md](./graph-db-evaluation.md)
