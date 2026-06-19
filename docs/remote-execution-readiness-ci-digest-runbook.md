# Remote Execution Readiness CI Digest Runbook

The readiness CI digest is the compact operator packet for CI promotion decisions. It combines the current readiness CI gate, retained CI gate history, and CI history alerts into one metadata-only report with a single state, score, top blocker, top alert, recommended exit code, latest safe command, and remediation hint.

## Commands

- `dx-agents parity release-gate-readiness-ci-digest --json`
- `dx-agents parity release-gate-readiness-ci --json`
- `dx-agents parity release-gate-readiness-ci-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-alerts --json`
- `dx-agents parity verify --json`

## States

- `clear`: current CI gate, retained history, alerts, exit-code policy, and payload-free policy are all ready.
- `warning`: at least one metadata-only signal needs fresh evidence, but no blocking regression is present.
- `blocked`: at least one signal recommends stopping promotion until remediation is complete.

## Remediation Order

1. Follow `top_blocker.command` first when present.
2. If the top blocker points to missing or stale history, archive a fresh redacted snapshot with `dx-agents parity release-gate-readiness-ci-history --archive --json`.
3. If the top alert points to CI history drift, inspect `dx-agents parity release-gate-readiness-ci-alerts --json`.
4. Re-run `dx-agents parity release-gate-readiness-ci-digest --json`.
5. Re-run `dx-agents parity verify --json` before committing automation-facing changes.

## Redaction Contract

The digest must never serialize secrets, node identities, gateway URLs, channel identities, delivery targets, prompt values, user command payloads, tool payloads, or workspace paths. Safe DX command names and relative runbook paths are allowed because they are operator metadata, not user payloads.
