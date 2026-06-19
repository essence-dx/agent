# DX Agents Launch Worker Notes

Date: 2026-05-21
Launch target: 2026-05-22

## Worker Role

This repo owns the ZeroClaw-derived DX Agents runtime. It remains CLI-first, while Zed receives a GPUI bridge for agent status, QR/connect, social accounts, automations, and background receipts.

Update 2026-06-06 provider streaming secret hardening: OpenAI-compatible streaming status failures now sanitize provider error bodies before returning `StreamError::ModelProvider`, shared provider error scrubbing covers reflected authorization headers and JSON/header credential fields, and the reliable provider wrapper sanitizes string-bearing stream errors before logging or forwarding them. `cargo test -p zeroclaw-providers -j6` passed with 944 tests.

Update 2026-06-06 route secret and policy hardening: model and embedding route `api_key` values now encrypt on config save/load, mask in object-array display and whole-config API responses, and restore masked PATCH echoes from the active config without guessing by duplicate hints. The action-rate tracker now keeps fresh actions during the first process-uptime hour instead of pruning everything when the one-hour cutoff underflows. `cargo test -p zeroclaw-config -j6` and `cargo check -p zeroclaw-gateway -j6` passed.

Update 2026-06-06 installer onboarding hardening: `install.sh` now skips first-run setup only when both a model provider and a runnable agent route are present, so provider-only configs still receive Quickstart/gateway setup. Linux/macOS setup docs now point to the DX Agents repository, `dx-agents` binary, Quickstart, and `dx-agents service` commands while keeping legacy service/config labels where the runtime still uses them. Verification used Git POSIX `sh -n install.sh` plus temp-config probes for missing, provider-only, agent-only, provider-plus-agent, nested-agent-only, and legacy-agent cases.

Update 2026-06-06 ACP/security launch hardening: `dx-agents-acp-bridge` now prefers `DX_AGENTS_ACP_BRIDGE_TOKEN` and `DX_AGENTS_ACP_PAIRING_CODE` with legacy `ZEROCLAW_*` fallback, ACP initialize responses advertise `dx-agents-acp` and `_meta.dxAgents` while retaining `_meta.zeroclaw`, and gateway pairing recovery/help copy points at `dx-agents`. File upload bundle/download headers plus Gmail push and voice-call secrets now encrypt on save. Docker release defaults keep gateway pairing required, Compose binds host ports to loopback by default, Kubernetes/Docker env examples are DX-first, Quick Start/Windows setup no longer send users to upstream ZeroClaw install/onboard paths, and a transient Groq smoke returned `dx-agents-provider-ok` with zero raw key hits in the temp config or repo scan.

Update 2026-06-06 web agent hardening: the web API client is back in sync with the gateway's current config, quickstart, skills, browse/workspace, cron, cost, memory, session, and TUI routes. Dashboard cron edits now include the owning agent alias for policy validation, and provider error sanitization redacts secret-bearing URL query parameters before they can surface in UI or logs.

Update 2026-06-06 runtime hardening: Gemini quota-exhausted 429 responses now fail fast in the reliable provider path instead of burning retries, the Gemini provider honors `GEMINI_API_KEY` and `GOOGLE_API_KEY` before OAuth fallback, Quickstart no longer accepts provider API keys directly in argv and uses `--api-key-env <ENV_VAR>` for non-interactive seeding, Bash/Fish/Zsh dynamic config completions now call `dx-agents config complete`, and WebSocket auth no longer accepts bearer tokens from URL query parameters.

Update 2026-06-06 quickstart/provider live hardening: fully flag-driven `dx-agents quickstart` now applies immediately with balanced risk/runtime defaults, SQLite memory, and no extra channels instead of opening the checklist TUI. The provider error sanitizer now redacts Groq `gsk_` and Google `AIza` keys. `dx-agents providers list --json` and `dx-agents models list --json` emit Zed-safe provider/model discovery JSON from the DX provider catalog. Live smoke with transient credentials confirmed Gemini reaches the API but fails closed on quota exhaustion, while Groq with `llama-3.3-70b-versatile` returns `dx-agents-provider-ok`; quick self-test passed and the temp secret scan found no raw key files.

Update 2026-06-06 self-update hardening: `dx-agents update` now defaults to the DX Agents GitHub releases repository, honors `DX_AGENTS_UPDATE_REPO` for controlled release testing, sends a `dx-agents/<version>` user agent, prefers `dx-agents-*` assets before legacy `zeroclaw-*` fallback assets, supports Windows `.zip` update archives, and validates downloaded version output against the DX binary name. Targeted Cargo update tests were added, but local compile attempts timed out before assertions; keep the bounded update tests as the next governed compile check.

Update 2026-06-06 release/container/install hardening: stable release archives now build and publish as `dx-agents-*`, release Docker images consume and run `/usr/local/bin/dx-agents`, Docker Compose and the Kubernetes sample point to `ghcr.io/millercarla211-ctrl/dx-agents`, and external website, package-manager, and announcement jobs are fail-closed behind explicit DX release variables. `install.sh`, `setup.bat`, and the Raspberry Pi deploy path now install and verify `dx-agents` from the DX Agents repository while preserving the current `.zeroclaw` runtime config path until the Rust config resolver is migrated.

Update 2026-06-06 credential/release governance hardening: config schema exports now carry `credential_class` into UI/wire field entries so secret fields remain redacted and typed, Windows native runtime tests await async process output correctly, Docker release publication waits for the GitHub release, website redeploys require a matching allowlisted repository and environment, announcement jobs receive explicit secrets only, package-manager workflows default to dry-run, Homebrew formula work is audit-only until DX owns a formula path, and Kubernetes/OpenShift plus AUR/Scoop templates now use DX Agents naming without sample ConfigMap secrets.

Update 2026-06-06 config/provider smoke hardening: `dx-agents config init` now writes disabled TLS, local Whisper, and OpenVPN tunnel blocks that reload cleanly after dirty-save omits default empty strings. Copied DX provider catalog loading now reports diagnostics, logs malformed catalog fallback, and keeps native providers available; desktop bridge live smoke uses the current `dx-agents agent -a dx ... --message` command instead of removed `self-test --provider` flags. The Tauri build script no longer emits an unreachable-code warning on Windows. Gemini live auth reached the API but was quota-blocked, an earlier supplied Groq key reached Groq but was rejected as invalid, and `dx-agents self-test --quick` passed all 9 checks against the temp config without persisting either secret.

Update 2026-06-06 live provider smoke: `cargo build --bin dx-agents -j1` completed warning-free after test-gating the update asset helper. Transient Gemini credentials reached the Gemini API but returned quota exhaustion for `gemini-2.0-flash`. Transient Groq credentials listed live models, and `dx-agents agent -a dx --message ...` returned the expected smoke token with a minimal risk profile that allowed only `memory_recall`; `dx-agents self-test --quick` then passed all 9 checks against the same temp config. No provider secret was written to config, docs, or repo files.

