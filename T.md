# Integration Plan: OpenClaw + Hermes + Deer-Flow → dx-agent

## Core Principles

1. **No rewrites.** Copy actual source files from each project. Keep them in their native language.
2. **Only take what's missing.** dx-agent already has 25+ channels, MCP client, subagent delegation, token budgets, loop detection, etc. — we don't duplicate those.
3. **Use each project's native CLI/tooling.** Don't reinvent bridges — if OpenClaw already has a CLI to manage channels, use it.
4. **Python files from Hermes/Deer-Flow run as proper Python** — via PyO3 or managed subprocess with JSON-RPC, not hacky shell-outs.

---

## What dx-agent Already Has (DO NOT DUPLICATE)

After full audit of `G:\Dx\agent\crates\` and `G:\Dx\agent\src\`:

| Category | dx-agent Status |
|----------|----------------|
| **Channels (25+)** | Telegram, Discord, Slack, Email, Matrix, Nostr, IRC, Signal, WhatsApp Cloud/Web, Bluesky, Twitter, Reddit, QQ, DingTalk, Lark/Feishu, WeCom, MQTT, MoChat, ClawdTalk, LINE, Voice Call, Webhook, ACP Server, NextCloud, LinQ, WATI, Notion, iMessage, Mattermost |
| **MCP Client** | Full client with stdio/HTTP/SSE transports, deferred loading, config validation |
| **MCP Transport** | `McpTransportConn` trait with stdio, HTTP SSE implementations |
| **Subagent Delegation** | `spawn_subagent` tool, depth-1 cap, security policy inheritance, shared iteration budget |
| **Loop Detection** | 3-pattern detector (exact repeat, ping-pong, no progress) with escalation (warning→block→break) |
| **Token Budget** | Shared iteration budget, context token budget, cost tracking, history pruner, thinking budget |
| **Sandbox (5 backends)** | Docker, Firejail, Bubblewrap, Landlock, Seatbelt — auto-detect |
| **Memory (LLM Extraction)** | Two-phase consolidation, SQLite/PostgreSQL/Qdrant, FTS5, vector similarity |
| **Web Search/Fetch (5+)** | DuckDuckGo, Brave, SearXNG, Tavily, Jina AI, Firecrawl fallback |
| **Image Generation** | fal.ai integration |
| **TTS** | Voice call channel with TTS |
| **Browser Automation** | 3 backends (agent-browser CLI, rust-native, computer-use sidecar) |
| **Computer Use** | Desktop control via computer-use sidecar |
| **Binary Self-Update** | GitHub releases, SHA-256 verification, rollback |
| **Skill Improvement** | LLM-based `SkillImprover` with cooldown, validation, audit trail |
| **Skill Auto-Creation** | Automatic skill creation after multi-step tasks |
| **Config Management** | TOML, env overrides, encrypted secrets, schema export |
| **i18n** | Fluent-based locale system |
| **Security Audit** | Merkle-hash chained audit trail |
| **Plugin System** | WASM plugins with manifest/signature |
| **Estop System** | Kill-all, network-kill, domain-block, tool-freeze |
| **Hardware** | USB discovery, STM32, RPi GPIO, Arduino |
| **Observability** | Prometheus + OpenTelemetry |
| **Device Pairing** | Pairing guard with QR codes |
| **Gateway** | HTTP/WebSocket REST API |
| **Cron** | Full cron scheduler |
| **Skills** | Skills management CLI |
| **Daemon** | OS service lifecycle |
| **WebAuthn** | Feature-gated |
| **OAuth2** | Provider auth flows |

---

## OpenClaw — What's Truly Missing From dx-agent

OpenClaw (TypeScript) is huge — 5,000+ files. Most features overlap with dx-agent. These are the **unique** features worth copying:

### 1. Commitments System (12 TS files)
- **What**: Infers commitments (promises, deadlines, check-ins) from conversation context. Extracts `CommitmentCandidate` objects with kinds: `event_check_in`, `deadline_check`, `care_check_in`, `open_loop`. Stores with scheduling windows, dedup keys, confidence scores. Heartbeat policy re-evaluates pending commitments.
- **dx-agent gap**: Memory/consolidation exists but **no commitment inference** from conversation context. Cron handles scheduling but doesn't infer commitments.
- **Files**: `src/commitments/` — factory.ts, store.ts, store-writer.ts, runtime.ts, config.ts, model-selection.ts, heartbeat-policy.ts, types.ts
- **Integration**: Copy files as-is. Rust calls the commitment extraction logic via Node.js subprocess/CLI. Results feed into dx-agent's cron system for scheduling.
- **Priority**: Medium

### 2. TTS Directive System (18 TS files)
- **What**: A directive system embedded in text (e.g., `<speaking rate="slow">`) for controlling speaking rate, pitch, emphasis within text streams. Auto-mode configuration, provider registry.
- **dx-agent gap**: TTS exists but **no directive system** for fine-grained speech control.
- **Files**: `src/tts/` — directives.ts, provider-registry.ts, tts-core.ts, tts.runtime.ts, tts-config.ts, auto-mode.ts
- **Integration**: Copy as-is. The directive parsing is pure logic (no I/O), could be ported to Rust or called via Node.
- **Priority**: Low

### 3. Real-Time Transcription Abstraction (4 TS files)
- **What**: Provider-agnostic streaming STT with `RealtimeTranscriptionSession` interface, WebSocket session management.
- **dx-agent gap**: Only file-based whisper.cpp exists. No streaming STT framework.
- **Files**: `src/realtime-transcription/` — provider-types.ts, provider-registry.ts, websocket-session.ts
- **Integration**: Copy as-is. Rust spawns Node.js process for streaming audio processing.
- **Priority**: Low

### 4. Web Content Extraction Framework (3 TS files)
- **What**: Structured web content extraction with pluggable extractors.
- **dx-agent gap**: Basic web-fetch exists but **no structured extraction framework**.
- **Files**: `src/web-fetch/` — content-extractor-types.ts, content-extractors.ts, content-extraction-runtime.ts
- **Integration**: Copy as-is. Call via Node.js subprocess for content extraction tasks.
- **Priority**: Low-Medium

### 5. Auto-Reply Inline Directives (200+ TS files)
- **What**: In-chat command system with inline directives (`/compact`, `/steer`, `/context`), inline actions (`+goal`, `+compact`), reply pipeline with staging, subagent management. This is a full in-chat operating system.
- **dx-agent gap**: CLI command system exists but **no in-chat inline directives** with this depth.
- **Files**: `src/auto-reply/` (200+ files — too large to copy wholesale)
- **Integration**: Cherry-pick the inline directive concept. Copy only the directive parser and channel-aware reply pipeline patterns.
- **Priority**: Low (good patterns but too large to blindly copy)

### How to use OpenClaw's CLI

OpenClaw has a full CLI at `src/cli/` (281 files) with commands:
- `channel list/add/remove/doctor/send/bind-*` — channel management
- `gateway start/restart/get-paircode` — gateway management
- `daemon` — full daemon mode
- `mcp` — MCP management
- `plugins install/list/update/uninstall` — plugins
- `skills`, `cron`, `config`, `memory`, `providers`, `models`, `security`, `setup`

**Strategy**: Copy the channel plugin files from `extensions/<channel>/`. Don't reinvent channel management — use the `openclaw` CLI directly for:
1. Channel setup/configuration (`openclaw channel add <name> --type discord`)
2. Channel diagnostics (`openclaw channel doctor`)
3. Gateway operations (`openclaw gateway start`)

dx-agent (Rust) shells out to `openclaw <subcommand>` for channel lifecycle. The existing dx-agent gateway talks to the OpenClaw gateway via ACP/HTTP for message routing.

---

## Hermes — What's Truly Missing From dx-agent

Hermes (Python, 2,723 files) has these unique capabilities dx-agent doesn't:

### 1. LSP Integration (10 Python files) — HIGH PRIORITY
- **What**: Runs language servers (pyright, gopls, rust-analyzer, typescript-language-server) as subprocesses. Delta baseline system: snapshots diagnostics BEFORE a tool write, returns only NEW diagnostics after. Range shifting for in-flight edits. Gated on git workspace detection.
- **dx-agent gap**: **Zero LSP support.** No language server spawning, no diagnostic delta filtering.
- **Files** (`agent/lsp/`):
  - `manager.py` — orchestrates LSP lifecycle
  - `client.py` — LSP client protocol
  - `protocol.py` — JSON-RPC types, method definitions
  - `servers.py` — language server definitions (command, args, env)
  - `workspace.py` — workspace resolution
  - `cli.py` — CLI commands (`hermes lsp install/doctor/status`)
  - `install.py` — server auto-installation
  - `eventlog.py` — event logging
  - `range_shift.py` — range shifting for in-flight edits
  - `reporter.py` — diagnostic reporting
- **Integration**: Copy all 10 files as-is into `dx-agent/hermes-lsp/`. Call from Rust via:
  - **PyO3** (tight integration): Rust spawns Python LSP manager, receives diagnostics
  - The delta baseline pattern (snapshot → write → diff) is the key value
- **Why high priority**: Major differentiator for coding agents. No other Rust agent has built-in LSP.

### 2. MoA — Mixture of Agents (2 Python files) — HIGH PRIORITY
- **What**: `/moa` slash command gathers "reference model" context from multiple LLMs in parallel before the main model responds. Per-reference token accounting at reference model's own rate. Reference prompt frames the model as analyst only (no tool calls).
- **dx-agent gap**: **Nothing equivalent.**
- **Files**: `agent/moa_loop.py` (893 LOC), `agent/moa_trace.py`
- **Integration**: Copy as-is into `dx-agent/hermes-moa/`. The parallel LLM invocation logic is pure Python. Rust calls MoA via PyO3: passes the conversation state, receives reference context, injects into main model prompt.
- **Why high priority**: Unique quality improvement. No cost for full ensembling — uses advisory models.

### 3. Coding Context Awareness (1 Python file) — HIGH PRIORITY
- **What**: Single 883-line module that decides coding vs general posture. Detects workspaces via git repo OR project markers (`Cargo.toml`, `pyproject.toml`, `package.json`). `RuntimeMode` (coding/general) resolved ONCE and immutable. Affects: system prompt (operating brief + live git snapshot), toolset selection (focus mode collapses to coding tools), delegation propagation.
- **dx-agent gap**: **No formal coding posture system.** No workspace detection, no automatic toolset switching.
- **File**: `agent/coding_context.py` (883 LOC)
- **Integration**: Copy as-is into `dx-agent/hermes-coding-context/`. The logic is pure — no async I/O except workspace detection. Call via PyO3 at session start to determine mode.
- **Why high priority**: Elegant single-module solution. Makes the agent behave appropriately without re-deriving context decision everywhere.

### 4. Curator — Skill Lifecycle Management (3 Python files) — MEDIUM
- **What**: Background system tracks usage on agent-created skills. Auto-archives stale skills (active→stale→archived). Never deletes — archives to `.archive/`. Pinned skills exempt. Tracks: use_count, view_count, patch_count, last_activity_at, state, pinned.
- **dx-agent gap**: Skills management exists but **no lifecycle management, no usage tracking, no auto-archiving** for agent-created skills.
- **Files**: `agent/curator.py` (1,976 LOC), `agent/curator_backup.py`, `tools/skill_usage.py` (947 LOC)
- **Integration**: Copy as-is. The curator runs on idle detection (background thread). Rust triggers curator pass via PyO3 after detecting idle period.
- **Why medium priority**: Useful for long-running deployments. dx-agent's skill improvement already does LLM-based improvement.

### 5. Background Review System (1 Python file) — MEDIUM
- **What**: After every agent turn, forks a daemon thread that replays the conversation snapshot. Asks: "should any skill/memory be saved or updated?" Writes go to memory + skill stores. Never touches main session's prompt cache.
- **dx-agent gap**: **Nothing equivalent.**
- **File**: `agent/background_review.py`
- **Integration**: Copy as-is. Rust agent loop calls `background_review.py` via PyO3 after each turn, passing the conversation snapshot. The review runs in a separate thread (same as Hermes does).
- **Why medium priority**: Powerful quality pattern, but adds latency/complexity.

### 6. Session Insights Engine (4 Python files) — MEDIUM
- **What**: Analyzes historical session data from SQLite — token consumption, cost estimates, tool usage patterns, activity trends, model/platform breakdowns.
- **dx-agent gap**: **No usage analytics/insights.** Only raw metrics.
- **Files**: `agent/insights.py` (921 LOC), `agent/account_usage.py`, `agent/credits_tracker.py`, `agent/usage_pricing.py`
- **Integration**: Copy as-is. The insights query the same SQLite database dx-agent uses. Python queries the DB and produces reports. Rust calls via PyO3.
- **Why medium priority**: Useful for users who want to track costs and usage.

### 7. Multi-Environment Terminals (10 Python files) — MEDIUM
- **What**: Pluggable execution environments: local, Docker, SSH, Modal, Daytona, Singularity.
- **dx-agent gap**: Docker exists as sandbox only. **No SSH/Modal/Daytona/Singularity terminal backends.**
- **Files**: `tools/environments/` — base.py, local.py, docker.py, ssh.py, modal.py, managed_modal.py, daytona.py, singularity.py, registry.py, exec.py
- **Integration**: Copy as-is. The environment abstraction is pure Python. Each environment implements `Environment` ABC with `exec_command()`, `write_file()`, `read_file()`. Rust delegates environment execution to Python.
- **Why medium priority**: Enterprise/cloud deployments need remote execution.

### 8. Iteration Budget System (1 Python file) — LOW
- **What**: Per-run budget tracking — limits iterations, time, or tokens. Grace call mechanism for one extra turn when budget almost exhausted.
- **dx-agent gap**: Has max_iterations but **no comprehensive budget** with token/time tracking and grace calls.
- **File**: `agent/iteration_budget.py`
- **Integration**: Copy and use Python logic, or port to Rust (it's small).

### 9. Self-Healing Infrastructure (multiple files) — LOW
- **What**: Runtime self-healing: stale-model portal drift recovery, dead-target garbage collection, SSL cert rotation, state DB malformed repair, split-brain session lock self-heal.
- **dx-agent gap**: **No self-healing.**
- **Files**: Scattered across codebase
- **Integration**: Cherry-pick patterns. Most are infrastructure-specific.

### 10. Skin/Theme Engine (1 Python file) — LOW
- **What**: YAML-based CLI visual customization. Banner colors, spinner faces, tool emojis, prompt symbols.
- **dx-agent gap**: Hardcoded theme. **No skin engine.**
- **File**: `hermes_cli/skin_engine.py` (926 LOC, self-contained)
- **Integration**: Copy as-is. Python reads YAML skins. Rust TUI calls Python for theme resolution. Or port to Rust (small file, pure logic).

---

## Deer-Flow — What's Truly Missing From dx-agent

Deer-Flow (Python, LangGraph-based) has these unique capabilities:

### 1. Middleware Pipeline Architecture (27 Python files) — HIGH PRIORITY
- **What**: 27 strictly-ordered middleware classes. Shared base (10) + lead-only (17). Each implements `wrap_model_call` (pre) and `after_model` (post). This is the architecture that enables everything else.
- **dx-agent gap**: Has policy guards and output filtering but **no formal composable middleware pipeline**.
- **Files** (`backend/packages/harness/deerflow/agents/middlewares/`):
  - **Shared (10)**: InputSanitization, ToolOutputBudget, ThreadData, Uploads, SandboxAudit, DanglingToolCall, LLMErrorHandling, Guardrail, SandboxAudit, ToolErrorHandling
  - **Lead-only (17)**: DynamicContext, SkillActivation, Summarization, TodoList, TokenUsage, Title, Memory, ViewImage, DeferredToolFilter, DelegationLedger, SystemMessageCoalescing, SubagentLimit, LoopDetection, TokenBudget, SafetyFinishReason, Clarification, Custom
- **Integration**: Copy the architecture pattern. The middleware chain is the **container** — each middleware is a discrete capability. Implement in Rust as a `Middleware` trait with ordered execution. Then individual middlewares can be ported or bridged to Python.

### 2. Guardrail System (4 Python files) — HIGH PRIORITY
- **What**: Pluggable `GuardrailProvider` for pre-tool-call authorization. Built-in `AllowlistProvider` (allow/deny lists). Extensible via custom providers.
- **dx-agent gap**: Channel-level allowlists exist but **no per-tool-call guardrail system** with pluggable providers.
- **Files**: `backend/packages/harness/deerflow/guardrails/` — builtin.py, middleware.py, provider.py, __init__.py
- **Integration**: Copy as-is. Call via PyO3 before each tool call. The GuardrailProvider ABC is the key pattern.

### 3. MCP Session Pool (1 Python file) — HIGH PRIORITY
- **What**: Persistent MCP sessions scoped by `(server_name, scope_key)` with LRU eviction (max 256). Solves stateful MCP server problem (Playwright losing state between calls).
- **dx-agent gap**: MCP client creates fresh sessions per call. **No session pool.**
- **File**: `backend/packages/harness/deerflow/mcp/session_pool.py` (455 LOC)
- **Integration**: Copy as-is. The session pool logic is pure. Can be ported to Rust or called via PyO3.

### 4. MCP OAuth Support — HIGH PRIORITY
- **What**: Supports `client_credentials` and `refresh_token` flows for MCP over HTTP/SSE. Automatic token refresh with Authorization header injection.
- **dx-agent gap**: Only static headers supported in MCP config. **No OAuth.**
- **Integration**: Copy the OAuth flow logic. Port to Rust's MCP client (it's protocol-level, small surface).

### 5. Deferred MCP Tools / tool_search — MEDIUM
- **What**: MCP tool schemas hidden from the bound model until `tool_search` promotes them. Hash-scoped per-thread promotions. Reduces prompt bloat from large MCP catalogs.
- **dx-agent gap**: Loads MCP tools eagerly. **No deferred/promoted pattern.**
- **Integration**: Copy the promotion logic. Port to Rust's MCP tool loading (touches `mcp_deferred.rs`).

### 6. Plan Mode / TodoList — MEDIUM
- **What**: `write_todos` tool for structured task tracking. One task in_progress at a time. Real-time updates. Runtime config `is_plan_mode`.
- **dx-agent gap**: **No plan mode** as a distinct agent persona with task tracking.
- **Integration**: Copy the pattern. The TodoList concept is clean. Implement as a Rust tool + middleware.

### 7. Delegation Ledger — MEDIUM
- **What**: `DelegationLedgerMiddleware` maintains ledger in `ThreadState.delegations`. Prevents re-delegating completed work. Terminal status never downgraded. Re-injects as system reminder.
- **dx-agent gap**: Subagent delegation exists but **no delegation ledger** to prevent duplicate work.
- **Integration**: Copy the logic. Port to Rust's subagent system.

### 8. System Message Coalescing — MEDIUM
- **What**: Merges all SystemMessages into one. Fixes constraint in vLLM/SGLang/Anthropic that require single system message.
- **dx-agent gap**: **No system message coalescing.** Could hit provider constraints.
- **Integration**: Simple Rust utility function. Copy the pattern.

### 9. Safety Finish Reason Handler — MEDIUM
- **What**: Suppresses tool execution when provider flags safety termination. Prevents wasted tool calls after safety rejection.
- **dx-agent gap**: **No handling** of provider safety finish reasons.
- **Integration**: Copy the logic. Port to Rust's agent loop.

### 10. Clarification Interception — MEDIUM
- **What**: Intercepts `ask_clarification` tool and pauses execution for user input. Must be LAST middleware.
- **dx-agent gap**: **No clarification flow.** Agent just guesses when ambiguous.
- **Integration**: Copy the tool + middleware pattern.

### 11. Document Conversion (part of Uploads) — MEDIUM
- **What**: Auto-converts PDF, PPT, Excel, Word to markdown on upload via `markitdown`.
- **dx-agent gap**: Has `rag-pdf` feature but **no auto-conversion** for Office formats.
- **Integration**: Copy the upload processing logic. Python `markitdown` library does the conversion.

### 12. Dynamic Content Extractors (community providers) — MEDIUM
- **What**: Pluggable content extraction providers (Brave, Browserless, DuckDuckGo, Exa, Firecrawl, Jina, SearXNG, Serper, Tavily).
- **dx-agent gap**: Has DuckDuckGo and Tavily but **missing**: Brave, Browserless, Exa, Firecrawl, Jina AI, SearXNG, Serper.
- **Files**: 14 provider directories in `backend/packages/harness/deerflow/community/`
- **Integration**: Copy provider implementations as-is. Each is a thin Python wrapper around an API. Rust calls via PyO3 or HTTP.

### 13. Self-Modifying Agent Tools (setup_agent / update_agent) — MEDIUM
- **What**: `setup_agent` tool persists new agent's SOUL.md and config.yaml. `update_agent` tool self-updates from inside chat with partial update + atomic write.
- **dx-agent gap**: **No self-modifying tools** that let the agent change its own persona/config at runtime.
- **Integration**: Copy the pattern. The atomic-write with partial update is the key value.

### 14. Feedback Store — LOW
- **What**: User ratings (thumbs up/down) and comments on agent responses. Stored in SQLite.
- **dx-agent gap**: **No feedback collection.**
- **Integration**: Copy the store schema and logic.

### 15. Subagent Status Contract — LOW
- **What**: JSON contract defining subagent statuses, shared between frontend and backend.
- **dx-agent gap**: **No formal status contract.**
- **Integration**: Copy the contract file and validation pattern.

---

## Features Already in dx-agent (CONFIRMED, NOT MENTIONED AGAIN)

- Subagent delegation ✓
- Browser automation ✓
- Computer use (sidecar) ✓
- Image generation ✓
- TTS ✓
- MCP client ✓
- Loop detection ✓
- Token budget ✓
- Sandbox (5 backends) ✓
- Memory (LLM extraction) ✓
- Web search/fetch ✓
- Binary self-update ✓
- Skill auto-improvement ✓
- i18n ✓
- Security audit ✓
- WASM plugin system ✓
- Estop system ✓
- Hardware discovery ✓
- Prometheus/OTel ✓
- Cron scheduling ✓
- Gateway HTTP/WS ✓
- Device pairing ✓
- OAuth2 ✓
- Channel management ✓
- Session management ✓
- Config management ✓
- Changelog generation ✓
- Shell completions ✓

---

## Integration Architecture (Updated)

### Approach: Native CLIs + PyO3

Instead of building generic "bridges", we use each project's native entry points:

```
┌──────────────────────────────────────────────────────────┐
│                    dx-agent (Rust)                        │
│                                                          │
│  ┌─────────────────────┐      ┌──────────────────────┐  │
│  │ Agent Loop           │      │ Gateway (HTTP/WS)    │  │
│  │ (existing)           │      │ (existing)           │  │
│  │ + Middleware Pipeline│      │ + OpenClaw channel   │  │
│  │ + LSP integration    │      │   routing via        │  │
│  │ + MoA                │      │   `openclaw channel` │  │
│  │ + Coding Context     │      │   CLI commands       │  │
│  │ + Guardrails         │      └──────────────────────┘  │
│  └──────────┬───────────┘                                │
│             │                                            │
│    ┌────────┴────────┐                                   │
│    │ PyO3 / subprocess│                                   │
│    └────────┬────────┘                                   │
│             │                                            │
│  ┌──────────┴──────────────────────────────────────┐     │
│  │              Python Sidecar                       │     │
│  │  ┌─────────────┐  ┌──────────┐  ┌────────────┐  │     │
│  │  │ Hermes LSP  │  │ Hermes   │  │ Deer-Flow  │  │     │
│  │  │ (lsp/*.py)  │  │ MoA      │  │ Middleware  │  │     │
│  │  │             │  │ (moa_*.py)│  │ Guards     │  │     │
│  │  ├─────────────┤  ├──────────┤  ├────────────┤  │     │
│  │  │ Hermes      │  │ Hermes   │  │ Deer-Flow  │  │     │
│  │  │ Coding Ctx  │  │ Curator/ │  │ MCP Pool   │  │     │
│  │  │ (coding_ctx)│  │ Review   │  │ (session)  │  │     │
│  │  └─────────────┘  └──────────┘  └────────────┘  │     │
│  └──────────────────────────────────────────────────┘     │
│                                                          │
│  ┌──────────────────────────────────────────────────┐     │
│  │              OpenClaw (Node.js)                    │     │
│  │  Via `openclaw channel <subcommand>` CLI           │     │
│  │  - Channel setup/doctor/remove                     │     │
│  │  - Gateway daemon for channel message routing       │     │
│  │  - Channel plugins from copied extensions/          │     │
│  └──────────────────────────────────────────────────┘     │
└──────────────────────────────────────────────────────────┘
```

### File Structure After Integration

```
dx-agent/
├── src/                            # Rust core (existing)
├── crates/                         # Rust crates (existing)
│
├── inspirations/                   # Originals (existing)
│   ├── openclaw/
│   ├── hermes-agent/
│   └── deer-flow/
│
├── channels/                       # Copied from OpenClaw extensions/
│   ├── discord/                    # Only UNIQUE channels:
│   ├── twitch/                     # - Twitch
│   ├── msteams/                    # - MS Teams
│   ├── googlechat/                 # - Google Chat
│   ├── synology-chat/              # - Synology Chat
│   ├── tlon/                       # - Tlon/Urbit
│   ├── sms/                        # - SMS
│   ├── google-meet/                # - Google Meet
│   ├── phone-control/              # - Phone Control
│   ├── parallel/                   # - Parallel dispatch
│   ├── qa-channel/                 # - Q&A workflow
│   └── workboard/                  # - Workboard/Kanban
│
├── python-sidecar/                 # Python files from Hermes + Deer-Flow
│   ├── lsp/                        # Hermes LSP (10 files)
│   ├── moa/                        # Hermes MoA (2 files)
│   ├── coding_context/             # Hermes Coding Context (1 file)
│   ├── curator/                    # Hermes Curator (3 files)
│   ├── background_review/          # Hermes Background Review (1 file)
│   ├── insights/                   # Hermes Insights (4 files)
│   ├── skin_engine/                # Hermes Skin Engine (1 file)
│   ├── environments/               # Hermes Multi-Environment (10 files)
│   ├── iteration_budget/           # Hermes Iteration Budget (1 file)
│   ├── guardrails/                 # Deer-Flow Guardrails (4 files)
│   ├── mcp_session_pool/           # Deer-Flow MCP Session Pool (1 file)
│   ├── content_extractors/         # Deer-Flow Community Providers (14 dirs)
│   ├── document_conversion/        # Deer-Flow Document Conversion
│   └── commitments/                # OpenClaw Commitments (12 files)
│
├── crates/dx-agent-middleware/      # NEW Rust crate: Middleware Pipeline
│   ├── src/
│   │   ├── traits.rs               # Middleware trait
│   │   ├── pipeline.rs             # Ordered execution
│   │   ├── guardrail.rs            # Ported from Deer-Flow
│   │   ├── deferred_mcp.rs         # Ported from Deer-Flow
│   │   ├── delegation_ledger.rs    # Ported from Deer-Flow
│   │   ├── system_coalesce.rs      # Ported from Deer-Flow
│   │   ├── safety_finish.rs        # Ported from Deer-Flow
│   │   ├── clarification.rs        # Ported from Deer-Flow
│   │   ├── plan_mode.rs            # Ported from Deer-Flow
│   │   └── tool_output_budget.rs   # Ported from Deer-Flow
│
├── crates/dx-agent-lsp/             # NEW Rust crate: LSP Integration
│   ├── src/
│   │   ├── bridge.rs               # PyO3 bridge to Hermes LSP
│   │   ├── types.rs                # Diagnostic types
│   │   └── workspace.rs            # Workspace detection
│
└── crates/dx-agent-commitments/     # NEW Rust crate: Commitments
    ├── src/
    │   ├── bridge.rs               # PyO3/Node bridge
    │   └── scheduler.rs            # Cron integration
```

### Priority Implementation Order

| Phase | Features | Effort | Impact |
|-------|----------|--------|--------|
| **1** | Middleware Pipeline architecture, LSP Integration, Coding Context | ~2 weeks | Highest — foundation for everything else |
| **2** | Guardrails, MCP Session Pool + OAuth, Deferred MCP Tools | ~1 week | High — security + MCP maturity |
| **3** | MoA, Delegation Ledger, System Coalescing, Safety Finish | ~1 week | Medium — quality improvements |
| **4** | Plan Mode, Clarification, Loop Detection (enhance existing) | ~1 week | Medium — structured execution |
| **5** | OpenClaw unique channels (Twitch, MS Teams, etc.) | ~1 week | Medium — platform coverage |
| **6** | Curator, Background Review, Commitments | ~1 week | Medium — self-improvement |
| **7** | Multi-Environment Terminals, Insights Engine, Doc Conversion | ~1 week | Medium — enterprise features |
| **8** | Content Extractors, Feedback Store, Skin Engine, Self-Healing | ~1 week | Low — polish |
