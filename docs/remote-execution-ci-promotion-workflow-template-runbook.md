# Strict CI Workflow Template Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-workflow-template --json` or `dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run --json` reports `warning` or `blocked`.

## Safe Default Workflow

Keep the default workflow non-failing while evidence is still being retained:

```powershell
cargo metadata --no-deps --format-version 1
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack --json
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-template-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run --preset safe-default --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run-history --preset safe-default --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run-alerts --preset safe-default --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-release-notes --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-config-draft --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export --output .github/workflows/dx-agents-strict-ci-promotion.yml --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-artifact-review --artifact .github/workflows/dx-agents-strict-ci-promotion.yml --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist --artifact .github/workflows/dx-agents-strict-ci-promotion.yml --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-handoff --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-handoff-history --archive --json
```

## Explicit Failing Mode

Only add strict CI failure behavior after the workflow template, retained rollout pack history, and local dry-run are clear:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run --preset explicit-failing --allow-failure-mode --json
dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --fail-on-non-clear --json
```

## Required Preflight

Run the lightweight preflight before changing a hosted CI template:

```powershell
cargo metadata --no-deps --format-version 1
cargo check --all-targets
cargo test --bin dx-agents parity_verification -- --nocapture
```

## Recovery

1. Run the workflow template command and inspect `top_blocker` plus `next_action`.
2. Archive a retained rollout pack snapshot when history is empty or stale:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-history --archive --json
```

3. Archive two workflow template snapshots before relying on template-history drift:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-template-history --archive --json
```

4. Archive two dry-run snapshots for every preset you want to promote:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run-history --preset safe-default --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run-history --preset explicit-failing --allow-failure-mode --archive --json
```

5. Re-run rollout pack and dry-run alerts until command drift, stale evidence, and accidental failure-mode alerts are clear:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack-alerts --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-dry-run-alerts --preset safe-default --json
```

6. Export the release-note handoff and inspect `release_notes_state`, `top_blocker`, `top_alert`, and `next_action` before drafting hosted CI changes:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-release-notes --json
```

7. Export the promotion bundle and inspect `promotion_bundle_state`, `verifier_state`, retained evidence counts, and sanitized `next_action` before drafting hosted CI changes:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle --json
```

8. Export the config draft and inspect `config_draft_state`, `safe_default_command_id_count`, and `strict_failure_mode_included` before turning placeholder refs into hosted CI commands:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-config-draft --json
```

9. Archive promotion bundle/config-draft snapshots and clear history alerts before relying on the handoff across runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-promotion-bundle-alerts --json
```

10. Export the hosted CI handoff and inspect `handoff_state`, `top_blocker`, `promotion_bundle_history_state`, `promotion_bundle_alert_state`, and `safe_default_command_id_count` before converting placeholders into hosted CI commands:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff --json
```

11. Archive hosted CI handoff snapshots and clear hosted handoff alerts before trusting a generated hosted CI workflow across repeated runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-hosted-ci-handoff-alerts --json
```

12. Write the placeholder-only GitHub Actions skeleton to a repo-relative path. Review every `dx-command-id:*` placeholder locally before replacing comments with real command values:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export --output .github/workflows/dx-agents-strict-ci-promotion.yml --json
```

13. Review the committed workflow artifact against the generated placeholder-only skeleton before promotion:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-artifact-review --artifact .github/workflows/dx-agents-strict-ci-promotion.yml --json
```

Treat `missing_placeholders`, `missing_step_ids`, `raw_dx_command_detected`, and `strict_failure_mode_detected` as blockers for the default workflow artifact. The artifact review is metadata-only and does not export workflow bodies or raw command values.

14. Archive GitHub Actions export snapshots and clear export alerts before treating the committed workflow skeleton as stable across automation runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-export-alerts --json
```

