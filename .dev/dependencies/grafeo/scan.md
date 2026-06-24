## cargo audit — 2026-05-24

Run immediately after `openpfe-graph-spike` manifest + `Cargo.lock` update (`grafeo@0.5.42`, `lpg` only). No `cargo build` / `check` / `test` before this run.

```
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1098 security advisories (from /Users/nilspetzall/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (260 crate dependencies)
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
│           └── openpfe-graph-spike 0.1.0
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

Exit code **0**. Transitive **`bincode` unmaintained** (RUSTSEC-2025-0141) via Grafeo — not fixable in openpfe without upstream; note in human review / `verdict.md`.

## cargo audit — 2026-05-24 (text-index feature)

After adding `text-index` to `openpfe-graph-spike` manifest: exit **0**, same `bincode` allowed warning (260+ deps).

## cargo audit — 2026-06-06

Run after `grafeo@0.5.42` on product crate `openpfe-graph` (`lpg`, `text-index`, `vector-index`, `hybrid-search`, `parallel`).

Exit code **0**. Same transitive **`bincode` unmaintained** (RUSTSEC-2025-0141) via Grafeo — accepted per `verdict.md` (not fixable in openpfe). **269** crate dependencies scanned.
