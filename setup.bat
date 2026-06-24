@echo off
setlocal enabledelayedexpansion

:: ============================================================================
:: DX Agents Windows Setup Script
:: Simplifies building and installing DX Agents on Windows.
:: Usage: setup.bat [--prebuilt | --minimal | --standard | --full | --help]
:: ============================================================================

set "VERSION=0.8.0-beta-2"
set "RUST_MIN_VERSION=1.87"
set "TARGET=x86_64-pc-windows-msvc"
set "PRODUCT_NAME=DX Agents"
set "BIN_NAME=dx-agents"
set "REPO=millercarla211-ctrl/dx-agents"
set "REPO_URL=https://github.com/%REPO%"
if defined DX_AGENTS_HOME (
    set "INSTALL_HOME=%DX_AGENTS_HOME%"
) else if defined DX_HOME (
    set "INSTALL_HOME=%DX_HOME%\agents"
) else (
    set "INSTALL_HOME=%USERPROFILE%\.dx-agents"
)
set "INSTALL_BIN_DIR=%INSTALL_HOME%\bin"
set "CONFIG_PATH=%USERPROFILE%\.dx_agent\config.toml"

:: Colors via ANSI (Windows 10+ Terminal)
set "GREEN=[32m"
set "YELLOW=[33m"
set "RED=[31m"
set "BLUE=[34m"
set "BOLD=[1m"
set "RESET=[0m"

:: Parse arguments
set "MODE=interactive"
if "%~1"=="--help"     goto :show_help
if "%~1"=="-h"         goto :show_help
if "%~1"=="--prebuilt" set "MODE=prebuilt" & goto :start
if "%~1"=="--minimal"  set "MODE=minimal"  & goto :start
if "%~1"=="--standard" set "MODE=standard" & goto :start
if "%~1"=="--full"     set "MODE=full"     & goto :start

:start
echo.
echo %BOLD%%BLUE%=========================================%RESET%
echo %BOLD%%BLUE%  %PRODUCT_NAME% Windows Setup  v%VERSION%%RESET%
echo %BOLD%%BLUE%=========================================%RESET%
echo.

:: ---- Step 1: Check prerequisites ----
echo %BOLD%[1/5] Checking prerequisites...%RESET%

:: Check available RAM (rough estimate via wmic)
for /f "tokens=2 delims==" %%a in ('wmic os get FreePhysicalMemory /value 2^>nul ^| find "="') do (
    set /a "FREE_RAM_MB=%%a / 1024"
)
if defined FREE_RAM_MB (
    if !FREE_RAM_MB! LSS 2048 (
        echo   %YELLOW%WARNING: Only !FREE_RAM_MB! MB free RAM detected. 2048 MB recommended for source builds.%RESET%
        echo   %YELLOW%Consider using --prebuilt instead.%RESET%
    ) else (
        echo   %GREEN%OK%RESET% Free RAM: !FREE_RAM_MB! MB
    )
)

:: Check disk space
for /f %%a in ('powershell -Command "[math]::Round((Get-PSDrive $env:SystemDrive).Free / 1GB)"') do (
    set "FREE_DISK_GB=%%a"
)

:: Check Rust
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo   %YELLOW%Rust not found.%RESET%
    goto :install_rust
) else (
    for /f "tokens=2" %%v in ('rustc --version 2^>nul') do set "RUST_VER=%%v"
    echo   %GREEN%OK%RESET% Rust !RUST_VER! found
)

:: Check Node.js (optional)
where node >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo   %YELLOW%Node.js not found - optional, web dashboard will use stub%RESET%
) else (
    for /f "tokens=1" %%v in ('node --version 2^>nul') do set "NODE_VER=%%v"
    echo   %GREEN%OK%RESET% Node.js !NODE_VER! found
)

:: Check Git
where git >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo   %RED%ERROR: Git is required but not found.%RESET%
    echo   Install Git from https://git-scm.com/download/win
    goto :error_exit
) else (
    echo   %GREEN%OK%RESET% Git found
)

goto :choose_mode

:: ---- Install Rust ----
:install_rust
echo.
echo %BOLD%Installing Rust...%RESET%
echo   Downloading rustup-init.exe...

