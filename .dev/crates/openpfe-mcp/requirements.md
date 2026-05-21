# openpfe-mcp — requirements

## FR-3 MCP (semantics)

- **FR-3.3** MCP semantics implemented in server (this crate); stdio path is transport only in `openpfe` binary.

## FR-9 Agents

- **FR-9.2** MCP exposes context-bounded queries for agents (context shield) via tools in [specification.md](./specification.md#mcp-tools-v1).
- **FR-9.6** v1 MCP tools are **read-only graph/contract** operations; LLM and git orchestration tools are deferred.

## Related

- [openpfe-graph/requirements.md](../openpfe-graph/requirements.md) — FR-9.1, FR-9.4, FR-9.5
- [openpfe_tooling.md](../../../openpfe_tooling.md)
