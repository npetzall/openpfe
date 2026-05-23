# Debug view — specification

## Panels

1. **Local LLM** — free-form conversation; active `model_id` read-only with navigation to [Configuration](../configuration/) for setup.
2. **MCP** — protocol-level send/receive or tool listing.

## Safety

Destructive graph mutations are not the default; prefer read-only probes unless user explicitly opts in.

Implements FR-UI-7 in [requirements.md](./requirements.md).
