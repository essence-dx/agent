# Remote Execution CI Promotion Opt-In Policy History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-alerts --json` reports a warning or blocked alert.

## Purpose

Opt-in policy alerts turn retained snapshots into a small automation-safe signal. They catch missing history, stale snapshots, release-candidate regressions, opt-in allowance regressions, would-exit regressions, effective-exit drift, command drift, and stable-history confirmation without exporting secrets or workspace paths.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-history --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json`

## Recovery

1. For `empty_history` or `single_snapshot`, run the opt-in policy history archive command until at least two snapshots exist.
2. For `stale_snapshot`, archive a fresh opt-in policy snapshot.
3. For release-candidate or opt-in allowance regressions, repair the current opt-in policy report before archiving another snapshot.
4. For `would_exit_regression`, resolve blockers until the would-exit code returns to `0`.
5. For `effective_exit_drift`, keep default CI behavior non-failing unless strict failure mode has been intentionally enabled.
6. For command drift, review the changed safe/archive command and archive another stable snapshot after confirming it is expected.

## Clear State

The alert report is clear only when retained opt-in policy history is fresh, has at least two stable snapshots, release-candidate state remains clear, opt-in allowance remains true, would-exit and effective-exit codes remain `0`, and command metadata is stable.
