# DX Agents — Codebase Reference

## Overview

**DX Agents** (`dx-agent` v0.8.0-beta-2) is a Rust-first AI agent runtime forked from the DX Agent / ZeroClaw project. It provides provider-agnostic LLM routing, multi-channel messaging, memory backends, cron scheduling, skills/SOPs, hardware control, an HTTP/WebSocket gateway, and a Zed GPUI bridge.

- **Binary**: `dx-agent` (also `dx-agent-acp-bridge` with `acp-bridge` feature)
- **Library**: `dx_agents`
- **Repository**: `https://github.com/millercarla211-ctrl/dx-agent`
- **License**: MIT OR Apache-2.0
- **Rust edition**: 2024, MSRV 1.87
- **Author**: `theonlyhennygod`

---

## Build Status

| Check | Status |
|---|---|
| `cargo check --lib -p dx-agent` | ✅ PASSES |
| `cargo check --all-targets` | ❌ FAILS — 2 errors in `benches/agent_benchmarks.rs` |
| `cargo test --lib -p dx-agent` | ✅ PASSES (quick tests) |
| `cargo fmt --check` | Unknown (not tested recently) |
| GitHub Actions CI | 🔴 ALL WORKFLOWS DISABLED |

### Known Build Failures

`benches/agent_benchmarks.rs` has 2 compile errors:
1. `unresolved import dx_agents::providers::Provider` — `Provider` was moved/renamed in the crate API.
2. `NoopTool` doesn't implement `dx_agent_api::attribution::Attributable` — the `Tool` trait requires this sealed supertrait.

These only affect `--all-targets` / `cargo bench`. Library + binary builds work fine.

---

## CLI Commands

All commands via `dx-agent <subcommand>`:

| Command | Description | Status |
|---|---|---|
| `quickstart` | One-shot agent setup (flags or interactive checklist) | ✅ Works |
| `agent -a <name>` | Interactive agent session | ✅ Works |
| `agent -a <name> -m "..."` | Single message (non-interactive) | ✅ Works |
| `gateway start` | Start HTTP/WebSocket gateway | ✅ Works |
| `gateway restart` | Restart gateway | ✅ Works |
| `gateway get-paircode` | Show pairing code | ✅ Works |
| `daemon` | Full autonomous daemon (gateway + channels + cron) | ✅ Works |
| `service install/start/stop/restart/status/uninstall/logs` | OS service lifecycle | ✅ Works |
| `config list/get/set/init/schema` | Config management | ✅ Works |
| `channel list/add/remove/doctor/send/bind-telegram` | Channel management | ✅ Works |
| `cron list/add/add-at/add-every/once/remove/update/pause/resume` | Scheduled tasks | ✅ Works |
| `memory list/get/stats/clear/reindex` | Memory management | ✅ Works |
| `models list` | Provider model catalog | ✅ Works |
| `providers list` | Provider listing | ✅ Works |
| `providers catalog regenerate` | Regenerate provider catalog binary cache | ✅ Works |
| `skills list/add/edit/bundle/audit/install/remove/test` | Skills management | ✅ Works |
| `sop list/validate/show` | SOP management | ✅ Works |
| `hardware discover/introspect/info` | USB hardware discovery | ✅ Works |
| `peripheral list/add/flash/setup-uno-q/flash-nucleo` | Peripheral management | ✅ Works |
| `migrate openclaw` | Import from OpenClaw workspace | ✅ Works |
| `update --check/--force/--version` | Self-update | ✅ Works |
| `self-test --quick` | Diagnostic self-tests | ✅ Works |
| `doctor` | Health diagnostics | ✅ Works |
| `status` | System status | ✅ Works |
| `estop` | Emergency-stop controls | ✅ Works |
| `completions bash/fish/zsh/powershell/elvish` | Shell completions | ✅ Works |
| `acp` | ACP JSON-RPC 2.0 server over stdio | ✅ Works |
| `browse <path>` | Browse shared workspace | ✅ Works |
| `integrations info` | Integration details | ✅ Works |
| `locales fetch <locale>` | Download translation files | ✅ Works |
| `desktop --install` | Companion desktop app | ✅ Works |
| `auth login/refresh/logout/list/status` | Provider auth profiles | ✅ Works |
| `plugin` (feature: plugins-wasm) | WASM plugin management | ✅ Works |
| `onboard` | DEPRECATED — use quickstart | ⚠️ Hidden |
| `props` | DEPRECATED — use config | ⚠️ Hidden |

### Zed Bridge Commands (JSON receipts)

These emit JSON receipts for the Zed GPUI integration:

```
dx-agents agents contract --json
dx-agents agents contract-audit --json
dx-agents agents snapshot --json
dx-agents agents status --json
dx-agents agents social list/connect/disconnect --json
dx-agents agents automate list --json
dx-agents agents run --json
dx-agents agents receipts list --json
dx-agents providers list --json
dx-agents providers catalog regenerate --json
dx-agents models list --json
```

