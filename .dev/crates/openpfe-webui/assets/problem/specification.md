# Problem view — specification

## Panels

1. **List** — unhandled only (default); sort/filter TBD at implementation.
2. **Graph** — relationships; selection syncs with list.
3. **Detail / work** — visible when a problem is selected.

## Selected problem — actions

| Action | Outcome |
|--------|---------|
| **Work with embedded LLM** | Chat/prompt scoped to problem; graph edits only via explicit user confirm |
| **MCP handoff** | Display command/snippet for external tooling with scoped context reference |
| **Mark handled** | Handled state; removed from default list |

## Graph interaction

Selecting a node selects the problem; drill-down reveals sub-problems per PFE methodology.

Rule C1: [../shared/specification.md](../shared/specification.md).

Implements FR-UI-3 in [requirements.md](./requirements.md).
