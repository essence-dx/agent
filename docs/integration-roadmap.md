# dx-agent Integration Roadmap

## Extract Patterns — Don't Import Monoliths

We fork each project and surgically strip it to only the parts worth keeping. Each fork stands alone as a lean reference implementation. dx-agent ports the *patterns* to native Rust crates.

---

## 1. OpenClaw Fork

**Fork purpose:** Extract only the channel adapter architecture. Strip everything else.

### What we keep (surgically extract)

```
openclaw-fork/
├── channels/                     # ONLY channel adapters
│   ├── telegram/                 # Pattern: how they handle Telegram
│   ├── discord/                  # Pattern: how they handle Discord
│   ├── slack/                    # Pattern: how they handle Slack
│   ├── whatsapp/                 # Pattern: how they handle WhatsApp
│   └── ... (any channel we need to reference)
├── channel-framework.md          # Documented pattern: adapter interface, auth, rate limiting
├── README.md
└── LICENSE
```

**Total: ~2 files per channel + 1 framework doc. Not 146 extensions.**

### What we discard

| Discard | Why |
|---------|-----|
| Plugin SDK + ClawHub | We don't need a plugin marketplace |
| Gateway (HTTP/WS server) | dx-agent has its own |
| Voice/Talk | Separate concern, not relevant |
| Canvas/A2UI | Not useful to us |
| Doctor system | dx-agent has doctor |
| Companion apps (macOS/iOS/Android) | Not our target |
| Security audit subsystem | Overkill, we have estop |
| Onboarding wizard | One-time use |
| 120+ remaining extensions | Irrelevant providers/tools |
| npm-shrinkwrap, pnpm-lock | We don't run their Node.js |
| Maturity taxonomy | Internal process, not code |

### What becomes a Rust crate

```rust
// crates/dx-agent-channels/
// Following OpenClaw's channel pattern but in Rust
pub trait ChannelAdapter: Send + Sync {
    fn name(&self) -> &'static str;
    async fn connect(&self, config: ChannelConfig) -> Result<()>;
    async fn send_message(&self, msg: OutboundMessage) -> Result<MessageId>;
    async fn on_message(&self, handler: Box<dyn Fn(InboundMessage) + Send>);
}
```

Implementation per channel follows OpenClaw's proven API patterns (auth flow, rate limiting, retry, media handling) — but in Rust, no Node.js needed.

---

## 2. Hermes-Agent Fork

**Fork purpose:** Extract only the auto-update algorithm. Strip everything else.

### What we keep (surgically extract)

```
hermes-fork/
├── update-engine/               # ONLY the update system
│   ├── update_check.py          # git/pip/nix version detection
│   ├── update_perform.py        # atomic swap, backup, crash recovery
│   ├── update_resilience.py     # file locking, zip-slip protection, rollback
│   └── update_spec.md           # Documented algorithm
├── README.md
└── LICENSE
```

**Total: ~4 files. Not 83 directories.**

### What we discard

| Discard | Why |
|---------|-----|
| run_agent.py (12k LOC) | We have our own agent loop |
| cli.py (11k LOC) | We have our own CLI |
| gateway/ (19k LOC) | We have our own gateway |
| ACP adapter | dx-agent already has acp-bridge |
| 25+ messaging platforms | We'll use OpenClaw patterns |
| Profile isolation | Not needed in single-binary setup |
| Plugin system | WASM plugins cover this |
| Skill system | We have skills |
| TUI | We have our own |
| Desktop app | Not our focus |
| Batch runner | Research tool, not needed |
| Curator system | Over-engineered for our needs |
| 17k tests | Cook once, reference spec |

### What becomes a Rust crate

```rust
// crates/dx-agent-update/
// Ported from Hermes's algorithm, in native Rust
pub struct UpdateEngine {
    current_version: Version,
}

impl UpdateEngine {
    /// Check for updates (git/pip/nix detection, cached 6h)
    pub async fn check(&self) -> Result<UpdateInfo>;

    /// Perform update with crash recovery
    /// 1. Pre-flight (file lock, backup, marker)
    /// 2. Atomic download to staging dir
    /// 3. Atomic swap (rename staging -> live)
    /// 4. Post-update (rebuild, verify)
    /// 5. On failure: restore from backup
    pub async fn perform(&self, progress: ProgressCB) -> Result<()>;
}
```

