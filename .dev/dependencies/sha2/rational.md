# sha2

## Need

SHA-256 verification after model download and before engine load (FR-8.5 / catalog `sha256` field) per [openpfe-llm/specification.md](../../crates/openpfe-llm/specification.md).

## Scope

**`openpfe-llm`** only.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.10` (resolve to latest 0.10.x) |
| **Features** | default (`sha2` algorithm) |

## Trade-off

- **Adopt:** Standard RustCrypto SHA-256; incremental hashing over download stream.
- **Cost:** Small transitive `digest` / `crypto-common` stack.
- **Re-implement:** Manual SHA-256 — error-prone, no benefit.

## Alternatives considered

- **`ring`** — heavier; TLS already uses `rustls` elsewhere.
- **`sha256` crate** — less common; `sha2` is idiomatic.
