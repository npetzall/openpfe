# nanograph — verdict

- **Decision:** accept
- **Version:** `1.3.0` (default features)
- **Workspace placement:** `openpfe-graph-spike` only
- **Audit:** `cargo audit` reports **RUSTSEC-2023-0071** (`rsa` via `lance`/`opendal`) — no fixed upgrade; **accepted for spike** with transitive risk documented. Warnings: `paste`, `rustls-pemfile` unmaintained via Lance stack.
- **Build:** requires **`protoc`** on PATH for first compile
- **Approved:** 2026-05-24 (implementation request — spike branch)
