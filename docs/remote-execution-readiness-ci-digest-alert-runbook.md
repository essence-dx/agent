# Remote Execution Readiness CI Digest Alert Runbook

Readiness CI digest alerts turn retained digest history into one compact regression stream for automation and operators. They should be used after `dx-agents parity release-gate-readiness-ci-digest-history --archive --json` has produced at least two redacted snapshots.

## Commands

- `dx-agents parity release-gate-readiness-ci-digest-alerts --json`
- `dx-agents parity release-gate-readiness-ci-digest-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-digest --json`
- `dx-agents parity release-gate-readiness-ci --json`
- `dx-agents parity verify --json`

## Alert States

- `clear`: retained digest history is stable and fresh.
- `warning`: the history is missing, sparse, stale, or changed in a non-blocking way.
- `blocked`: digest state, recommended exit code, or blocked signal counts regressed.

## Remediation

1. Follow `top_alert.recovery_hint` when present.
2. If history is empty or single-snapshot, archive fresh digest snapshots before trusting drift.
3. If the recommended exit code regressed, run `dx-agents parity release-gate-readiness-ci-digest --json` and clear the top blocker.
4. If blocked or warning signal counts changed, inspect the digest signals and archive a clean snapshot after remediation.
5. Re-run `dx-agents parity verify --json` before using alerts as promotion evidence.

## Redaction Contract

The alert report may contain alert ids, states, counts, safe DX command names, relative runbook paths, and recovery hints. It must not serialize secrets, node identities, gateway URLs, channel identities, delivery targets, prompts, user command payloads, tool payloads, workspace paths, or retained snapshot absolute paths.