:: Download rustup-init.exe
curl -sSfL -o "%TEMP%\rustup-init.exe" https://win.rustup.rs
if %ERRORLEVEL% NEQ 0 (
    echo   %RED%ERROR: Failed to download rustup-init.exe%RESET%
    echo   Please install Rust manually from https://rustup.rs
    goto :error_exit
)

:: Run rustup-init with defaults
"%TEMP%\rustup-init.exe" -y --default-toolchain stable --target %TARGET%
if %ERRORLEVEL% NEQ 0 (
    echo   %RED%ERROR: Rust installation failed.%RESET%
    goto :error_exit
)

:: Refresh PATH
set "PATH=%USERPROFILE%\.cargo\bin;%PATH%"
echo   %GREEN%OK%RESET% Rust installed successfully.
echo   %YELLOW%NOTE: You may need to restart your terminal for PATH changes.%RESET%
goto :choose_mode

:: ---- Choose build mode ----
:choose_mode
echo.

if "%MODE%"=="prebuilt" goto :install_prebuilt
if "%MODE%"=="minimal"  goto :build_minimal
if "%MODE%"=="standard" goto :build_standard
if "%MODE%"=="full"     goto :build_full

:: Interactive mode
echo %BOLD%[2/5] Choose installation method:%RESET%
echo.
echo   1) Prebuilt binary   - Download pre-compiled release (fastest, ~2 min)
echo   2) Minimal build     - Core only (^--no-default-features, ~15 min)
echo   3) Standard build    - Default + Lark/Feishu + Matrix (~20 min)
echo   4) Full build        - All features including hardware + browser (~30 min)
echo.
set /p "CHOICE=  Select [1-4] (default: 1): "

if "%CHOICE%"=="" set "CHOICE=1"
if "%CHOICE%"=="1" goto :install_prebuilt
if "%CHOICE%"=="2" goto :build_minimal
if "%CHOICE%"=="3" goto :build_standard
if "%CHOICE%"=="4" goto :build_full

echo   %RED%Invalid choice. Please enter 1-4.%RESET%
goto :choose_mode

:: ---- Prebuilt binary ----
:install_prebuilt
echo.
echo %BOLD%[3/5] Downloading prebuilt binary...%RESET%

:: Try to get latest release URL via gh or curl
where gh >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%u in ('gh release view --repo %REPO% --json assets --jq ".assets[] | select(.name | test(\"windows-msvc\")) | .url" 2^>nul') do (
        set "DOWNLOAD_URL=%%u"
    )
)

if not defined DOWNLOAD_URL (
    :: Fallback: construct URL from known release pattern
    set "DOWNLOAD_URL=https://github.com/%REPO%/releases/latest/download/%BIN_NAME%-%TARGET%.zip"
)

echo   Downloading from release...
curl -sSfL -o "%TEMP%\%BIN_NAME%-windows.zip" "!DOWNLOAD_URL!"
if %ERRORLEVEL% NEQ 0 (
    echo   %YELLOW%Prebuilt binary not available. Falling back to source build - standard%RESET%
    goto :build_standard
)

:: Extract
echo   Extracting...
set "EXTRACT_DIR=%TEMP%\%BIN_NAME%-extract-%RANDOM%%RANDOM%"
if exist "!EXTRACT_DIR!" rmdir /S /Q "!EXTRACT_DIR!" >nul 2>&1
mkdir "!EXTRACT_DIR!" 2>nul
tar -xf "%TEMP%\%BIN_NAME%-windows.zip" -C "!EXTRACT_DIR!"
if %ERRORLEVEL% NEQ 0 (
    powershell -Command "Expand-Archive -Force '%TEMP%\%BIN_NAME%-windows.zip' '!EXTRACT_DIR!'"
)

set "EXTRACTED_BIN="
for /r "!EXTRACT_DIR!" %%f in (%BIN_NAME%.exe) do (
    if not defined EXTRACTED_BIN set "EXTRACTED_BIN=%%f"
)

if not defined EXTRACTED_BIN (
    echo   %YELLOW%Release archive did not contain %BIN_NAME%.exe. Falling back to source build - standard%RESET%
    rmdir /S /Q "!EXTRACT_DIR!" >nul 2>&1
    goto :build_standard
)

