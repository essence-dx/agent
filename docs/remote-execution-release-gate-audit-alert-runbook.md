# Remote Execution Release Gate Audit Alert Runbook

DX Agents release gate audit alerts are diagnostic-only. They do not enable live remote execution, send channel messages, invoke remote nodes, or export node identities, gateway URLs, prompts, commands, delivery targets, workspace paths, or tool payloads.

## Alert States

- `clear`: retained release gate history is stable enough for the next release-gate audit check.
- `warning`: audit history is missing, too short, changed, or stale; refresh evidence before promotion work.
- `blocked`: audit drift shows newly blocked dependencies, payload-free regression, accidental live-execution enablement, approval regression, or rollback regression.

## Safe Commands

- `dx-agents parity release-gate --json`
- `dx-agents parity release-gate-history --archive --json`
- `dx-agents parity release-gate-history --json`
- `dx-agents parity release-gate-alerts --json`
- `dx-agents parity verify --json`

## Remediation

### `empty_history`

Run `dx-agents parity release-gate-history --archive --json` once to retain the first redacted release gate audit snapshot under `target/remote-execution-release-gate`.

### `single_snapshot`

Archive another release gate snapshot before relying on audit drift. Two snapshots are the minimum useful comparison.

### `gate_state_changed`

Review the release gate state change and archive another clean snapshot before promotion work. Treat unexpected state changes as release-risk evidence until they repeat as stable.

### `worsening_drift`

Resolve newly blocked release gate dependencies first. Do not enable production remote execution while the latest retained release gate has more blocked dependencies than the previous snapshot.

### `payload_free_regression`

Pause promotion work. Release gate evidence must stay payload-free. Inspect the release gate and history implementation before archiving new evidence.

### `live_execution_enabled_regression`

Disable live remote execution immediately. Release gate history must remain diagnostic-only until production remote execution is explicitly approved through a future release process.

### `approval_regression`

Restore approval readiness before using release gate audit history as promotion evidence.

### `rollback_regression`

Restore local-only rollback readiness before using release gate audit history as promotion evidence.

### `stale_snapshot`

Archive a fresh release gate snapshot, then rerun `dx-agents parity release-gate-alerts --json`.

## Promotion Rule

Production remote execution remains blocked unless `dx-agents parity release-gate-alerts --json` reports `alert_state: clear`, `dx-agents parity release-gate-history --json` reports stable retained audit history, and `dx-agents parity verify --json` passes.
