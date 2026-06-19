# Remote Execution CI Promotion Strict Checklist History Alert Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-alerts --json` reports a `warning` or `blocked` alert state.

## Purpose

Strict-checklist history alerts turn retained strict CI promotion checklist snapshots into one metadata-only regression stream. They prove the final checklist stayed stable across runs before strict CI opt-in can be treated as a release candidate.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist --json`
- `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json`
- `dx-agents parity verify --json`

## Recovery

1. If `empty_history` or `single_snapshot` is triggered, archive enough strict-checklist snapshots with `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-history --archive --json`.
2. If `stale_snapshot` is triggered, archive a fresh strict-checklist snapshot before release-candidate promotion.
3. If `checklist_state_regression` is triggered, inspect the current strict checklist and clear the top blocker.
4. If `promotion_allowed_regression` is triggered, keep strict CI opt-in disabled until `promotion_allowed` returns to `true`.
5. If `blocked_item_regression` or `warning_item_regression` is triggered, resolve the changed checklist items and archive a clean snapshot.
6. If top-signal or command drift is triggered, review the new top blocker, top alert, safe command, or required archive command before relying on release-candidate evidence.

## Clear State

A clear alert packet has `alert_state` set to `clear`, `alert_count` set to `0`, at least two retained snapshots, a fresh latest snapshot, stable checklist state, `promotion_allowed` set to `true`, zero blocked items, zero warning items, and closed redaction flags.
