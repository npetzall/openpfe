# openpfe-ipc — requirements

## Functional requirements — FR-5 IPC

- **FR-5.1** Support admin envelopes (`echo`, `shutdown`, `server_config_get`, `server_config_put`) and MCP request/response (`type: mcp`). **Streaming over IPC is out of v1** (request/response envelopes only; see [design.md](./design.md)). `server.json` I/O is **admin only** — not exposed on MCP ([openpfe-mcp/specification.md](../openpfe-mcp/specification.md)).
- **FR-5.4** Every frame includes `"v": 1`; reject unsupported versions with `type: error`.
- **FR-5.5** Enforce **16 MiB** max frame body; reject invalid length prefixes.
- **FR-5.2** Multiple concurrent IPC clients without cross-talk.
- **FR-5.3** Extensible for future TUI and CLI subcommands.

## Non-functional

- **NFR-1** IPC echo round-trip &lt; 50ms on localhost under idle load.
- **NFR-6** v1 Unix domain sockets; Windows backend later behind transport trait.

## Related

- [specification.md](./specification.md) — envelope normative draft
