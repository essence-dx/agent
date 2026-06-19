# Strict CI Rollout Pack Runbook

Use this runbook when `dx-agents parity release-gate-readiness-ci-promotion-rollout-pack --json` reports `warning` or `blocked`.

## Safe Default

Keep the default automation path non-failing until retained rollout evidence is stable:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-pack --json
dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest --json
```

## Explicit Failing Mode

Only wire strict CI failure behavior after the rollout pack is `clear`, retained audit digest history is fresh, and alert history is clear:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-opt-in-policy --fail-on-non-clear --json
```

## Recovery

1. Run the rollout pack and inspect `top_blocker` plus `next_action`.
2. If retained evidence is missing or stale, archive a fresh rollout audit digest snapshot:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-history --archive --json
```

3. Re-run the rollout audit digest alerts command until the alert state is clear:

```powershell
dx-agents parity release-gate-readiness-ci-promotion-rollout-audit-digest-alerts --json
```

4. Keep the safe default command in CI until the pack explicitly reports `strict_ci_opt_in_ready: true`.

The rollout pack is metadata-only. It exports states, counts, safe DX command names, runbook paths, and recovery hints only.
