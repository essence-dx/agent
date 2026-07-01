# dx-agent Integration Plan — Feature Catalog

Raw catalog of every feature in OpenClaw, Hermes-Agent, and Deer-Flow.
No architecture analysis — just what exists, where, and how many files.
You decide what to keep.

---

## 1. OpenClaw

**Path:** `G:\Dx\agent\inspirations\openclaw`
**Language:** TypeScript (ESM)
**Total src/ files:** 9,145 in 202 subdirectories
**Total project source:** 18,713 files (excl node_modules/.git/dist)

### 1.1 src/ — Core (67 subdirectories)

| Subsystem | Files | Path |
|-----------|-------|------|
| Agent runtime | 1,021 | `src/agents/` — embedded agent, CLI agent, subagent, cron agent, ACP agent, session compactor, tool inventory, sandbox, model routing, fallback, system prompt, identity, workspace |
| HTTP/WS Gateway | 542 | `src/gateway/` — server, auth, rate limit, chat, cron, plugins, sessions, tools invoke, node pairing, OpenAI-compat endpoint, embeddings, control UI, broadcast, credentials, hooks, discovery |
| Plugin system | 534 | `src/plugins/` — loader, registry, runtime, install, uninstall, update, discovery, enable/disable, slots, hooks, marketplace, ClawHub, manifest, bundled sources, bundled channels, provider catalog, provider wizard |
| Plugin SDK | 533 | `src/plugin-sdk/` — 100+ SDK modules: channel, agent, provider, memory, ACP, config, secrets, security, browser, CLI, gateway, routing, session, cron, skills, image/video/music generation, speech, TTS, realtime transcription, web search, web fetch, sandbox, SSRF, dedupe, async locks, SQLite |
| Commands | 433 | `src/commands/` — agent, configure, channels, doctor (30+ check types), status, health, sandbox, setup, onboard, migrate, uninstall, backup, export, flows, tasks, reset, docs |
| Config | 308 | `src/config/` — Zod schemas, 40+ type files (agents, channels, models, tools, sandbox, browser, auth, secrets, approvals, hooks, ACP, MCP, cron, memory, skills, gateway, node), I/O, env vars, merge-patch, includes, secrets, legacy migration, auto-enable |
| CLI | 281 | `src/cli/` — Commander-based program, argv, banner, JSON output, daemon/gateway/cron/update/nodes/devices/secrets/plugins/channels/skills/models CLIs |
| Channels | 254 | `src/channels/` — 26 adapter interfaces, 17 core adapters, session, streaming, targets, registry, chat type, conversation, typing indicators, status reactions, thread binding, mention gating, command gating, debounce, allowlists |
| Secrets | 127 | `src/secrets/` — 3 secret source types (env/file/exec), target registry, credential matrix, config collectors per domain (channels, plugins, TTS, auth), secret scanning, JSON pointer |
| Security | 81 | `src/security/` — audit (DM policy, gateway, filesystem/Windows ACL, Docker, plugins, secrets, dangerous flags), fix/repair, context visibility, dangerous tools, install policy, safe regex, path scanning |
| Agent Control Protocol | 56 | `src/acp/` — server, client, translator, replay, event ledger, permission relay, approval classifier, persistent bindings, protocol schema |
| Talk/Voice | 42 | `src/talk/` — voice conversations |
| Tools | ~30 | `src/tools/` — types, protocol, execution, planner, descriptors, availability, diagnostics, boundary |
| LLM | ~40 | `src/llm/` — types, stream, model registry, API registry, env API keys, OAuth, session resources, model utils |
| Cron | ~15 | `src/cron/` — scheduling |
| Daemon | ~10 | `src/daemon/` — autonomous daemon |
| TUI | ~20 | `src/tui/` — terminal UI |
| Memory | ~10 | `src/memory/` — root memory files |
| Other | ~6,000 | bootstrap, compat, context-engine, chat, state, i18n, hooks, commitments, transcripts, bindings, media, image/video/music generation, media understanding, realtime transcription, web fetch, web search, talk, TTS, model catalog, node host, pairing, provider runtime, routing, skills, status, trajectory, wizard, floats/flows |

### 1.2 Channel Plugin Architecture

**Location:** `src/channels/plugins/` (153 files)
**Core type:** `ChannelPlugin<ResolvedAccount, Probe, Audit>` in `types.plugin.ts`

**26 adapter interfaces:**