15. Run the raw-command promotion checklist before suggesting `--allow-raw-command-values`. The checklist should only suggest unsafe raw-command mode when export history, export alerts, artifact review, hosted handoff alerts, safe-default dry-run evidence, dry-run history, dry-run alerts, placeholder-only export posture, and payload-free redaction are all clear:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist --artifact .github/workflows/dx-agents-strict-ci-promotion.yml --json
```

16. If the checklist is not clear, use the metadata-only explain command for a compact blocker list and next recovery action:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-explain --artifact .github/workflows/dx-agents-strict-ci-promotion.yml --json
```

17. Archive raw-command promotion checklist snapshots and clear checklist alerts before treating an unsafe-mode suggestion as stable across automation runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-checklist-alerts --json
```

18. Export the raw-command promotion maintainer handoff and inspect `handoff_state`, `top_blocker`, `top_alert`, and `operator_actions` before manually deciding whether raw command values are appropriate for a hosted workflow:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-handoff --json
```

19. Archive raw-command promotion handoff snapshots and clear handoff history alerts before trusting the maintainer packet across automation runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-handoff-history --archive --json
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-handoff-alerts --json
```

20. Export the raw-command promotion release-audit digest before a maintainer manually decides whether `--allow-raw-command-values` should ever be used. The digest combines the maintainer handoff, retained handoff history, handoff alerts, checklist alerts, verifier metadata, safe archive command, and payload-free redaction policy:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-release-audit-digest --json
```

21. Archive raw-command promotion release-audit digest snapshots before trusting the one-summary audit packet across repeated automation runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-release-audit-digest-history --archive --json
```

22. Clear raw-command promotion release-audit digest history alerts before treating the retained audit packet as stable:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-release-audit-digest-alerts --json
```

23. Export the final raw-command promotion operator packet before any manual unsafe-mode decision. The packet combines release-audit digest state, retained digest history, digest alerts, runbooks, and explicit placeholder-only unsafe-mode policy:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet --json
```

24. Archive raw-command promotion operator packet snapshots before relying on the final handoff across repeated automation runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet-history --archive --json
```

25. Clear raw-command promotion operator packet history alerts before treating the retained final handoff as stable:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet-alerts --json
```

26. Export the raw-command operator packet release checklist before maintainer signoff. The checklist keeps placeholder-only defaults visible, requires retained operator packet history and clear packet alerts, and lists explicit manual review steps before any unsafe raw-command mode:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet-release-checklist --json
```

27. Archive raw-command operator packet release-checklist snapshots before trusting maintainer signoff posture across repeated release runs:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet-release-checklist-history --archive --json
```

28. Clear raw-command operator packet release-checklist history alerts before treating retained signoff evidence as stable:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-workflow-github-actions-raw-command-promotion-operator-packet-release-checklist-alerts --json
```

29. Use the local dry-run before copying commands into CI. If `accidental_failure_mode_detected` is true, remove `--fail-on-non-clear` from the default workflow or pass `--allow-failure-mode` only for the explicit strict workflow.

The workflow template, template history, dry-run, retained dry-run history, alerts, release-note handoff, promotion bundle, config draft, retained promotion bundle history, promotion bundle alerts, hosted CI handoff, retained hosted CI handoff history, hosted handoff alerts, default GitHub Actions export, artifact review, retained GitHub Actions export history, export alerts, raw-command promotion checklist, raw-command promotion explanation, retained raw-command promotion checklist history, raw-command promotion checklist alerts, raw-command promotion maintainer handoff, retained raw-command promotion handoff history, raw-command promotion handoff history alerts, raw-command promotion release-audit digest, retained raw-command promotion release-audit digest history, raw-command promotion release-audit digest history alerts, raw-command promotion operator packet, retained raw-command promotion operator packet history, raw-command promotion operator packet history alerts, raw-command operator packet release checklist, retained raw-command operator packet release-checklist history, and raw-command operator packet release-checklist alerts are metadata-only. They export command counts, known template command ids, phases, opt-in booleans, runbook paths, snapshot counts, alert ids, action ids, placeholder refs, step ids, drift booleans, comments, guard booleans, promotion booleans, blocker ids, signal ids, verifier counts, manual unsafe-mode policy, maintainer review steps, and recovery actions only. Raw command values require `--allow-raw-command-values` and should stay out of the default workflow.