Update 2026-06-05 onboarding brand: the desktop onboarding page now presents a static rainbow DX wordmark and ASCII train rail animation, then guards the final completion action while the train exits. Legacy ZeroClaw storage/protocol aliases remain compatibility data only.

Update 2026-05-22: the bridge contract now exposes QR/connect and Automations as first-class Zed GPUI surfaces (`qr_connect_panel` and `automations_entrypoint`) in addition to the existing social cards/actions, automation rows, and receipt rail. Zed should continue to execute only the public fixed `dx agents ... --json` commands and render only metadata-safe row fields from receipts.

Update 2026-05-22 production-hardening: `dx agents run --json` remains a metadata-only background task receipt producer, but the run receipt now includes task lifecycle state, provider/model readiness, tool-call counters, social/account and automation summaries, duration state, safe retry/cancel support flags, and redacted config metadata. It still does not execute live tasks in this lane, and it never serializes raw credentials, prompts, transcripts, account targets, automation bodies, or tool payloads.

Update 2026-05-22 redaction contract hardening: the common Zed bridge redaction object now publishes explicit booleans for task payloads, transcripts, and provider credentials in addition to secret values, account targets, automation bodies, and tool payloads. Contract audit counts those flags, so Zed can mark unsafe receipts for review from data instead of prose.

Update 2026-05-22 scalar-redaction hardening: receipt-list summaries now treat bearer headers, authorization labels, provider-key labels, private-token strings, passwords, bare `sk-` values, and OAuth token names as non-renderable scalar metadata before they reach Zed receipt rows.

Update 2026-05-22 contract-audit receipt indexing: `dx-agents agents receipts list --json` now treats `contract-audit-latest.json` as a first-class `contract_audit` row, so Zed can render bridge audit evidence from the receipt rail instead of relying only on the contract command output.

Update 2026-05-22 final hardening validation: the provider/model catalog crate contract now explicitly enables `rkyv` `size_32` while keeping the `memmap2` binary-cache direction, and the binary-side `models list --json` path calls the shared provider-catalog module. Focused validation passed with `cargo test -q --test component zed_agent_bridge::`; full workspace Cargo validation and live Zed visual proof remain reserved for the broader launch validation pass.

Update 2026-05-22 folder rename and maintainability pass: the active source folder is now `G:\Dx\agent`. Internal Cargo names, binaries, library names, and runtime receipt command labels remain `dx-agents` for compatibility. Run-receipt lifecycle code now lives in `src/zed_agent_run_receipts.rs`, with `zed_agent_bridge` retaining the existing public API through re-exports.

Update 2026-05-22 report-contract split: Zed bridge report DTOs, schema constants, stable command maps, public command translation, and common redaction metadata now live in `src/zed_agent_bridge_reports.rs`. `zed_agent_bridge` remains the compatibility API surface, while report-contract ownership is separated from command orchestration.

Update 2026-05-22 bridge-contract maintainability split: Zed contract DTOs and schema constants now live in `src/zed_agent_bridge_contract_types.rs`, handoff command/path metadata in `src/zed_agent_bridge_contract_handoff.rs`, and provider-catalog safety metadata in `src/zed_agent_bridge_contract_provider_catalog.rs`. `zed_agent_bridge_contract` keeps the existing public re-exports and focuses on assembling the receipt and surface contract.

Update 2026-05-22 surface-map split: the Zed GPUI surface map now lives in `src/zed_agent_bridge_contract_surfaces.rs`, keeping the contract assembler under 900 lines while preserving the same `zed_surfaces` JSON shape for Zed.

Update 2026-05-22 CLI helper split: top-level CLI helper routines now live in `src/cli_helpers.rs`, so `src/main.rs` keeps command definitions and dispatch while helper logic for config comments, JSON set-prop coercion, temperature parsing, no-command help, and terminal canaries is isolated.

Update 2026-05-22 completion split: shell completion parsing and script generation now live in `src/cli_completion.rs`, preserving dynamic config-path completion while keeping completion-specific clap code out of the command dispatcher.

Update 2026-05-22 estop split: emergency-stop CLI parsing and engage/resume/status helpers now live in `src/cli_estop.rs`, so safety-sensitive estop logic is isolated from the main command dispatcher while preserving the same CLI contract.

Update 2026-05-22 CLI i18n split: recursive clap command translation now lives in `src/cli_i18n.rs`, keeping localized help decoration separate from command parsing and dispatch.

Update 2026-05-23 gateway helper split: gateway admin URL construction, pair-code fetch, health fetch, and health printing now live in `src/cli_gateway.rs`, keeping network-facing gateway helper logic separate from the main command dispatcher.

Update 2026-05-23 pending OAuth split: pending OAuth login state persistence now lives in `src/cli_auth_pending.rs`, isolating encrypted code-verifier storage, load/save/clear behavior, and owner-only permission handling from the main CLI dispatcher.

Update 2026-05-23 auth helper split: auth input reading, OpenAI Codex auth import parsing, OpenAI account-id extraction, and token-expiry formatting now live in `src/cli_auth_helpers.rs`, keeping auth support routines separate from command dispatch.

Update 2026-05-23 onboarding split: onboarding section parsing, retired `--*-only` flag target resolution, and resolver unit tests now live in `src/cli_onboard.rs`, keeping onboarding routing separate from the main command dispatcher while preserving the `dx-agents onboard <section>` contract.

Update 2026-05-23 kernel command handler split: the no-runtime CLI fallback now lives in `src/cli_kernel_handler.rs`, keeping provider fallback mutation, simple chat execution, interactive stdin handling, and shared `agents` dispatch out of `src/main.rs` while preserving the kernel-only `dx-agents agent` behavior. Runtime-only command handlers are feature-gated again, and runtime preflight now reports cron as unavailable in no-runtime builds instead of referencing gated cron internals.

Update 2026-05-23 config command split: config CLI command definitions now live in `src/cli_config_commands.rs`, keeping schema/list/get/set/init/migrate/patch/docs clap wiring separate from the dispatcher while preserving the existing config subcommand behavior.

Update 2026-05-23 auth command split: auth CLI command definitions now live in `src/cli_auth_commands.rs`, keeping login/paste-token/refresh/logout/use/list/status clap wiring separate from auth execution and support helpers.

Update 2026-05-23 model command split: model CLI command definitions and provider health/failover value enums now live in `src/cli_model_commands.rs`, keeping model/provider clap wiring separate from command execution.

Update 2026-05-23 memory command split: memory CLI command definitions now live in `src/cli_memory_commands.rs`, keeping memory list/get/stats/learning/receipt/clear/reindex clap wiring separate from memory command execution.