1. `ChannelConfigAdapter` — config read/write, account listing, enable/disable
2. `ChannelSetupAdapter` — account setup wizard
3. `ChannelPairingAdapter` — device pairing flows
4. `ChannelSecurityAdapter` — DM policies, security audits, config fixes
5. `ChannelGroupAdapter` — group mention requirements, tool policies
6. `ChannelOutboundAdapter` — message sending, delivery capabilities
7. `ChannelStatusAdapter` — health probes, audits, diagnostics
8. `ChannelGatewayAdapter` — gateway lifecycle, QR login
9. `ChannelAuthAdapter` — login flow
10. `ChannelElevatedAdapter` — elevated access
11. `ChannelCommandAdapter` — command UI, native commands
12. `ChannelLifecycleAdapter` — config change hooks, account removal, startup maintenance
13. `ChannelSecretsAdapter` — secret targets, ref surfaces
14. `ChannelAllowlistAdapter` — allowlist config editing
15. `ChannelDoctorAdapter` — doctor checks, config migrations, repair
16. `ChannelHeartbeatAdapter` — health checks, typing indicators
17. `ChannelConfiguredBindingProvider` — conversation bindings
18. `ChannelConversationBindingSupport` — conversation binding lifecycle
19. `ChannelMessageActionAdapter` — message action discovery
20. `ChannelMessagingAdapter` — core messaging
21. `ChannelAgentPromptAdapter` — agent prompt customization
22. `ChannelMentionAdapter` — mention handling
23. `ChannelStreamingAdapter` — streaming messages
24. `ChannelThreadingAdapter` — thread support
25. `ChannelResolverAdapter` — target resolution
26. `ChannelDirectoryAdapter` — directory listing

**ChannelMeta:** id, label, selectionLabel, docsPath, blurb, order, aliases, markdownCapable, exposure, showConfigured, preferOver, quickstartAllowFrom
**ChannelCapabilities:** chatTypes, polls, reactions, edit, unsend, reply, effects, groupManagement, threads, media, tts, nativeCommands, blockStreaming

### 1.3 Extensions (140 directories)

**LLM Providers (~50):** Anthropic, Anthropic Vertex, OpenAI, Google, Google Vertex, Groq, Mistral, Cohere, DeepSeek, Qwen, Alibaba, Moonshot, StepFun, MiniMax, Cerebras, DeepInfra, Fireworks, Together, Novita, Perplexity, xAI, ZAI, Venice, HuggingFace, Ollama, LM Studio, LlamaCpp, vLLM, SGLang, LiteLLM, OpenRouter, Cloudflare AI Gateway, Vercel AI Gateway, AWS Bedrock, AWS Bedrock Mantle, Microsoft Foundry, Microsoft Azure, BytePlus, Volcengine, Qianfan (Baidu), Tencent, Arcee, Chutes, Synthetic, Parallel, Gradium, Kimi Coding, NVIDIA NIM, GitHub Copilot, OpenCode, SearXNG, Exa, Tavily, Brave, Firecrawl, Voyage

**Channel Extensions (~25):** Discord, Slack, Telegram, WhatsApp, Signal, Matrix, IRC, Nostr, MS Teams, Google Chat, LINE, Feishu, QQ, Zalo, Xiaomi, Twitch, SMS, Webhooks, Nextcloud Talk, Synology Chat, Mattermost, Tlon, GMI, iMessage, Bonjour, Phone Control, Voice Call, Google Meet, Workboard

**Media Extensions (~15):** Image generation (Fal, PixVerse, Runway, ComfyUI), TTS (ElevenLabs, Deepgram, Azure, Local CLI), SenseAudio, Sherpa ONNX

**Memory Extensions:** LanceDB, Wiki

**Diagnostics:** OpenTelemetry, Prometheus

**Other:** Browser, Canvas, Document extract, Web readability, File transfer, Policy engine, Diff viewer, Active memory, Thread ownership, Codex supervisor, Copilot proxy, Migrate Claude/Hermes, ClickClack UX, Device pair, Admin HTTP RPC, Open Prose

### 1.4 Packages (21 packages)

acp-core, agent-core, gateway-client, gateway-protocol (27 TypeBox schema files: frames, primitives, snapshot, agents, channels, config, cron, sessions, secrets, exec-approvals, nodes, plugins, devices + ConnectParams with protocol version negotiation, device auth), llm-core, llm-runtime, markdown-core, media-core, media-generation-core, media-understanding-common, memory-host-sdk, model-catalog-core, net-policy, normalization-core, plugin-package-contract, plugin-sdk, sdk, speech-core, terminal-core, tool-call-repair, web-content-core

### 1.5 Security Audit

**Location:** `src/security/audit.ts`
**Checks:** channel DM policy, gateway auth mode, gateway exposure, filesystem perms (Windows ACL via icacls), Docker sandbox config, plugin code safety, secret scanning, dangerous config flags
**Auto-fix:** `src/security/fix.ts` — file permissions, gateway auth, config issues, safeChmod() with symlink protection

### 1.6 DM Policy

3-tier: `"allow" | "block" | "pairing"` with 10 reason codes. Challenge codes with expiration. Per-channel/per-account allowlists. Group access policy separate from DM policy. Command authorization gated on allowlists.

