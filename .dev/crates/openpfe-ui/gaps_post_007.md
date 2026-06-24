# openpfe-ui — gaps after plan 007

**Status:** Open — **docs partially closed**; **LLM HTTP slice implemented** ([008-openpfe-ui.md](../../plans/008-openpfe-ui.md)); graph routes still pending.

**Context:** [007-openpfe-graph.md](../../plans/007-openpfe-graph.md) completed the `openpfe-graph` crate (Grafeo adapter, S6/S6+, integration tests). This file tracks mismatches and missing detail between `openpfe-ui` docs and `openpfe-graph`, `openpfe-llm`, `openpfe-server`, plus implied needs from [openpfe-webui](../openpfe-webui/assets/README.md).

**Normative targets:** [specification.md](./specification.md), [design.md](./design.md), [requirements.md](./requirements.md).

---

## Re-evaluation summary (2026-06-06)

| Area | Docs closed | Docs open | Impl |
|------|-------------|-----------|------|
| **Graph — search & edges** | G-1, G-2 | G-3–G-8 | Pending |
| **LLM** | L-1, L-2, L-3 | — | `openpfe-llm` + UI LLM handlers done |
| **Wiring** | S-1, S-2, S-3 | — | LLM slice wired |
| **WebUI scope** | W-4 | W-1–W-3 (informative) | N/A |
| **Stale cross-docs** | UI orchestration | graph design, plans README | N/A |

**Closed since last review:** G-1, G-2, S-1, S-3, W-4; **`server.json` removed from human HTTP** (IPC admin — [openpfe-ipc/specification.md](../openpfe-ipc/specification.md)); **FR-6.5**, **FR-6.6**, **FR-6.7**, **FR-6.8** in [requirements.md](./requirements.md).

**Biggest remaining doc gaps (human API):** G-6 (`/graph/validate` JSON), G-3 (`/graph/nodes/:id/edges`), G-7 (subgraph query defaults), G-4 (`offset`), G-5 (`/graph/overview` algorithm). **Node CRUD** (`POST`/`PATCH`/`GET` node) still route-table-only; fold into next spec pass or a dedicated **G-9**.

**Blocks implementation:** S-2 **closed** for LLM slice — [008-openpfe-server.md](../../plans/008-openpfe-server.md) wires graph + `AppState`; graph HTTP routes still pending.

---

## What still aligns

