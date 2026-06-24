# Debug view — specification

**API:** `POST /llm/complete`, `GET /llm/status` (LLM); `POST /debug/mcp` (MCP JSON-RPC) — [openpfe-ui/specification.md](../../../openpfe-ui/specification.md). Same `McpHandler` as IDE agents over IPC.

## Panels

1. **Local LLM** — free-form conversation via `POST /llm/complete`; active `model_id` read-only with navigation to [Configuration](../configuration/) for setup.
2. **MCP** — send/receive raw JSON-RPC (`POST /debug/mcp`); e.g. `tools/list`, read-only tool calls; display response body verbatim.

## Safety

Destructive graph mutations are not the default; prefer read-only probes unless user explicitly opts in.

Implements FR-UI-7 in [requirements.md](./requirements.md).
