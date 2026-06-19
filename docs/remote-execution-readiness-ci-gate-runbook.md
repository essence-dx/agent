# Remote Execution Readiness CI Gate Runbook

This runbook is for `dx-agents parity release-gate-readiness-ci --json`.
The report is designed for CI and desktop automation that need one
metadata-only promotion packet instead of merging readiness, readiness history,
and readiness alert outputs by hand.

The command does not exit nonzero by default. Automation should inspect
`recommended_exit_code` and `strict_mode_hint` before enforcing a blocking CI
policy.

## Response

1. If `ci_state` is `blocked`, resolve `top_blocker` first and archive a clean
   readiness snapshot.
2. If `ci_state` is `warning`, inspect `readiness_state`, `history_state`, and
   `alert_state`, then run the `latest_safe_command` or the reported
   `next_action`.
3. If `recommended_exit_code` is `1`, keep remote execution promotion disabled
   until a fresh report returns `0`.
4. If `payload_free_policy` is not ready, do not export or attach the report to
   CI logs until redaction is repaired.

## Lightweight Checks

```powershell
cargo test -p dx-agents --bin dx-agents remote_execution_operator_readiness_ci_gate
cargo test -p dx-agents --bin dx-agents parity_release_gate_readiness_ci_json_cli_parses
cargo test -p dx-agents --bin dx-agents parity_verification
cargo check -p dx-agents --bin dx-agents
```
