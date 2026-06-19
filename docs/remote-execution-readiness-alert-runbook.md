# Remote Execution Readiness Alert Runbook

This runbook is for `dx-agents parity release-gate-readiness-alerts --json`.
The report is metadata-only: it contains alert ids, states, counts, safe DX
commands, and recovery hints. It must not include secrets, node identities,
gateway URLs, channel identities, delivery targets, prompts, user command
payloads, tool payloads, or workspace paths.

## Alert Response

1. If `empty_history` is triggered, run
   `dx-agents parity release-gate-readiness-history --archive --json` to retain
   the first redacted readiness snapshot.
2. If `single_snapshot` is triggered, archive one more readiness snapshot before
   trusting readiness drift automation.
3. If `blocked_component_regression` is triggered, resolve the reported
   readiness component first, then archive a clean readiness snapshot.
4. If `warning_component_regression` or `stale_source_regression` is triggered,
   refresh the underlying readiness evidence and archive a clean snapshot.
5. If `readiness_state_regression`, `top_signal_changed`, or
   `latest_safe_command_changed` is triggered, review the changed operator
   signal before using readiness history as promotion evidence.
6. If `stale_snapshot` is triggered, archive a fresh readiness snapshot before
   promoting or enabling any remote execution surface.

## Verification

Use the lightweight checks before relying on automation output:

```powershell
cargo test -p dx-agents --bin dx-agents remote_execution_operator_readiness_alerts
cargo test -p dx-agents --bin dx-agents parity_verify
cargo check -p dx-agents --bin dx-agents
```

Only run broader builds at a release checkpoint.
