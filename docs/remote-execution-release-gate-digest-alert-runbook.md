# Remote Execution Release Gate Digest Alert Runbook

Use this runbook when `dx-agents parity release-gate-digest-alerts --json` reports warnings or blockers. The alert report is metadata-only; it should not contain secrets, node identities, gateway URLs, prompts, tool payloads, delivery targets, or workspace paths.

## First Response

1. Run `dx-agents parity release-gate-digest-alerts --json`.
2. Follow `top_alert.recovery_hint` first when present.
3. If history is empty or single-snapshot, run `dx-agents parity release-gate-digest-history --archive --json` after the current digest is safe to retain.
4. If a blocked signal is present, inspect `dx-agents parity release-gate-digest --json`, fix the named signal, then archive a clean digest snapshot.
5. Re-run `dx-agents parity verify --json` before treating the remote execution surface as release-ready.

## Alert Guide

| Alert | Meaning | Action |
| --- | --- | --- |
| `empty_history` | No retained digest snapshots exist. | Archive the first digest snapshot. |
| `single_snapshot` | Drift cannot be trusted yet. | Archive one more clean snapshot. |
| `blocked_signal_regression` | The latest digest is blocked or newly blocked. | Resolve the top blocker and archive a clean snapshot. |
| `warning_signal_regression` | The latest digest still needs evidence. | Resolve warning signals and archive a clean snapshot. |
| `operator_state_regression` | The operator state moved away from clear. | Inspect the digest and address the state change. |
| `top_blocker_changed` | The main blocker changed. | Review the new blocker before relying on history. |
| `latest_safe_command_changed` | The recommended safe command changed. | Run the new command and compare the result. |
| `stale_snapshot` | The latest retained snapshot is too old. | Archive a fresh digest snapshot. |

Keep live remote execution disabled until digest alerts are clear and retained evidence is stable.