Hermes's update code is the **spec**. We implement in Rust following the exact same algorithm (file locking, crash recovery markers, atomic os.rename swaps, pre-update backups, zip-slip protection). No Python dependency.

---

## 3. Deer-Flow Fork

**Fork purpose:** Extract only the multi-agent orchestration architecture. Strip everything else.

### What we keep (surgically extract)

```
deerflow-fork/
├── orchestration/               # ONLY the agent orchestration pattern
│   ├── lead_agent.md            # Pattern: how the lead agent delegates
│   ├── subagent.md              # Pattern: subagent lifecycle, isolation, polling
│   ├── middleware.md            # Pattern: 26-middleware chain architecture
│   ├── sandbox.md               # Pattern: sandbox abstraction (local/Docker)
│   ├── contracts.md             # Pattern: cross-component status contracts
│   └── deferred_mcp.md          # Pattern: lazy MCP tool promotion
├── README.md
└── LICENSE
```

**Total: ~7 markdown spec files + maybe a couple reference Python files. Not 39 directories and 4 services.**

### What we discard

| Discard | Why |
|---------|-----|
| LangGraph dependency | We implement orchestration in Rust directly |
| FastAPI Gateway | We have our own gateway |
| Next.js frontend | We don't need a web UI |
| Nginx reverse proxy | Irrelevant |
| Provisioner service | Only for Kubernetes |
| 4-service topology | We run a single binary |
| Config hot-reload system | Nice but not critical |
| Alembic migrations | We use our own DB schema |
| TUI | We have our own |
| MCP client code | We implement our own in Rust |
| Community integrations | Irrelevant |
| 26 middlewares (direct port) | Implement only the ones we need in Rust |

### What becomes a Rust crate

```rust
// crates/dx-agent-orchestrate/
// Architecture inspired by Deer-Flow, implemented in Rust
pub struct Orchestrator {
    lead_agent: Box<dyn Agent>,
    subagent_pool: ThreadPool,
    middleware_chain: Vec<Box<dyn Middleware>>,
}

impl Orchestrator {
    /// Fan out a task to N parallel subagents
    /// Each subagent: isolated context, dedicated model, separate tools
    /// Lead agent synthesizes results
    pub async fn delegate(&self, task: TaskSpec) -> Result<TaskResult> {
        // 1. Middleware chain processes the task
        // 2. Lead agent decides decomposition
        // 3. Subagents execute in parallel on thread pool
        // 4. Poll for completion with streaming events
        // 5. Lead agent merges results
    }
}
```

---

## 4. Relationship to dx-agent

```
dx-agent (Rust)
├── crates/dx-agent-channels/     ← Pattern from OpenClaw
├── crates/dx-agent-update/       ← Algorithm from Hermes
└── crates/dx-agent-orchestrate/  ← Architecture from Deer-Flow
```

**No FFI. No sidecars. No embedding runtimes.**

Each pattern is reimplemented in native Rust, guided by the forked reference code. The forks exist purely as documentation — we read them to understand the proven patterns, then write idiomatic Rust.

---

## 5. Fork Maintenance Policy

| Activity | Frequency |
|----------|-----------|
| Rebase fork against upstream | Only when upstream has a channel fix worth adopting |
| Update pattern docs | When we discover a gap in our Rust implementation |
| Keep stripped fork lean | Never re-add discarded components |
| Reference in commit messages | "Pattern from OpenClaw channel framework: ..." |

---

## 6. Implementation Priority

| Crate | Source | Complexity | Value | Order |
|-------|--------|-----------|-------|-------|
| `dx-agent-update` | Hermes | Low (algorithm well-documented, ~500 LoC) | High (missing critical feature) | **1st** |
| `dx-agent-channels` | OpenClaw | Medium (need working Telegram/Discord/Slack) | High (production-proven patterns) | **2nd** |
| `dx-agent-orchestrate` | Deer-Flow | High (multi-agent async coordination) | Medium (nice-to-have vs core loop) | **3rd** |

---

## 7. The Fork Repositories

```
github.com/millercarla211-ctrl/
├── openclaw-channels-fork/      # Stripped: only channel adapter patterns
├── hermes-update-fork/          # Stripped: only auto-update algorithm
└── deerflow-orchestrate-fork/   # Stripped: only multi-agent architecture
```

Each lives in `inspirations/<name>/` as local reference, with the GitHub fork as the canonical stripped version.