### 1.7 Secrets Management

3 source types: `"env" | "file" | "exec"`. Target registry with discovery and runtime collection. JSON pointer-based config assignment.

---

## 2. Hermes-Agent

**Path:** `G:\Dx\agent\inspirations\hermes-agent`
**Language:** Python
**Total files:** 5,924 (2,723 .py files, 84,595 Python lines)
**Tests:** 1,872 .py files (41,582 lines)

### 2.1 Top-Level

| File | Lines |
|------|-------|
| `run_agent.py` | 5,816 — AIAgent class, chat(), run_conversation(), OpenAI proxy |
| `cli.py` | 15,824 — HermesCLI, KawaiiSpinner, slash commands, REPL |
| `hermes_state.py` | 5,714 — SessionDB SQLite store |
| `hermes_constants.py` | 980 — path resolution, get_hermes_home() |
| `model_tools.py` | 1,259 — tool orchestration |
| `toolsets.py` | 941 — toolset definitions, _HERMES_CORE_TOOLS |
| `toolset_distributions.py` | 358 — toolset distribution rules |
| `trajectory_compressor.py` | 1,574 — context compression |
| `batch_runner.py` | 1,321 — parallel batch processing |
| `mcp_serve.py` | 904 — MCP server |
| `mini_swe_runner.py` | 732 — SWE-bench runner |
| `utils.py` | 509 — shared utilities |

### 2.2 agent/ — Agent Internals (57 files, 87,640 lines)

| File | Lines |
|------|-------|
| `conversation_loop.py` | 5,101 — core agent loop, model calls, tool dispatch, retries, fallbacks, compression, post-turn hooks |
| `auxiliary_client.py` | 6,755 — side-LLM work: curator, vision, embedding, title gen, session search |
| `chat_completion_helpers.py` | 2,913 — OpenAI chat completions adapter |
| `context_compressor.py` | 2,913 — conversation compression |
| `agent_runtime_helpers.py` | 2,978 — shared agent init helpers |
| `anthropic_adapter.py` | 2,750 — Anthropic Messages API |
| `credential_pool.py` | 2,341 — multi-source API key pool |
| `model_metadata.py` | 2,296 — model metadata, context lengths, provider info |
| `curator.py` | 1,976 — background skill lifecycle (archive, review) |
| `prompt_builder.py` | 1,971 — system prompt construction |
| `agent_init.py` | 1,888 — agent initialization and config |
| `tool_executor.py` | 1,539 — tool execution and dispatch |
| `error_classifier.py` | 1,513 — error classification for failover |
| `display.py` | 1,426 — KawaiiSpinner, tool progress |
| `bedrock_adapter.py` | 1,342 — AWS Bedrock |
| `codex_responses_adapter.py` | 1,336 — OpenAI Codex Responses API |
| `conversation_compression.py` | 1,194 — compression algorithms |
| `memory_manager.py` | 1,081 — multi-provider memory orchestration |
| `plugin_llm.py` | 1,046 — LLM plugin interface |
| `gemini_native_adapter.py` | 1,017 — Google Gemini native |
| `usage_pricing.py` | 963 — usage tracking, cost estimation |
| `insights.py` | 921 — session insights |
| `moa_loop.py` | 893 — Mixture of Agents |
| `learning_graph.py` + `.py` | ~1,500 — learning graph |
| `lsp/` (10 files) | ~3,000 — LSP manager, protocol, install, servers, workspace |
| `secret_sources/` (2 files) | ~500 — Bitwarden |
| `pet/` (8 files) | ~2,000 — digital companion (atlas, imagegen, orchestrate, prompts, manifest, render, state, store) |
| `transports/` (7 files) | ~3,000 — API transport adapters |

### 2.3 tools/ — Tool Registry (97 files, 88,116 lines)

**Environment backends:** local, docker, ssh, modal, managed_modal, daytona, singularity + file_sync

