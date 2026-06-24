# reqwest

## Need

HTTPS model download (FR-8.5): stream catalog `url` to `<dir>/<filename>.partial`, enforce 32 GiB cap, then verify `sha256` per [openpfe-llm/specification.md](../../crates/openpfe-llm/specification.md).

## Scope

**`openpfe-llm`** (product dependency). `openpfe-server` already uses `reqwest` as a **dev-dependency** for integration tests — same crate/version, new product edge.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.12` (resolve to latest 0.12.x, e.g. `0.12.28`) |
| **Features** | `default-features = false`, `rustls-tls`, `blocking` |
| **Not used** | `stream` — downloads run on a dedicated `std::thread` with `reqwest::blocking` (no `tokio` in `openpfe-llm`) |

`rustls-tls` avoids OpenSSL; aligns with `openpfe-server` dev-dep choice.

## Trade-off

- **Adopt:** De-facto HTTP client; streaming body for large GGUF files.
- **Cost:** Transitive TLS stack already present in workspace lock from server tests.
- **Re-implement:** Raw `hyper` client is more boilerplate for little gain.

## Alternatives considered

- **`ureq`** — sync-only; awkward inside async download jobs without extra thread pool.
- **`hyper` direct** — lower level; reqwest is workspace-aligned via existing lock entry.
- **`curl` crate** — system libcurl dependency; rejected for portability.
