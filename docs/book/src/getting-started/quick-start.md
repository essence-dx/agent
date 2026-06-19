# Quick Start

The shortest path from zero to talking to the agent.

## Install

Pick one:

**Linux / macOS (one-liner):**

```bash
curl -fsSL https://raw.githubusercontent.com/millercarla211-ctrl/dx-agents/master/install.sh | bash
```

**Homebrew (macOS, Linux):**

Homebrew packaging is not published yet. Use the install script or a release archive until the `dx-agents` formula is available.

**Windows:**

Run `setup.bat` from the latest release, or see [Setup → Windows](../setup/windows.md).

**From source:**

```bash
cargo install --locked --path . # inside a clone
```

## Quickstart

```bash
dx-agents quickstart --model-provider gemini --model gemini-2.0-flash --api-key-env GOOGLE_API_KEY --agent dx
```

`dx-agents quickstart` creates a ready-to-run agent, model provider, risk profile, memory backend, and local CLI channel in one pass. Keep provider secrets in environment variables and pass the variable name with `--api-key-env`; the key value does not belong in shell history or source files. Minimum inputs:

1. An **LLM provider** (Gemini, Groq, Anthropic, OpenAI, Ollama, OpenRouter, etc.) and its API key or endpoint environment variable
2. An **agent alias** such as `dx`; the default local `cli` channel works immediately

Everything else has safe defaults. Total time: ~2 minutes.

## Talk to it

```bash
dx-agents agent -a <alias>
```

`<alias>` matches your `[agents.<alias>]` config entry — required, no default. This drops you into an interactive session using the `cli` channel. Pass `-m "one-shot message"` for a single non-interactive turn.

For always-on deployment, register the service:

```bash
dx-agents service install
dx-agents service start
```

Then use a chat platform channel to reach the agent from Discord, Telegram, or wherever you configured.

## If onboarding's questions annoy you

Create an Ollama-backed local profile without a hosted API key:

```bash
dx-agents quickstart --model-provider ollama --model qwen3.6:35b-a3b --agent dx
```

Or go all the way and use [YOLO mode](./yolo.md) — one config preset that disables approvals and safety gates. For dev boxes and home labs only.

## Next

- [Multi-model setup](./multi-model-setup.md) — multi-agent dispatch, hint-based routes
- [Setup → Service management](../setup/service.md) — running as a daemon
- [Channels → Overview](../channels/overview.md) — wiring up chat platforms
- [Security → Autonomy levels](../security/autonomy.md) — what the agent is allowed to do