| Tool | Lines |
|------|-------|
| `mcp_tool.py` | 4,943 — MCP client |
| `browser_tool.py` | 4,510 — Playwright browser |
| `delegate_tool.py` | 3,527 — subagent delegation, batch, orchestrator |
| `terminal_tool.py` | 2,978 — terminal (local/docker/ssh/modal) |
| `skills_hub.py` | 4,073 — skill hub install |
| `tts_tool.py` | 2,846 — text-to-speech |
| `approval.py` | 2,767 — tool approval workflows |
| `file_operations.py` | 2,423 — file read/write/edit/search |
| `process_registry.py` | 2,219 — background process tracking |
| `file_tools.py` | 2,000 — glob, grep |
| `code_execution_tool.py` | 1,868 — sandboxed code execution |
| `transcription_tools.py` | 1,799 — audio transcription |
| `send_message_tool.py` | 1,796 — cross-platform messaging |
| `vision_tools.py` | 1,733 — image analysis/OCR |
| `image_generation_tool.py` | 1,680 — image gen |
| `checkpoint_manager.py` | 1,675 — session checkpoints |
| `kanban_tools.py` | 1,669 — kanban board |
| `skills_tool.py` | 1,662 — skill management |
| `skill_manager_tool.py` | 1,542 — skill lifecycle |
| `browser_supervisor.py` | 1,506 — browser coordination |
| `voice_mode.py` | 1,218 — STT + TTS loop |
| `skills_sync.py` | 1,182 — skill sync |
| `memory_tool.py` | 1,146 — memory query/management |
| `cronjob_tools.py` | 1,137 — cron scheduling from agent |
| `skills_guard.py` | 1,086 — skill security guardrails |
| `web_tools.py` | 1,012 — web search and extraction |
| `registry.py` | 708 — central tool registry |
| 70+ more tool files | |

### 2.4 hermes_cli/ — CLI Subcommands (137 files, 152,722 lines)

| File | Lines |
|------|-------|
| `web_server.py` | 14,244 — web dashboard + PTY bridge |
| `main.py` | 13,654 — argparser, cmd_update, command routing |
| `kanban_db.py` | 8,723 — kanban SQLite |
| `auth.py` | 8,411 — provider auth (OAuth, device code) |
| `config.py` | 7,848 — config management, DEFAULT_CONFIG, migration |
| `gateway.py` | 6,843 — gateway lifecycle |
| `tools_config.py` | 4,258 — tools config UI |
| `models.py` | 4,208 — model management, catalog |
| `setup.py` | 3,411 — setup wizard |
| `kanban.py` | 2,845 — kanban CLI |
| `model_setup_flows.py` | 2,827 — model setup flows |
| `cli_commands_mixin.py` | 2,678 — command handling mixin |
| `doctor.py` | 2,412 — health diagnostics |
| `model_switch.py` | 2,371 — model switching |
| `plugins.py` | 2,286 — plugin manager |
| `skin_engine.py` | — data-driven theming (default/ares/mono/slate + custom YAML) |

**35 subcommands:** acp, auth, backup, config, cron, dashboard, debug, doctor, dump, gateway, gui, hooks, insights, login, logout, logs, mcp, memory, model, pairing, plugins, postinstall, profile, prompt_size, security, setup, skills, slack, status, tools, uninstall, update, version, webhook, whatsapp

### 2.5 gateway/ — Messaging Gateway (59 files, 73,332 lines)

| File | Lines |
|------|-------|
| `run.py` | 19,180 — gateway runner, agent cache (128 cap, 1hr TTL), message routing, slash dispatch |
| `platforms/base.py` | 5,563 — base platform adapter ABC |
| `slash_commands.py` | 4,202 — slash command registry |
| `session.py` | 1,917 — session management |
| `stream_consumer.py` | 1,752 — stream event consumer |
| `status.py` | 1,440 — status monitoring, lock mgmt |
| `config.py` | 2,096 — gateway config |
| `kanban_watchers.py` | 1,185 — kanban dispatcher |
| `authz_mixin.py` | 604 — authorization |
| `delivery.py` | 549 — message delivery |
| `pairing.py` | 450 — pairing codes |
| `drain_control.py` | 273 — graceful drain |
| `channel_directory.py` | 423 — channel directory |
| `hooks.py` | 227 — built-in hooks |
| `mirror.py` | 184 — message mirroring |

**Platform adapters:** Telegram, Discord, Slack, WhatsApp Cloud (1,992), Signal (1,701), WeChat/Weixin (2,359), Yuanbao (5,359 + media 665 + proto 1,418 + sticker 558), BlueBubbles/iMessage (1,040), Webhook (1,043), REST API server (4,658), Microsoft Graph webhook (421), QQ Bot (6 files), Signal format, Signal rate limit

**Relay subsystem:** adapter, auth, descriptor, transport, ws_transport

### 2.6 acp_adapter/ — Agent Control Protocol (11 files, 5,251 lines)

`server.py` (2,065), `session.py` (627), `tools.py` (1,291), `events.py` (434), `auth.py` (279), `permissions.py` (234), `provenance.py`, `edit_approval.py`, `entry.py`, `__main__.py`

**Features:** session lifecycle (initialize, new_session, load, resume, fork, close), streaming (message/thought/tool deltas), ToolKind mapping (read/edit/search/execute/fetch/think/other), resources (text, blob, image, audio), auth (API key, OAuth device, external OAuth), per-session model/mode/options, SessionDB persistence

### 2.7 cron/ — Scheduler (9 files, 6,597 lines)

