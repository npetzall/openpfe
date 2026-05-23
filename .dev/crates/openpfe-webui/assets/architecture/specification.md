# Architecture view — specification

## Panels

1. **Domain map** — domains as clusters; problems shown within or linked.
2. **Dependency & contract** — when a domain or edge is selected.

## Actions

| Action | Outcome |
|--------|---------|
| **Create / edit domain** | Assign problems; name/describe domain |
| **Add dependency** | Link domains; surface cycles when rules require acyclicity |
| **Edit contract** | Define or update boundary contract between domains |
| **Create specification for domain** | Creates/opens **unhandled** specification; user directed to Specification view |

Rule C4: [../shared/specification.md](../shared/specification.md).

Implements FR-UI-4 in [requirements.md](./requirements.md).
