# openpfe-server — requirements

## Functional requirements

### FR-1 Single instance (server)

- **FR-1.1** At most one server process **per project** (`./.openpfe/`).
- **FR-1.2** Server enforces exclusivity with **non-blocking exclusive flock** on `./.openpfe/server/pid`; hold fd until exit.
- **FR-1.4** Stale **`socket`** after crash must not block new server (unlink when connect/echo fails); stale **lock** must not persist after process death.

### FR-6 HTTP (listener)

- **FR-6.5** Bind **`127.0.0.1` only** in v1; no HTTP or IPC auth tokens ([design.md](./design.md)).
- Mount **FR-6.1** static (`openpfe-webui`) and **FR-6.2–6.4** API (`openpfe-ui`).
- **FR-6.6** Do **not** apply CORS middleware in v1 (same-origin Web UI; non-browser clients).

### FR-1 shutdown (server)

- **FR-1.5** On shutdown (IPC, `openpfe stop`, SIGINT/SIGTERM): stop accept → drain in-flight work up to configurable timeout (default **5s**) → cancel remainder → remove `pid` and `socket`.
- **FR-1.6** Detached server logs to `./.openpfe/server/openpfe.log`; foreground mode logs to stderr.

## Non-functional

- Contribute to **NFR-1** cold start until echo; **NFR-2** clean runtime dir on shutdown.
- **NFR-3** v1 security model: loopback HTTP + project UDS path only — no optional tokens ([guidelines/protocols.md](../../guidelines/protocols.md)).

## Related

- [openpfe/requirements.md](../openpfe/requirements.md) — client FR-1.7–1.8
- [openpfe-webui/requirements.md](../openpfe-webui/requirements.md) — FR-6.1
- [openpfe-ui/requirements.md](../openpfe-ui/requirements.md) — FR-6.2–6.4
