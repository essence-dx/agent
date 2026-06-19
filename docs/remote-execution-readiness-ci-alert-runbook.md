# Remote Execution Readiness CI Alert Runbook

This runbook is for `dx-agents parity release-gate-readiness-ci-alerts --json`.
The report converts retained readiness CI gate history into metadata-only alert
signals for CI and desktop automation.

The report may include alert ids, severities, counts, state names, safe DX
command names, and recovery hints. It must not include secrets, node identities,
gateway URLs, channel identities, delivery targets, prompts, user command
payloads, tool payloads, or workspace paths.

## Response

1. If `empty_history` is triggered, run
   `dx-agents parity release-gate-readiness-ci-history --archive --json`.
2. If `single_snapshot` is triggered, archive one more CI gate snapshot before
   relying on drift alerts.
3. If `ci_state_regression` or `recommended_exit_code_regression` is blocked,
   resolve readiness CI gate blockers until the recommended exit code returns
   to `0`, then archive a clean snapshot.
4. If `readiness_signal_state_changed`, `top_signal_changed`, or
   `latest_safe_command_changed` is warning, review the changed signal and
   archive another stable snapshot.
5. If `stale_snapshot` is warning, archive a fresh readiness CI gate snapshot
   before using the alert report as release evidence.

## Lightweight Checks

```powershell
cargo test -p dx-agents --bin dx-agents remote_execution_operator_readiness_ci_gate_alerts
cargo test -p dx-agents --bin dx-agents parity_release_gate_readiness_ci_alerts_json_cli_parses
cargo test -p dx-agents --bin dx-agents parity_verification
cargo check -p dx-agents --bin dx-agents
```
