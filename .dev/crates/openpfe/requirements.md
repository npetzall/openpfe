# openpfe (binary) — requirements

## Functional requirements

### FR-1 (client aspects)

- **FR-1.3** Second `openpfe` without subcommand does not start a second server; opens UI only (echo on `socket`, or failed lock + successful echo).
- **FR-1.5** Assume **cwd is project root**; if `./.openpfe/` missing, fail clearly unless command creates project.
- **FR-1.6** After echo succeeds, default command uses **`http_base_url` from echo** to open browser (no address file).
- **FR-1.7** If **`pid` flock fails** and echo fails, **retry echo** with backoff until configurable timeout.
- **FR-1.8** After timeout, exit non-zero with **diagnostics** (recorded PID, lock-holder, hint).

### FR-2 CLI — default (`openpfe`)

- **FR-2.1** If server not running: start detached server, wait until IPC echo succeeds (bounded timeout).
- **FR-2.2** Open default browser to server HTTP base URL.
- **FR-2.3** Parent exits; terminal returns to user.
- **FR-2.4** User never required to pick HTTP port in v1.

### FR-3 CLI — MCP (`openpfe mcp`)

- **FR-3.1** If server not running: start detached server, then attach.
- **FR-3.2** Expose MCP over process **stdio** to host.
- **FR-3.4** Multiple MCP clients via separate `openpfe mcp` invocations.

### FR-4 CLI — shutdown (`openpfe stop`)

- **FR-4.1** If server running: request graceful shutdown via IPC.
- **FR-4.2** If server not running: exit **0** with informative stderr message (idempotent stop).
- **FR-4.3** Expect runtime files under `.openpfe/server/` removed on clean shutdown (server responsibility).

### FR-5 CLI — LLM init (`openpfe llm init`)

- **FR-5.1** Read committed **`catalog.json`**; probe hardware via **`openpfe-llm`**; write gitignored **`llm.json`** with best **recommended ∩ capable** model.
- **FR-5.2** When weights not installed: **`ensure_server`**, IPC **`echo`** for **`http_base_url`**, HTTP **`POST /catalog/:id/download`**; stdout **`job_id`** and absolute **`status_url`**.
- **FR-5.3** **`openpfe llm download status <job_id>`** — poll download via echo + HTTP.
- **FR-5.4** Support **`--dry-run`**, **`--force`**, **`--no-download`** per [specification.md](./specification.md#llm-init).
- **FR-5.5** Do **not** overwrite existing **`llm.json`** without **`--force`**.

## Decisions (resolved)

| Topic | Decision |
|-------|----------|
| **v1 command set** | **In scope:** default, `mcp`, `stop`, **`llm init`**, **`llm download status`**, internal `--server`. **Out of v1:** full-project `init`, `status`, `logs`. |
| **Browser on headless** | Not required. Use `--no-browser` / `OPENPFE_NO_BROWSER=1` in CI; failed open after server is up → warn, exit 0. |
| **IPC versioning** | **No negotiation in v1** — fixed `"v": 1` per frame; explicit reject on mismatch. |

## Related

- [openpfe-server/requirements.md](../openpfe-server/requirements.md) — FR-1 server-side
- [openpfe-mcp/requirements.md](../openpfe-mcp/requirements.md) — FR-3.3 MCP semantics
- [cross-cutting.md](../../cross-cutting.md)
