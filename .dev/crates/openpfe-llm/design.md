# openpfe-llm — design

**Required v1** — local llama.cpp inference. Models from `USER_HOME/.openpfe/models`. **HTTP Web UI only** in v1 (`openpfe-ui` → `LlmEngine`); MCP does not call inference.

## Decisions

| Topic | Decision |
|-------|----------|
| **Workspace** | Required member in v1 (not deferred) |
| **Model path** | Resolved via `openpfe-core` registry |
| **Build (v1)** | **[`llama-cpp-2`](https://crates.io/crates/llama-cpp-2)** + **`llama-cpp-sys-2`** — compiles vendored llama.cpp via crate `build.rs`. **No** git submodule in the openpfe repo. |
| **GPU features** | Default: **`metal`** on macOS, CPU elsewhere. Optional crate features: `cuda`, `vulkan` for release/CI matrices — not required for dev spike. |
| **Thread model** | All inference on **`tokio::task::spawn_blocking`** — never block the async runtime on llama calls. |
| **Scheduler (v1)** | **Single-flight** — at most one inference job inside the process; concurrent requests get **503 busy** (HTTP) or JSON-RPC error `inference_busy` (if exposed later). **No queue** in v1. |
| **Server without model** | **Allowed** — server starts graph-only; `LlmEngine` stays unloaded until a valid model path exists; inference API returns **`model_not_loaded`**. |
| **Exposure** | **HTTP only in v1** (`openpfe-ui` routes). **Not** MCP tools in v1 ([openpfe-mcp/design.md](../openpfe-mcp/design.md)). |

## `LlmEngine` (internal)

| Responsibility | Notes |
|----------------|--------|
| `try_load(config)` | Load GGUF from registry path; idempotent |
| `complete(prompt, options)` | Sync generation; called from `spawn_blocking` |
| `status()` | `unloaded` \| `loading` \| `ready` \| `error` |

Options map from `[llm]` config: `n_ctx`, `n_threads`, temperature, max_tokens (spec in [specification.md](./specification.md)).

## Integration

- `openpfe-server` holds `Arc<LlmEngine>` (or `Option`) and passes to `openpfe-ui` router state.
- Reload model when project `[llm].model` changes (HTTP PUT) — unload previous handle before load.

## Spike (before merge)

- Confirm `llama-cpp-2` builds on macOS + Linux CI with CPU-only feature set.
- Measure cold load time for a small GGUF.

## Related

- [openpfe-core/design.md](../openpfe-core/design.md) — model paths
- [openpfe-ui/specification.md](../openpfe-ui/specification.md) — HTTP inference routes
