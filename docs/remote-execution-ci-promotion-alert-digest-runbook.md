# Remote Execution CI Promotion Alert Digest Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-alert-digest --json` reports a `warning` or `blocked` digest state.

## Digest Packet

- Run `dx-agents parity release-gate-readiness-ci-promotion-alert-digest --json` for the compact metadata-only promotion status packet.
- Inspect `digest_state`, `recommended_exit_code`, `top_blocker`, `top_alert`, `required_archive_command`, `latest_safe_command`, and `next_action`.
- The digest is promotion-ready only when `digest_state` is `clear` and `recommended_exit_code` is `0`.

## Remediation Order

1. If `required_archive_command` is not `none`, run that command before trusting retained history.
2. If `top_blocker` is present, run its `command` and follow its `recovery_hint`.
3. If `top_alert` is present, open the alert runbook listed in `runbooks`.
4. Rerun `dx-agents parity verify --json` after the digest returns to `clear`.

## Safety Rules

- Keep the digest metadata-only; do not export provider keys, node identities, gateway URLs, prompts, user commands, tool payloads, delivery targets, or workspace paths.
- Do not enable strict CI promotion on a warning or blocked digest.
- Do not treat single-snapshot or stale history as promotion evidence.