`jobs.py` (~2,500), `scheduler.py` (~2,000), `blueprint_catalog.py` (~500), `suggestion_catalog.py` (~500), `suggestions.py` (~500), `scheduler_provider.py` (~300), `scripts/classify_items.py` (~200)

**Schedule formats:** duration ("30m"), phrase ("every monday 9am"), 5-field cron, ISO timestamp. Features: per-job skill/model/provider overrides, script pre-run, `no_agent=True`, `context_from` chaining, multi-platform delivery, 3-min hard interrupt, catchup/grace windows, file-lock duplicate prevention

### 2.8 tui_gateway/ — TUI Backend (11 files, 15,975 lines)

JSON-RPC server over stdio/WS. Methods: prompt.submit, tool.start/progress/complete, approval.respond, clarify/sudo/secret.respond, session.list/resume, slash.exec, complete.slash/path, gateway.ready, commands.catalog

### 2.9 providers/ — Provider Registry (3 files, 408 lines)

`ProviderProfile` dataclass: name, api_mode, aliases, display_name, description, signup_url, env_vars, base_url, models_url, auth_type (api_key/oauth_device_code/oauth_external/copilot/aws_sdk), supports_health_check/vision/vision_tool_messages, fallback_models, hostname, default_headers, fixed_temperature, default_max_tokens, default_aux_model, hooks

### 2.10 plugins/ — Plugin System (176 .py files, ~10,214 lines)

**Categories:**
- **Memory providers (8):** Honcho, Mem0, Supermemory, Byterover, Hindsight, Holographic, OpenViking, RetainDB
- **Model providers (30+):** alibaba, anthropic, arcee, azure-foundry, bedrock, copilot, deepseek, gemini, gmi, huggingface, kimi, minimax, nous, novita, nvidia, ollama-cloud, openai-codex, opencode-zen, openrouter, qwen-oauth, stepfun, xai, xiaomi, zai, etc.
- **Message platforms (20):** DingTalk, Discord, Email, Feishu, Google Chat, HomeAssistant, IRC, LINE, Matrix, Mattermost, Ntfy, Photon, Raft, SimpleX, Slack, SMS, Teams, Telegram, WeCom, WhatsApp
- **Web search (8):** Brave Free, DuckDuckGo, Exa, Firecrawl, Parallel, SearXNG, Tavily, xAI
- **Image gen (6):** Fal, Krea, OpenAI, OpenAI Codex, OpenRouter, xAI
- **Video gen (2):** Fal, xAI
- **Browser (3):** Browser Use, Browserbase, Firecrawl
- **Dashboard auth (4):** basic, drain, nous, self_hosted
- **Other:** context engine, cron (Chronos), disk cleanup, Google Meet, hermes-achievements, kanban, observability (Langfuse, Nemo Relay), security-guidance, Spotify, Teams pipeline

### 2.11 skills/ — Skill System (803+ files)

**Built-in skills (451 files):**
- **Apple:** apple-notes, apple-reminders, findmy, imessage
- **Autonomous AI:** claude-code, codex, hermes-agent, opencode
- **Computer Use**
- **Creative:** architecture-diagram, ascii-art, ascii-video, comfyui (with references/scripts/tests/workflows), claude-design, design-md, excalidraw, humanizer, manim-video, p5js, pretext, sketch, songwriting-and-ai-music, touchdesigner-mcp
- **Data Science:** jupyter-live-kernel
- **Dogfood:** dogfood (with references/templates)
- **Email:** himalaya (with references)
- **GitHub:** codebase-inspection, github-auth, github-code-review, github-issues, github-pr-workflow, github-repo-management
- **Media:** gif-search, heartmula, songsee, youtube-content
- **MLOps:** evaluation (lm-evaluation-harness, weights-and-biases), huggingface-hub, inference (llama-cpp, vllm), models (audiocraft, segment-anything)
- **Note Taking:** obsidian
- **Productivity:** airtable, google-workspace, maps, nano-pdf, notion, ocr-and-documents, petdex, powerpoint (with Office schemas), teams-meeting-pipeline
- **Research:** arxiv, blogwatcher, llm-wiki, polymarket, research-paper-writing (with AAAI/ACL/ICLR/ICML/NeurIPS templates)
- **Smart Home:** openhue
- **Social Media:** xurl
- **Software Dev:** hermes-agent-skill-authoring, node-inspect-debugger, plan, python-debugpy, requesting-code-review, simplify-code, spike, systematic-debugging, test-driven-development
- **Yuanbao:** yuanbao