Public aliases: `dx agents ... --json` (same commands, shorter prefix).

---

## Build Commands

```powershell
# Quick library check (fast)
cargo check --lib -p dx-agent

# Full check (includes benches — will fail until bench is fixed)
cargo check --all-targets

# Debug build
cargo build

# Release build (optimized for size: z + fat LTO + panic=abort)
cargo build --release

# Fast release build (parallel codegen)
cargo build --profile release-fast

# CI build (thin LTO, fast)
cargo build --profile ci

# Tests
cargo test --lib -p dx-agent
cargo test -p dx-agent-config dx_agents_env
cargo test -p dx-agent-providers resolve_provider_credential

# Format
cargo fmt --check

# Justfile build (release + copy to G:\Dx\bin)
just build
```

---

## Feature Flags

Default features: `agent-runtime`, `default-channels`, `acp-bridge`, `gateway`, `observability-prometheus`, `schema-export`.

### Major Subsystems

| Feature | What it enables |
|---|---|
| `agent-runtime` | Full agent loop, tools, persistence, cron, skills, TUI |
| `gateway` | HTTP/WebSocket REST API + dashboard |
| `acp-bridge` | Agent Control Protocol (JSON-RPC 2.0 stdio) |
| `schema-export` | JSON Schema export for config |
| `observability-prometheus` | Prometheus metrics |
| `observability-otel` | OpenTelemetry tracing |

### Channels (messaging platforms)

| Feature | Channel |
|---|---|
| `default-channels` | ACP server + Webhook + Email + Telegram |
| `channels-full` | All ~25 channels |
| `channel-telegram` | Telegram bot |
| `channel-discord` | Discord bot |
| `channel-slack` | Slack app |
| `channel-email` | Email (IMAP/SMTP) |
| `channel-matrix` | Matrix |
| `channel-nostr` | Nostr |
| `channel-irc` | IRC |
| `channel-signal` | Signal |
| `channel-whatsapp-cloud` | WhatsApp Cloud API |
| `channel-bluesky` | Bluesky |
| `channel-twitter` | Twitter/X |
| `channel-reddit` | Reddit |
| `channel-qq` | QQ |
| `channel-dingtalk` | DingTalk |
| `channel-lark` / `channel-feishu` | Lark / Feishu |
| `channel-wecom` / `channel-wecom-ws` | WeCom |
| `channel-mqtt` | MQTT |
| `channel-mochat` | MoChat |
| `channel-clawdtalk` | ClawdTalk |
| `channel-line` | LINE |
| `channel-voice-call` | Voice calls |
| `channel-webhook` | Generic webhooks |
| `channel-acp-server` | ACP over gateway |
| `channel-nextcloud` | NextCloud |
| `channel-linq` | LinQ |
| `channel-wati` | WATI |
| `channel-notion` | Notion |
| `channel-imessage` | iMessage |

### Backends & Platform

| Feature | What it enables |
|---|---|
| `hardware` | USB/serial hardware discovery + peripherals |
| `peripheral-rpi` | Raspberry Pi GPIO (rppal) |
| `probe` | probe-rs chip debugging |
| `dev-sim` | Simulator serial ports for dev |
| `sandbox-landlock` | Linux Landlock sandboxing |
| `sandbox-bubblewrap` | Bubblewrap sandboxing |
| `browser-native` | Browser automation (fantoccini) |
| `plugins-wasm` | WASM plugin execution |
| `rag-pdf` | PDF RAG support |
| `webauthn` | WebAuthn authentication |
| `memory-postgres` | PostgreSQL memory backend |
| `embedded-web` | Embedded web dashboard (include_dir) |
| `whatsapp-web` | WhatsApp Web (non-Cloud) |
| `voice-wake` | Voice wake word detection |
| `ci-all` | Meta-feature: all of the above |

---

## Workspace Structure (21 crates)

### Root

| Crate | Path | Description |
|---|---|---|
| `dx-agent` | `./` | CLI binary + library entrypoint |

### Core Crates

| Crate | Path | Description |
|---|---|---|
| `dx-agent-api` | `crates/dx-agent-api` | Trait definitions, shared types, attribution |
| `dx-agent-config` | `crates/dx-agent-config` | Config schema, secrets, env overrides, migration |
| `dx-agent-log` | `crates/dx-agent-log` | Structured event schema, JSONL persistence, tracing |
| `dx-agent-spawn` | `crates/dx-agent-spawn` | Attribution-propagating tokio::spawn wrapper |
| `dx-agent-macros` | `crates/dx-agent-macros` | Proc macros for config field derivation |

### Subsystems

