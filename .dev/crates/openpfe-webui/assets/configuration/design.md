# Configuration view — design

**Primary question:** Which local LLM model is active, are weights installed, and are inference settings correct?

**Scope (v1):** Project **`llm.json`** (catalog + `llm`), download into **shared** `~/.openpfe/models/`, active model, tuning. Server settings: **`server.json`** via `/server/config` (separate UI later).

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
