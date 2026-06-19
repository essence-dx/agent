# Strict CI Rollout Pack History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-alerts --json` reports `warning` or `blocked`.

## Required Checks

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack --json
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-history --json
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-alerts --json
```

## Recovery

1. If history is empty or has one snapshot, retain a fresh redacted rollout pack snapshot:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-history --archive --json
```

2. If the alert state is blocked, keep strict CI failure mode disabled and repair the first `top_alert`.
3. If command guidance drifted, keep the default command non-failing and keep nonzero CI behavior behind `--fail-on-non-clear`.
4. If `accidental_failure_mode_enabled` is triggered, remove any default command that can exit nonzero and archive a clean snapshot.
5. Re-run the alerts command until the state is `clear`.

The alert report is metadata-only. It exports alert ids, severities, counts, safe DX command names, and recovery hints only.
