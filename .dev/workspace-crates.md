# Workspace crate evaluation

Proposed **Cargo workspace** members for `openpfe`, with boundaries, dependencies, and per-crate docs under [crates/](./crates/).

## Client transports (decided)

| Client | Primary | Also | Notes |
|--------|---------|------|--------|
| **CLI** (simple: `stop`, echo, spawn) | **IPC** | — | Control plane; MCP uses IPC via stdio bridge |
| **Web UI** (browser) | **HTTP** | — | Static from `openpfe-webui`; data from `openpfe-ui` API |
| **TUI** | **HTTP** | **IPC** (optional) | Same REST as Web UI for most work; IPC for trusted/local control when needed |
| **IDE / agents** | **IPC** (MCP) | — | `openpfe-mcp` — parallel to `openpfe-ui`, not HTTP |

**Symmetry:**

| Audience | Crate | Transport |
|----------|-------|-----------|
| **Humans** (browser, TUI) | **`openpfe-ui`** | HTTP `/api/v1/…` |
| **Agents** (Cursor, Claude, …) | **`openpfe-mcp`** | IPC (MCP envelopes) |

**`openpfe-webui`** is only the **embedded static front-end** (HTML/CSS/JS). It does not define the HTTP API — that is **`openpfe-ui`**.

---

## Principles

| Principle | Implication |
|-----------|-------------|
| **Thin binary** | `openpfe` — CLI, IPC client, MCP stdio bridge |
| **Server wires, not rules** | `openpfe-server` — listeners, lock, mount `openpfe-ui` + `openpfe-webui` |
| **Human API one place** | `openpfe-ui` — all HTTP handlers Web UI and TUI share |
| **Agent API one place** | `openpfe-mcp` — MCP tools/resources |
| **Static assets only** | `openpfe-webui` — embed + MIME; no graph/config logic |
| **Domain without I/O** | `openpfe-core`, `openpfe-graph` |
| **IPC without domain** | `openpfe-ipc` — frames + UDS only |
| **Local LLM in v1** | **`openpfe-llm`** — llama.cpp ships with the product; models in `USER_HOME/.openpfe/models` |

---

## Recommended members (v1)

```mermaid
flowchart BT
  bin[openpfe bin]
  srv[openpfe-server]
  ipc[openpfe-ipc]
  mcp[openpfe-mcp]
  ui[openpfe-ui]
  webui[openpfe-webui]
  core[openpfe-core]
  graph[openpfe-graph]
  llm[openpfe-llm]
  bin --> ipc
  bin --> core
  srv --> ipc
  srv --> mcp
  srv --> ui
  srv --> webui
  srv --> core
  srv --> llm
  ui --> core
  ui --> graph
  mcp --> core
  mcp --> graph
  core --> graph
  llm --> core
```

| Crate | Type | Responsibility | Stays out of |
|-------|------|----------------|--------------|
| **`openpfe`** | `bin` | CLI; IPC for control + MCP bridge; optional HTTP client later for rich CLI | HTTP route definitions; MCP tools; graph |
| **`openpfe-server`** | `lib` | `pid` flock; UDS + HTTP listeners; mount API + static; shutdown | Business handlers (delegates to `openpfe-ui`, `openpfe-mcp`) |
| **`openpfe-ipc`** | `lib` | Framing, envelope, UDS, echo/shutdown | Domain; MCP semantics; HTTP |
| **`openpfe-ui`** | `lib` | **HTTP API for humans** — graph, config, models; route handlers | Static embed; IPC; MCP |
| **`openpfe-webui`** | `lib` | **Embedded browser UI** — `assets/` → bytes + content-type | API handlers; domain logic |
| **`openpfe-mcp`** | `lib` | **MCP for agents** — tools/resources → core/graph | HTTP; static assets |
| **`openpfe-core`** | `lib` | Config merge, paths, domain types, model registry | Sockets |
| **`openpfe-graph`** | `lib` | Embedded graph store | HTTP; IPC; MCP |
| **`openpfe-llm`** | `lib` | llama.cpp — load models from `USER_HOME/.openpfe/models`, inference for UI/MCP | — |

**v1 count: 9 crates** (all included; **`openpfe-llm` is not deferred**).

---

## Alternatives considered

| Idea | Verdict |
|------|---------|
| **`openpfe-embed`** | **Renamed → `openpfe-webui`** (static assets only). |
| **`openpfe-api`** | **Renamed concept → `openpfe-ui`** (human HTTP API). |
| **HTTP handlers inside `openpfe-server`** | Reject — use `openpfe-ui` so server stays wiring-only. |
| **TUI-only IPC for all data** | Reject — duplicate Web UI; HTTP primary, IPC for trusted extras. |
| **Merge `openpfe-graph` into `core`** | Reject for v1 — keep **`openpfe-graph`** separate; engine **IndraDB + RocksDB** ([openpfe-graph/design.md](./crates/openpfe-graph/design.md)). |

