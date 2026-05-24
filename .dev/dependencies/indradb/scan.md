## cargo audit — 2026-05-24

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1098 security advisories (from /Users/nilspetzall/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (256 crate dependencies)
Crate:     bincode
Version:   1.3.3
Warning:   unmaintained
Title:     Bincode is unmaintained
Date:      2025-12-16
ID:        RUSTSEC-2025-0141
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0141
Dependency tree:
bincode 1.3.3
└── indradb-lib 5.0.0
    └── openpfe-graph-spike 0.1.0

warning: 1 allowed warning found
```

Run immediately after adding `indradb-lib` to `crates/openpfe-graph-spike/Cargo.toml` (no `cargo build` / `cargo test` before audit).
