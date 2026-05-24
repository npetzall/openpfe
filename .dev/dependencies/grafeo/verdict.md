# grafeo — verdict

- **Decision:** accept
- **Version:** `0.5.42` (`default-features = false`, `features = ["lpg", "text-index"]`)
- **Workspace placement:** `openpfe-graph-spike` only
- **Audit:** exit 0; transitive `bincode` 2.0.1 unmaintained (RUSTSEC-2025-0141) via Grafeo — **accepted for spike** with upstream risk documented; not fixable in openpfe
- **Approved:** 2026-05-24 (human)
