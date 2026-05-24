# axum

## Need

Minimal loopback HTTP stub in `openpfe-server` (`GET /` → OK) and future router composition with `openpfe-ui` / `openpfe-webui`.

## Scope

`openpfe-server` only in phase 1 (stub router); workspace `[workspace.dependencies]` for later `openpfe-ui`.

## Trade-off

Workspace-standard HTTP stack per [architcture.md](../../architcture.md); hyper/tower directly would duplicate axum’s router ergonomics.

## Alternatives considered

- **hyper only** — more boilerplate for routing.
- **actix-web** — not workspace-aligned.
