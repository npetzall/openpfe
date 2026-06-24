# reqwest — verdict

- **Decision:** accept
- **Version:** `0.12` (`default-features = false`, `features = ["rustls-tls", "stream"]`)
- **Workspace placement:** `openpfe-llm` product dependency (reuses lock entry from `openpfe-server` dev-dep)
- **Audit:** exit 0; no new findings
- **Approved:** 2026-06-06 (human, [plan 008-intake](../../plans/008-intake.md))
