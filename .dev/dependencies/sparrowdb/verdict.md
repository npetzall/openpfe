# sparrowdb — verdict

- **Decision:** accept (spike scope)
- **Version:** `0.1.16` — direct: `sparrowdb`, `sparrowdb-execution`
- **Workspace placement:** `openpfe-graph-spike` only
- **Audit:** exit 0; allowed warning: `bincode` 1.3.3 unmaintained (RUSTSEC-2025-0141) via `sparrowdb-execution` — **accepted for spike** with upstream risk documented; not fixable in openpfe
- **C++ / native:** Default build is pure Rust; no `librocksdb-sys` or similar in this intake batch
- **Approved:** 2026-05-24 (human — resume plan)

**Follow-ups:** Pin exact version in [spike-sparrowdb.md](../../crates/openpfe-graph/spike-sparrowdb.md) Results when spike closes; re-run `cargo audit` if lock changes; evaluate `0.1.22+` on crates.io when published.
