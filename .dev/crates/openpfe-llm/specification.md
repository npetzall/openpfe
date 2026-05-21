# openpfe-llm — specification

## Model input

- Resolved path from `openpfe-core` registry: `$HOME/.openpfe/models/<model_id>/`
- Weights: **`.gguf`** (primary v1 format)
- Load only after `sha256` manifest matches on disk ([openpfe-core/specification.md](../openpfe-core/specification.md))

## Native dependency (v1)

| Crate | Role |
|-------|------|
| `llama-cpp-2` | Safe-ish Rust API over llama.cpp |
| `llama-cpp-sys-2` | FFI + build of llama.cpp sources |

Build requirements: **clang**, **cmake** (or crate-documented equivalent). Pin crate versions in workspace `Cargo.toml` at implementation time.

## Internal Rust API

```rust
pub enum LlmStatus { Unloaded, Loading, Ready, Error(String) }

pub struct CompleteOptions {
    pub max_tokens: u32,
    pub temperature: f32,
    // n_ctx / n_threads from merged config when not overridden
}

pub trait LlmEngine: Send + Sync {
    fn status(&self) -> LlmStatus;
    fn load(&self, model_path: &Path, config: &LlmConfig) -> Result<(), LlmError>;
    fn complete(&self, prompt: &str, opts: CompleteOptions) -> Result<String, LlmError>;
}
```

- **`complete` is blocking** — callers must use `spawn_blocking`.
- **Single-flight:** if `complete` is invoked while another call is active, return `LlmError::Busy`.

## HTTP exposure (v1)

Implemented in **`openpfe-ui`**, delegated to `LlmEngine`:

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/llm/status` | `{ "status", "model_id", "error" }` |
| `POST` | `/llm/complete` | Body: `{ "prompt", "max_tokens?", "temperature?" }` → `{ "text" }` |

When model not loaded: **503** + `{ "error": { "code": "model_not_loaded", "message": "…" } }`.

When inference busy: **503** + `code: inference_busy`.

**Not exposed over MCP in v1.**

## Config mapping

From merged `[llm]` ([openpfe-core/specification.md](../openpfe-core/specification.md)):

| Key | Default | Maps to |
|-----|---------|---------|
| `model` | — | catalog id → path |
| `n_ctx` | `4096` | context size |
| `n_threads` | `0` (auto) | llama.cpp threads |
| `temperature` | `0.7` | completion (if not in request body) |
| `max_tokens` | `1024` | cap per request |

## Related

- [design.md](./design.md) — build and scheduler
- [openpfe-ui/specification.md](../openpfe-ui/specification.md)
