# Grafeo — specification (openpfe mapping)

**Read when:** implementing or reviewing the `openpfe-graph` adapter.

Normative product schema: [../specification.md](../specification.md). Code constants: `crates/openpfe-graph/src/schema.rs`. Adapter behavior: [design.md](./design.md).

---

## Mental model

| Grafeo concept | openpfe concept | Notes |
|----------------|-----------------|-------|
| **Node label** | Node **`type`** | e.g. `problem`, `cluster`. Set at `create_node_with_props(&[label], …)`. Returned as `Node.node_type` in the API. **Not** stored as a duplicate property unless callers add one. |
| **Relationship name** | Edge **`type`** | e.g. `depends_on`, `member_of`, `interfaces`. |
| **Node property** | JSON field on node | `title`, `description`, `status`, `cluster_id`, `embedding`, … |
| **Edge property** | JSON field on edge | `contract_body`, `version`, … on `interfaces` edges |
| **Internal `NodeId`** | — | Adapter-only; never exposed on HTTP/MCP |
| **Property `id`** | Canonical **UUID** | Stable openpfe node id (v4 string). Indexed in adapter `id_to_node` map. |

Grafeo **labels** and **relationship names** use the **same strings** as openpfe `type` values (snake_case).

---

## Node labels (`type` → label)

| openpfe `type` | Grafeo label | v1 |
|----------------|--------------|-----|
| `problem` | `problem` | Required |
| `cluster` | `cluster` | Required |
| `component` | `component` | Optional |
| `contract` | `contract` | Optional (may be edge-only metadata early v1) |

---

## Relationship names (edge `type` → rel name)

| openpfe edge `type` | Grafeo relationship | Direction |
|---------------------|----------------------|-----------|
| `depends_on` | `depends_on` | directed |
| `member_of` | `member_of` | directed |
| `interfaces` | `interfaces` | directed |

---

## Properties (JSON → Grafeo `Value`)

### All nodes

| Property | Required | Indexed | Purpose |
|----------|----------|---------|---------|
| **`id`** | Yes | No (adapter map) | UUID string — openpfe stable id |
| `title` | problems/clusters | BM25 (`text-index`) | Short label |
| `description` | problems/clusters | BM25 | Long text |
| `status` | problems/clusters | No | `open`, `refined`, `done`, … |
| `cluster_id` | problems | No | Denormalized filter; must match `member_of` |

### `problem` only (vector / hybrid search)

| Property | Required | Indexed | Purpose |
|----------|----------|---------|---------|
| **`embedding`** | No | HNSW (`vector-index`) | `f32[]` from `openpfe-llm`; enables `hybrid_search` |

### `interfaces` edges

| Property | Purpose |
|----------|---------|
| `contract_body` | Contract text |
| `version` | Version string |
| `consumer_id` | Consumer node id |
| `provider_id` | Provider node id |

JSON arrays of numbers for `embedding` are stored as `Value::Vector`. Other nested JSON is serialized to string `Value` in v1.
