# macOS

Install, update, run as a LaunchAgent, and uninstall on macOS (Intel or Apple Silicon).

## Install

`install.sh` is the preferred path; Homebrew is a reasonable alternative if you want `brew services` integration.

### Option 1 — `install.sh` via curl (fastest)

```bash
curl -fsSL https://raw.githubusercontent.com/millercarla211-ctrl/dx-agents/master/install.sh | bash
```

### Option 2 — `install.sh` from a clone

```bash
git clone https://github.com/millercarla211-ctrl/dx-agents.git
cd dx-agents
./install.sh
```

### What the installer does

1. Asks whether you want a prebuilt binary or to build from source
2. Installs to `~/.cargo/bin/dx-agents`
3. Offers CLI Quickstart or gateway setup unless a configured agent route already exists

Flags:

```bash
./install.sh --prebuilt                      # always prebuilt, skip the prompt
./install.sh --source                        # always build from source
./install.sh --minimal                       # kernel only (~6.6 MB)
./install.sh --source --features agent-runtime,channel-discord   # custom features
./install.sh --skip-onboard                  # install only; run `dx-agents quickstart` later
./install.sh --list-features                 # print available features and exit
./install.sh --help                          # full flag reference
```

### Option 3 — Homebrew

DX Agents does not currently publish a DX-owned Homebrew formula. Use `install.sh` for release installs; legacy DX Agent Homebrew paths remain migration-only compatibility data.

**Legacy workspace location gotcha:** with a legacy Homebrew install, the service user and the CLI user may be different, so the workspace lives at `$HOMEBREW_PREFIX/var/zeroclaw/` rather than `~/.dx_agent/`. Point CLI invocations at the same workspace:

```bash
export DX_AGENTS_WORKSPACE="$HOMEBREW_PREFIX/var/zeroclaw"
```

Add that to your shell profile if you want it permanent.

## System dependencies

Most features work with a stock macOS install. Optional extras:

| Feature | Install |
|---|---|
| Docs translation | `brew install gettext` |
| Browser tool | Playwright pulls Chromium automatically on first use |
| Hardware | No native GPIO on macOS; use a USB peripheral like Aardvark. See [Hardware → Aardvark](../hardware/aardvark.md) |
| iMessage channel | Requires macOS 11+. See [Channels → Other chat platforms](../channels/chat-others.md) |

## Running as a service

```bash
dx-agents service install   # writes ~/Library/LaunchAgents/com.dx_agent.daemon.plist
dx-agents service start
dx-agents service status
```

Logs go to `~/Library/Logs/DX Agent/`:

```bash
tail -f ~/Library/Logs/DX Agent/zeroclaw.log
```

The service command produces a loaded LaunchAgent that starts on login.

Full details: [Service management](./service.md).

## Update

Re-run the installer — it detects the existing install and upgrades in place:

```bash
curl -fsSL https://raw.githubusercontent.com/millercarla211-ctrl/dx-agents/master/install.sh | bash -s -- --skip-onboard
dx-agents service restart
```

Or from a clone:

```bash
cd /path/to/dx-agents
git pull
./install.sh --skip-onboard
dx-agents service restart
```

## Uninstall

```bash
# stop and unregister the service
dx-agents service stop
dx-agents service uninstall

# bootstrap / cargo
rm ~/.cargo/bin/dx-agents
```

Remove config and workspace (optional — this deletes conversation history):

```bash
# Homebrew workspace
rm -rf "$HOMEBREW_PREFIX/var/zeroclaw"

# Default workspace
rm -rf ~/.dx_agent ~/.config/zeroclaw

# Logs
rm -rf ~/Library/Logs/DX Agent
```

## Gotchas

- **Legacy Homebrew config path mismatch.** A legacy `brew services` daemon reads `$HOMEBREW_PREFIX/var/zeroclaw/config.toml`, not `~/.dx_agent/config.toml`. If your service is reading stale config, check which one the daemon sees and set `DX_AGENTS_WORKSPACE` accordingly.
- **First launch of the browser tool** downloads Chromium (~150 MB) via Playwright.
- **Apple Silicon** and **Intel** builds are both released. The bootstrap script auto-detects the right archive.

## Next

- [Service management](./service.md)
- [Quick start](../getting-started/quick-start.md)
- [Operations → Overview](../ops/overview.md)
