# Grafeo — requirements

**Read when:** updating `openpfe-graph/Cargo.toml`, running dependency intake, or bumping the engine version.

Engine lock: [../decision.md](../decision.md). Adapter design: [design.md](./design.md).

---

## Search features (v1)

Engine features below support **FR-9.6** / **FR-9.7** ([../requirements.md](../requirements.md)):

| Feature | S6 / S6+ use |
|---------|----------------|
| `text-index` | `search_problems`; lexical leg of `find_similar` |
| `vector-index` | HNSW on `problem.embedding` |
| `hybrid-search` | `find_similar` when draft includes `embedding` |
| `parallel` | Batch search performance |

Adapter algorithms: [design.md](./design.md). API shapes: [../specification.md](../specification.md#search-and-similarity-v1).

---

## Engine dependency (normative)

| Item | Value |
|------|--------|
| **Crate** | [`grafeo`](https://crates.io/crates/grafeo) **0.5.42** (pin exact version; bump only via intake) |
| **Features (v1)** | `default-features = false`, `features = ["lpg", "text-index", "vector-index", "hybrid-search", "parallel"]` |
| **Not enabled (v1)** | `embedded`, `ai`, `embed`, `rdf`, `enterprise`, `server`, `grafeo-mcp` |
| **MSRV** | Grafeo cites **1.91.1** — align workspace toolchain |
| **License** | Apache-2.0 |

---

## Intake

| Phase | Scope |
|-------|--------|
| Spike | Approved for `openpfe-graph-spike` — [.dev/dependencies/grafeo/](../../../dependencies/grafeo/) |
| Product | **New intake** required on `openpfe-graph` before implementation merge |

---

## Supply chain

- `cargo audit` exit 0 at intake; allowed warning: `bincode` 2.0.1 unmaintained via `grafeo-*` (RUSTSEC-2025-0141) — accepted for v1.
- ~38 transitive packages at spike intake (`grafeo-engine`, `grafeo-core`, `grafeo-storage`, …).

---

## Backup

Graceful server shutdown; copy **`./.openpfe/graph/`** (or `store/` subtree). Use Grafeo `backup_full` where applicable — see [../spike/grafeo-outcome.md](../spike/grafeo-outcome.md).

---

## Related

- [specification.md](./specification.md) — openpfe ↔ Grafeo mapping
- [../requirements.md](../requirements.md) — product FR-9 graph requirements
- [../spike/grafeo-outcome.md](../spike/grafeo-outcome.md) — spike evidence