---

## Dependency rules (normative)

1. `openpfe-ui` → `core`, `graph`; **not** on `openpfe-mcp`, `openpfe-webui`, `openpfe-server`.
2. `openpfe-mcp` → `core`, `graph`; **not** on `openpfe-ui`.
3. `openpfe-webui` → minimal (embed only); **not** on `core`, `ui`, `graph`.
4. `openpfe-server` → `ipc`, `ui`, `webui`, `mcp`, `core`; mounts routes from `ui` + static from `webui`.
5. `openpfe` bin → `ipc`, `core`; avoid linking `ui`/`mcp`/`llm` on client-only paths.
6. `openpfe-llm` → `core`; used by `openpfe-server` (and exposed via `openpfe-ui` / `openpfe-mcp` as needed).

---

## Physical workspace layout

```
crates/
  openpfe/
  openpfe-server/
  openpfe-ipc/
  openpfe-ui/
  openpfe-webui/       # assets/ (embed) + tests/ (JS unit tests)
  openpfe-mcp/
  openpfe-core/
  openpfe-graph/
  openpfe-llm/
```

---

## Documentation convention

Each workspace member has **`.dev/crates/<crate-name>/`** with three authoritative files:

| File | Purpose |
|------|---------|
| `design.md` | How the crate is built |
| `requirements.md` | What it must do (FRs owned by this crate) |
| `specification.md` | Normative APIs, formats, and contracts it owns |

**Rules:**

1. **Normative detail lives in the crate folder** — not duplicated at `.dev/` root.
2. **Root `.dev/`** holds only system overview ([architcture.md](./architcture.md)), multi-crate contracts ([cross-cutting.md](./cross-cutting.md)), and workspace evaluation (this file).
3. **Extra research or deep dives** sit next to the crate they belong to (e.g. [openpfe-graph/graph-db-evaluation.md](./crates/openpfe-graph/graph-db-evaluation.md)).
4. **Cross-crate FRs** — split by owning crate; product-wide traceability in [cross-cutting.md](./cross-cutting.md#fr-traceability).

Folder index: [crates/README.md](./crates/README.md).

---

## Phasing

| Phase | Crates | Goal |
|-------|--------|------|
| **1** | `openpfe`, `openpfe-ipc`, `openpfe-server`, `openpfe-core` | Lock, socket, echo, HTTP stub |
| **2** | `openpfe-graph`, `openpfe-webui`, `openpfe-ui` | Graph + static shell + human API |
| **3** | `openpfe-mcp`, `openpfe-llm` | MCP graph tools (read-only) + **llama-cpp-2** inference over HTTP |

---

## Decision summary

| # | Decision | Recorded in |
|---|----------|-------------|
| 1 | **9 members** — including **`openpfe-llm`** in v1 (not deferred). | This file |
| 2 | **`openpfe-ui`** = HTTP API for Web UI + TUI (like **`openpfe-mcp`** for agents). | This file |
| 3 | **`openpfe-webui`** = embedded static browser assets (was `openpfe-embed`). | This file |
| 4 | **CLI** → IPC for control; **Web/TUI** → HTTP for data. | [architcture.md](./architcture.md) |
| 5 | **`openpfe-server`** mounts `openpfe-ui` + `openpfe-webui`; does not own handler logic. | [openpfe-server/design.md](./crates/openpfe-server/design.md) |
| 6 | **`openpfe-llm`** in v1 — `llama-cpp-2`, HTTP `/llm/*`, graph-only MCP. | [openpfe-llm/design.md](./crates/openpfe-llm/design.md), [openpfe-mcp/specification.md](./crates/openpfe-mcp/specification.md) |
| 7 | **HTTP stack:** **axum** (handlers/router in `openpfe-ui`; server mounts). | [architcture.md](./architcture.md), [openpfe-ui/design.md](./crates/openpfe-ui/design.md) |
| 8 | **Async runtime:** **tokio** workspace-wide for v1. | [architcture.md](./architcture.md), [openpfe-server](./crates/openpfe-server/), [openpfe-ipc](./crates/openpfe-ipc/) |
| 9 | **`openpfe-graph`:** separate member; **IndraDB + RocksDB** at `./.openpfe/graph/store/`. | [openpfe-graph/design.md](./crates/openpfe-graph/design.md) |
| 10 | **TUI:** graph/config/models via **HTTP only**; **IPC** for echo + shutdown (same admin envelopes as CLI). | [architcture.md](./architcture.md#tui-v1) |

---

## Related

- [architcture.md](./architcture.md) — system overview (references this doc for crate matrix)
- [cross-cutting.md](./cross-cutting.md) — multi-crate contracts
- [README.md](./README.md)
- [crates/README.md](./crates/README.md) — authoritative per-crate docs
