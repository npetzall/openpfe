# openpfe — development documentation

Design and planning for the Rust workspace. **Not** user-facing product docs (see [openpfe_tooling.md](../openpfe_tooling.md)).

## Layout

```
.dev/
  README.md                 ← you are here
  guidelines.md             # development guidelines hub
  guidelines/               # topic guides (Rust, JS, protocols, MCP, testing, …)
  plans/                    # implementation plans (index: plans/README.md)
  architcture.md            # system overview (diagrams, decisions index)
  cross-cutting.md          # multi-crate contracts, FR traceability, NFRs
  workspace-crates.md       # members, deps, phasing, documentation convention
  crates/
    <crate-name>/
      design.md             # authoritative for this crate
      requirements.md
      specification.md
    openpfe-graph/
      graph-db-evaluation.md
      graph-db-spike.md       # v1 spike program (shared bar)
      spike-indradb.md
      spike-grafeo.md
```

**Rule:** Normative design, requirements, and specs live under `crates/<name>/` only — see [workspace-crates.md#documentation-convention](./workspace-crates.md#documentation-convention).

**Agents:** Start at [guidelines.md](./guidelines.md) for strict rules and a path-indexed guideline list.

## Where to read what

| Question | Document |
|----------|----------|
| How should I implement / review code? | [guidelines.md](./guidelines.md) |
| How is the local service shaped? | [architcture.md](./architcture.md) |
| What spans multiple crates? | [cross-cutting.md](./cross-cutting.md) |
| Which Cargo crates and dependency rules? | [workspace-crates.md](./workspace-crates.md) |
| What order should I implement crates? | [plans/README.md](./plans/README.md) |
| Where do I document a crate change? | [workspace-crates.md#documentation-convention](./workspace-crates.md#documentation-convention) |
| Which graph DB engine? | [crates/openpfe-graph/graph-db-evaluation.md](./crates/openpfe-graph/graph-db-evaluation.md) |
| How do we spike graph engines for v1? | [crates/openpfe-graph/graph-db-spike.md](./crates/openpfe-graph/graph-db-spike.md) |
| Detail for one crate | [crates/&lt;name&gt;/](./crates/) (`design`, `requirements`, `specification`) |
