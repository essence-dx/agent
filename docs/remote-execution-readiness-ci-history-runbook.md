# Remote Execution Readiness CI History Runbook

This runbook is for `dx-agents parity release-gate-readiness-ci-history --json`.
History snapshots are written only when `--archive` is supplied, and only under
`target/remote-execution-readiness-ci`.

The report is metadata-only. It may include file names, timestamps, states,
counts, recommended exit-code metadata, top blocker ids, top alert ids, safe DX
command names, and recovery hints. It must not include secrets, node identities,
gateway URLs, channel identities, delivery targets, prompts, user command
payloads, tool payloads, or workspace paths.

## Response

1. If `snapshot_count` is `0`, run
   `dx-agents parity release-gate-readiness-ci-history --archive --json`.
2. If only one snapshot exists, archive another snapshot before trusting drift.
3. If `exit_code_stability` is blocked, resolve readiness CI gate blockers
   until the recommended exit code returns to `0`.
4. If `state_stability`, `top_signal_stability`, or `safe_command_stability`
   is warning, review the changed readiness signal and archive another clean
   snapshot.
5. If `staleness_gate` is warning, archive a fresh CI gate snapshot before
   using the history as release evidence.

## Lightweight Checks

```powershell
cargo test -p dx-agents --bin dx-agents remote_execution_operator_readiness_ci_gate_history
cargo test -p dx-agents --bin dx-agents parity_release_gate_readiness_ci_history_json_cli_parses
cargo test -p dx-agents --bin dx-agents parity_verification
cargo check -p dx-agents --bin dx-agents
```
