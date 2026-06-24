# openpfe-graph — requirements

## FR-9 Problem graph

- **FR-9.0** Use graph storage paths in [specification.md](./specification.md) relative to **process cwd** (project root); no shared path-helper crate.
- **FR-9.1** Maintain problem graph as source of truth in an **embedded graph engine** (**Grafeo**) under `./.openpfe/graph/` ([specification.md](./specification.md), [decision.md](./decision.md)).
- **FR-9.5** Expose traversals and subgraph limits for MCP context shield ([specification.md](./specification.md#traversal-limits-context-shield)).
- **FR-9.6** **S6 (lexical search)** — `search_problems` returns ranked `problem` nodes by BM25 on `title` and `description` so callers can answer “is this already recorded?” ([specification.md](./specification.md#search-and-similarity-v1)).
- **FR-9.7** **S6+ (find similar)** — `find_similar` returns a merged ranked list with per-hit `match_kinds` (`lexical`, `semantic`, `structural`), `score`, and `snippet` ([specification.md](./specification.md#search-and-similarity-v1), [grafeo/design.md](./grafeo/design.md)).
- **FR-9.3** Align with repo skills/workflows (`openpfe-init`, `openpfe-drill`) over time.
- **FR-9.4** Graph data project-scoped; not in `USER_HOME`.

**FR-9.2** (MCP context shield): [openpfe-mcp/requirements.md](../openpfe-mcp/requirements.md).

## Related

- [decision.md](./decision.md) — Grafeo (v1 engine)
- [grafeo/requirements.md](./grafeo/requirements.md) — engine dependency and intake
- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md) — human graph interaction (FR-6.2)
