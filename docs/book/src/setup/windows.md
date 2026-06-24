# Windows

Install, update, run as a scheduled task / Windows Service, and uninstall on Windows 10 / 11.

`setup.bat` is the Windows counterpart to `install.sh` — same job, different shell. If you're running WSL2, you can follow the [Linux setup](./linux.md) instead; `install.sh` runs unchanged under WSL.

## Install

### Option 1 — `setup.bat` from a release

Download the latest DX Agents release, unzip, and run:

```cmd
setup.bat
```

Flags:

| Flag | Behaviour |
|---|---|
| `--prebuilt` | Download prebuilt binary from GitHub Releases (fastest — no Rust toolchain needed) |
| `--minimal` | Build core only (`--no-default-features`; no channels, no hardware) |
| `--standard` | Build with lean default channels (ACP server, webhook, email, Telegram) |
| `--full` | Build with all channels (`channels-full`) |

The script:

1. Checks for `rustup`; downloads `rustup-init.exe` and installs stable toolchain if missing
2. Builds (or downloads) the binary
3. Installs to `%USERPROFILE%\.dx-agents\bin\dx-agents.exe`
4. Prints mode-specific next steps:
   - `--prebuilt`, `--standard`, `--full`: run `dx-agents quickstart`
   - `--minimal`: Quickstart is unavailable; configure `%USERPROFILE%\.dx_agent\config.toml` manually and use the reduced CLI path (`dx-agents agent ...`)

For source builds, `setup.bat` now prints the exact `cargo build ...` command it executes and reports the installed `dx-agents.exe` size so command shape and artifact expectations stay visible.

### Option 2 — Scoop

```cmd
scoop install dx-agents
dx-agents quickstart
```

Use this only after the DX Agents Scoop manifest is published to the bucket you have configured. Otherwise, use the release archive or source install path above.

### Option 3 — From source

Requires Rust (`rustup`) and Visual Studio Build Tools:

```cmd
git clone https://github.com/millercarla211-ctrl/dx-agents
cd dx-agents
cargo install --locked --path .
dx-agents quickstart
```

## System dependencies

Windows builds use the MSVC toolchain. You need:

- Visual Studio Build Tools (or full Visual Studio) with the "Desktop development with C++" workload
- Rust stable (via `rustup`)

If you're using `--prebuilt` you don't need the Rust toolchain — the binary is self-contained.

## Running as a service

Windows has two options: a scheduled task (user session) or a Windows Service (system session).

### Scheduled task (recommended for single-user machines)

```cmd
dx-agents service install
dx-agents service start
```

This creates a task that runs under your user account and starts on login. Managed via Task Scheduler (`taskschd.msc`).

Logs go to `%LOCALAPPDATA%\DX Agent\logs\`.

### Windows Service (for server installs)

Running as a true service requires Administrator privileges during install. Open an elevated `cmd.exe` and:

```cmd
dx-agents service install
```

When run elevated, the installer registers a Windows Service under `LocalSystem` instead of a user-scoped scheduled task. Consider creating a dedicated service account if the agent touches user-scoped resources.

Full details: [Service management](./service.md).

## Update

### From `setup.bat` / release zip

Re-download the latest release and re-run `setup.bat --prebuilt` (or whichever flag you used originally). Then:

```cmd
dx-agents service restart
```

### Scoop

```cmd
scoop update dx-agents
dx-agents service restart
```

### From source

```cmd
cd C:\path\to\dx-agents
git pull
cargo install --locked --path . --force
dx-agents service restart
```

## Uninstall

Stop and remove the service:

```cmd
dx-agents service stop
dx-agents service uninstall
```

Remove the binary:

```cmd
:: setup.bat
del "%USERPROFILE%\.dx-agents\bin\dx-agents.exe"

:: cargo install
del "%USERPROFILE%\.cargo\bin\dx-agents.exe"

:: Scoop
scoop uninstall dx-agents
```

Remove config and workspace (optional — this deletes conversation history):

```cmd
rmdir /s /q "%USERPROFILE%\.dx_agent"
rmdir /s /q "%LOCALAPPDATA%\DX Agent"
```

## Gotchas

- **Long paths.** Some Windows file systems still cap path lengths at 260 characters. Enable long path support if you hit `path too long` errors during build (`reg add HKLM\SYSTEM\CurrentControlSet\Control\FileSystem /v LongPathsEnabled /t REG_DWORD /d 1 /f`).
- **SmartScreen.** The unsigned binary may trip SmartScreen on first launch. Right-click → Properties → "Unblock" is the standard workaround until we add a signed MSI.
- **Task Scheduler stop-at-idle.** By default Windows may terminate scheduled tasks on idle / battery. The installed task explicitly disables these conditions; verify under Task Scheduler → DX Agent → Properties → Conditions.

## Next

- [Service management](./service.md)
- [Quick start](../getting-started/quick-start.md)
- [Operations → Overview](../ops/overview.md)
