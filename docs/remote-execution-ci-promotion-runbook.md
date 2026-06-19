# Remote Execution CI Promotion Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion --json` reports a `warning` or `blocked` promotion state.

## Promotion Packet

- Run `dx-agents parity release-gate-readiness-ci-promotion --json` for the final metadata-only promotion packet.
- Inspect `promotion_state`, `recommended_exit_code`, `top_blocker`, `top_alert`, `latest_safe_command`, and `required_archive_command`.
- The packet is promotion-ready only when `promotion_state` is `clear` and `recommended_exit_code` is `0`.

## Remediation Order

1. If `required_archive_command` is not `none`, run that archive command and then rerun the promotion packet.
2. If `top_blocker` is present, run its `command` and follow its `recovery_hint`.
3. If `top_alert` is present, inspect the alert runbook named in `runbooks`.
4. Rerun `dx-agents parity verify --json` before enabling strict CI use.

## Safety Rules

- Do not enable live remote execution from this packet alone.
- Do not store provider keys, node identities, gateway URLs, prompts, user commands, tool payloads, delivery targets, or workspace paths in CI artifacts.
- Keep strict CI enforcement separate from local operator review until the packet stays clear across retained digest history.
