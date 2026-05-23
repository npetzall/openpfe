# openpfe-graph — design

Embedded graph store under `./.openpfe/graph/`. **No** HTTP, IPC, or MCP.

## Decisions

| Topic | Decision |
|-------|----------|
| **Engine (v1)** | **[IndraDB](https://github.com/indradb/indradb)** + **RocksDB** datastore (`rocksdb-datastore` feature). Evaluation: [graph-db-evaluation.md](./graph-db-evaluation.md). |
| **Persistence path** | `./.openpfe/graph/store/` — RocksDB directory; created on server start. |
| **Graph model** | **Labeled property graph** — not RDF in v1. |
| **Query (v1)** | `GraphStore` trait + IndraDB adapter; **no** exposed Cypher/Datalog. |
| **Concurrency** | **Single writer** (server); in-process readers only. |
| **Backup** | Copy `./.openpfe/graph/` directory when server is stopped. |
| **Ownership** | Server process owns writes; `openpfe-ui` / `openpfe-mcp` call sync store API via `Arc`. |
| **Format** | DB-native from first ship; no legacy markdown problem-tree in product. |
| **Workspace boundary** | Dedicated graph crate; phase 2 implementation. |

## `GraphStore` trait (v1)

Abstraction over IndraDB so HTTP/MCP do not depend on engine types directly:

| Operation | Purpose |
|-----------|---------|
| `open(path)` / `create(path)` | Open `./.openpfe/graph/store/` |
| `get_node(id)` | Fetch node + JSON properties |
| `list_nodes(filter)` | By `type`, `cluster_id`, etc. |
| `upsert_node` | Create/update |
| `delete_node` | Remove node (and incident edges per policy) |
| `create_edge` / `delete_edge` | Typed directed edges |
| `neighbors(id, edge_types?, direction)` | Adjacency |
| `subgraph(cluster_id, limits)` | Bounded traversal for UI + context shield |
| `validate_acyclic_deps()` | Cycle detection on `depends_on` (tooling alignment) |

Heavy traversals may run on `spawn_blocking` from async HTTP handlers.

## Scope

- Engine adapter (IndraDB RocksDB)
- Schema constants for node/edge types — [specification.md](./specification.md)
- Open/create at `./.openpfe/graph/store/` when server starts

## Spikes (engine proof)

Before phase 2 merge, run [graph-db-spike.md](./graph-db-spike.md): [spike-indradb.md](./spike-indradb.md), [spike-grafeo.md](./spike-grafeo.md). Update engine lines here and in [specification.md](./specification.md) from spike outcomes.

## Related

- [openpfe-mcp/design.md](../openpfe-mcp/design.md) — agent queries
- [openpfe-ui/design.md](../openpfe-ui/design.md) — human graph HTTP
