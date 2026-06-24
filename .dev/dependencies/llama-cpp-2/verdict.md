# llama-cpp-2 — verdict

- **Decision:** accept
- **Version:** `0.1.146` (default features; `openmp` via `llama-cpp-sys-2`)
- **Workspace placement:** `openpfe-llm` direct dependency
- **Audit:** exit 0; no new findings from llama stack; transitive `bincode` unmaintained via grafeo only (RUSTSEC-2025-0141)
- **Build:** CMake, C++ toolchain, clang/libclang required; first compile is slow — documented in `rational.md`
- **MSRV:** compatible with workspace `stable` toolchain at intake time
- **Approved:** 2026-06-06 (human, [plan 008-intake](../../plans/008-intake.md))
