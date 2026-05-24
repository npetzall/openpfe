## cargo audit — 2026-05-24

Run from repo root immediately after `openpfe-graph-spike` manifest + `Cargo.lock` update (`sparrowdb@0.1.16`, `sparrowdb-execution@0.1.16`). No `cargo build` / `check` / `test` before this run.

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1098 security advisories (from /Users/nilspetzall/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (260 crate dependencies)
Crate:     bincode
Version:   1.3.3
Warning:   unmaintained
Title:     Bincode is unmaintained
Date:      2025-12-16
ID:        RUSTSEC-2025-0141
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0141
Dependency tree:
bincode 1.3.3
└── sparrowdb-execution 0.1.16
    ├── sparrowdb 0.1.16
    │   └── openpfe-graph-spike 0.1.0
    └── openpfe-graph-spike 0.1.0

warning: 1 allowed warning found
```

Exit code **0**. Transitive **`bincode` unmaintained** (RUSTSEC-2025-0141) via `sparrowdb-execution` — not fixable in openpfe without upstream; note in human review / `verdict.md`.
