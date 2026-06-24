# llama-cpp-2

## Need

Local GGUF inference for plan 008 ([openpfe-llm/specification.md](../../crates/openpfe-llm/specification.md) FR-8.4). `LlmService::complete` and `reload_engine` load weights from `$HOME/.openpfe/models/<id>/` via llama.cpp.

## Scope

**`openpfe-llm`** only. Transitive **`llama-cpp-sys-2`** (FFI + native build) is accepted as part of this intake.

## Version / features (intake)

| Item | Value |
|------|--------|
| **Version** | `0.1.146` (crates.io) |
| **License** | MIT OR Apache-2.0 |
| **Features (v1)** | **default** (`openmp` via `llama-cpp-sys-2`) |
| **Explicitly not enabled** | `cuda`, `metal`, `vulkan`, `rocm`, `dynamic-link`, `llguidance`, `mtmd` |

## Build prerequisites

| Prerequisite | Purpose |
|--------------|---------|
| **CMake** ≥ 3.14 | `llama-cpp-sys-2` builds llama.cpp via `cmake` crate |
| **C++ toolchain** | Native compile (clang++ / g++ / MSVC) |
| **clang + libclang** | `bindgen` for FFI headers (`clang-sys`) |
| **OpenMP** (default feature) | Linked when `openmp` feature enabled; macOS may need `libomp` (Homebrew) |

First `cargo build -p openpfe-llm` compiles llama.cpp — expect **long CI compile** and large `target/` artifacts. All `llama-cpp-sys-2` API surface is **`unsafe`**.

## MSRV

`llama-cpp-2` / `llama-cpp-sys-2` do not declare `rust-version`. Workspace **`rust-toolchain.toml`** uses **stable** (edition 2024); no conflict observed at resolution time.

## Trade-off

- **Adopt:** Mature llama.cpp stack with maintained Rust bindings (`utilityai/llama-cpp-rs`).
- **Cost:** Native build chain, `unsafe` FFI boundary, binary size, platform-specific OpenMP/Metal/CUDA optional features deferred.
- **Re-implement:** llama.cpp from scratch is out of scope.

## Alternatives considered

- **`candle` / `mistral.rs`** — different weight formats; GGUF + llama.cpp is specified.
- **`llama-cpp-rs` (older)** — superseded by `llama-cpp-2` ecosystem naming.
- **External inference server only** — rejected; v1 requires in-process engine per specification.
