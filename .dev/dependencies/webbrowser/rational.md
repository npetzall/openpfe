# webbrowser

## Need

Open default browser to `http_base_url` from echo on the default command (FR-2).

## Scope

`openpfe` binary crate only.

## Trade-off

Cross-platform `open` without shelling out to platform-specific commands in application code.

## Alternatives considered

- `xdg-open` / `open` via `std::process` — duplicated platform logic.
- No crate — rejected; browser launch is a v1 requirement.
