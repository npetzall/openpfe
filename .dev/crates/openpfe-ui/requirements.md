# openpfe-ui — requirements

## FR-6 HTTP / human API

- **FR-6.2** Problem graph interaction per PFE (drill-down, architecture view) via [specification.md](./specification.md#graph) routes; live updates via **polling** in v1.
- **FR-6.3** Configuration UI: read merged config, write project overrides.
- **FR-6.4** Model selection and download into `USER_HOME/.openpfe/models`.

Human-facing API stability for **Web UI** and **TUI**.

## Non-goals

- MCP tools → `openpfe-mcp`
- Serving static files → `openpfe-webui`
- **FR-6.1** embedded assets → `openpfe-webui`

## Related

- [openpfe-webui/requirements.md](../openpfe-webui/requirements.md)
