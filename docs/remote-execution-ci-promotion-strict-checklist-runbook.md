# Remote Execution CI Promotion Strict Checklist Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist --json` reports a `warning` or `blocked` checklist state.

## Purpose

The strict checklist is the final metadata-only promotion gate before strict CI behavior can be treated as production-ready. It refuses promotion unless enforcement evidence, retained enforcement history, enforcement alerts, the current strict policy, retained strict-policy history, strict-policy alerts, and payload-free redaction are all clear.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-alerts --json`
- `dx-agents parity verify --json`

## Recovery

1. If `required_archive_command` is not `none`, run that command first.
2. If `enforcement_packet` is not ready, clear the current enforcement packet before strict promotion.
3. If `enforcement_history` is not ready, archive stable enforcement snapshots until at least two fresh snapshots exist.
4. If `enforcement_history_alerts` is not ready, resolve the top enforcement alert and archive a clean enforcement snapshot.
5. If `strict_policy` is not ready, keep strict CI failure behavior disabled until `would_exit_code` and `effective_exit_code` are both `0`.
6. If `strict_policy_history` is not ready, archive stable strict-policy snapshots until at least two fresh snapshots exist.
7. If `strict_policy_history_alerts` is not ready, resolve the top strict-policy history alert and archive another clean strict-policy snapshot.
8. If `payload_free_policy` is not ready, repair redaction before exposing checklist output to automation.

## Promotion Rule

Promotion is allowed only when `promotion_allowed` is `true`, `checklist_state` is `clear`, every checklist item is `ready`, `required_archive_command` is `none`, retained strict-policy history is stable, strict-policy alerts are clear, and all redaction flags are closed.