Update 2026-05-23 doctor command split: doctor CLI command definitions now live in `src/cli_doctor_commands.rs`, keeping model-catalog and trace diagnostic clap wiring separate from doctor command execution.

Update 2026-05-23 continuation command split: continuation CLI command definitions now live in `src/cli_continuation_commands.rs`, keeping status, record, and journal-path clap wiring separate from continuation execution.

Update 2026-05-23 deprecated props command split: the legacy `props` compatibility parser now lives in `src/cli_deprecated_props_commands.rs`, keeping deprecated clap scaffolding out of the main dispatcher.

Update 2026-05-23 plugin command split: optional `plugins-wasm` CLI command definitions now live in `src/cli_plugin_commands.rs`, preserving the feature gate while keeping plugin clap wiring out of the main dispatcher.

Update 2026-05-23 provider command split: provider list and catalog CLI command definitions now live in `src/cli_provider_commands.rs`, keeping provider/model catalog clap wiring separate from command execution.

Update 2026-05-23 agents command split: Zed bridge `agents` CLI command definitions now live in `src/cli_agents_commands.rs`, keeping contract/status/social/automation/run/receipt clap wiring separate from bridge execution.

Update 2026-05-23 root command split: the root `Cli` and `Commands` clap definitions now live in `src/cli_commands.rs`, keeping top-level command shape separate from execution and parity dispatch.

Update 2026-05-23 parity command definition split: the large `ParityCommands` clap enum now lives in `src/cli_parity_commands.rs`, keeping long parity command schema ownership separate from the root binary bootstrap while preserving the existing `dx-agents parity ...` surface.

Update 2026-05-23 CLI parser test split: the large root CLI parser test module now lives in `src/cli_parser_tests.rs`, leaving `src/main.rs` focused on bootstrap and dispatch while preserving the existing command-surface coverage.

Update 2026-05-23 parity parser test split: parity CLI parser coverage now lives in `src/cli_parity_parser_tests.rs`, keeping core parser tests in `src/cli_parser_tests.rs` and separating the large parity command surface from general CLI coverage.

Update 2026-05-23 parity handler dependency cleanup: `src/cli_parity_handler.rs` now imports its parity/readiness modules explicitly instead of using a crate-wide wildcard import, making the large dispatch table's dependencies visible and easier to audit.

Update 2026-05-23 gateway command handler split: gateway start/restart/status/pair-code command execution now lives in `src/cli_gateway_handler.rs`, keeping gateway dispatch and startup helpers separate from the main CLI orchestration.

Update 2026-05-23 auth command handler split: auth login, paste-redirect, paste-token, refresh, logout, profile selection, list, and status execution now lives in `src/cli_auth_handler.rs`, keeping provider auth flows separate from the main CLI orchestration.

Update 2026-05-23 desktop command handler split: companion desktop install guidance, download opening, binary discovery, and launch execution now live in `src/cli_desktop_handler.rs`, keeping desktop-app concerns separate from the main CLI orchestration.

Update 2026-05-23 provider command handler split: provider list, provider catalog regeneration, and the human-readable supported-provider table now live in `src/cli_provider_handler.rs`, keeping provider command execution separate from the main CLI orchestration.

Update 2026-05-23 doctor command handler split: doctor models, traces, and default diagnostic execution now live in `src/cli_doctor_handler.rs`, keeping diagnostic command execution separate from the main CLI orchestration.

Update 2026-05-23 tool command handler split: tool safety-drill, execution receipts, receipt history, and receipt alert execution now live in `src/cli_tool_handler.rs`, keeping tool safety/receipt commands separate from the main CLI orchestration.

Update 2026-05-23 self-test command handler split: quick/full diagnostics, selected check validation, provider/model smoke tests, result printing, and failure exit semantics now live in `src/cli_self_test_handler.rs`, keeping diagnostic self-test execution separate from the main CLI orchestration.

Update 2026-05-23 service command handler split: service lifecycle init-system parsing and install/start/stop/restart/status/log dispatch now live in `src/cli_service_handler.rs`, keeping OS service command execution separate from the main CLI orchestration.

Update 2026-05-23 channel command handler split: channel start/doctor async-runtime routing and normal channel subcommand dispatch now live in `src/cli_channel_handler.rs`, keeping channel command execution separate from the main CLI orchestration.

Update 2026-05-23 update command handler split: update check/install branching and user-facing update status printing now live in `src/cli_update_handler.rs`, keeping self-update command execution separate from the main CLI orchestration.

Update 2026-05-23 dashboard compatibility handler split: dashboard compatibility report, retained history, and alert command dispatch now live in `src/cli_dashboard_compatibility_handler.rs`, keeping dashboard readiness command execution separate from the main CLI orchestration.

Update 2026-05-23 memory command handler split: memory list/get/stats/learning/receipt/clear/reindex execution now routes through `src/cli_memory_handler.rs`, keeping memory command execution separate from the main CLI orchestration.

Update 2026-05-23 continuation command handler split: continuation status, record, and journal-path execution now routes through `src/cli_continuation_handler.rs`, keeping continuation command execution separate from the main CLI orchestration.

Update 2026-05-23 model command handler split: model refresh/list/set/health/failover/status execution now routes through `src/cli_model_handler.rs`, keeping model command execution separate from the main CLI orchestration.

Update 2026-05-23 cron command handler split: cron list/preview/history/delivery/add/update/remove/pause/resume execution now routes through `src/cli_cron_handler.rs`, keeping scheduled-task command execution separate from the main CLI orchestration.

Update 2026-05-23 device command handler split: hardware discovery/introspection/info and peripheral list/add/flash/setup execution now routes through `src/cli_device_handler.rs`, keeping device command execution separate from the main CLI orchestration.

Update 2026-05-23 workflow command handler split: SOP, migration, and session command execution now routes through `src/cli_workflow_handler.rs`, keeping operator workflow command execution separate from the main CLI orchestration.

Update 2026-05-23 extension command handler split: integration info and skill list/audit/install/remove/test execution now routes through `src/cli_extension_handler.rs`, keeping extension command execution separate from the main CLI orchestration.

Update 2026-05-23 status command handler split: compact dashboard routing, exit-code health probing, full human status rendering, cost usage display, channel summary, and peripheral summary now live in `src/cli_status_handler.rs`, keeping status command execution separate from the main CLI orchestration.

Update 2026-05-23 parity command handler split: the large parity subcommand dispatch table now lives in `src/cli_parity_handler.rs`, keeping the main CLI orchestrator focused while preserving the existing `dx-agents parity ...` command contract.

Update 2026-05-23 agents bridge handler split: shared Zed bridge `agents` command execution now lives in `src/cli_agents_handler.rs`, removing duplicate kernel-only and full-runtime dispatch while preserving the same JSON bridge command behavior.

