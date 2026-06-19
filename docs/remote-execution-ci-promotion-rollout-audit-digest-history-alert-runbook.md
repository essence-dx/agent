# Remote Execution CI Promotion Rollout Audit Digest History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-alerts --json` reports a warning or blocked alert.

## Purpose

Rollout audit digest alerts turn retained handoff snapshots into concise automation-safe signals. They catch missing history, stale snapshots, digest regressions, rollout readiness regressions, opt-in policy regressions, release-candidate regressions, alert-state regressions, exit-code drift, command drift, and stable handoff history without exporting secrets or workspace paths.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-history --json`
- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-alerts --json`

## Recovery

1. For `empty_history` or `single_snapshot`, archive rollout audit digest snapshots until at least two snapshots exist.
2. For `stale_snapshot`, archive a fresh rollout audit digest snapshot.
3. For digest, rollout-ready, opt-in policy, release-candidate, or alert-state regressions, repair the current report before archiving another snapshot.
4. For `would_exit_regression`, resolve blockers until the would-exit code returns to `0`.
5. For `effective_exit_drift`, keep default CI behavior non-failing unless strict failure mode was intentionally enabled after a clear handoff.
6. For command drift, review the changed archive or safe command and archive another stable snapshot after confirming the change is expected.

## Clear State

The alert report is clear only when retained rollout audit digest history is fresh, has at least two stable snapshots, rollout-ready remains true, opt-in policy and release-candidate states remain clear, alert state remains clear, exit codes remain `0`, and command metadata is stable.
