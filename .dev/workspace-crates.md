# Workspace crate evaluation

Proposed **Cargo workspace** members for `openpfe`, with boundaries, dependencies, and per-crate docs under [crates/](./crates/).

## Client transports (decided)

| Client | Primary | Also | Notes |
|--------|---------|------|--------|
| **CLI** (simple: `stop`, echo, spawn) | **IPC** | — | Control plane; MCP uses IPC via stdio bridge |
| **Web UI** (browser) | **HTTP** | — | Static from `openpfe-webui`; data from `openpfe-ui` API |
| **TUI** | **HTTP** | **IPC** (admin) | HTTP: graph + `llm.json` (same as Web UI). IPC: `echo`, `shutdown`, **`server_config_*`** |
| **IDE / agents** | **IPC** (MCP) | — | `openpfe-mcp` handler via `type: mcp` |
| **Web UI Debug** | **HTTP** (MCP) | — | Same `McpHandler` via `POST /api/v1/debug/mcp` |

**Symmetry:**

| Audience | Crate | Transport |
|----------|-------|-----------|
| **Humans** (browser, TUI) | **`openpfe-ui`** | HTTP `/api/v1/…` (REST graph + LLM; MCP debug JSON-RPC) |
| **Agents** (Cursor, Claude, …) | **`openpfe-mcp`** | IPC `type: mcp` (stdio bridge) |

Both MCP transports call the same **`McpHandler`** instance in the server process.

**`openpfe-webui`** is only the **embedded static front-end** (HTML/CSS/JS). It does not define the HTTP API — that is **`openpfe-ui`**.

---

## Principles

| Principle | Implication |
|-----------|-------------|
| **Thin binary** | `openpfe` — CLI, IPC client, MCP stdio bridge |
| **Server wires, not rules** | `openpfe-server` — listeners, lock, mount `openpfe-ui` + `openpfe-webui` |
| **Human API one place** | `openpfe-ui` — HTTP handlers for graph + LLM + MCP debug transport (Web UI and TUI). **`server.json`** is IPC admin only |
| **MCP semantics one place** | `openpfe-mcp` — `McpHandler`, tools/resources; wired to IPC (agents) and HTTP debug (browser) |
| **Static assets only** | `openpfe-webui` — embed + MIME; no graph/config logic |
| **Domain without I/O** | `openpfe-graph` (graph store only) |
| **IPC without domain** | `openpfe-ipc` — frames + UDS only |
| **Config format** | **JSON** per owning crate (`serde_json`); HTTP uses same shapes — **no TOML** |
| **Local LLM in v1** | **`openpfe-llm`** — project `llm.json`, shared `USER_HOME/.openpfe/models/`, llama.cpp |
| **Paths** | **Documented per crate** in `design.md` / `requirements.md` / `specification.md` — no shared path-helper crate or API |

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
  graph[openpfe-graph]
  llm[openpfe-llm]
  bin --> ipc
  srv --> ipc
  srv --> mcp
  srv --> ui
  srv --> webui
  srv --> llm
  ui --> graph
  mcp --> graph
  ui --> llm
  ui --> mcp
