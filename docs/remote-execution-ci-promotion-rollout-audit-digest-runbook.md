# Remote Execution CI Promotion Rollout Audit Digest Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest --json` reports a `warning` or `blocked` digest state.

## Purpose

The rollout audit digest is the operator handoff packet for strict CI promotion. It summarizes opt-in policy, retained opt-in history, opt-in alerts, release-candidate evidence, verifier metadata, exit behavior, required archive command, safe command, runbooks, and next remediation action without exporting secrets or workspace paths.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest --json`
- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-history --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-alerts --json`
- `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json`
- `dx-agents parity verify --json`

## Recovery

1. If `required_archive_command` is not `none`, run that archive command first.
2. If `opt_in_policy` is not ready, keep strict CI failure mode disabled and repair the opt-in policy report.
3. If `opt_in_history` is not ready, archive stable opt-in policy snapshots until at least two fresh snapshots exist.
4. If `opt_in_alerts` is not ready, resolve the top alert and archive another clean opt-in policy snapshot.
5. If `release_candidate` is not ready, repair release-candidate evidence before treating the digest as a rollout handoff.
6. If `verifier_metadata` is not ready, run `dx-agents parity verify --json` and resolve blocked checks.
7. If `payload_free_policy` is not ready, repair redaction before exposing the digest to CI automation.

## Handoff Rule

Treat `rollout_ready=true`, `digest_state=clear`, `would_exit_code=0`, `effective_exit_code=0`, `required_archive_command=none`, clear alerts, and fresh retained history as the minimum condition before moving the strict CI opt-in command into a failing CI job.
