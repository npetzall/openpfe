# openpfe-graph — design

Embedded graph store under `./.openpfe/graph/`. **No** HTTP, IPC, or MCP.

## Decisions

| Topic | Decision |
|-------|----------|
| **Engine (v1)** | **[Grafeo](https://github.com/GrafeoDB/grafeo)** — [decision.md](./decision.md). Adapter: [grafeo/design.md](./grafeo/design.md). |
| **Persistence path** | `./.openpfe/graph/store/` — engine persistent files; created on server start. |
| **Graph model** | **Labeled property graph** — not RDF in v1. |
| **Query (v1)** | `GraphStore` trait + **`GrafeoGraphStore`** ([`crates/openpfe-graph`](../../../crates/openpfe-graph)); **no** exposed Cypher/GQL on HTTP/MCP. |
| **Search (v1)** | **S6** lexical (`search_problems`) + **S6+** multi-signal (`find_similar`) — normative in [specification.md](./specification.md#search-and-similarity-v1). |
| **Concurrency** | **Single writer** (server); in-process readers only. |
| **Backup** | Copy `./.openpfe/graph/` directory when server is stopped. |
| **Ownership** | Server process owns writes; `openpfe-ui` / `openpfe-mcp` call sync store API via `Arc`. |
| **Format** | DB-native from first ship; no legacy markdown problem-tree in product. |
| **Workspace boundary** | Dedicated graph crate; phase 2 implementation. |

## `GraphStore` trait (v1)

Abstraction over the chosen engine so HTTP/MCP do not depend on engine types directly:

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
| `search_problems(query, k)` | **S6** — lexical BM25 on `problem` `title` / `description` |
| `find_similar(draft, k)` | **S6+** — merged ranked candidates with `match_kinds` (lexical, semantic, structural) |
| `backup_full(path)` | Engine backup + operator directory copy |
| `rebuild_text_indexes()` / `rebuild_vector_index()` | After bulk import |

Heavy traversals and search may run on `spawn_blocking` from async HTTP handlers.

## Search workflow (v1 product intent)

During drill-down or manual entry, callers discover whether a **problem is already in the graph** before creating a duplicate:

```
New problem draft (title + description [+ embedding])
        │
        ▼
  find_similar / search_problems  ──► ranked existing `problem` nodes
        │                              (score, match_kinds, snippet)
        ▼
  Agent or user: link to existing | refine draft | create new node
```

MCP/UI tool surfaces are owned by `openpfe-mcp` / `openpfe-ui`; this crate owns store behavior only.

## Scope

- `GrafeoGraphStore` — [grafeo/specification.md](./grafeo/specification.md), [grafeo/design.md](./grafeo/design.md), and `schema.rs`
- Schema constants for node/edge types — [specification.md](./specification.md)
- Open/create at `./.openpfe/graph/store/` when server starts

## Engine selection (complete)

Adapter implemented in **`crates/openpfe-graph`** (`GrafeoGraphStore`). Engine locked — [decision.md](./decision.md). Spike evidence: [spike/](./spike/).

## Related

- [decision.md](./decision.md) — locked engine
- [grafeo/](./grafeo/) — Grafeo adapter details
- [openpfe-mcp/design.md](../openpfe-mcp/design.md) — agent queries
- [openpfe-ui/design.md](../openpfe-ui/design.md) — human graph HTTP
