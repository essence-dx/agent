# Remote Execution CI Promotion Opt-In Policy Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --json` reports a `warning` or `blocked` opt-in policy state.

## Purpose

The opt-in policy is the controlled CI behavior layer above release-candidate evidence. It keeps default execution metadata-only and non-failing, then exposes `--fail-on-non-clear` as the explicit path for future strict CI failure mode.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --fail-on-non-clear --json`
- `dx-agents parity release-gate-readiness-ci-promotion-release-candidate --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy-alerts --json`
- `dx-agents parity verify --json`

## Recovery

1. If `release_candidate_state` is not `clear`, run the `latest_safe_command` and fix the release-candidate blocker first.
2. If `required_archive_command` is not `none`, run that archive command before enabling strict CI behavior.
3. If `would_exit_code` is nonzero, keep strict CI failure mode disabled until release-candidate evidence is clear.
4. If `effective_exit_code` is nonzero, remove accidental opt-in or resolve the blocker before using the command in CI.
5. If `payload_free_policy` is not ready, repair redaction before exposing the report to automation.

## Rollout Rule

Use the default command in local automation and dashboards. Use `--fail-on-non-clear` only in a CI job that explicitly wants strict failure behavior and only after retained opt-in policy history and alerts are clear.
