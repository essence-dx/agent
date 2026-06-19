# Remote Execution Canary Drift Alert Runbook

DX Agents keeps remote execution canary alerts diagnostic-only. The alert report never enables live remote tools, never sends channel messages, and never exports node identities, gateway URLs, prompts, commands, delivery targets, workspace paths, or tool payloads.

## Alert States

- `clear`: retained canary history is stable enough for the next release-gate check.
- `warning`: history is missing, too short, changed, or stale; refresh evidence before promotion work.
- `blocked`: drift shows newly blocked canary surfaces, payload-free regression, live invocation regression, or rollback regression.

## Safe Commands

- `dx-agents parity canary --json`
- `dx-agents parity canary-history --archive --json`
- `dx-agents parity canary-history --json`
- `dx-agents parity canary-alerts --json`
- `dx-agents parity verify --json`

## Remediation

### `empty_history`

Run `dx-agents parity canary-history --archive --json` once to retain the first redacted canary snapshot under `target/remote-execution-canary`.

### `single_snapshot`

Archive another dry-run canary snapshot before relying on drift trends. Two snapshots are the minimum useful comparison.

### `canary_state_changed`

Review the state change and archive another clean snapshot before promotion work. Treat unexpected changes as release-risk evidence until they repeat as stable.

### `worsening_drift`

Resolve newly blocked canary surfaces first. Do not enable live remote execution while the latest retained canary has more blocked surfaces than the previous snapshot.

### `payload_or_live_regression`

Pause promotion work. Canary evidence must remain dry-run, payload-free, and local-only. Inspect the canary implementation before archiving new evidence.

### `rollback_regression`

Restore local-only rollback readiness before continuing. Remote execution promotion remains blocked until rollback evidence is ready again.

### `stale_snapshot`

Archive a fresh dry-run canary snapshot, then rerun `dx-agents parity canary-alerts --json`.

## Promotion Rule

Remote execution can move toward a release gate only when `dx-agents parity canary-alerts --json` reports `alert_state: clear`, `dx-agents parity canary-history --json` reports stable retained history, and `dx-agents parity verify --json` passes.