Update 2026-05-23 config command handler split: config schema/list/get/set/init/migrate/patch/docs/complete execution now lives in `src/cli_config_handler.rs`, keeping config mutation, JSON Patch, and docs probing out of the main CLI orchestrator.

Update 2026-05-23 daemon command handler split: daemon startup checks, channel/peripheral registration, shared canvas-store setup, and reload-loop subsystem wiring now live in `src/cli_daemon_handler.rs`, keeping long-running runtime orchestration out of the main CLI dispatcher.

Update 2026-05-23 runtime agent handler split: full-runtime `agent` and `acp` command execution now lives in `src/cli_agent_handler.rs`, keeping CLI channel registration, final temperature resolution, and ACP server setup out of the main dispatcher.

Update 2026-05-23 plugin command handler split: optional `plugins-wasm` list/install/remove/info execution now lives in `src/cli_plugin_handler.rs`, preserving the feature gate while keeping plugin host operations out of the main dispatcher.

Update 2026-05-23 plugin feature contract hardening: the root `plugins-wasm` feature now explicitly enables `agent-runtime`, matching the binary command surface so `--features plugins-wasm` no longer selects an incoherent half-runtime build.

Update 2026-05-23 onboarding handler split: onboard backup handling, quick/CLI/TUI selection, deprecated flag warnings, and config save now live in `src/cli_onboard_handler.rs`, keeping early onboarding execution out of the main CLI bootstrap.

## Required Reading

- `G:\Dx\WORKER_PROMPTS.md`
- `G:\Dx\DX.md`
- `G:\Dx\agent\AGENTS.md`

## Current Targets

- JSON status command for Zed.
- Social account list/connect/disconnect status without storing secrets in Zed.
- Automation list/run status.
- Provider/model list status backed by a generated binary catalog cache.
- Background task receipts under `G:\Dx\.dx\receipts\agents`.
- Zed-facing contract docs so GPUI workers know exactly what to call.

## Zed CLI And Receipt Contract

Zed should call the public DX CLI commands, not import `dx-agents` crates directly. The lower-level runtime commands remain the receipt owners and are listed here so CLI bridge workers can keep the root `dx agents ... --json` aliases exact:

- `dx-agents agents contract --json`
- `dx-agents agents contract-audit --json`
- `dx-agents agents snapshot --json`
- `dx-agents agents status --json`
- `dx-agents agents social list --json`
- `dx-agents agents social connect --json`
- `dx-agents agents social connect --platform <platform> --json`
- `dx-agents agents social disconnect --json`
- `dx-agents agents social disconnect --platform <platform> --json`
- `dx-agents agents automate list --json`
- `dx-agents agents run --json`
- `dx-agents agents receipts list --json`
- `dx-agents providers list --json`
- `dx-agents providers catalog regenerate --json`
- `dx-agents models list --json`

Preferred public root commands for Zed are now:

- `dx agents contract --json`
- `dx agents contract-audit --json`
- `dx agents snapshot --json`
- `dx agents status --json`
- `dx agents social list --json`
- `dx agents social connect --platform <platform> --json`
- `dx agents social disconnect --platform <platform> --json`
- `dx agents automate list --json`
- `dx agents run --json`
- `dx agents receipts list --json`
- `dx agents providers list --json`
- `dx agents providers catalog regenerate --json`
- `dx agents models list --json`

Receipts live under `G:\Dx\.dx\receipts\agents`. JSON reports are the Zed launch contract; Zed must not import `dx-agents` crates directly for launch.

`contract-latest.json` now carries both `commands` for the runtime-owned `dx-agents ... --json` receipt producers and `public_commands` for the Zed-facing root `dx agents ... --json` bridge. Handoff commands, GPUI surface rows, and row actions also include public command metadata. Zed should execute the public command fields or its own fixed typed command variants; runtime receipts may still report their owning `dx-agents` command in `command` and `commands`.

Source-review polish: the runtime-to-public command helper now returns owned command strings for both transformed `dx-agents ...` labels and already-public/fallback labels before the governed Cargo validation window.

Coding-completion handoff: Zed GPUI now consumes the structured social and automation `actions[]` descriptors, validates receipt filenames and `secrets_exposed=false`, renders action readiness in settings/right-rail surfaces, and still executes only fixed safe bridge command vectors rather than receipt-supplied command strings.

The DX CLI also owns the Zed import bridge around those live calls. `dx agents release-gate --json` now checks action-map recovery-control coverage before final import, `dx agents import-summary --json` exposes compact `action_map` readiness for first paint, and `dx agents evidence --json` embeds the full metadata-only action map. These bridge packets stay CLI-local and do not require Zed to import `dx-agents` crates.

Launch-safety continuation: Batch 170 now starts with `dx-agents parity runtime-budget-ci-promotion-release-automation-readiness-operator-handoff-promotion-future-live-handoff-implementation-readiness-manifest --json`. It is a metadata-only implementation-readiness manifest over the existing future live-handoff gate design packet. It declares ownership, safe command allowlists, rollback owners, audit evidence, live-gate preconditions, disabled-by-default policy, and redaction boundaries while keeping release automation, live handoff execution, archive writes, and change writes disabled. It does not change the Zed CLI/receipt bridge contract.

Receipt filenames:

- `contract-latest.json`
- `contract-audit-latest.json`
- `snapshot-latest.json`
- `status-latest.json`
- `social-list-latest.json`
- `social-connect-latest.json`
- `social-disconnect-latest.json`
- `automate-list-latest.json`
- `providers-list-latest.json`
- `models-list-latest.json`
- `provider-model-catalog-latest.json`
- `agent-run-<timestamp>-<pid>.json`
- `run-latest.json`
- `receipts-list-latest.json`

Common render fields on every report:

```json
{
  "schema_version": "dx.agents.zed.status.v1",
  "command": "dx-agents agents status --json",
  "generated_at": "2026-05-21T00:00:00Z",
  "status": "ready",
  "connected_accounts_summary": {
    "supported": 0,
    "configured": 0,
    "connected": 0,
    "needs_connection": 0,
    "needs_auth": 0,
    "qr_connect_supported": 0
  },
  "automation_count": 0,
  "active_task_count": 0,
  "last_error": null,
  "next_action": "agent_bridge_ready_for_zed",
  "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\status-latest.json"
}
```

Run receipts add launch-grade background task fields for Zed:

