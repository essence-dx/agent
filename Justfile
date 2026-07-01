set shell := ["pwsh.exe", "-c"]

features := "channels-full"

# Build release + copy to G:\Dx\bin
build:
    cargo build --release -j 12 --features {{features}}
    @New-Item -ItemType Directory -Force -Path G:\Dx\bin | Out-Null
    @Copy-Item target\release\dx-agent.exe G:\Dx\bin\ -Force -ErrorAction SilentlyContinue

# Build release with all channels + run interactively
# Usage: just run [agent_alias] -- [dx-agent args...]
#        just run dx -- -m "Hello"
run agent_alias="dx" *args: build
    $a = "{{args}}" -replace "^-- ",""; .\target\release\dx-agent.exe agent -a {{agent_alias}} $(if ($a) { $a })

# Run without building (uses existing binary)
run-fast agent_alias="dx" *args:
    $a = "{{args}}" -replace "^-- ",""; .\target\release\dx-agent.exe agent -a {{agent_alias}} $(if ($a) { $a })

# Quick setup: enable delegation, skill_creation on existing agent
setup agent_alias="dx":
    .\setup.ps1 -Agent {{agent_alias}}
