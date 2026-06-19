# Remote Execution CI Promotion Release Candidate Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json` reports a `warning` or `blocked` release-candidate state.

## Purpose

The release-candidate packet is the last metadata-only evidence bundle before strict CI opt-in. It refuses opt-in unless the strict checklist, retained strict-checklist history, strict-checklist alerts, strict-policy alerts, enforcement alerts, verifier metadata, and payload-free policy are all clear.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-checklist-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-strict-policy-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-enforcement-alerts --json`
- `dx-agents parity verify --json`

## Recovery

1. If `required_archive_command` is not `none`, run that command first.
2. If `strict_checklist` is not ready, clear the strict checklist before treating the batch as release-candidate evidence.
3. If `strict_checklist_history` is not ready, archive stable checklist snapshots until at least two fresh snapshots exist.
4. If `strict_checklist_alerts`, `strict_policy_alerts`, or `enforcement_alerts` are not ready, resolve the top alert and archive another clean snapshot at the affected layer.
5. If `verifier_metadata` is not ready, run `dx-agents parity verify --json` and resolve blocked checks.
6. If `payload_free_policy` is not ready, repair redaction before exposing release-candidate output to automation.

## Promotion Rule

Strict CI opt-in is allowed only when `strict_ci_opt_in_allowed` is `true`, `release_candidate_state` is `clear`, every release-candidate item is `ready`, retained strict-checklist history is stable, release-candidate verifier metadata is clear, and all redaction flags are closed.