mkdir "%INSTALL_BIN_DIR%" 2>nul
copy /Y "!EXTRACTED_BIN!" "%INSTALL_BIN_DIR%\%BIN_NAME%.exe" >nul
if %ERRORLEVEL% NEQ 0 (
    echo   %RED%ERROR: Failed to install %BIN_NAME%.exe from release archive.%RESET%
    rmdir /S /Q "!EXTRACT_DIR!" >nul 2>&1
    goto :error_exit
)

set "EXTRACTED_TUI="
for /r "!EXTRACT_DIR!" %%f in (zerocode.exe) do (
    if not defined EXTRACTED_TUI set "EXTRACTED_TUI=%%f"
)
if defined EXTRACTED_TUI (
    copy /Y "!EXTRACTED_TUI!" "%INSTALL_BIN_DIR%\zerocode.exe" >nul
)
rmdir /S /Q "!EXTRACT_DIR!" >nul 2>&1

:: Add to PATH if not already there
echo %PATH% | findstr /I /C:"%INSTALL_BIN_DIR%" >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    setx PATH "%PATH%;%INSTALL_BIN_DIR%" >nul 2>&1
    set "PATH=%PATH%;%INSTALL_BIN_DIR%"
    echo   %GREEN%OK%RESET% Added to PATH
)

echo   %GREEN%OK%RESET% Binary installed to %INSTALL_BIN_DIR%\%BIN_NAME%.exe
if exist "%INSTALL_BIN_DIR%\zerocode.exe" (
    echo   %GREEN%OK%RESET% TUI installed to %INSTALL_BIN_DIR%\zerocode.exe
)
goto verify

:: ---- Minimal build ----
:build_minimal
set "FEATURES=--no-default-features"
set "BUILD_DESC=minimal (core only, no default features)"
goto :do_build

:: ---- Standard build ----
:build_standard
set "FEATURES=--features channel-matrix,channel-lark"
set "BUILD_DESC=standard (Matrix + Lark/Feishu)"
goto :do_build

:: ---- Full build ----
:build_full
set "FEATURES=--features channel-matrix,channel-lark,browser-native,hardware,rag-pdf,observability-otel"
set "BUILD_DESC=full (all features)"
goto :do_build

:: ---- Build from source ----
:do_build
echo.
echo %BOLD%[3/5] Building %PRODUCT_NAME% (%BUILD_DESC%)...%RESET%
echo   Target: %TARGET%

:: Ensure we're in the repo root (check for Cargo.toml)
if not exist "Cargo.toml" (
    echo   %RED%ERROR: Cargo.toml not found. Run this script from the dx-agents repository root.%RESET%
    echo   Example:
    echo     git clone %REPO_URL%
    echo     cd dx-agents
    echo     setup.bat
    goto :error_exit
)

:: Add target if missing
rustup target add %TARGET% >nul 2>&1

echo   This may take 15-30 minutes on first build...
echo.

echo   Command: cargo build --release --locked -p %BIN_NAME% --bin %BIN_NAME% %FEATURES% --target %TARGET%
cargo build --release --locked -p %BIN_NAME% --bin %BIN_NAME% %FEATURES% --target %TARGET%
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo   %RED%ERROR: Build failed.%RESET%
    echo   Common fixes:
    echo   - Ensure Visual Studio Build Tools are installed - C++ workload
    echo   - Run: rustup update
    echo   - Check disk space - 6 GB needed
    goto :error_exit
)

echo   %GREEN%OK%RESET% Build succeeded.

echo   Command: cargo build --release --locked -p zerocode --target %TARGET%
cargo build --release --locked -p zerocode --target %TARGET%
if %ERRORLEVEL% NEQ 0 (
    echo   %YELLOW%WARNING: zerocode TUI build failed; continuing with %BIN_NAME% only.%RESET%
)

