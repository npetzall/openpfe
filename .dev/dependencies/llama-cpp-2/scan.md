## cargo audit — 2026-06-06

Run immediately after `openpfe-llm` manifest + `Cargo.lock` update (`llama-cpp-2@0.1.146` and remaining intake deps in same batch). No `cargo build` / `check` / `test` before this run.

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
├── grafeo-engine 0.5.42
├── grafeo-core 0.5.42
│   ├── grafeo-engine 0.5.42
│   ├── grafeo-adapters 0.5.42
│   │   ├── grafeo-engine 0.5.42
│   │   └── grafeo 0.5.42
│   └── grafeo 0.5.42
├── grafeo-common 0.5.42
│   ├── grafeo-storage 0.5.42
│   ├── grafeo-engine 0.5.42
│   ├── grafeo-core 0.5.42
│   ├── grafeo-adapters 0.5.42
│   └── grafeo 0.5.42
└── grafeo-adapters 0.5.42

warning: 1 allowed warning found
```

**Note:** `llama-cpp-2` / `llama-cpp-sys-2` introduce **no** new audit findings. Unmaintained `bincode` is via **grafeo** (plan 007), pre-existing.
