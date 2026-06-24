# AGENTS.md for G:\Dx\agent

This workspace belongs to essencefromexistence. Codex is the temporary coding partner until Friday and the DX super app are ready. Work like a senior engineer: protect the user's intent, ship real improvements, keep the codebase maintainable, and leave the repo easier to continue than you found it.

## Product Direction

- The active product is `dx-agents`, forked from latest DX Agent.
- Public branding should say DX Agents, not DX Agent.
- Keep migration and compatibility readers for old `zeroclaw`, archived `agent`, OpenClaw, and Hermes data.
- Internal `zeroclaw-*` workspace crates may remain to preserve upstream mergeability.
- Prefer real runtime features over mock UI, fake state, or decorative scaffolding.

## Single Source Of Truth

No piece of state should live in two places. Before adding a new struct field, config entry, schema field, runtime cache, or generated surface, identify the canonical source of truth.

Allowed patterns:

- A new field that is the canonical source for new data.
- Resolver closures, getters, or `&Config` parameters that read canonical state at use time.
- On-demand materialized views that are not stored as long-lived duplicate state.
- Macros or generation steps that emit multiple surfaces from one input table.

Forbidden patterns:

- Caching config-derived allowlists, credentials, provider metadata, or channel permissions in runtime handles when config is already reachable.
- Duplicating schema variants across independent tables.
- Hiding stale snapshots behind names like cache, mirror, or convenience state.
- Adding speculative config keys without a concrete caller.

## Code Quality

- Preserve working behavior unless the user explicitly asks to change it.
- Keep Rust modules small, typed, and maintainable.
- Put domain logic in libraries/services; keep CLI handlers thin.
- Use `Result` propagation instead of panics in application paths.
- Treat filesystem, process, shell, network, and secrets as high-risk surfaces.
- Never write secrets to files or logs.
- Do not leave `unwrap()` or `expect()` in production paths unless the invariant is documented and impossible to violate through application input.

## Project Snapshot

- `src/main.rs` - CLI entrypoint and command routing.
- `src/lib.rs` - module re-exports and CLI command enum definitions.
- `crates/dx-agent-api` - public trait definitions.
- `crates/dx-agent-config` - schema, config loading, migration, and env override logic.
- `crates/dx-agent-log` - unified log events and persistence.
- `crates/dx-agent-providers` - model providers and routing.
- `crates/dx-agent-channels` - messaging platform integrations.
- `crates/dx-agent-tools` - tool execution surface.
- `crates/dx-agent-runtime` - agent loop, security, cron, SOP, skills, observability, and runtime services.
- `crates/dx-agent-memory` - memory backends.
- `crates/dx-agent-infra` - shared infrastructure.
- `crates/dx-agent-gateway` - HTTP/WebSocket gateway.
- `crates/dx-agent-hardware` - peripherals, serial, GPIO, and hardware support.
- `apps/zerocode` - upstream TUI onboarding application.
- `apps/tauri` - desktop shell.
- `docs` - documentation and parity notes.
- `.github` - CI, templates, and automation workflows.

## Verification

- Use fast checks while coding:
  - `cargo metadata --no-deps`
  - targeted `cargo test` modules for provider config, memory, gateway pairing, CLI parsing, migrations, and env aliases
- Run stronger checks only at meaningful checkpoints:
  - `cargo fmt --check`
  - targeted tests for touched modules
  - `cargo check --all-targets`
- Do not repeatedly run expensive builds after every small edit.
- For docs-only changes, use lightweight checks and conflict-marker scans.

## Risk Tiers

- Low risk: docs, comments, tests-only changes, and metadata-only updates.
- Medium risk: most `crates/*/src/**` behavior changes without boundary or security impact.
- High risk: runtime security, gateway access control, tool execution, filesystem/process/network behavior, workflows, secrets, and permission boundaries.

When uncertain, classify as higher risk.

## Workflow

1. Read before writing.
2. Map non-trivial architecture, config, security, workflow, governance, CI, or agent-assisted contribution changes before editing.
3. Keep one concern per commit.
4. Implement minimal, concrete patches.
5. Validate by risk tier.
6. Document behavior changes, side effects, and rollback notes when they matter.

## Subagents

Subagents inherit the parent identity and permissions but run in isolated sessions. Before running shell commands or filesystem operations, subagents must explicitly set their working directory to the repository root containing `Cargo.toml` and `AGENTS.md`.

## Git Safety

- Inspect branch/status before edits.
- Never revert user changes unless explicitly asked.
- Keep commits focused when the user asks for commits.
- Leave unrelated dirty files alone.
- Do not push to upstream DX Agent. Upstream is fetch-only for this fork.

## Localization

- User-facing CLI messages, tool descriptions, and onboarding prompts should use Fluent strings where the surrounding code expects localization.
- Logs, tracing spans/events, and panic messages stay in English with stable identifiers where relevant.

## Anti-Patterns

- Do not add heavy dependencies for minor convenience.
- Do not silently weaken security policy or access constraints.
- Do not mix massive formatting-only changes with functional changes.
- Do not modify unrelated modules while resolving a narrow task.
- Do not bypass failing checks without explicit explanation.
- Do not include personal identity, sensitive information, or real secrets in test data, examples, docs, or commits.
