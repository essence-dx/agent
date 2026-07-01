# ── DX Agent: Full-featured setup ──────────────────────────────────
# Run this after `just build` to enable channels, sub-agents, and
# auto-improving skills on your existing agent.

param(
    [string]$Agent = "test_agent",
    [switch]$Build
)

if ($Build) {
    Write-Host "Building with all channels..." -ForegroundColor Cyan
    & cargo build -p dx-agent --release --features "channels-full"
    if ($LASTEXITCODE -ne 0) { exit 1 }
    Write-Host "Build OK" -ForegroundColor Green
}

$AgentBin = ".\target\release\dx-agent.exe"
if (-not (Test-Path $AgentBin)) {
    $AgentBin = ".\target\debug\dx-agent.exe"
}

if (-not (Test-Path $AgentBin)) {
    Write-Error "Build first: cargo build -p dx-agent --features channels-full"
    exit 1
}

Write-Host "`nConfiguring agent '$Agent'..." -ForegroundColor Cyan

# ── 1. Sub-agents: enable delegation ───────────────────────────────
& $AgentBin config set risk_profiles.balanced.delegation_policy.mode allow
& $AgentBin config set runtime_profiles.balanced.max_delegation_depth 3
& $AgentBin config set runtime_profiles.balanced.agentic true

# ── 2. Auto-improving skills (Hermes-style) ────────────────────────
& $AgentBin config set skills.skill_creation.enabled true

# ── 3. Channels (pick what you need) ───────────────────────────────
# & $AgentBin config set channels.telegram.default.enabled true
# & $AgentBin config set channels.telegram.default.bot_token "your-token"

Write-Host "`nDone! Config updated." -ForegroundColor Green
Write-Host "`nRun: just run agent_alias=$Agent" -ForegroundColor Yellow
Write-Host "Or:  $AgentBin agent -a $Agent" -ForegroundColor Yellow