| Crate | Path | Description |
|---|---|---|
| `dx-agent-providers` | `crates/dx-agent-providers` | LLM providers (Anthropic, OpenAI, Gemini, Groq, Ollama...), auth, multimodal |
| `dx-agent-memory` | `crates/dx-agent-memory` | Memory backends (SQLite, PostgreSQL), embeddings, consolidation |
| `dx-agent-channels` | `crates/dx-agent-channels` | 25+ messaging platform channel implementations |
| `dx-agent-tools` | `crates/dx-agent-tools` | Agent-callable tool implementations |
| `dx-agent-runtime` | `crates/dx-agent-runtime` | Agent loop, security, cron, SOP, skills, observability, TUI, i18n |
| `dx-agent-infra` | `crates/dx-agent-infra` | Channel infrastructure: sessions, debouncing, stall watchdog |
| `dx-agent-tool-call-parser` | `crates/dx-agent-tool-call-parser` | Parses tool calls from LLM responses (JSON, XML, GLM, etc.) |

### Gateway & Hardware

| Crate | Path | Description |
|---|---|---|
| `dx-agent-gateway` | `crates/dx-agent-gateway` | HTTP/WebSocket REST API, web dashboard, webhooks |
| `dx-agent-hardware` | `crates/dx-agent-hardware` | USB discovery, peripherals (STM32, RPi GPIO, Arduino) |
| `dx-agent-plugins` | `crates/dx-agent-plugins` | WASM plugin host, manifests, signatures |

### Legacy / Internal

| Crate | Path | Description |
|---|---|---|
| `aardvark-sys` | `crates/aardvark-sys` | Total Phase Aardvark I2C/SPI/GPIO USB adapter bindings (stub) |
| `zeroclaw-robot-kit` | `crates/robot-kit` | Robot control toolkit (drive, vision, speech, sensors) |

### Apps & Tools

| Crate | Path | Description |
|---|---|---|
| `zerocode` | `apps/agentcode` | TUI onboarding/configuration app |
| `dx-agents-desktop` | `apps/tauri` | Tauri desktop system tray companion |
| `fill-translations` | `tools/fill-translations` | Translation file filler |
| `xtask` | `xtask` | Dev tools (mdbook, fluent, web) |

---

## Architecture Highlights

### Config
- TOML-based config with env var override (`DX_AGENT_<path>`)
- Schema export via schemars
- Encrypted secret storage (AEAD with chacha20poly1305)
- Migration system from legacy paths
- Presets system for quickstart (risk profiles, runtime profiles)

### Providers
- Anthropic, OpenAI (with Codex responses API), Gemini, Groq, Ollama, OpenRouter
- OpenAI-compatible custom endpoints
- Provider-agnostic routing with model aliases
- Binary catalog cache (rkyv + memmap2)
- Reliable provider wrapper with retry/backoff
- OAuth2 flows for some providers

### Channels
- 25+ messaging platforms
- Session backends with debouncing and stall watchdog
- ACP (Agent Control Protocol) server for IDE/tool integration
- All channels are feature-gated individually

### Memory
- SQLite backend (default)
- PostgreSQL backend (optional)
- Embeddings with vector search
- FTS (full-text search) indexing
- Memory consolidation and categorization

### Gateway
- Axum HTTP/WebSocket server
- REST API for config, memory, cron, skills, sessions
- Web dashboard (Vite + TypeScript frontend)
- Webhook ingress
- Gateway pairing (QR codes)

### Security
- Emergency stop (estop) system with levels (kill-all, network-kill, domain-block, tool-freeze)
- Sandboxing: Landlock (Linux), Bubblewrap
- Tool receipts with approval workflows
- Secret encryption at rest
- Verifiable intent system
- Trust system for tool execution

### Zed Bridge
- JSON receipt system for Zed GPUI integration
- Contract-audit safety checks
- Receipts under `G:\Dx\.dx\receipts\agents\`
- Safe render fields + never-render fields for secret safety
- Provider catalog with binary cache

---

## CI / GitHub Actions

**ALL WORKFLOWS DISABLED.** Files moved to `.github/workflows-disabled/` with `.disabled` suffix to preserve limited GitHub Actions minutes.

Disabled workflows:
- `ci.yml` — lint + test + build on PRs
- `release-stable-manual.yml` — stable release pipeline
- `cross-platform-build-manual.yml` — full platform build matrix
- `daily-audit.yml` — daily security audit
- `pr-path-labeler.yml` — automatic PR labeling
- `pr-title.yml` — PR title validation
- `docs-deploy.yml` — documentation deployment
- `pub-aur.yml` / `pub-homebrew-core.yml` / `pub-scoop.yml` — package manager publish
- `discord-release.yml` / `tweet-release.yml` — release announcements
- Only `.github/workflows/` content is documentation (`master-branch-flow.md`, `README.md`)

### CI Configuration (when enabled)
- Targets: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc`
- Also builds: `aarch64-unknown-linux-gnu`, `armv7-unknown-linux-gnueabihf`, `arm-unknown-linux-gnueabihf`, `aarch64-linux-android`
- Tools: `cargo fmt`, `cargo clippy`, `cargo nextest`, `cargo deny`
- Feature set: `ci-all` for full feature coverage

