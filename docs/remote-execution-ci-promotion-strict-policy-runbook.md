# Remote Execution CI Promotion Strict Policy Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-strict-policy --json` reports a `warning` or `blocked` strict policy state.

## Purpose

The strict policy command is the dry-run bridge between metadata-only readiness evidence and future CI failure behavior. By default it never exits nonzero; it reports `would_exit_code` and keeps `effective_exit_code` at `0`. CI jobs must opt in with `--fail-on-non-clear` before the command can return the reported nonzero exit code.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy --fail-on-non-clear --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-alerts --json`
- `dx-agents parity verify --json`

## Recovery

1. If `required_archive_command` is not `none`, run that command before relying on strict-policy evidence.
2. If `blocking_reason` is `enforcement_packet`, clear the current enforcement packet first.
3. If `blocking_reason` is `enforcement_history`, archive stable enforcement snapshots until the retained history is ready.
4. If `blocking_reason` is `enforcement_history_alerts`, resolve the top enforcement-history alert.
5. If `blocking_reason` is `strict_exit_dry_run_policy`, keep CI in dry-run mode until the would-exit code returns to `0`.
6. If `blocking_reason` is `payload_free_policy`, repair redaction before exposing strict-policy output to automation.

## Promotion Rule

Promote strict CI behavior only when `strict_policy_state` is `clear`, `would_exit_code` is `0`, retained enforcement history has at least two fresh stable snapshots, enforcement-history alerts are clear, and redaction flags are closed.
