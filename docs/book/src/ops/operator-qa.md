# Operator QA Smoke Checklist

Use this checklist before a release package, a demo build, or a long continuation run. It is intentionally command-level and avoids release builds.

Run from the repo root unless a command says otherwise:

```powershell
Set-Location G:\Dx\agent
```

## 1. Baseline Status

Confirm the active batch, recent workloop entries, and compact runtime health:

```powershell
target\debug\dx-agents.exe workloop status --limit 5 --json
target\debug\dx-agents.exe status --compact --json
```

Pass conditions:

- The active batch and status score match `TODO.md`.
- The compact status JSON parses.
- No secret values appear in command output.

## 2. Provider Smoke

Start with deterministic checks, then use live mode only when a shell-local API key is already set:

```powershell
target\debug\dx-agents.exe models health --mode mock --json
target\debug\dx-agents.exe models health --mode dry-run --json
target\debug\dx-agents.exe agent -a dx -p groq --model llama-3.3-70b-versatile --message "Reply with exactly: dx-agents-provider-ok"
```

Pass conditions:

- Mock mode completes without network access.
- Dry-run mode resolves provider profiles without printing credentials.
- Live smoke returns the expected short response when credentials are present.

## 3. Gateway Pairing

Check gateway health and pairing-code recovery:

```powershell
target\debug\dx-agents.exe gateway status
target\debug\dx-agents.exe gateway get-paircode --new
```

Pass conditions:

- `gateway status` reports whether the server is reachable.
- `get-paircode --new` returns a pair code or a clear recovery error.
- The desktop bridge shows a remediation action when pairing is required.

## 4. Cron Delivery

Preview delivery and inspect run history without executing scheduled jobs:

```powershell
target\debug\dx-agents.exe cron preview --limit 5 --json
target\debug\dx-agents.exe cron history --limit 5 --json
target\debug\dx-agents.exe cron list
```

Pass conditions:

- Preview output is valid JSON.
- Run history summarizes failed and slow runs.
- `cron list` does not require modifying scheduled tasks.

## 5. Memory Roundtrip

Verify memory backend health and indexes:

```powershell
target\debug\dx-agents.exe memory stats
target\debug\dx-agents.exe memory list --limit 5
target\debug\dx-agents.exe memory reindex
```

Pass conditions:

- Stats load from the configured workspace.
- Listing works even when the memory store is empty.
- Reindex finishes or reports a specific backend recovery action.

## 6. Migration Dry Runs

Preview supported migration readers without writing data:

```powershell
target\debug\dx-agents.exe migrate openclaw --dry-run
target\debug\dx-agents.exe migrate hermes --dry-run
target\debug\dx-agents.exe migrate agent --dry-run
target\debug\dx-agents.exe migrate zeroclaw --dry-run
```

Pass conditions:

- Each command reports a dry-run summary or a clear missing-source message.
- No memory rows are written during dry runs.
- Legacy names are accepted without reintroducing old product branding in user-facing DX Agents copy.

## 7. Desktop Bridge

Check desktop bridge IPC, release readiness, redacted exports, and safe export review:

```powershell
cargo test -p dx-agents-desktop dx_cli -- --nocapture
cargo test -p dx-agents-desktop release_readiness_report_scores_local_release_inputs -- --nocapture
target\debug\dx-agents.exe workloop status --limit 3 --json
```

Then use the desktop bridge controls:

- Run bridge self-test.
- Run release readiness.
- Export redacted status.
- Open the latest `bridge-status-export-*.json` from the bridge.

Pass conditions:

- Bridge self-test lists declared IPC commands.
- Release readiness includes repo, binaries, Tauri bundle config, version metadata, icons, bundle targets, host contract, docs, migration surface, and expected distribution outputs.
- Status exports stay under `target/host-telemetry` and do not contain API keys.

## 8. Embedded Terminal TUI Canary

Use this checklist only for developer canary evidence. It must not change normal terminal actions or production media routing.

```powershell
cargo test -p dx-agents-desktop embedded_terminal_tui_canary -- --nocapture
cargo check -p dx-agents-desktop
```

Desktop bridge checks:

- Confirm `TUI canary gate` is `off` by default.
- Use `Enable guide` only to see the developer environment opt-in; do not expect the button to mutate production settings.
- Use `Disable guide` and `Rollback` to confirm the visible recovery path keeps `external_terminal` as the production route.
- Confirm lifecycle evidence lists `tui.open`, `tui.resize`, `tui.interrupt`, `tui.close`, and `tui.cleanup`.
- Confirm every lifecycle event is redacted and bounded by a timeout.

Repeated-run pass conditions:

- Three refreshes show the same gate state unless `DX_AGENTS_TUI_CANARY` is changed and the desktop bridge is restarted.
- Interrupt and close evidence always requires cleanup.
- No child process is reported while the canary is only gated or armed.
- Renderer state does not persist after rollback.
- mpv, tplay, viu, and Windows Terminal remain the production media and terminal routes.

Rollback conditions:

- Any normal terminal action changes route.
- Any raw key, paste, mouse, or control-sequence payload appears in diagnostics.
- Any canary evidence reports arbitrary shell input or payload storage.
- Any process exceeds the lifecycle duration or output caps.

## 9. Final Lightweight Verification

Use this as the default release-readiness checkpoint:

```powershell
cargo fmt --check
cargo test -p dx-agents-desktop release_readiness_report_scores_local_release_inputs -- --nocapture
cargo check -p dx-agents-desktop
cargo check -p dx-agents --bin dx-agents
git diff --check
```

Do not run a release build during this checklist unless the release candidate is stable enough to justify one full packaging pass.

## Evidence Paths

Keep QA evidence in predictable places so a later operator can review it without searching chat history:

- Continuation journal: `C:\Users\Computer\.dx-agents\workspace\continuation\journal.jsonl`
- Redacted bridge exports: `target\host-telemetry\bridge-status-export-*.json`
- Release readiness: desktop bridge `Release readiness` action, backed by `get_dx_agents_release_readiness`
- Operator QA checklist: `docs\book\src\ops\operator-qa.md`
- Desktop bridge runbook: `docs\book\src\ops\desktop-bridge.md`

The release-readiness report checks the operator QA and desktop bridge docs directly, so missing evidence docs should show up before packaging.
