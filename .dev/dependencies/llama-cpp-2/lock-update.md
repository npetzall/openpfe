# llama-cpp-2 — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh llama-cpp-2@0.1.146 --package openpfe-llm` (2026-06-06; lock already updated — diff vs `HEAD` `Cargo.lock`).

Applied manifest (`crates/openpfe-llm/Cargo.toml`):

```toml
llama-cpp-2 = "0.1.146"
```

**Direct:** `llama-cpp-2` 0.1.146, `llama-cpp-sys-2` 0.1.146.

**Notable transitive (llama-only vs rest of workspace at intake time):**

| Crate | Role |
|-------|------|
| `llama-cpp-2`, `llama-cpp-sys-2` | Direct + FFI |
| `cmake`, `cc`, `jobserver`, `shlex`, `find-msvc-tools` | Native build |
| `bindgen`, `clang-sys`, `cexpr`, `nom`, `prettyplease`, `rustc-hash` | FFI bindings generation |
| `encoding_rs`, `enumflags2`, `tracing` | `llama-cpp-2` runtime |
| `find_cuda_helper` | Build helper (CUDA path not enabled) |

Many build crates (`bindgen`, `cmake`, `regex`, …) overlap with **`grafeo`** (plan 007); **~28 packages** are unique to the `openpfe-llm` dependency tree when grafeo is already present.

Workspace dependency count for audit after full `openpfe-llm` manifest: **301** crate dependencies.
