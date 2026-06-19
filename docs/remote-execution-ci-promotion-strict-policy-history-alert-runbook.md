# Remote Execution CI Promotion Strict Policy History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-alerts --json` reports a `warning` or `blocked` alert state.

## Purpose

Strict-policy history alerts turn retained strict CI dry-run snapshots into one metadata-only regression stream. They prove the strict policy is stable across runs before default nonzero CI behavior is promoted.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist --json`
- `dx-agents parity verify --json`

## Recovery

1. If `empty_history` or `single_snapshot` is triggered, archive enough strict-policy snapshots with `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-history --archive --json`.
2. If `stale_snapshot` is triggered, archive a fresh strict-policy snapshot before promotion.
3. If `strict_policy_state_regression` is triggered, inspect the current strict policy and clear the top blocker.
4. If `would_exit_regression` is triggered, keep strict CI behavior disabled until `would_exit_code` returns to `0`.
5. If `effective_exit_regression` is triggered, remove accidental default nonzero behavior and keep strict mode opt-in only.
6. If `blocking_reason_changed` is triggered, review the new blocker before relying on retained strict-policy evidence.
7. If command drift is triggered, review the latest safe command or required archive command before exposing strict mode to automation.

## Clear State

A clear alert packet has `alert_state` set to `clear`, `alert_count` set to `0`, at least two retained snapshots, a fresh latest snapshot, stable strict-policy state, `would_exit_code` equal to `0`, `effective_exit_code` equal to `0`, and closed redaction flags.