```json
{
  "task_state": "pending",
  "provider_model": {
    "provider_status": "configured",
    "model_status": "configured",
    "catalog_source": "provider_model_catalog_binary_cache_preferred",
    "credentials_state": "not_exported"
  },
  "tool_calls": {
    "requested": 0,
    "completed": 0,
    "failed": 0,
    "payloads_redacted": true
  },
  "duration_state": "not_started",
  "safe_actions": {
    "retry_supported": false,
    "cancel_supported": false
  },
  "safe_config": {
    "exports_secret_values": false,
    "exports_account_targets": false,
    "exports_automation_bodies": false,
    "exports_tool_payloads": false,
    "exports_task_payloads": false,
    "exports_transcripts": false,
    "exports_provider_credentials": false,
    "provider_credentials": "explicit_user_approval_required"
  }
}
```

`dx-agents agents contract --json` uses schema `dx.agents.zed.bridge_contract.v1`, writes `contract-latest.json`, and is the preferred first call for Zed GPUI workers. It defines the stable command set, receipt filenames, required fields, Zed render fields, nested item render fields, provider-catalog fallback path, and redaction guarantees:

```json
{
  "schema_version": "dx.agents.zed.bridge_contract.v1",
  "command": "dx-agents agents contract --json",
  "status": "ready",
  "connected_accounts_summary": {
    "supported": 0,
    "configured": 0,
    "connected": 0,
    "needs_connection": 0,
    "needs_auth": 0,
    "qr_connect_supported": 0
  },
  "automation_count": 0,
  "active_task_count": 0,
  "last_error": null,
  "receipt_root": "G:\\Dx\\.dx\\receipts\\agents",
  "commands": {
    "contract": "dx-agents agents contract --json",
    "contract_audit": "dx-agents agents contract-audit --json",
    "snapshot": "dx-agents agents snapshot --json",
    "status": "dx-agents agents status --json",
    "social_list": "dx-agents agents social list --json",
    "social_connect": "dx-agents agents social connect --json",
    "social_connect_platform": "dx-agents agents social connect --platform <platform> --json",
    "social_disconnect": "dx-agents agents social disconnect --json",
    "social_disconnect_platform": "dx-agents agents social disconnect --platform <platform> --json",
    "automate_list": "dx-agents agents automate list --json",
    "run": "dx-agents agents run --json",
    "receipts_list": "dx-agents agents receipts list --json",
    "providers_list": "dx-agents providers list --json",
    "provider_catalog_regenerate": "dx-agents providers catalog regenerate --json",
    "models_list": "dx-agents models list --json"
  },
  "public_commands": {
    "contract": "dx agents contract --json",
    "contract_audit": "dx agents contract-audit --json",
    "snapshot": "dx agents snapshot --json",
    "status": "dx agents status --json",
    "social_list": "dx agents social list --json",
    "social_connect": "dx agents social connect --json",
    "social_connect_platform": "dx agents social connect --platform <platform> --json",
    "social_disconnect": "dx agents social disconnect --json",
    "social_disconnect_platform": "dx agents social disconnect --platform <platform> --json",
    "automate_list": "dx agents automate list --json",
    "run": "dx agents run --json",
    "receipts_list": "dx agents receipts list --json",
    "providers_list": "dx agents providers list --json",
    "provider_catalog_regenerate": "dx agents providers catalog regenerate --json",
    "models_list": "dx agents models list --json"
  },
  "common_render_fields": [
    "schema_version",
    "command",
    "generated_at",
    "status",
    "connected_accounts_summary",
    "automation_count",
    "active_task_count",
    "last_error",
    "next_action",
    "receipt_path"
  ],
  "receipts": [
    {
      "name": "receipts_list",
      "schema_version": "dx.agents.zed.receipts_list.v1",
      "command": "dx-agents agents receipts list --json",
      "filename": "receipts-list-latest.json",
      "zed_render_fields": [
        "status",
        "receipt_count",
        "returned_receipt_count",
        "active_task_count",
        "latest_receipt_path",
        "receipts",
        "last_error",
        "next_action",
        "receipt_path"
      ],
      "item_render_fields": [
        "receipts[].id",
        "receipts[].kind",
        "receipts[].schema_version",
        "receipts[].command",
        "receipts[].generated_at",
        "receipts[].task_id",
        "receipts[].status",
        "receipts[].active_task",
        "receipts[].safe_to_render",
        "receipts[].metadata_redacted",
        "receipts[].receipt_path",
        "receipts[].size_bytes",
        "receipts[].modified_at",
        "receipts[].last_error",
        "receipts[].next_action"
      ]
    }
  ],
  "zed_handoff": {
    "mode": "cli_receipt_bridge",
    "crate_import_policy": "Zed calls dx-agents JSON commands and reads receipt files; it must not import dx-agents crates for launch.",
    "receipt_root": "G:\\Dx\\.dx\\receipts\\agents",
    "startup_commands": [
      {
        "name": "snapshot",
        "command": "dx-agents agents snapshot --json",
        "public_command": "dx agents snapshot --json",
        "writes_receipt": true,
        "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\snapshot-latest.json",
        "zed_surface": "ai_panel_first_paint",
        "user_action_required": false,
        "secrets_exposed": false
      }
    ],
    "action_commands": [
      {
        "name": "social_connect_platform",
        "command": "dx-agents agents social connect --platform <platform> --json",
        "public_command": "dx agents social connect --platform <platform> --json",
        "writes_receipt": true,
        "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\social-connect-latest.json",
        "zed_surface": "social_connection_action",
        "user_action_required": true,
        "secrets_exposed": false
      }
    ],
    "receipt_paths": [
      {
        "name": "agent_run_detail",
        "schema_version": "dx.agents.zed.run_receipt.v1",
        "path": "G:\\Dx\\.dx\\receipts\\agents\\agent-run-<timestamp>-<pid>.json",
        "path_pattern": "G:\\Dx\\.dx\\receipts\\agents\\agent-run-<timestamp>-<pid>.json",
        "owner_command": "dx-agents agents run --json",
        "zed_surface": "background_task_detail",
        "safe_to_read_directly": true
      }
    ],
    "safe_render_fields": [
      "schema_version",
      "command",
      "generated_at",
      "status",
      "connected_accounts_summary",
      "automation_count",
      "active_task_count",
      "last_error",
      "next_action",
      "receipt_path"
    ],
    "never_render_fields": [
      "provider_api_keys",
      "channel_tokens",
      "account_targets",
      "requested_platform_values",
      "unmatched_platform_values",
      "qr_payload",
      "link_payload",
      "cookies",
      "automation_bodies",
      "shell_command_bodies",
      "tool_payloads",
      "task_payloads",
      "transcripts"
    ]
  },
  "provider_catalog": {
    "schema_version": "dx.agents.provider_model_catalog.v1",
    "source_format": "json_import_only",
    "normal_use_path": "binary_cache_or_cli_receipt",
    "json_import_policy": "JSON is an import/regeneration format only; normal Zed startup should use the binary cache-backed CLI reports or existing receipts.",
    "binary_cache_policy": "Use the rkyv/memmap catalog only when schema version, source hash, generated timestamp, provider count, and model count match the current metadata.",
    "binary_cache_path": "G:\\Dx\\.dx\\catalog\\agents\\provider-model-catalog.rkyv",
    "zed_render_fields": [
      "schema_version",
      "source_format",
      "normal_use_path",
      "binary_cache_path",
      "binary_cache_present",
      "binary_cache_stale",
      "source_hash",
      "provider_count",
      "model_count",
      "compatibility_fallback",
      "safe_regeneration_command",
      "source_kinds"
    ],
    "safety_guards": [
      "schema_version",
      "binary_cache_schema_version",
      "binary_cache_generated_at",
      "source_hash",
      "provider_count",
      "model_count",
      "safe_regeneration_command",
      "compatibility_fallback",
      "source_kinds"
    ],
    "redacted_fields": [
      "provider_api_keys",
      "env_values",
      "base_urls",
      "prompt_payloads",
      "cookies",
      "tokens"
    ],
    "safe_regeneration_command": "dx-agents providers catalog regenerate --json"
  }
}
```

