# Remote Execution CI Promotion Enforcement History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-enforcement-alerts --json` reports a `warning` or `blocked` alert state.

## Purpose

Enforcement history alerts turn retained readiness CI promotion enforcement snapshots into one metadata-only regression stream. They help operators prove the final go/no-go packet is stable across runs before any strict CI exit policy is promoted.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement --json`
- `dx-agents parity verify --json`

## Recovery

1. If `empty_history` or `single_snapshot` is triggered, archive enough enforcement snapshots with `dx-agents parity release-gate-readiness-ci-promotion-enforcement-history --archive --json`.
2. If `stale_snapshot` is triggered, archive a fresh enforcement snapshot before promotion.
3. If `enforcement_state_regression` is triggered, inspect the current enforcement packet and clear the top blocker.
4. If `recommended_exit_code_regression` is triggered, keep strict CI behavior disabled until the recommended exit code returns to `0`.
5. If `blocked_gate_regression` or `warning_gate_regression` is triggered, resolve the reported gate drift and archive another clean snapshot.
6. If command drift is triggered, review the latest safe command or required archive command before exposing the gate to automation.

## Clear State

A clear alert packet has `alert_state` set to `clear`, `alert_count` set to `0`, at least two retained snapshots, a fresh latest snapshot, and closed redaction flags.
