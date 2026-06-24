# Configuration view — design

**Primary question:** Which local LLM model is active, are weights installed, and are inference settings correct?

**Scope:** Committed **`catalog.json`**, local **`llm.json`**, download into **shared** `~/.openpfe/models/`, active model, tuning. **`server.json`** is **not** in the Web UI — IPC admin for CLI/TUI ([openpfe-ipc/specification.md](../../../openpfe-ipc/specification.md)).

**Bootstrap:** **`openpfe llm init`** (CLI) — not this view.

## Backend

| Crate | Role |
|-------|------|
| **openpfe-llm** | `catalog.json`, `llm.json`, registry, discover, download, engine |
| **openpfe-ui** | REST catalog + runtime routes; **`reload_engine`** after load-affecting changes |

```
./.openpfe/catalog.json (commit)  →  ~/.openpfe/models/<id>/  →  llm.json model id
```

**JSON** on disk matches API documents. **cwd** = project root.

## Discover → add

1. `GET /catalog/discover` — installed / curated / search.
2. User picks candidate → optional edit `recommended`, `requirements`.
3. `POST /catalog` — single entry.
4. Sequential POSTs for multiple adds (loopback; no bulk API).

Index: [../README.md](../README.md).
