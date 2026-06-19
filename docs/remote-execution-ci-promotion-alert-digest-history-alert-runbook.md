# Remote Execution CI Promotion Alert Digest History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-alert-digest-history-alerts --json` reports a `warning` or `blocked` alert state.

## Alert Packet

- Run `dx-agents parity release-gate-readiness-ci-promotion-alert-digest-history-alerts --json` for the compact metadata-only alert packet.
- Inspect `alert_state`, `top_alert`, `warning_alert_count`, `blocked_alert_count`, `snapshot_count`, and `next_action`.
- The alert packet is clear only when `alert_state` is `clear` and `alert_count` is `0`.

## Remediation Order

1. If `empty_history` or `single_snapshot` is triggered, archive enough digest snapshots with `dx-agents parity release-gate-readiness-ci-promotion-alert-digest-history --archive --json`.
2. If `recommended_exit_code_regression` or `blocked_signal_regression` is triggered, resolve the referenced digest blocker before enabling stricter CI promotion.
3. If state-change or command-change alerts are triggered, review the latest digest, archive another stable snapshot, and rerun `dx-agents parity verify --json`.
4. If `stale_snapshot` is triggered, archive a fresh digest snapshot before treating the history as release evidence.

## Safety Rules

- Keep the alert packet metadata-only; do not export provider keys, node identities, gateway URLs, prompts, user commands, tool payloads, delivery targets, or workspace paths.
- Do not treat warning or blocked digest-history alerts as promotion-ready evidence.
- Do not enable strict CI promotion until the alert packet is clear across retained snapshots.