**Optional skills (352+ files):** Autonomous AI, Blockchain, Communication, Creative, DevOps, Dogfood, Email, Finance, Gaming, Health, MCP, Migration, MLOps (accelerate, chroma, clip, faiss, flash-attention, guidance, huggingface-tokenizers, inference, instructor, lambda-labs, llava, modal, nemo-curator, obliteratus, peft, pinecone, pytorch-fsdp, pytorch-lightning, qdrant, dspy, saelens, simpo, slime, stable-diffusion, tensorrt-llm, torchtitan, training/axolotl/trl-fine-tuning/unsloth, whisper), Payments (mpp-agent, stripe-link-cli, stripe-projects), Productivity (canvas, here-now, memento-flashcards, shop, shopify, siyuan, telephony), Research (bioinformatics, darwinian-evolver, domain-intel, drug-discovery, duckduckgo-search, gitnexus-explorer, osint-investigation, parallel-cli, qmd, scrapling, searxng-search), Security (1password, godmode, oss-forensics, sherlock, web-pentest), Software Dev (code-wiki, rest-graphql-debug, subagent-driven-development), Web Dev (cloudflare-temporary-deploy, page-agent)

### 2.12 Auto-Update

**Location:** `hermes_cli/main.py`, `cmd_update` (~4,500 lines)

**11 phases:**
1. Pre-flight: managed install check, Docker check, `--check` flag, hangup protection (SIGHUP→SIG_IGN, BrokenPipeError absorption)
2. Setup: gateway prompt via file IPC, non-interactive decision (stash/discard), Windows concurrent instance detection (psutil), pre-update backup (zip, keep last 5), Windows gateway pause
3. Install method detection: git, PyPI (pip install --upgrade), ZIP fallback
4. Git fetch: branch resolution, scoped fetch, branch switching
5. Commit check: rev-list count
6. Pre-pull snapshot: state.db, config.yaml, .env, auth.json, cron/jobs.json, gateway_state, pairing, kanban
7. Pull + Syntax Guard: git pull --ff-only, auto-rollback on syntax error (`_validate_critical_files_syntax()` compiles 8 bootstrap files via py_compile)
8. Stash restore: discard or interactive with conflict detection
9. Cache cleanup: remove .update_check, all __pycache__/
10. Dependency install: fork upstream sync, `.update-incomplete` marker, `uv pip install` → `pip` fallback → per-extra fallback, marker clear
11. Post-update: npm install, Vite rebuild, model catalog seed, module reload, skills sync, config migration

**ZIP fallback** (`_update_via_zip`, ~400 lines): GitHub download, zip-slip protection, symlink rejection, atomic 3-stage directory swap

**Crash recovery** (`_recover_from_interrupted_install`, ~200 lines): single-flight guard (O_CREAT|O_EXCL), 1-hour stale lock auto-break, cron job rescue

### 2.13 Tests (1,872 .py files, 41,582 lines)

tests/acp/, acp_adapter/, agent/ (incl lsp/, transports/), ci/, cli/, computer_use/, cron/, docker/, e2e/, fakes/, fixtures/, gateway/ (incl platforms/, relay/), hermes_cli/, hermes_state/, honcho_plugin/, integration/, openviking_plugin/, plugins/ (browser, dashboard_auth, image_gen, memory, model_providers, platforms, transcription, tts, video_gen, web), providers/, run_agent/, scripts/, skills/, stress/, tools/, tui_gateway/, website/

---

## 3. Deer-Flow

**Path:** `G:\Dx\agent\inspirations\deer-flow`
**Version:** 2.1.0
**Language:** Python
**Total files:** ~820+ (170 backend core + 30 gateway + 307 tests + 180 frontend + 35 docs + 31 scripts + 25 skills)

### 3.1 Core Orchestration (`backend/packages/harness/deerflow/`, ~170 files)

**Lead Agent Factory** (`agents/lead_agent/agent.py`):
- `make_lead_agent(config)` — model resolution, tool assembly (config-defined + MCP + built-in + community), skill policy filtering, middleware chain, 12-section system prompt, LangGraph CompiledStateGraph

**ThreadState** (`agents/lead_agent/thread_state.py`):
- sandbox, thread_data, title, artifacts, todos, uploaded_files, viewed_images, promoted — custom reducers

**Subagent System** (`agents/subagents/`):
- `SubagentConfig` dataclass: name, description, system_prompt, tools allowlist/denylist, skills, model, max_turns (default 50), timeout (900s)
- `SubagentExecutor` (~600 lines): isolated event loop per subagent, dual thread pool (3 scheduler + persistent loop), cooperative cancellation, message dedup by message_id, SSE streaming
- Polling: 5-second intervals, timeout + 60s buffer, try_set_terminal() with lock

**26 Middlewares:**

