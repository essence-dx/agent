# DX Agents

DX Agents is a Rust-first agent runtime forked from the latest ZeroClaw codebase and shaped for the DX workspace. It keeps the upstream strengths that matter for a serious local agent: provider routing, OpenAI-compatible providers, memory, gateway pairing, cron, skills, session tooling, desktop support, and maintainable CLI contracts.

This repository intentionally keeps the internal `zeroclaw-*` workspace crates so upstream changes can be compared and merged without unnecessary module-boundary churn. Public package names, binaries, config directory names, environment aliases, and user-facing copy should stay branded as DX Agents.

## Quick Start

```powershell
cargo metadata --no-deps
$env:GOOGLE_API_KEY = "..."
cargo run --bin dx-agents -- quickstart --model-provider gemini --model gemini-2.0-flash --api-key-env GOOGLE_API_KEY --agent dx
cargo run --bin dx-agents -- agent -a dx --message "Say hello from DX Agents"
```

Secrets stay outside git and outside command history. Put provider keys in the current shell or a local ignored `.env`, then pass the variable name to Quickstart with `--api-key-env`:

```powershell
$env:GOOGLE_API_KEY = "..."
dx-agents quickstart --model-provider gemini --model gemini-2.0-flash --api-key-env GOOGLE_API_KEY --agent dx
```

Legacy readers for `ZEROCLAW_*` remain in place for migration, but new DX scripts should prefer `DX_AGENTS_*`. The self-updater defaults to `millercarla211-ctrl/dx-agents`; override it with `DX_AGENTS_UPDATE_REPO` only when testing another release repository.

## Core Commands

```powershell
dx-agents quickstart
dx-agents agent -a dx --message "Summarize this repo"
dx-agents gateway start
dx-agents gateway get-paircode --new
dx-agents cron list
dx-agents memory stats
dx-agents self-test --quick
dx-agents completions powershell
```

## What DX Agents Inherits

- Multi-channel runtime adapters from ZeroClaw.
- Provider-agnostic model routing across Anthropic, OpenAI, Ollama, OpenAI-compatible gateways, and other providers.
- Security controls around autonomy, approval, sandboxing, workspace boundaries, and tool receipts.
- Gateway and dashboard surfaces for chat, memory browsing, configuration, cron, and tool inspection.
- SOP, skills, cron, hardware, and ACP integration foundations.
- ZeroClaw v0.8 schema-mirror environment override support through `ZEROCLAW_<path>`, plus DX compatibility aliases where this fork keeps them.

## Configuration

The DX fork resolves config from the DX aliases first where supported, then legacy ZeroClaw locations for migration. A minimal provider setup can be configured through TOML or environment variables:

```toml
[providers.models.openai.coding]
model = "gpt-5-codex"
wire_api = "responses"
requires_openai_auth = true
```

Point an agent at that provider with `model_provider = "openai.coding"`.

## Development Checks

Prefer fast checks during iteration:

```powershell
cargo metadata --no-deps
cargo test -p dx-agents --test test_component provider_resolution
cargo test -p zeroclaw-config dx_agents_env
cargo test -p zeroclaw-providers resolve_provider_credential_uses_dx_agents_generic_api_key
```

Before calling a batch complete:

```powershell
cargo fmt --check
cargo test -p dx-agents --bin dx-agents
cargo check --all-targets
```

## First-Batch Parity

The implementation notes and parity matrix live in [docs/parity/openclaw-hermes-core.md](docs/parity/openclaw-hermes-core.md). That document tracks what was implemented now, what is inherited from upstream, and what is intentionally deferred.

## License

The upstream codebase is dual-licensed under MIT or Apache-2.0. Keep upstream license files and notices intact while DX Agents evolves.
