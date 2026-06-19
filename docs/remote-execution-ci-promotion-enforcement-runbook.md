# Remote Execution CI Promotion Enforcement Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-enforcement --json` reports a `warning` or `blocked` enforcement state.

## Purpose

The enforcement gate is the final metadata-only go/no-go packet before any future CI policy turns readiness promotion findings into a strict process exit. It combines the current promotion alert digest, retained digest history, digest history alerts, strict exit-code policy, and payload-free policy without exporting secrets, identities, gateway URLs, prompts, user command payloads, tool payloads, delivery targets, or workspace paths.

## Commands

- `dx-agents parity release-gate-readiness-ci-promotion-enforcement --json`
- `dx-agents parity release-gate-readiness-ci-promotion-alert-digest --json`
- `dx-agents parity release-gate-readiness-ci-promotion-alert-digest-history --archive --json`
- `dx-agents parity release-gate-readiness-ci-promotion-alert-digest-history-alerts --json`
- `dx-agents parity verify --json`

## Recovery

1. If `required_archive_command` is not `none`, run that command until at least two redacted digest snapshots exist.
2. If the top blocker is `current_promotion_alert_digest`, resolve the digest next action and rerun the enforcement command.
3. If the top blocker is `retained_digest_history`, archive fresh digest snapshots and verify the history state becomes stable.
4. If the top blocker is `digest_history_alerts`, inspect the top alert and clear the retained-history regression.
5. If the top blocker is `strict_exit_policy`, clear every upstream recommended exit code before enabling strict CI behavior.
6. If the top blocker is `payload_free_policy`, repair redaction metadata before exposing the packet to automation.

## Clear State

A clear enforcement packet has `enforcement_state` set to `clear`, `recommended_exit_code` set to `0`, all five gates ready, `required_archive_command` set to `none`, and redaction flags closed.