- Three-slice split: `openpfe-ui` HTTP (graph + LLM); `openpfe-server` mount + `AppState` + IPC admin; domain in `openpfe-graph` / `openpfe-llm`.
- **`AppState`:** graph + LLM + shared `McpHandler` — [specification.md#appstate-v1](./specification.md#appstate-v1).
- **MCP dual transport:** `openpfe-mcp` handler; IPC for agents, `POST /debug/mcp` for Web UI Debug — [specification.md#mcp-debug-json-rpc](./specification.md#mcp-debug-json-rpc), FR-6.8.
- **`server.json`:** IPC admin (`server_config_get` / `server_config_put`) — not HTTP, not MCP.
- LLM route **table** matches [openpfe-llm/specification.md](../openpfe-llm/specification.md).
- v1 cross-cutting: REST + poll, JSON only, no CORS, no auth, `reload_engine` after load-affecting LLM changes.

---

## Closed — docs (implementation pending)

### G-1. Search APIs — **closed (docs)**

[specification.md#search-and-similarity](./specification.md#search-and-similarity), FR-6.5. MCP `openpfe_graph_find_similar` still deferred.

### G-2. Edge delete identity — **closed (docs)**

[specification.md#edges](./specification.md#edges), FR-6.6. Composite `(src_id, dst_id, type)`; no edge UUID.

### S-1. `AppState` — **closed (docs)**

[specification.md#appstate-v1](./specification.md#appstate-v1), FR-6.7. `Arc<Mutex<dyn GraphStore + Send>>` + `Arc<dyn LlmService>` + `Arc<McpHandler>`.

### S-3. `server_config_put` runtime effects — **closed (docs)**

[openpfe-server/specification.md#serverjson-project-config](../openpfe-server/specification.md#serverjson-project-config). Out of `openpfe-ui` scope.

### L-2. No embedding endpoint — **closed (deferral)**

v1: omit `embedding` on `find_similar`; lexical + structural legs only — [specification.md](./specification.md) (find-similar), [design.md](./design.md). `/llm/embed` deferred with production embeddings.

### W-4. Debug MCP panel — **closed (docs)**

[specification.md#mcp-debug-json-rpc](./specification.md#mcp-debug-json-rpc), FR-6.8. `POST /debug/mcp` delegates to shared `McpHandler` (`openpfe-mcp`); same JSON-RPC as IPC `type: mcp`. Web UI: [openpfe-webui/assets/debug/specification.md](../openpfe-webui/assets/debug/specification.md).

---

## Critical — graph (post–007)

### G-3. `/graph/nodes/:id/edges` underspecified

MCP: `openpfe_graph_neighbors` with `edge_type` and `direction` filters.

UI: `/graph/nodes/:id/edges` — no filters, no response shape.

`GraphStore::neighbors` returns **node id strings** only; full `Edge` objects require internal traversal (as in `subgraph`). Spec should define response JSON and whether filters match MCP.

---

### G-4. `list_nodes` pagination — `offset` not in graph API

UI: `GET /graph/nodes` query includes `offset`.

`NodeFilter` has `node_type`, `cluster_id`, `limit` only — no `offset` (`crates/openpfe-graph/src/types.rs`).

**Options:** Drop `offset` from UI spec; or extend `NodeFilter` + `GraphStore`.

---

### G-5. `/graph/overview` undefined

Not a `GraphStore` method. “Counts by node `type`” is composable from `list_nodes`. **“Root problem ids”** is undefined in graph spec (e.g. no incoming `depends_on`? not `member_of` any cluster?).

Define algorithm or remove field from overview response.

---

### G-6. `/graph/validate` response shape missing

MCP normative: `{ "ok": true }` or `{ "cycles": [ … ] }` ([openpfe-mcp/specification.md](../openpfe-mcp/specification.md)).

UI spec: “cycle check” only. `GraphStore::validate_acyclic_deps` returns `Vec<Vec<String>>` (cycle paths as UUID lists).

Align human API with MCP or document intentional difference.

---

### G-7. Subgraph query params

MCP: optional `max_depth`, `max_nodes` with hard caps (`max_depth` ≤ 5, `max_nodes` ≤ 500).

UI: `GET /graph/clusters/:id/subgraph` — no query params.

If v1 is defaults-only, state explicitly (defaults: `max_depth=3`, `max_nodes=200` per [openpfe-graph/specification.md#traversal-limits-context-shield](../openpfe-graph/specification.md#traversal-limits-context-shield)).

---

### G-8. Contract retrieval — no HTTP equivalent

MCP: `openpfe_contract_get` (`from`/`to`/`type=interfaces`).

Architecture view needs contract read/edit ([openpfe-webui/assets/architecture/specification.md](../openpfe-webui/assets/architecture/specification.md)). UI has generic edge CRUD only.

Consider `GET /graph/contracts/...` or document that contract body is read via edge list/subgraph only in v1.

---

### G-9. Node CRUD wire shapes (new)

Route table lists `POST`/`PATCH`/`GET` `/graph/nodes` but no normative JSON (unlike [edges](./specification.md#edges) and [search](./specification.md#search-and-similarity)). Should mirror `Node` from `openpfe-graph` (`id`, `type`, properties).

---

## LLM

### L-1. Request/response bodies — **closed (docs)**

Normative HTTP JSON mirrors in [openpfe-llm/specification.md](../openpfe-llm/specification.md#http-exposure-v1) (2026-06-06, plan 008). `openpfe-ui` handlers reference those DTOs.

### L-3. `reload_engine` triggers — **closed (docs)**

[openpfe-llm/specification.md#reload_engine-triggers](../openpfe-llm/specification.md#reload_engine-triggers) — load vs persist-only fields enumerated.

---

## Server / wiring

### S-2. Server graph lifecycle — **closed (LLM slice)**

[008-openpfe-server.md](../../plans/008-openpfe-server.md): server opens `./.openpfe/graph/store/` at startup, builds `AppState`, mounts `api_router`. Graph HTTP routes remain a separate UI plan.

---

## WebUI product scope (informative)

From [openpfe-webui/assets/shared/design.md](../openpfe-webui/assets/shared/design.md) — not all covered by current graph schema or UI routes.

| ID | WebUI need | Gap |
|----|------------|-----|
| W-1 | **Dashboard** (specs, tasks, handled counts) | No `specification` / `task` node types; `/graph/overview` cannot satisfy FR-UI-2 as written. |
| W-2 | **Problem “handled”** | WebUI handled/unhandled vs graph `status` (`open`, `refined`, `done`) — mapping not documented. |
| W-3 | **Specification / Task views** | No node types, routes, or approval → task API. |
| W-4 | **Debug MCP panel** | **Closed** — `POST /debug/mcp` → shared `McpHandler` |
| W-1–W-3 | Dashboard, handled flag, spec/task views | Still open — product scope vs graph schema |

**Action:** Document which webui views are in vs out of v1 `openpfe-ui` API scope (W-1–W-3).

---

## Stale or thin docs

| Item | Issue | Status |
|------|--------|--------|
| [openpfe-graph/design.md](../openpfe-graph/design.md) | “phase 2 implementation” — plan 007 **Complete** | **Open** |
| [requirements.md](./requirements.md) | FR-6.5–6.7 added; still thin on general FR-6.2 / node CRUD | **Partial** |
| [plans/README.md](../../plans/README.md) | `openpfe-ui` graph routes plan **TBD**; LLM slice → [008-openpfe-ui.md](../../plans/008-openpfe-ui.md) | **Partial** |
| [design.md](./design.md) orchestration | Server config via IPC admin, not `AppState` | **Closed** |

---

## Suggested closure order

**Docs (next):**

1. **G-6** + **G-7** — quick wins; align validate/subgraph with MCP + graph defaults.
2. **G-4** — drop `offset` or extend `NodeFilter` (prefer drop for v1 unless WebUI needs it).
3. **G-3** + **G-9** — neighbors + node CRUD JSON.
4. **G-5** — overview algorithm or slim response.
5. **G-8** + **WebUI scope** (W-1–W-3) — architecture/contracts vs v1 deferral.
7. Stale: **graph design** “phase 2”, **plans README** `openpfe-ui` plan.

**Implementation (after docs or in parallel where clear):**

1. **S-2** — `openpfe-server` graph lifecycle + `AppState` construction.
2. **`openpfe-ui`** — handlers per closed route specs (G-1, G-2, S-1 first).

---

## Related

- [openpfe-graph/specification.md](../openpfe-graph/specification.md)
- [openpfe-llm/specification.md](../openpfe-llm/specification.md)
- [openpfe-server/specification.md](../openpfe-server/specification.md)
- [openpfe-ipc/specification.md](../openpfe-ipc/specification.md)
- [openpfe-mcp/specification.md](../openpfe-mcp/specification.md)
- [007-openpfe-graph.md](../../plans/007-openpfe-graph.md)