The `zed_handoff` block is the exact GPUI bridge map. Zed should use each command row's `public_command` for execution, while `command` identifies the runtime-owned receipt producer. Use `startup_commands` for first paint, `action_commands` for explicit user actions, `provider_catalog_commands` for provider/model discovery and cache recovery, and `receipt_paths` for latest receipt files and the dynamic `agent-run-<timestamp>-<pid>.json` background-task pattern.

The `zed_surfaces` array is the exact GPUI surface map. Each entry is safe to render without importing `dx-agents` crates and has this shape:

```json
{
  "id": "background_task_rail",
  "title": "Background task rail",
  "primary_command": "dx-agents agents receipts list --json",
  "public_command": "dx agents receipts list --json",
  "refresh_command": "dx-agents agents receipts list --json",
  "public_refresh_command": "dx agents receipts list --json",
  "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\receipts-list-latest.json",
  "receipt_schema_version": "dx.agents.zed.receipts_list.v1",
  "render_fields": [
    "status",
    "receipt_count",
    "returned_receipt_count",
    "active_task_count",
    "latest_receipt_path",
    "receipts",
    "last_error",
    "next_action",
    "receipt_path"
  ],
  "item_render_fields": [
    "receipts[].id",
    "receipts[].status",
    "receipts[].safe_to_render",
    "receipts[].metadata_redacted",
    "receipts[].receipt_path"
  ],
  "empty_state": "No bridge receipts are available yet.",
  "error_state": "Show last_error and ignore unsafe receipt rows.",
  "user_action_required": false,
  "writes_receipt": true,
  "secrets_exposed": false,
  "related_actions": [
    "dx-agents agents run --json"
  ]
}
```

Zed GPUI surfaces currently exposed by `zed_surfaces`:

- `settings_and_bridge_bootstrap`: `dx-agents agents contract --json` writes `contract-latest.json`.
- `bridge_contract_audit`: `dx-agents agents contract-audit --json` writes `contract-audit-latest.json`.
- `ai_panel_first_paint`: `dx-agents agents snapshot --json` writes `snapshot-latest.json`.
- `agent_status_header`: `dx-agents agents status --json` writes `status-latest.json`.
- `social_connection_cards`: `dx-agents agents social list --json` writes `social-list-latest.json`.
- `social_connection_action`: `dx-agents agents social connect --platform <platform> --json` writes `social-connect-latest.json` and refreshes with `dx-agents agents social list --json`.
- `social_disconnect_action`: `dx-agents agents social disconnect --platform <platform> --json` writes `social-disconnect-latest.json` and refreshes with `dx-agents agents social list --json`.
- `automation_rows`: `dx-agents agents automate list --json` writes `automate-list-latest.json`.
- `background_task_start`: `dx-agents agents run --json` writes `run-latest.json` and dynamic `agent-run-<timestamp>-<pid>.json` receipts.
- `background_task_rail`: `dx-agents agents receipts list --json` writes `receipts-list-latest.json`.
- `provider_settings_and_model_picker`: `dx-agents providers list --json` emits provider discovery JSON for capture as `providers-list-latest.json`.
- `model_picker`: `dx-agents models list --json` emits model discovery JSON for capture as `models-list-latest.json`.
- `provider_catalog_recovery`: `dx-agents providers catalog regenerate --json` writes `provider-model-catalog-latest.json`.

Every receipt contract now includes `item_render_fields`. It is empty for scalar-only reports and populated for row-based reports, including `accounts[]`, `accounts[].actions[]`, `automations[]`, `automations[].actions[]`, `receipts[]`, `providers[]`, and `models[]`. GPUI should render only those row paths plus `zed_render_fields`; anything in `redacted_fields` or `zed_handoff.never_render_fields` stays non-renderable.

Social account rows expose explicit, secret-safe Zed actions so GPUI does not need to build command strings:

```json
{
  "platform": "telegram",
  "status": "connected",
  "secret_state": "configured_redacted",
  "connect_command": "dx-agents agents social connect --platform telegram --json",
  "disconnect_command": "dx-agents agents social disconnect --platform telegram --json",
  "refresh_command": "dx-agents agents social list --json",
  "actions": [
    {
      "id": "connect",
      "label": "Connect",
      "command": "dx-agents agents social connect --platform telegram --json",
      "public_command": "dx agents social connect --platform telegram --json",
      "enabled": false,
      "user_action_required": true,
      "writes_receipt": true,
      "receipt_filename": "social-connect-latest.json",
      "refresh_command": "dx-agents agents social list --json",
      "public_refresh_command": "dx agents social list --json",
      "secrets_exposed": false
    },
    {
      "id": "disconnect",
      "label": "Disconnect",
      "command": "dx-agents agents social disconnect --platform telegram --json",
      "public_command": "dx agents social disconnect --platform telegram --json",
      "enabled": true,
      "user_action_required": true,
      "writes_receipt": true,
      "receipt_filename": "social-disconnect-latest.json",
      "refresh_command": "dx-agents agents social list --json",
      "public_refresh_command": "dx agents social list --json",
      "secrets_exposed": false
    },
    {
      "id": "refresh",
      "label": "Refresh",
      "command": "dx-agents agents social list --json",
      "public_command": "dx agents social list --json",
      "enabled": true,
      "user_action_required": false,
      "writes_receipt": true,
      "receipt_filename": "social-list-latest.json",
      "refresh_command": "dx-agents agents social list --json",
      "public_refresh_command": "dx agents social list --json",
      "secrets_exposed": false
    }
  ]
}
```

Automation rows expose a bridge-owned run/refresh entrypoint without exporting prompt, command, or delivery bodies:

