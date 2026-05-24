    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 1098 security advisories (from /Users/nilspetzall/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (609 crate dependencies)
Crate:     rsa
Version:   0.9.10
Title:     Marvin Attack: potential key recovery through timing sidechannels
Date:      2023-11-22
ID:        RUSTSEC-2023-0071
URL:       https://rustsec.org/advisories/RUSTSEC-2023-0071
Severity:  5.9 (medium)
Solution:  No fixed upgrade is available!
Dependency tree:
rsa 0.9.10
└── reqsign 0.16.5
    └── opendal 0.55.0
        ├── object_store_opendal 0.55.0
        │   └── lance-io 6.0.1
        │       ├── nanograph 1.3.0
        │       │   └── openpfe-graph-spike 0.1.0
        │       ├── lance-table 6.0.1
        │       │   ├── nanograph 1.3.0
        │       │   ├── lance-namespace-impls 6.0.1
        │       │   │   └── nanograph 1.3.0
        │       │   ├── lance-index 6.0.1
        │       │   │   ├── nanograph 1.3.0
        │       │   │   ├── lance-namespace-impls 6.0.1
        │       │   │   └── lance 6.0.1
        │       │   │       ├── nanograph 1.3.0
        │       │   │       └── lance-namespace-impls 6.0.1
        │       │   └── lance 6.0.1
        │       ├── lance-namespace-impls 6.0.1
        │       ├── lance-index 6.0.1
        │       ├── lance-file 6.0.1
        │       │   ├── nanograph 1.3.0
        │       │   ├── lance-table 6.0.1
        │       │   ├── lance-index 6.0.1
        │       │   └── lance 6.0.1
        │       └── lance 6.0.1
        └── lance-io 6.0.1

Crate:     paste
Version:   1.0.15
Warning:   unmaintained
Title:     paste - no longer maintained
Date:      2024-10-07
ID:        RUSTSEC-2024-0436
URL:       https://rustsec.org/advisories/RUSTSEC-2024-0436
Dependency tree:
paste 1.0.15
├── random_word 0.5.2
│   └── lance-datagen 6.0.1
│       ├── lance-index 6.0.1
│       │   ├── nanograph 1.3.0
│       │   │   └── openpfe-graph-spike 0.1.0
│       │   ├── lance-namespace-impls 6.0.1
│       │   │   └── nanograph 1.3.0
│       │   └── lance 6.0.1
│       │       ├── nanograph 1.3.0
│       │       └── lance-namespace-impls 6.0.1
│       └── lance-datafusion 6.0.1
│           ├── lance-index 6.0.1
│           └── lance 6.0.1
├── lance-bitpacking 6.0.1
│   └── lance-encoding 6.0.1
│       ├── lance-index 6.0.1
│       ├── lance-file 6.0.1
│       │   ├── nanograph 1.3.0
│       │   ├── lance-table 6.0.1
│       │   │   ├── nanograph 1.3.0
│       │   │   ├── lance-namespace-impls 6.0.1
│       │   │   ├── lance-index 6.0.1
│       │   │   └── lance 6.0.1
│       │   ├── lance-index 6.0.1
│       │   └── lance 6.0.1
│       └── lance 6.0.1
├── datafusion-physical-expr 53.1.0
│   ├── nanograph 1.3.0
│   ├── lance-index 6.0.1
│   ├── lance-datafusion 6.0.1
│   ├── lance 6.0.1
│   ├── datafusion-pruning 53.1.0
│   │   └── datafusion-physical-optimizer 53.1.0
│   │       └── datafusion 53.1.0
│   │           ├── lance-index 6.0.1
│   │           ├── lance-datafusion 6.0.1
│   │           └── lance 6.0.1
│   ├── datafusion-physical-plan 53.1.0
│   │   ├── nanograph 1.3.0
│   │   ├── lance 6.0.1
│   │   ├── datafusion-session 53.1.0
│   │   │   ├── datafusion-datasource-json 53.1.0
│   │   │   │   └── datafusion 53.1.0
│   │   │   ├── datafusion-datasource-csv 53.1.0
│   │   │   │   └── datafusion 53.1.0
│   │   │   ├── datafusion-datasource-arrow 53.1.0
│   │   │   │   └── datafusion 53.1.0
│   │   │   ├── datafusion-datasource 53.1.0
│   │   │   │   ├── datafusion-pruning 53.1.0
│   │   │   │   ├── datafusion-datasource-json 53.1.0
│   │   │   │   ├── datafusion-datasource-csv 53.1.0
│   │   │   │   ├── datafusion-datasource-arrow 53.1.0
│   │   │   │   ├── datafusion-catalog-listing 53.1.0
│   │   │   │   │   └── datafusion 53.1.0
│   │   │   │   ├── datafusion-catalog 53.1.0
│   │   │   │   │   ├── datafusion-functions-table 53.1.0
│   │   │   │   │   │   └── datafusion 53.1.0
│   │   │   │   │   ├── datafusion-catalog-listing 53.1.0
│   │   │   │   │   └── datafusion 53.1.0
│   │   │   │   └── datafusion 53.1.0
│   │   │   ├── datafusion-catalog 53.1.0
│   │   │   └── datafusion 53.1.0
│   │   ├── datafusion-pruning 53.1.0
│   │   ├── datafusion-physical-optimizer 53.1.0
│   │   ├── datafusion-functions-table 53.1.0
│   │   ├── datafusion-datasource-json 53.1.0
│   │   ├── datafusion-datasource-csv 53.1.0
│   │   ├── datafusion-datasource-arrow 53.1.0
│   │   ├── datafusion-datasource 53.1.0
│   │   ├── datafusion-catalog-listing 53.1.0
│   │   ├── datafusion-catalog 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-physical-optimizer 53.1.0
│   ├── datafusion-physical-expr-adapter 53.1.0
│   │   ├── datafusion-datasource 53.1.0
│   │   ├── datafusion-catalog-listing 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-optimizer 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-functions-window 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-functions-aggregate 53.1.0
│   │   ├── nanograph 1.3.0
│   │   ├── datafusion-functions-nested 53.1.0
│   │   │   ├── datafusion-sql 53.1.0
│   │   │   │   ├── lance-index 6.0.1
│   │   │   │   ├── lance-core 6.0.1
│   │   │   │   │   ├── lance-table 6.0.1
│   │   │   │   │   ├── lance-namespace-impls 6.0.1
│   │   │   │   │   ├── lance-namespace 6.0.1
│   │   │   │   │   │   ├── nanograph 1.3.0
│   │   │   │   │   │   ├── lance-namespace-impls 6.0.1
│   │   │   │   │   │   ├── lance-io 6.0.1
│   │   │   │   │   │   │   ├── nanograph 1.3.0
│   │   │   │   │   │   │   ├── lance-table 6.0.1
│   │   │   │   │   │   │   ├── lance-namespace-impls 6.0.1
│   │   │   │   │   │   │   ├── lance-index 6.0.1
│   │   │   │   │   │   │   ├── lance-file 6.0.1
│   │   │   │   │   │   │   └── lance 6.0.1
│   │   │   │   │   │   └── lance 6.0.1
│   │   │   │   │   ├── lance-linalg 6.0.1
│   │   │   │   │   │   ├── nanograph 1.3.0
│   │   │   │   │   │   ├── lance-namespace-impls 6.0.1
│   │   │   │   │   │   ├── lance-index 6.0.1
│   │   │   │   │   │   └── lance 6.0.1
│   │   │   │   │   ├── lance-io 6.0.1
│   │   │   │   │   ├── lance-index 6.0.1
│   │   │   │   │   ├── lance-file 6.0.1
│   │   │   │   │   ├── lance-encoding 6.0.1
│   │   │   │   │   ├── lance-datafusion 6.0.1
│   │   │   │   │   └── lance 6.0.1
│   │   │   │   └── datafusion 53.1.0
│   │   │   └── datafusion 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-datasource 53.1.0
│   ├── datafusion-catalog-listing 53.1.0
│   ├── datafusion-catalog 53.1.0
│   └── datafusion 53.1.0
├── datafusion-functions-window 53.1.0
├── datafusion-functions-table 53.1.0
├── datafusion-functions-nested 53.1.0
├── datafusion-functions-aggregate 53.1.0
├── datafusion-expr-common 53.1.0
│   ├── datafusion-pruning 53.1.0
│   ├── datafusion-physical-optimizer 53.1.0
│   ├── datafusion-physical-expr-common 53.1.0
│   │   ├── datafusion-pruning 53.1.0
│   │   ├── datafusion-physical-plan 53.1.0
│   │   ├── datafusion-physical-optimizer 53.1.0
│   │   ├── datafusion-physical-expr-adapter 53.1.0
│   │   ├── datafusion-physical-expr 53.1.0
│   │   ├── datafusion-functions-window-common 53.1.0
│   │   │   ├── datafusion-physical-plan 53.1.0
│   │   │   ├── datafusion-functions-window 53.1.0
│   │   │   └── datafusion-expr 53.1.0
│   │   │       ├── nanograph 1.3.0
│   │   │       ├── lance-index 6.0.1
│   │   │       ├── lance 6.0.1
│   │   │       ├── datafusion-sql 53.1.0
│   │   │       ├── datafusion-session 53.1.0
│   │   │       ├── datafusion-physical-plan 53.1.0
│   │   │       ├── datafusion-physical-optimizer 53.1.0
│   │   │       ├── datafusion-physical-expr-adapter 53.1.0
│   │   │       ├── datafusion-physical-expr 53.1.0
│   │   │       ├── datafusion-optimizer 53.1.0
│   │   │       ├── datafusion-functions-window 53.1.0
│   │   │       ├── datafusion-functions-table 53.1.0
│   │   │       ├── datafusion-functions-nested 53.1.0
│   │   │       ├── datafusion-functions-aggregate 53.1.0
│   │   │       ├── datafusion-functions 53.1.0
│   │   │       │   ├── lance-datafusion 6.0.1
│   │   │       │   ├── lance 6.0.1
│   │   │       │   ├── datafusion-physical-plan 53.1.0
│   │   │       │   ├── datafusion-physical-expr-adapter 53.1.0
│   │   │       │   ├── datafusion-functions-nested 53.1.0
│   │   │       │   └── datafusion 53.1.0
│   │   │       ├── datafusion-execution 53.1.0
│   │   │       │   ├── nanograph 1.3.0
│   │   │       │   ├── datafusion-session 53.1.0
│   │   │       │   ├── datafusion-physical-plan 53.1.0
│   │   │       │   ├── datafusion-physical-optimizer 53.1.0
│   │   │       │   ├── datafusion-functions-nested 53.1.0
│   │   │       │   ├── datafusion-functions-aggregate 53.1.0
│   │   │       │   ├── datafusion-functions 53.1.0
│   │   │       │   ├── datafusion-datasource-json 53.1.0
│   │   │       │   ├── datafusion-datasource-csv 53.1.0
│   │   │       │   ├── datafusion-datasource-arrow 53.1.0
│   │   │       │   ├── datafusion-datasource 53.1.0
│   │   │       │   ├── datafusion-catalog-listing 53.1.0
│   │   │       │   ├── datafusion-catalog 53.1.0
│   │   │       │   └── datafusion 53.1.0
│   │   │       ├── datafusion-datasource-json 53.1.0
│   │   │       ├── datafusion-datasource-csv 53.1.0
│   │   │       ├── datafusion-datasource-arrow 53.1.0
│   │   │       ├── datafusion-datasource 53.1.0
│   │   │       ├── datafusion-catalog-listing 53.1.0
│   │   │       ├── datafusion-catalog 53.1.0
│   │   │       └── datafusion 53.1.0
│   │   ├── datafusion-functions-window 53.1.0
│   │   ├── datafusion-functions-nested 53.1.0
│   │   ├── datafusion-functions-aggregate-common 53.1.0
│   │   │   ├── datafusion-physical-plan 53.1.0
│   │   │   ├── datafusion-physical-expr 53.1.0
│   │   │   ├── datafusion-functions-nested 53.1.0
│   │   │   ├── datafusion-functions-aggregate 53.1.0
│   │   │   └── datafusion-expr 53.1.0
│   │   ├── datafusion-functions-aggregate 53.1.0
│   │   ├── datafusion-expr 53.1.0
│   │   ├── datafusion-execution 53.1.0
│   │   ├── datafusion-datasource-json 53.1.0
│   │   ├── datafusion-datasource-csv 53.1.0
│   │   ├── datafusion-datasource-arrow 53.1.0
│   │   ├── datafusion-datasource 53.1.0
│   │   ├── datafusion-catalog-listing 53.1.0
│   │   └── datafusion 53.1.0
│   ├── datafusion-physical-expr 53.1.0
│   ├── datafusion-optimizer 53.1.0
│   ├── datafusion-functions-nested 53.1.0
│   ├── datafusion-functions-aggregate-common 53.1.0
│   ├── datafusion-functions 53.1.0
│   ├── datafusion-expr 53.1.0
│   └── datafusion 53.1.0
├── datafusion-expr 53.1.0
└── datafusion-common 53.1.0
    ├── nanograph 1.3.0
    ├── lance-index 6.0.1
    ├── lance-file 6.0.1
    ├── lance-datafusion 6.0.1
    ├── lance-core 6.0.1
    ├── datafusion-sql 53.1.0
    ├── datafusion-session 53.1.0
    ├── datafusion-pruning 53.1.0
    ├── datafusion-physical-plan 53.1.0
    ├── datafusion-physical-optimizer 53.1.0
    ├── datafusion-physical-expr-common 53.1.0
    ├── datafusion-physical-expr-adapter 53.1.0
    ├── datafusion-physical-expr 53.1.0
    ├── datafusion-optimizer 53.1.0
    ├── datafusion-functions-window-common 53.1.0
    ├── datafusion-functions-window 53.1.0
    ├── datafusion-functions-table 53.1.0
    ├── datafusion-functions-nested 53.1.0
    ├── datafusion-functions-aggregate-common 53.1.0
    ├── datafusion-functions-aggregate 53.1.0
    ├── datafusion-functions 53.1.0
    ├── datafusion-expr-common 53.1.0
    ├── datafusion-expr 53.1.0
    ├── datafusion-execution 53.1.0
    ├── datafusion-datasource-json 53.1.0
    ├── datafusion-datasource-csv 53.1.0
    ├── datafusion-datasource-arrow 53.1.0
    ├── datafusion-datasource 53.1.0
    ├── datafusion-catalog-listing 53.1.0
    ├── datafusion-catalog 53.1.0
    └── datafusion 53.1.0

Crate:     rustls-pemfile
Version:   2.2.0
Warning:   unmaintained
Title:     rustls-pemfile is unmaintained
Date:      2025-11-28
ID:        RUSTSEC-2025-0134
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0134
Dependency tree:
rustls-pemfile 2.2.0
└── object_store 0.12.5
    ├── object_store_opendal 0.55.0
    │   └── lance-io 6.0.1
    │       ├── nanograph 1.3.0
    │       │   └── openpfe-graph-spike 0.1.0
    │       ├── lance-table 6.0.1
    │       │   ├── nanograph 1.3.0
    │       │   ├── lance-namespace-impls 6.0.1
    │       │   │   └── nanograph 1.3.0
    │       │   ├── lance-index 6.0.1
    │       │   │   ├── nanograph 1.3.0
    │       │   │   ├── lance-namespace-impls 6.0.1
    │       │   │   └── lance 6.0.1
    │       │   │       ├── nanograph 1.3.0
    │       │   │       └── lance-namespace-impls 6.0.1
    │       │   └── lance 6.0.1
    │       ├── lance-namespace-impls 6.0.1
    │       ├── lance-index 6.0.1
    │       ├── lance-file 6.0.1
    │       │   ├── nanograph 1.3.0
    │       │   ├── lance-table 6.0.1
    │       │   ├── lance-index 6.0.1
    │       │   └── lance 6.0.1
    │       └── lance 6.0.1
    ├── nanograph 1.3.0
    ├── lance-table 6.0.1
    ├── lance-namespace-impls 6.0.1
    ├── lance-io 6.0.1
    ├── lance-index 6.0.1
    ├── lance-file 6.0.1
    ├── lance-core 6.0.1
    │   ├── lance-table 6.0.1
    │   ├── lance-namespace-impls 6.0.1
    │   ├── lance-namespace 6.0.1
    │   │   ├── nanograph 1.3.0
    │   │   ├── lance-namespace-impls 6.0.1
    │   │   ├── lance-io 6.0.1
    │   │   └── lance 6.0.1
    │   ├── lance-linalg 6.0.1
    │   │   ├── nanograph 1.3.0
    │   │   ├── lance-namespace-impls 6.0.1
    │   │   ├── lance-index 6.0.1
    │   │   └── lance 6.0.1
    │   ├── lance-io 6.0.1
    │   ├── lance-index 6.0.1
    │   ├── lance-file 6.0.1
    │   ├── lance-encoding 6.0.1
    │   │   ├── lance-index 6.0.1
    │   │   ├── lance-file 6.0.1
    │   │   └── lance 6.0.1
    │   ├── lance-datafusion 6.0.1
    │   │   ├── lance-index 6.0.1
    │   │   └── lance 6.0.1
    │   └── lance 6.0.1
    └── lance 6.0.1

error: 1 vulnerability found!
warning: 2 allowed warnings found