| # | Middleware | Scope |
|---|-----------|-------|
| 1 | InputSanitization | shared | escape blocked XML tags |
| 2 | ToolOutputBudget | shared | cap per-result output, persist overflow |
| 3 | ThreadData | shared | create per-thread directories |
| 4 | Uploads | lead | track/inject uploaded files |
| 5 | Sandbox | shared | acquire sandbox, store sandbox_id |
| 6 | DanglingToolCall | shared | inject ToolMessages for interrupted calls |
| 7 | LLMErrorHandling | shared | retry/backoff, user-facing errors |
| 8 | Guardrail | shared (opt) | pre-tool-call authorization |
| 9 | SandboxAudit | shared | audit bash by risk level |
| 10 | ToolErrorHandling | shared | exceptions → error ToolMessages, stamp status |
| 11 | DynamicContext | lead | inject date + memory as <system-reminder> |
| 12 | SkillActivation | lead | detect /skill-name, inject SKILL.md |
| 13 | Summarization | lead (opt) | context reduction near token limits |
| 14 | Todo | lead (opt) | task tracking for plan mode |
| 15 | TokenUsage | lead (opt) | record with step attribution |
| 16 | Title | lead | auto-generate thread title |
| 17 | Memory | lead | queue conversations for async update |
| 18 | ViewImage | lead (opt) | inject base64 image data |
| 19 | DeferredToolFilter | lead (opt) | hide MCP tool schemas until promoted |
| 20 | SystemMessageCoalescing | lead | merge SystemMessages |
| 21 | SubagentLimit | lead (opt) | cap task() calls at 3 |
| 22 | LoopDetection | lead (opt) | hash tool call patterns, halt on loops |
| 23 | TokenBudget | lead (opt) | per-run token limit |
| 24 | Custom | lead (opt) | user-injected middlewares |
| 25 | SafetyFinishReason | lead (opt) | suppress tools on provider safety termination |
| 26 | Clarification | lead | intercept ask_clarification (MUST be last) |

**Sandbox** (`sandbox/`):
- ABC: execute_command, read_file, download_file, list_dir, write_file, glob, grep, update_file
- Path mapping: container ↔ host, regex-based translation, read-only skills dir, agent-written path tracking
- Windows: subprocess.run() with shell detection (PowerShell, cmd, MSYS Git Bash)
- POSIX: process group isolation, bounded pipe drain, SIGKILL on timeout, stdin=/dev/null
- Default timeout: 600s, output cap: 10MB
- LocalSandboxProvider: per-thread acquire, LRU cache (256), two-phase acquire

**Run Manager** (`runtime/runs/manager.py`):
- create/get/list/set_status/cancel/update_completion/update_progress/cleanup/shutdown/reconcile_orphaned_inflight_runs
- Exponential backoff retry (5 attempts, 50ms–1s)
- Token usage tracking with step attribution (lead_agent/subagent/middleware/by-model)

### 3.2 Contracts (`contracts/subagent_status_contract.json`)

5 statuses: completed, failed, cancelled, timed_out, polling_timed_out. 14 test cases.

### 3.3 Backend App (`backend/app/`, ~30 files)

FastAPI REST gateway: auth/OIDC, file uploads, channel connections, memory, MCP, streaming (2 parallel paths), guardrails, tracing (LangSmith/Langfuse), user isolation

### 3.4 Config (`config.example.yaml`, 1,581 lines, version 15)

Sections: models[], tools[], tool_groups[], sandbox, memory, summarization, title, subagents, token_usage, token_budget, channels, channel_connections, auth (incl OIDC), guardrails, loop_detection, safety_finish_reason, uploads, tracing

### 3.5 Frontend (`frontend/`, ~180+ TS(X) files)

Next.js app

### 3.6 Tests (`backend/tests/`, 307 files)

292 unit tests + 15 blocking_io detectors (Blockbuster runtime gate). Categories: auth, sandbox, subagent, channels, MCP, memory, skills, routing, persistence, TUI, streaming, tracing, migration, user isolation, E2E, replay

### 3.7 Scripts (25 root + 6 backend)

setup_wizard.py, doctor.py (754 lines), configure.py, check.py, serve.sh, docker.sh, deploy.sh, detect_blocking_io_static.py, detect_thread_boundaries.py, export_claude_code_oauth.py, sandbox_memory_profile.py, sync_labels.py, wizard/ (ui, writer, providers, steps)

---

## 4. Fork Structures

### openclaw-fork (~15 files)

```
openclaw-fork/
├── package.json
├── index.mjs                     # JSON-RPC 2.0 stdio server
├── types/
│   ├── plugin.ts                 # ChannelPlugin type + 17 adapter interfaces
│   ├── core.ts                   # ChannelMeta, ChannelCapabilities
│   └── adapters.ts              # All adapter interfaces
├── channels/
│   ├── telegram.mjs
│   ├── discord.mjs
│   ├── slack.mjs
│   └── whatsapp.mjs
├── pairing/
│   ├── challenge.mjs
│   ├── store.mjs
│   └── messages.mjs
├── security/
│   ├── audit.mjs
│   ├── dm-policy.mjs
│   └── fix.mjs
├── config-schema.mjs
├── secrets.mjs
├── protocol/
│   └── schema/
├── README.md
└── LICENSE
```