```json
{
  "id": "automation-001",
  "status": "ready",
  "prompt_redacted": true,
  "command_redacted": true,
  "run_command": "dx-agents agents run --json",
  "refresh_command": "dx-agents agents automate list --json",
  "actions": [
    {
      "id": "run",
      "label": "Run now",
      "command": "dx-agents agents run --json",
      "public_command": "dx agents run --json",
      "enabled": true,
      "user_action_required": true,
      "writes_receipt": true,
      "receipt_filename": "run-latest.json",
      "refresh_command": "dx-agents agents automate list --json",
      "public_refresh_command": "dx agents automate list --json",
      "secrets_exposed": false
    },
    {
      "id": "refresh",
      "label": "Refresh",
      "command": "dx-agents agents automate list --json",
      "public_command": "dx agents automate list --json",
      "enabled": true,
      "user_action_required": false,
      "writes_receipt": true,
      "receipt_filename": "automate-list-latest.json",
      "refresh_command": "dx-agents agents automate list --json",
      "public_refresh_command": "dx agents automate list --json",
      "secrets_exposed": false
    }
  ]
}
```

`dx-agents agents contract-audit --json` uses schema `dx.agents.zed.bridge_contract_audit.v1`, writes `contract-audit-latest.json`, and is the lightweight source-level readiness check for the Zed bridge contract. It re-emits only audit metadata and checks, never the raw contract payload, provider credentials, QR/link payloads, task payloads, automation bodies, or transcripts:

```json
{
  "schema_version": "dx.agents.zed.bridge_contract_audit.v1",
  "command": "dx-agents agents contract-audit --json",
  "status": "ready",
  "contract_schema_version": "dx.agents.zed.bridge_contract.v1",
  "contract_receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\contract-latest.json",
  "surface_count": 13,
  "receipt_contract_count": 13,
  "command_count": 15,
  "secret_exposure_count": 0,
  "missing_required_surfaces": [],
  "unknown_surface_receipts": [],
  "schema_mismatches": [],
  "undiscoverable_surface_commands": [],
  "checks": [
    {
      "id": "zed_contract_surfaces_do_not_expose_secrets",
      "status": "ready",
      "detail": "Surfaces, handoff commands, and common redaction flags do not expose raw secrets."
    }
  ],
  "last_error": null,
  "next_action": "zed_bridge_contract_audit_ready_for_gpui",
  "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\contract-audit-latest.json"
}
```

Zed can call the audit after `dx-agents agents contract --json` or before enabling the GPUI surface bridge. If `status` is `blocked`, render `checks[]`, `last_error`, and `next_action`; do not inspect arbitrary receipt files or import `dx-agents` crates to recover.

The `provider_catalog` contract is the product-owner safety story for model/provider discovery. Zed should prefer `normal_use_path: "binary_cache_or_cli_receipt"`, treat JSON as import/regeneration input only, render the listed `zed_render_fields`, validate the listed `safety_guards`, and never render provider API keys, environment values, base URLs, prompt payloads, cookies, or tokens.

`dx-agents agents snapshot --json` uses schema `dx.agents.zed.snapshot.v1`, writes `snapshot-latest.json`, and is the preferred first-paint call when Zed needs agent health, social account cards, automation rows, and background receipt rows in one process invocation:

```json
{
  "schema_version": "dx.agents.zed.snapshot.v1",
  "command": "dx-agents agents snapshot --json",
  "status": "ready",
  "connected_accounts_summary": {
    "supported": 0,
    "configured": 0,
    "connected": 0,
    "needs_connection": 0,
    "needs_auth": 0,
    "qr_connect_supported": 0
  },
  "accounts": [],
  "automation_count": 0,
  "active_task_count": 0,
  "automations": [],
  "receipt_count": 0,
  "returned_receipt_count": 0,
  "latest_receipt_path": null,
  "receipts": [],
  "last_error": null,
  "next_action": "agent_snapshot_ready_for_zed",
  "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\snapshot-latest.json"
}
```

`dx-agents agents status --json` adds:

```json
{
  "schema_version": "dx.agents.zed.status.v1",
  "commands": {
    "contract": "dx-agents agents contract --json",
    "contract_audit": "dx-agents agents contract-audit --json",
    "snapshot": "dx-agents agents snapshot --json",
    "status": "dx-agents agents status --json",
    "social_list": "dx-agents agents social list --json",
    "social_connect": "dx-agents agents social connect --json",
    "social_connect_platform": "dx-agents agents social connect --platform <platform> --json",
    "social_disconnect": "dx-agents agents social disconnect --json",
    "social_disconnect_platform": "dx-agents agents social disconnect --platform <platform> --json",
    "automate_list": "dx-agents agents automate list --json",
    "run": "dx-agents agents run --json",
    "receipts_list": "dx-agents agents receipts list --json",
    "providers_list": "dx-agents providers list --json",
    "provider_catalog_regenerate": "dx-agents providers catalog regenerate --json",
    "models_list": "dx-agents models list --json"
  },
  "redaction": {
    "exports_secret_values": false,
    "exports_account_targets": false,
    "exports_automation_bodies": false,
    "exports_tool_payloads": false,
    "exports_task_payloads": false,
    "exports_transcripts": false,
    "exports_provider_credentials": false,
    "detail": "Receipts export counts, labels, states, schedules, and recovery hints only. Provider keys, channel tokens, account target values, unmatched social platform values, automation bodies, tool inputs, task payloads, transcripts, and provider credentials are not serialized."
  }
}
```

`dx-agents agents social list --json` uses schema `dx.agents.zed.social_list.v1` and adds `accounts`:

```json
{
  "accounts": [
    {
      "id": "social-001",
      "platform": "telegram",
      "label": "Telegram",
      "status": "connected",
      "configured": true,
      "connected": true,
      "qr_connect_supported": false,
      "secret_state": "configured_redacted",
      "next_action": "ready_for_zed_account_state"
    }
  ]
}
```

`dx-agents agents social connect --json` and `dx-agents agents social connect --platform <platform> --json` use schema `dx.agents.zed.social_connect.v1`, write `social-connect-latest.json`, and add an `account` plus a redacted `flow`:

```json
{
  "flow": {
    "connect_supported": true,
    "qr_supported": true,
    "link_supported": false,
    "connect_method": "qr",
    "qr_payload_present": false,
    "qr_payload_redacted": true,
    "link_payload_present": false,
    "link_payload_redacted": true,
    "explicit_user_action_required": true,
    "refresh_command": "dx-agents agents social list --json",
    "safe_config_state": "not_configured"
  }
}
```

