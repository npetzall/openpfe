# grafeo — verdict

- **Decision:** accept
- **Version:** `0.5.42` (`default-features = false`, `features = ["lpg", "text-index", "vector-index", "hybrid-search", "parallel"]`)
- **Workspace placement:** `openpfe-graph` (product adapter); spike intake extended — [grafeo/specification.md](../../crates/openpfe-graph/grafeo/specification.md)
- **Audit:** exit 0; transitive `bincode` 2.0.1 unmaintained (RUSTSEC-2025-0141) via Grafeo — **accepted** with upstream risk documented; not fixable in openpfe
- **Clippy:** `cargo clippy -p openpfe-graph -- -D warnings` clean (2026-06-06)
- **Approved:** 2026-05-24 (human, spike); product scope on `openpfe-graph` confirmed 2026-06-06 ([plan 007](../../plans/007-openpfe-graph.md))
