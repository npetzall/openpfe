## cargo audit — 2026-06-06

Run immediately after `openpfe-llm` manifest + `Cargo.lock` update (batch intake). No `cargo build` / `check` / `test` before this run.

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
[... grafeo tree ...]

warning: 1 allowed warning found
```

**Note:** `sha2` introduces **no** new audit findings.