When Zed supplies `--platform <platform>`, the social action receipt `command` field records the matched sanitized platform-specific invocation, for example `dx-agents agents social connect --platform telegram --json`. If the requested value does not match a supported account, receipts use `--platform <unmatched>` and the recovery hint does not echo the requested value. The contract marks `requested_platform_values` and `unmatched_platform_values` as never-render fields so Zed does not display accidental token-like input. The bridge never serializes tokens, account targets, QR payloads, link payloads, cookies, channel user IDs, or unmatched platform text.

`dx-agents agents social disconnect --json` and `dx-agents agents social disconnect --platform <platform> --json` use schema `dx.agents.zed.social_disconnect.v1`, write `social-disconnect-latest.json`, and record provider-owned revoke/logout readiness without exporting or purging token payloads from Zed.

`dx-agents agents automate list --json` uses schema `dx.agents.zed.automation_list.v1` and adds `automations`:

```json
{
  "automations": [
    {
      "id": "automation-001",
      "job_type": "agent",
      "status": "ready",
      "enabled": true,
      "source": "declarative",
      "schedule_kind": "cron",
      "next_run": null,
      "last_status": null,
      "prompt_redacted": true,
      "command_redacted": false,
      "delivery_configured": false,
      "allowed_tools_count": 1,
      "next_action": "automation_ready_for_zed"
    }
  ]
}
```

`dx-agents agents run --json` uses schema `dx.agents.zed.run_receipt.v1`, writes a metadata-only queued receipt, and adds `task_id`.

`dx-agents agents receipts list --json` uses schema `dx.agents.zed.receipts_list.v1`, writes `receipts-list-latest.json`, and lists known bridge receipt files for the Zed background task and receipt rails. It reads only scalar metadata fields from known receipt filenames and `agent-run-<timestamp>-<pid>.json` receipts. Token-like scalar metadata is replaced with `<redacted>` and marked through `metadata_redacted`. The bridge contract's `receipts_list.item_render_fields` is the row schema Zed should use for the receipt rail:

```json
{
  "schema_version": "dx.agents.zed.receipts_list.v1",
  "status": "ready",
  "receipt_root": "G:\\Dx\\.dx\\receipts\\agents",
  "receipt_root_present": true,
  "receipt_count": 4,
  "returned_receipt_count": 4,
  "receipt_limit": 50,
  "truncated": false,
  "active_task_count": 1,
  "latest_receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\agent-run-20260521T000000Z-1234.json",
  "receipts": [
    {
      "id": "agent-run-20260521T000000Z-1234",
      "kind": "run",
      "schema_version": "dx.agents.zed.run_receipt.v1",
      "command": "dx-agents agents run --json",
      "generated_at": "2026-05-21T00:00:00Z",
      "task_id": "agent-run-20260521T000000Z-1234",
      "task_state": "pending",
      "status": "queued",
      "active_task": true,
      "safe_to_render": true,
      "metadata_redacted": false,
      "provider_status": "configured",
      "model_status": "configured",
      "duration_state": "not_started",
      "retry_supported": false,
      "cancel_supported": false,
      "social_connected": 1,
      "social_needs_auth": 0,
      "automation_enabled": 1,
      "automation_warning": 0,
      "receipt_path": "G:\\Dx\\.dx\\receipts\\agents\\agent-run-20260521T000000Z-1234.json",
      "last_error": null,
      "next_action": "background_task_receipt_ready_for_zed"
    }
  ]
}
```

If the receipt root is absent before `dx-agents agents receipts list --json` runs, the command writes `receipts-list-latest.json` after collecting the pre-existing state and reports `status="missing_config"`, `receipt_root_present=false`, `last_error="agent receipt root is missing"`, and `next_action="repair_agent_receipt_directory"`. This keeps the runtime-owned receipt index aligned with the root DX CLI cached inbox: an absent folder is missing configuration, while missing latest rows inside an existing folder are cold-start metadata. Unknown non-redacted run `task_state` values are treated as unsafe review rows and make the index warn; active-task counting still falls back to queued/running receipt `status` so concurrent or future-state receipts do not silently disappear from the background task rail.

`dx-agents providers list --json` uses schema `dx.agents.zed.providers_list.v1` and emits provider discovery JSON that callers can capture as `providers-list-latest.json`. `dx-agents models list --json` uses schema `dx.agents.zed.models_list.v1` and emits model discovery JSON that callers can capture as `models-list-latest.json`. Both include a `catalog` block:

```json
{
  "catalog": {
    "schema_version": "dx.agents.provider_model_catalog.v1",
    "source_format": "json_import_only",
    "normal_use_path": "binary_cache_or_cli_receipt",
    "binary_cache_path": "G:\\Dx\\.dx\\catalog\\agents\\provider-model-catalog.rkyv",
    "binary_cache_present": true,
    "binary_cache_stale": false,
    "source_hash": "<sha256>",
    "provider_count": 0,
    "model_count": 0,
    "compatibility_fallback": false,
    "safe_regeneration_command": "dx-agents providers catalog regenerate --json"
  }
}
```

`dx-agents providers catalog regenerate --json` is the safe cache regeneration command. It writes a generated binary catalog with artifact/schema version, generated timestamp, source hash, provider count, and model count. Normal app use should read the binary cache or these JSON receipts; JSON remains the import/regeneration format, not the Zed startup hot path. If any safety guard is absent, stale, or incompatible, Zed should show the `compatibility_fallback` state and offer the safe regeneration command instead of parsing source JSON directly.

Do not render or persist provider keys, channel tokens, account target values, automation prompts, shell commands, tool inputs, task payloads, or transcript bodies from these reports. Zed should render `status`, `connected_accounts_summary`, `automation_count`, `active_task_count`, `last_error`, `next_action`, and `receipt_path` first, then use the list commands for detail rows. For receipt rails, Zed should call the public `dx agents receipts list --json` bridge instead of scanning arbitrary files; the receipt index intentionally ignores unknown filenames, requires the dynamic run-receipt filename pattern, redacts sensitive scalar metadata, carries safe run-row social/auth and automation counters, and marks malformed known receipts as unsafe to render.

## Maintainability Notes

- Runtime receipt parity parser coverage now lives in `src/cli_parity_runtime_receipt_parser_tests.rs`, keeping receipt-specific command coverage out of the shared parser-test file.
- Release-gate parity parser coverage now lives in `src/cli_parity_release_gate_parser_tests.rs`, keeping release-gate command coverage out of the shared parser-test file.
- Legacy-alias parity parser coverage now lives in `src/cli_parity_legacy_alias_parser_tests.rs`, keeping legacy command coverage out of the shared parser-test file.
- Runtime-budget parity parser coverage now lives in `src/cli_parity_runtime_budget_parser_tests.rs`, keeping the shared parser-test file focused on core parity command coverage.

## Rules

- Use `[@superpowers](plugin://superpowers@openai-curated)`.
- Do not start local servers.
- Do not run heavy builds without permission.
- Keep secrets out of Zed and out of receipts.
