set shell := ["pwsh.exe", "-c"]

features := "channels-full"

# Build release + copy to G:\Dx\bin
build:
    cargo build --release -j 12 --features {{features}}
    @New-Item -ItemType Directory -Force -Path G:\Dx\bin | Out-Null
    @Copy-Item target\release\dx-agent.exe G:\Dx\bin\ -Force -ErrorAction SilentlyContinue

# Build release then run agent interactively
# Usage: just run [agent_alias] -- [dx-agent args...]
#        just run test_agent -- -m "Hello"
run agent_alias="test_agent" *args: build
    $a = "{{args}}" -replace "^-- ",""; .\target\release\dx-agent.exe agent -a {{agent_alias}} $(if ($a) { $a })

# Run without rebuilding (uses existing binary)
run-fast agent_alias="test_agent" *args:
    $a = "{{args}}" -replace "^-- ",""; .\target\release\dx-agent.exe agent -a {{agent_alias}} $(if ($a) { $a })

# Run dx-config self-test: verifies dx discovery, .sr write, .sr read
test-config:
    .\target\release\dx-agent.exe config get runtime.kind

# Quick setup: enable delegation, skill_creation on existing agent
setup agent_alias="test_agent":
    .\setup.ps1 -Agent {{agent_alias}}
