# openpfe-webui — requirements

**Product UI (views, user workflows):** [assets/README.md](./assets/README.md) (FR-UI-* per view). This file is **embed and build** only.

## FR-6 HTTP / static

- **FR-6.1** Serve embedded HTML/CSS/JS from binary.
- Modular **vanilla** front-end (multiple ES modules; **Vite** build in v1; output embedded from generated **`assets/`** / **`assets-dev/`** trees).
- **FR-6.2** Front-end build runs from **`openpfe-webui/build.rs`** during **`cargo build`**, aligned with Cargo **debug** vs **release** profile (see [specification.md](./specification.md#cargo--npm-build)).
- **FR-6.3** Front-end build may be **skipped** when **`OPENPFE_SKIP_WEBUI_BUILD=1`** or **`npm` is unavailable**, with a Cargo warning; embed may be empty until a successful build.

## Non-goals

- HTTP API → `openpfe-ui`
- Domain / graph logic
- UI frameworks (React, Vue, Svelte, etc.) in v1

## Related

- [assets/README.md](./assets/README.md) — product UI index
- [specification.md](./specification.md) — static route layout, Cargo/npm contract
- [design.md](./design.md) — Vite / Vitest / `build.rs` toolchain
