# Desktop Bridge Operations

The DX CLI Bridge is the desktop operator surface for local DX Agents work. It is intentionally a control panel, not a second runtime: the bridge calls the same `dx-agents` CLI, reads the generated host contract, and stores only redacted operator telemetry under `target/host-telemetry`.

Use this page when you need to verify providers, recover gateway pairing, inspect cron delivery, or share a redacted status bundle.

## Daily Checks

Start with the bridge self-test and release-readiness panels.

| Panel | Healthy signal | Recovery |
| --- | --- | --- |
| Bridge self-test | IPC inventory is `ok`; repo, CLI, and host contract are found | Rebuild or point `DX_AGENTS_CLI` / `DX_AGENTS_REPO_DIR` at the active checkout |
| Release readiness | Score is high enough for the current batch; required docs and host contract exist | Follow the first `Next action` item before packaging |
| Compact status | Provider, gateway, memory, cron, and channel allowlist states are visible | Run `dx-agents status --compact --json` in a terminal for the same source data |
| Command history | Recent bridge actions appear without raw input paths | Regenerate the host contract if telemetry paths are missing |

Do not use a release build just to refresh the bridge. During active development, the debug CLI is the expected fast feedback path.

## Live Provider Smoke

The provider-health panel supports mock, dry-run, and live modes. Use dry-run first because it checks local profile shape without spending tokens.

Recommended flow:

1. Select `dry-run` in Provider health and run it for the active profile.
2. If dry-run is healthy, switch to `live` only when you need network/auth proof.
3. Use `Live smoke` for one minimal response check.
4. Review provider smoke history to compare the latest success, model, duration, and redacted output summary.

Equivalent CLI checks:

```powershell
dx-agents models health --mode dry-run --json
dx-agents models health --mode live --json
dx-agents agent -a dx -p groq --model llama-3.3-70b-versatile --message "Reply with exactly: dx-agents-provider-ok"
```

Secrets must stay in the shell environment or the OS secret store. The bridge redacts common token prefixes before display and before provider smoke history writes.

## Gateway Pairing Recovery

The compact status panel shows whether gateway pairing is required and how many paired tokens are configured. When pairing is required and no token is cached, the bridge enables `Pairing code`.

Recovery flow:

1. Start or restart the gateway.
2. Use `Pairing code` in the bridge or run the CLI command below.
3. Pair the requesting device or ACP bridge with the one-time code.
4. Refresh compact status and confirm paired-token count moved above zero.

```powershell
dx-agents gateway start
dx-agents gateway get-paircode --new
dx-agents gateway status
```

If the bridge cannot fetch a pairing code, confirm the gateway host, port, and path prefix in config, then rerun `dx-agents gateway status`.

## Cron Preview And History

Use cron preview before running scheduler work. It shows enabled jobs, disabled jobs, due-now count, continuation-like prompts, delivery mode, and next scheduled runs without executing anything.

Use cron history after scheduled work runs. It shows recent run status, failure count, slow-run count, duration, and bounded output preview.

Equivalent CLI checks:

```powershell
dx-agents cron preview --limit 5 --json
dx-agents cron history --limit 3 --json
dx-agents cron list
```

If cron history is empty but jobs exist, wait for the next scheduled run or trigger the intended scheduler path. If preview fails, verify the config/workspace path and confirm the cron database is readable.

## Redacted Status Exports

Use `Export diagnostics` when you need to share the bridge state with another operator or keep a support artifact for a continuation run. Exports are written under:

```text
target/host-telemetry/bridge-status-export-<timestamp>.json
```

The export includes:

- bridge self-test
- command history metadata
- provider health
- provider smoke history
- compact status
- cron preview and cron history
- continuation status

The bridge only opens files named `bridge-status-export-*.json` from `target/host-telemetry`. This prevents accidental arbitrary-file opens from the diagnostics panel.

Before sharing an export, quickly scan it for workspace-specific paths. Provider auth tokens should be redacted, but local paths can still reveal machine layout.

## Embedded Terminal Readiness

The bridge can display embedded PTY and embedded terminal-session readiness from the generated host contract. These panels are explanatory until all required gates are ready.

Keep Windows Terminal as the production route when any of these gates are missing:

- input forwarding
- resize propagation
- lifecycle cleanup
- renderer attachment
- media-session routing

When the host contract reports `embedded_pty.production_ready = true` and embedded terminal sessions are enabled, the bridge can move from route planning to actual in-app terminal lifecycle operations.

## Fast Verification

For bridge changes, prefer the focused desktop checks:

```powershell
cargo test -p dx-agents-desktop dx_cli -- --nocapture
cargo check -p dx-agents-desktop
```

For CLI surfaces used by the bridge, add the narrow CLI check that matches the changed command:

```powershell
cargo check -p dx-agents --bin dx-agents
```

Avoid repeated release builds during feature work. Run a release build only when the release-readiness panel and focused checks are stable.