:: Copy binary to a convenient location
echo.
echo %BOLD%[4/5] Installing binary...%RESET%
mkdir "%INSTALL_BIN_DIR%" 2>nul
copy /Y "target\%TARGET%\release\%BIN_NAME%.exe" "%INSTALL_BIN_DIR%\%BIN_NAME%.exe" >nul
if %ERRORLEVEL% NEQ 0 (
    echo   %RED%ERROR: Failed to copy %BIN_NAME%.exe into %INSTALL_BIN_DIR%.%RESET%
    goto :error_exit
)
if exist "target\%TARGET%\release\zerocode.exe" (
    copy /Y "target\%TARGET%\release\zerocode.exe" "%INSTALL_BIN_DIR%\zerocode.exe" >nul
    echo   %GREEN%OK%RESET% TUI installed to %INSTALL_BIN_DIR%\zerocode.exe
)
set "BIN_PATH=%INSTALL_BIN_DIR%\%BIN_NAME%.exe"
for /f %%S in ('powershell -NoProfile -Command "[math]::Round(((Get-Item -LiteralPath ''%BIN_PATH%'').Length / 1MB), 2)"') do (
    set "BINARY_MB=%%S"
)
if defined BINARY_MB (
    echo   %GREEN%OK%RESET% Installed to %INSTALL_BIN_DIR%\%BIN_NAME%.exe ^(%BINARY_MB% MB^)
) else (
    echo   %GREEN%OK%RESET% Installed to %INSTALL_BIN_DIR%\%BIN_NAME%.exe ^(size unavailable^)
)

:: Add to PATH if not already there
echo %PATH% | findstr /I /C:"%INSTALL_BIN_DIR%" >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    setx PATH "%PATH%;%INSTALL_BIN_DIR%" >nul 2>&1
    set "PATH=%PATH%;%INSTALL_BIN_DIR%"
    echo   %GREEN%OK%RESET% Added to PATH
)

goto verify

:: ---- Post install ----
:verify
echo.
echo %BOLD%[5/5] Verifying installation...%RESET%

"%INSTALL_BIN_DIR%\%BIN_NAME%.exe" --version >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    for /f "tokens=*" %%v in ('"%INSTALL_BIN_DIR%\%BIN_NAME%.exe" --version 2^>nul') do (
        echo   %GREEN%OK%RESET% %%v
    )
) else (
    echo   %RED%ERROR: %BIN_NAME%.exe was not installed correctly at %INSTALL_BIN_DIR%.%RESET%
    goto :error_exit
)

echo.
echo %BOLD%%GREEN%=========================================%RESET%
echo %BOLD%%GREEN%  %PRODUCT_NAME% setup complete!%RESET%
echo %BOLD%%GREEN%=========================================%RESET%
echo.
echo   Next steps:
echo     1. Restart your terminal (for PATH changes)
if /I "%MODE%"=="minimal" (
echo     2. Minimal build excludes Quickstart ^(%BIN_NAME% quickstart is unavailable^)
echo     3. Configure model providers manually in %CONFIG_PATH%
echo     4. Use reduced CLI path: %BIN_NAME% agent --message "Hello"
) else (
echo     2. Run: %BIN_NAME% quickstart
echo     3. Keep provider API keys in environment variables, not in shell history
echo     4. Launch the TUI: zerocode
)
echo.
echo   Release downloads:
echo     %REPO_URL%/releases
echo.
echo   Documentation: %REPO_URL%
echo.
goto :end

:: ---- Help ----
:show_help
echo.
echo %PRODUCT_NAME% Windows Setup Script
echo.
echo Usage: setup.bat [OPTIONS]
echo.
echo Options:
echo   --prebuilt    Download pre-compiled binary (fastest)
echo   --minimal     Build core only ^(--no-default-features^)
echo   --standard    Build with Matrix + Lark/Feishu
echo   --full        Build with all features
echo   --help, -h    Show this help message
echo.
echo Without arguments, runs in interactive mode.
echo.
echo Prerequisites:
echo   - Git (required)
echo   - Rust 1.87+ (auto-installed if missing)
echo   - Visual Studio Build Tools with C++ workload (for source builds)
echo   - Node.js (optional, for web dashboard)
echo.
goto :end

:: ---- Error exit ----
:error_exit
echo.
echo %RED%Setup failed. See errors above.%RESET%
echo Need help? Open an issue at %REPO_URL%/issues
echo.
endlocal
exit /b 1

:: ---- Clean exit ----
:end
endlocal
exit /b 0
