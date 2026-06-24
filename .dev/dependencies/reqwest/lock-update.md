# reqwest — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh reqwest@0.12 --package openpfe-llm` (2026-06-06).

Applied manifest:

```toml
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls", "stream"] }
```

**Lock impact:** **0 new packages** — `reqwest` 0.12.x and its transitive TLS/`hyper` stack were already resolved for **`openpfe-server`** dev-dependency (`rustls-tls` only). Adding `stream` to the product edge reuses the existing lock entry; `tokio-util` / `wasm-streams` already present.

**New edge:** `openpfe-llm` → `reqwest` (product dependency).
