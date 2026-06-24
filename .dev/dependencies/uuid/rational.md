# uuid

## Need

Opaque **`JobId`** for async model download jobs (`start_download` / `download_status`) per [openpfe-llm/specification.md](../../crates/openpfe-llm/specification.md).

## Scope

**`openpfe-llm`** only.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `1` (resolve to latest 1.x) |
| **Features** | `v4` (random UUIDs for job identifiers) |

## Trade-off

- **Adopt:** Standard UUID type + serde support available if needed later.
- **Cost:** Minimal; `getrandom` already in workspace transitive set.
- **Re-implement:** Ad-hoc random hex strings — weaker typing and collision handling.

## Alternatives considered

- **`ulid`** — sortable but not required for job polling.
- **Monotonic `u64` counter** — simpler but leaks job count; UUID is specified pattern for opaque IDs.