---

## Release Profiles

| Profile | opt-level | LTO | codegen-units | strip | panic |
|---|---|---|---|---|---|
| `dev` | 0 | - | - | - | unwind |
| `release` | `z` (size) | fat | 1 | yes | abort |
| `release-fast` | `z` (size) | fat | 8 | yes | abort |
| `ci` | `z` (size) | thin | 16 | yes | abort |
| `dist` | `z` (size) | fat | 1 | yes | abort |

---

## Git History

- 11 commits on `master`
- Forked from upstream DX Agent
- Recent commits: CI fixes, Windows GH_TOKEN fix, automatic commits
- Single branch model (`master` only)
- No push to upstream — fetch-only fork

---

## Testing

### Test Harnesses

| Test | File | Purpose |
|---|---|---|
| `architecture` | `tests/test_architecture.rs` | Workspace invariant enforcement (no duplicate state) |
| `component` | `tests/test_component.rs` | Component-level tests |
| `integration` | `tests/test_integration.rs` | Integration tests |
| `system` | `tests/test_system.rs` | System-level tests |
| `live` | `tests/test_live.rs` | Live tests (require real credentials) |

### Benchmark

| Benchmark | File | Status |
|---|---|---|
| `agent_benchmarks` | `benches/agent_benchmarks.rs` | ❌ BROKEN — 2 compile errors |

### Fuzzing

- `fuzz/` directory with `fuzz_targets/` subdirectory

---

## Key Files

| File | Purpose |
|---|---|
| `src/main.rs` | CLI entrypoint, command routing, Cli struct |
| `src/lib.rs` | Library re-exports, command enum definitions |
| `build.rs` | Minimal (rerun-if-changed=build.rs) |
| `DX.md` | Launch worker notes and hardening history |
| `AGENTS.md` | This file — architecture rules and codebase reference |
| `CLAUDE.md` | Claude AI instructions |
| `Justfile` | Build shortcut (release + copy to bin) |
| `Cargo.toml` | Workspace root with all crate deps and features |
| `README.md` | Public-facing project overview |
| `CHANGELOG.md` / `CHANGELOG-next.md` | Release changelogs |
| `rustfmt.toml` | Rustfmt configuration |
| `clippy.toml` | Clippy configuration |
| `deny.toml` | Cargo deny (security advisory) configuration |
| `taplo.toml` | TOML formatter configuration |
| `release-plz.toml` | Release-plz configuration |
| `flake.nix` / `flake.lock` | Nix flake for reproducible builds |
| `locales.toml` | Locale/translation configuration |
| `install.sh` | Unix installer script |
| `setup.bat` | Windows setup script |
| `docker-compose.yml` | Docker Compose configuration |
| `Dockerfile` | Docker build files |
| `deploy-k8s/` | Kubernetes deployment samples |

---

## Additional Directories

| Directory | Contents |
|---|---|
| `docs/` | Book (mdbook), runbooks, parity docs, migration guides |
| `docs/parity/` | Parity matrices with upstream ZeroClaw/Hermes |
| `web/` | Vite + TypeScript web dashboard frontend |
| `scripts/` | CI scripts, deploy scripts, systemd service file |
| `dev/` | Dev tooling, CI helpers, sandbox configs |
| `tests/` | Test fixtures, test support modules |
| `benches/` | Criterion benchmarks |
| `fuzz/` | Cargo-fuzz targets |
| `plugins/` | External plugins (excluded from workspace) |
| `dist/` | Distribution artifacts |
| `memory/` | Memory-related data |
| `firmware/` | Hardware firmware files |
| `.cargo/` | Cargo config |
| `.circleci/` | Legacy CircleCI config |
| `.githooks/` | Git hooks |
| `.vscode/` / `.zed/` / `.gemini/` | Editor configs |
| `nix/` | Nix-related files |
| `deploy-k8s/` | Kubernetes deployment manifests |

---

## Quick Start

```powershell
# 1. Set provider API key
$env:GOOGLE_API_KEY = "your-key-here"

# 2. Quickstart (one-shot)
cargo run -- quickstart --model-provider gemini --model gemini-2.0-flash --api-key-env GOOGLE_API_KEY --agent dx

# 3. Chat with the agent
cargo run -- agent -a dx --message "Hello!"
```

Or use a pre-built binary on Windows:
```powershell
dx-agents quickstart ... --api-key-env GOOGLE_API_KEY --agent dx
dx-agents agent -a dx
```
