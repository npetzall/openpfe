# Configuration view — design

**Primary question:** Which local LLM model is active, are weights installed, and are inference settings correct?

**Scope (v1):** Project **`llm.json`** (catalog + `llm`), download into **shared** `~/.openpfe/models/`, active model, tuning. **`server.json`** is **not** in the Web UI — IPC admin for CLI/TUI ([openpfe-ipc/specification.md](../../../openpfe-ipc/specification.md)).

## Backend

| Crate | Role |
|-------|------|
| **openpfe-llm** | `llm.json`, registry, download, engine |
| **openpfe-ui** | HTTP + **`reload_engine`** after changes |

```
./.openpfe/llm.json  →  ~/.openpfe/models/<id>/  →  llm.model
```

**JSON** on disk matches API documents. **cwd** = project root.

Index: [../README.md](../README.md).
