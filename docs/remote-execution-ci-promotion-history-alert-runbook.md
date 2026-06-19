# Remote Execution CI Promotion History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-history-alerts --json` reports a `warning` or `blocked` alert state.

## Alert Packet

- Run `dx-agents parity release-gate-readiness-ci-promotion-history-alerts --json` for the metadata-only promotion history alert packet.
- Inspect `alert_state`, `top_alert`, `blocked_alert_count`, `warning_alert_count`, and `next_action`.
- The alert packet is clear only when retained promotion history has at least two fresh, stable snapshots with no blocked or warning dependency regressions.

## Remediation Order

1. If `empty_history` or `single_snapshot` is triggered, run `dx-agents parity release-gate-readiness-ci-promotion-history --archive --json` until at least two snapshots exist.
2. If `recommended_exit_code_regression` or `blocked_dependency_regression` is triggered, resolve the current promotion blocker and archive a clean promotion snapshot.
3. If `required_archive_command_changed` is triggered, run the required archive command from the latest promotion packet, then archive a fresh promotion snapshot.
4. If only warning alerts remain, review the changed top signal or safe command and archive another stable snapshot.

## Safety Rules

- Keep this report metadata-only; do not export provider keys, node identities, gateway URLs, prompts, user commands, delivery targets, tool payloads, or workspace paths.
- Do not enable strict CI promotion on a single retained snapshot.
- Do not treat stale snapshots as release evidence.
