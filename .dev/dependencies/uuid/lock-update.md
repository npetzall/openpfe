# uuid — lock-update

Resolution preview: `.dev/scripts/dependency-lock-diff.sh uuid@1 --package openpfe-llm` (2026-06-06).

Applied manifest:

```toml
uuid = { version = "1", features = ["v4"] }
```

**Direct:** `uuid` 1.23.2.

**Transitive:** `getrandom` (already in workspace lock). WASM-only deps (`js-sys`, `wasm-bindgen`) listed in lock for target unification; not used on native openpfe targets.

**Net new at intake time:** **`uuid`** crate entry.