```

| Crate | Type | Responsibility | Stays out of |
|-------|------|----------------|--------------|
| **`openpfe`** | `bin` | CLI; IPC for control + MCP bridge; optional HTTP client later for rich CLI | HTTP route definitions; MCP tools; graph |
| **`openpfe-server`** | `lib` | `pid` flock; UDS + HTTP; **`server.json`**; mount API + static; shutdown | Domain handlers (delegates to `openpfe-ui`, `openpfe-mcp`) |
| **`openpfe-ipc`** | `lib` | Framing, envelope, UDS, echo/shutdown | Domain; MCP semantics; HTTP |
| **`openpfe-ui`** | `lib` | **HTTP API for humans** — graph + `llm.json` + MCP debug route; `AppState` (graph + LLM + `McpHandler`) from server | Static embed; IPC framing; MCP tool defs; `server.json`; owning config files |
| **`openpfe-webui`** | `lib` | **Embedded browser UI** — `assets/` → bytes + content-type | API handlers; domain logic |
| **`openpfe-mcp`** | `lib` | **`McpHandler`** — MCP tools/resources → graph; transport-agnostic | HTTP route impl; static assets; IPC framing |
| **`openpfe-graph`** | `lib` | Embedded graph store | HTTP; IPC; MCP |
| **`openpfe-llm`** | `lib` | `llm.json`, registry, downloads, inference | HTTP route impl in `openpfe-ui` |

**v1 count: 8 crates** (all included; **`openpfe-llm` is not deferred**).

---

## Alternatives considered

| Idea | Verdict |
|------|---------|
| **`openpfe-embed`** | **Renamed → `openpfe-webui`** (static assets only). |
| **`openpfe-api`** | **Renamed concept → `openpfe-ui`** (human HTTP API). |
| **HTTP handlers inside `openpfe-server`** | Reject — use `openpfe-ui` so server stays wiring-only. |
| **TUI-only IPC for all data** | Reject — duplicate Web UI; HTTP primary, IPC for trusted extras. |
| **`openpfe-core` (shared path helpers / domain types)** | **Reject** — paths documented in each owning crate; no shared helper API. |
| **Merge `openpfe-graph` into a “core” crate** | Reject — keep **`openpfe-graph`** separate; engine **Grafeo** ([openpfe-graph/decision.md](./crates/openpfe-graph/decision.md)). |

---

## Dependency rules (normative)

1. `openpfe-ui` → `graph`, `llm`, `mcp` (`McpHandler` for debug transport only); **not** on `openpfe-webui`, `openpfe-server`.
2. `openpfe-mcp` → `graph`; **not** on `openpfe-ui` or `openpfe-llm`.
3. `openpfe-webui` → minimal (embed only); **not** on `ui`, `graph`, `llm`.
4. `openpfe-server` → `ipc`, `ui`, `webui`, `mcp`, `llm`; mounts routes from `ui` + static from `webui`.
5. `openpfe` bin → `ipc` only; avoid linking `ui`/`mcp`/`llm` on client-only paths.
6. `openpfe-llm` → self-contained LLM paths + config (documented in crate specs).

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
3. **Extra research or deep dives** sit next to the crate they belong to (e.g. [openpfe-graph/decision.md](./crates/openpfe-graph/decision.md)).
4. **Cross-crate FRs** — split by owning crate; product-wide traceability in [cross-cutting.md](./cross-cutting.md#fr-traceability).

Folder index: [crates/README.md](./crates/README.md).

---

## Phasing

| Phase | Crates | Goal |
|-------|--------|------|
| **1** | `openpfe`, `openpfe-ipc`, `openpfe-server` | Lock, socket, echo, HTTP stub |
| **2** | `openpfe-graph`, `openpfe-webui`, `openpfe-ui` | Graph + static shell + human API |
| **3** | `openpfe-mcp`, `openpfe-llm` | MCP graph tools (read-only) + **llama-cpp-2** inference over HTTP |

---

## Decision summary

| # | Decision | Recorded in |
|---|----------|-------------|
| 1 | **8 members** — including **`openpfe-llm`** in v1 (not deferred). | This file |
| 2 | **`openpfe-ui`** = HTTP API for Web UI + TUI; **`openpfe-mcp`** = shared `McpHandler` (IPC + HTTP debug). | This file |
| 3 | **`openpfe-webui`** = embedded static browser assets (was `openpfe-embed`). | This file |
| 4 | **CLI/TUI** → IPC for control + **`server.json`**; **Web/TUI** → HTTP for graph + `llm.json`. | [architcture.md](./architcture.md) |
| 5 | **`openpfe-server`** mounts `openpfe-ui` + `openpfe-webui`; does not own handler logic. | [openpfe-server/design.md](./crates/openpfe-server/design.md) |
| 6 | **`openpfe-llm`** in v1 — `llama-cpp-2`, HTTP `/llm/*`, graph-only MCP. | [openpfe-llm/design.md](./crates/openpfe-llm/design.md), [openpfe-mcp/specification.md](./crates/openpfe-mcp/specification.md) |
| 7 | **HTTP stack:** **axum** (handlers/router in `openpfe-ui`; server mounts). | [architcture.md](./architcture.md), [openpfe-ui/design.md](./crates/openpfe-ui/design.md) |
| 8 | **Async runtime:** **tokio** workspace-wide for v1. | [architcture.md](./architcture.md), [openpfe-server](./crates/openpfe-server/), [openpfe-ipc](./crates/openpfe-ipc/) |
| 9 | **`openpfe-graph`:** separate member; **Grafeo** at `./.openpfe/graph/store/`. | [openpfe-graph/decision.md](./crates/openpfe-graph/decision.md) |
| 10 | **TUI:** graph + `llm.json` via **HTTP**; **IPC** for echo, shutdown, **`server_config_*`** (admin). | [architcture.md](./architcture.md#tui-v1) |
| 11 | **No `openpfe-core`** — project paths are normative in each crate’s docs, not a shared Rust helper module. | This file, [cross-cutting.md](./cross-cutting.md) |

---

## Related

- [architcture.md](./architcture.md) — system overview (references this doc for crate matrix)
- [cross-cutting.md](./cross-cutting.md) — multi-crate contracts
- [README.md](./README.md)
- [crates/README.md](./crates/README.md) — authoritative per-crate docs
