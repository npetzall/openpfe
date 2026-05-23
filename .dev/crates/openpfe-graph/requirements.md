# openpfe-graph — requirements

## FR-9 Problem graph

- **FR-9.0** Use graph storage paths in [specification.md](./specification.md) relative to **process cwd** (project root); no shared path-helper crate.
- **FR-9.1** Maintain problem graph as source of truth in **IndraDB (RocksDB)** under `./.openpfe/graph/store/` ([specification.md](./specification.md), [graph-db-evaluation.md](./graph-db-evaluation.md)).
- **FR-9.5** Expose traversals and subgraph limits for MCP context shield ([specification.md](./specification.md#traversal-limits-context-shield)).
- **FR-9.3** Align with repo skills/workflows (`openpfe-init`, `openpfe-drill`) over time.
- **FR-9.4** Graph data project-scoped; not in `USER_HOME`.

**FR-9.2** (MCP context shield): [openpfe-mcp/requirements.md](../openpfe-mcp/requirements.md).

## Related

- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md) — human graph interaction (FR-6.2)
