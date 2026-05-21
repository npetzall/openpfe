# openpfe-core — requirements

## FR-7 Configuration

- **FR-7.1** Load `USER_HOME/.openpfe/config.toml` when present.
- **FR-7.2** Merge `./.openpfe/config.toml` with project overriding user per [specification.md](./specification.md#merge-semantics-v1).
- **FR-7.3** Missing files are not errors; use defaults.
- **FR-7.4** Nested sections `[server]`, `[http]`, `[llm]` and `[[models.catalog]]` in `config.toml` only (no separate `models.toml` in v1).
- **FR-7.5** API/UI config writes update **project** `./.openpfe/config.toml` only.

## FR-8 Models (registry / paths)

- **FR-8.1** Store downloaded models under `USER_HOME/.openpfe/models/<id>/`.
- **FR-8.2** Models immutable on disk after verify; shared across projects.
- **FR-8.3** Config references models by stable **catalog `id`**; resolve path and validate before load.
- **FR-8.5** Download via **HTTPS** with **sha256** verification; atomic install (`*.partial` → rename). No Hugging Face CLI in v1.
- **FR-8.6** Support **user-provided** weights via catalog `path` or files placed under `models/<id>/`.

Implementation of **FR-8.4** inference: [openpfe-llm/requirements.md](../openpfe-llm/requirements.md).

## Related

- [specification.md](./specification.md) — config schema draft
- [cross-cutting.md](../../cross-cutting.md) — data placement index