### hermes-fork (~8 files)

```
hermes-fork/
├── pyproject.toml
├── hermes_update/
│   ├── __init__.py
│   ├── server.py                 # JSON-RPC 2.0 stdio server
│   ├── check.py                  # check_for_updates
│   ├── perform.py                # cmd_update (11-phase)
│   ├── resilience.py             # zip-slip, symlink, atomic swap, crash recovery
│   └── backup.py                 # pre-update backup, state snapshot
├── acp_adapter/
│   ├── server.py
│   ├── session.py
│   ├── tools.py
│   └── events.py
├── ALGORITHM.md
├── README.md
└── LICENSE
```

### deerflow-fork (~20 files)

```
deerflow-fork/
├── pyproject.toml
├── deerflow_server/
│   ├── __init__.py
│   ├── server.py                 # JSON-RPC 2.0 stdio server
│   └── orchestration/
│       ├── lead_agent.py         # make_lead_agent, tool assembly, prompt template
│       ├── thread_state.py       # ThreadState + custom reducers
│       ├── subagent/
│       │   ├── config.py         # SubagentConfig
│       │   ├── executor.py       # SubagentExecutor
│       │   ├── registry.py       # built-in + custom registry
│       │   └── status_contract.py
│       ├── middleware/
│       │   ├── chain.py
│       │   ├── input_sanitize.py
│       │   ├── tool_budget.py
│       │   ├── thread_data.py
│       │   ├── sandbox.py
│       │   ├── dangling_tool.py
│       │   ├── llm_error.py
│       │   ├── guardrail.py
│       │   ├── sandbox_audit.py
│       │   ├── tool_error.py
│       │   ├── dynamic_context.py
│       │   ├── skill_activate.py
│       │   ├── summarization.py
│       │   ├── todos.py
│       │   ├── token_usage.py
│       │   ├── title.py
│       │   ├── memory.py
│       │   ├── view_image.py
│       │   ├── deferred_tool.py
│       │   ├── coalesce.py
│       │   ├── subagent_limit.py
│       │   ├── loop_detect.py
│       │   ├── token_budget.py
│       │   ├── safety_finish.py
│       │   └── clarification.py
│       ├── sandbox/
│       │   ├── interface.py      # Sandbox ABC (9 methods)
│       │   ├── local.py          # LocalSandbox, path mapping
│       │   ├── provider.py       # LocalSandboxProvider, LRU cache
│       │   └── tools.py          # bash, ls, read_file, write_file, str_replace
│       └── runtime/
│           └── run_manager.py    # RunManager
├── contracts/
│   └── subagent_status.json
├── README.md
└── LICENSE
```

---

## 5. Sidecar Protocol

**JSON-RPC 2.0 over stdin/stdout.**

```
dx-agent (Rust)
  ├── spawns → openclaw-fork (node index.mjs --stdio)
  ├── spawns → hermes-fork (python -m hermes_update.server --stdio)
  └── spawns → deerflow-fork (python -m deerflow_server.server --stdio)
```

### `crates/dx-agent-sidecars/`

```
crates/dx-agent-sidecars/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── protocol.rs               # SidecarProcess::spawn, .request, .stream_request, .notifications
│   ├── openclaw.rs               # .spawn, .send_message, .on_message, .get_capabilities
│   ├── hermes.rs                 # .spawn, .check_update, .perform_update
│   └── deerflow.rs              # .spawn, .delegate, .poll, .stream_events
└── tests/
    ├── test_protocol.rs
    └── test_sidecars.rs
```

### Config

```toml
[sidecars.openclaw]
enabled = false
path = "C:\\Dx\\repos\\openclaw-fork"
channels = ["telegram", "discord", "slack"]

[sidecars.hermes]
enabled = false
path = "C:\\Dx\\repos\\hermes-fork"
update_policy = "notify"

[sidecars.deerflow]
enabled = false
path = "C:\\Dx\\repos\\deerflow-fork"
max_concurrent = 3
```

### Cargo.toml

```toml
[workspace]
members = ["crates/dx-agent-sidecars", ...]

[features]
sidecar-openclaw = ["dx-agent-sidecars/openclaw"]
sidecar-hermes = ["dx-agent-sidecars/hermes"]
sidecar-deerflow = ["dx-agent-sidecars/deerflow"]
sidecar-all = ["sidecar-openclaw", "sidecar-hermes", "sidecar-deerflow"]
```

---

## 6. Summary

| Fork | Files kept | Files discarded |
|------|-----------|----------------|
| OpenClaw | ~15 | ~9,100+ |
| Hermes | ~8 | ~5,900+ |
| Deer-Flow | ~20 | ~800+ |
