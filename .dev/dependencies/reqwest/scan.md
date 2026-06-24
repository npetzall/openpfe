## cargo audit — 2026-06-06

Run immediately after `openpfe-llm` manifest + `Cargo.lock` update (batch with `llama-cpp-2`, `sha2`, `uuid`). No `cargo build` / `check` / `test` before this run.

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1121 security advisories (from /Users/nilspetzall/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (301 crate dependencies)
Crate:     bincode
Version:   2.0.1
Warning:   unmaintained
Title:     Bincode is unmaintained
Date:      2025-12-16
ID:        RUSTSEC-2025-0141
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0141
Dependency tree:
bincode 2.0.1
├── grafeo-storage 0.5.42
│   └── grafeo-engine 0.5.42
│       └── grafeo 0.5.42
│           └── openpfe-graph 0.1.0
│               ├── openpfe-ui 0.1.0
│               └── openpfe-mcp 0.1.0
[... grafeo tree truncated ...]

warning: 1 allowed warning found
```

**Note:** `reqwest` product edge adds **no** new audit findings vs prior `openpfe-server` dev-dep lock entry.
