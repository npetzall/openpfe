# openpfe-graph — specification

## Storage location

| Path | Contents |
|------|----------|
| `./.openpfe/graph/store/` | IndraDB **RocksDB** datastore files |
| `./.openpfe/graph/` | Parent dir; may add non-DB metadata later (keep `store/` for engine) |

Open with `GraphStore::open(project_graph_dir().join("store"))` — see [openpfe-core/specification.md](../openpfe-core/specification.md#path-resolution).

## Schema (v1)

### Node types (`type` property)

| `type` | Purpose |
|--------|---------|
| `problem` | Atomic problem node (PFE drill-down leaf or branch) |
| `cluster` | Logical grouping → component boundary candidate |
| `component` | Approved architecture unit (optional v1; may merge with `cluster` until UI needs split) |
| `contract` | Consumer-driven contract artifact (optional node; may be edge-only metadata in early v1) |

### Edge types (`type` on edge)

| `type` | Direction | Meaning |
|--------|-----------|---------|
| `depends_on` | directed | Source problem/cluster **depends on** target (prerequisite) |
| `member_of` | directed | Problem **`member_of`** cluster |
| `interfaces` | directed | Cluster/component **interfaces** with target (contract boundary) |

### Common JSON properties (nodes)

| Property | Used on | Notes |
|----------|---------|--------|
| `title` | all | Short label |
| `description` | problem, cluster | Markdown/plain text |
| `status` | problem, cluster | e.g. `open`, `refined`, `done` |
| `cluster_id` | problem | Denormalized helper for filters (must match `member_of` edge) |

Edges may carry `contract_body`, `version`, `consumer_id`, `provider_id` when `type = interfaces`.

### Identifiers

- **UUID** strings (v4) for node ids in v1 — stable across export/import later.

## Traversal limits (context shield)

`subgraph(cluster_id, { max_nodes, max_depth })` defaults (implementation constants, overridable later):

| Limit | Default |
|-------|---------|
| `max_depth` | **3** hops from cluster boundary |
| `max_nodes` | **200** nodes |

Used by `openpfe-mcp` tools and `GET /api/v1/graph/clusters/:id/subgraph` — [openpfe-ui/specification.md](../openpfe-ui/specification.md).

## Validation

- `depends_on` edges among problems/clusters must form a **DAG** — `validate_acyclic_deps()` returns cycles for UI/MCP.
- `member_of` target must be `type = cluster`.

## Engine dependency (normative)

- Crate: `indradb` / `indradb-lib` **5.x**
- Feature: **`rocksdb-datastore`**
- License: verify `LICENSE` at spike (Apache-2.0 expected)

## Related

- [graph-db-evaluation.md](./graph-db-evaluation.md) — selection rationale
- [openpfe-mcp/specification.md](../openpfe-mcp/specification.md) — agent query surfaces
