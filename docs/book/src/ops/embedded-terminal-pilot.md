# Embedded Terminal Pilot Plan

DX Agents can show embedded terminal readiness today, but the production terminal path remains Windows Terminal until every embedded PTY gate is proven. This pilot plan keeps that boundary explicit.

## Goal

Pilot an in-app PTY session surface without breaking existing terminal workflows:

- Ratatui shell and file-browser actions continue to launch through Windows Terminal.
- Media routes keep using mpv, tplay, viu, and the external terminal path.
- Embedded PTY work starts as read-only readiness evidence, then moves through isolated pilot sessions before it can become a production route.

## Non-Goals

- Do not replace Windows Terminal for normal users during the pilot.
- Do not store raw key text, paste text, mouse selection text, or escape sequence bytes.
- Do not route terminal-video or audio through embedded PTY until renderer, input, resize, lifecycle, and media-session gates all pass.
- Do not run a release build just to test the pilot gates.

## Required Gates

The desktop bridge and host contract must agree on these gates:

- `embedded_pty.production_ready`
- `embedded_terminal_sessions.enabled`
- `embedded_terminal_renderer.enabled`
- `embedded_terminal_input.enabled`
- `embedded_terminal_resize.enabled`
- `embedded_terminal_media_session.enabled`

The pilot can advance only when the readiness export reports safe evidence for the current phase.

## Phase 0: Evidence Only

Status: current production state.

Required evidence:

- Host contract fields survive the desktop bridge boundary.
- Readiness export reports input, resize, and media-session schemas.
- No-payload fixtures cover keyboard, paste, focus, mouse, control sequence, viewport resize, debounce, renderer reflow, and PTY resize metadata.
- External terminal fallback remains active.

Allowed behavior:

- Display readiness diagnostics in the bridge.
- Export redacted evidence.
- Keep all embedded session operations disabled.

## Phase 1: Synthetic Session

Run an in-memory session fixture that never spawns a process.

Required evidence:

- Open, resize, input, interrupt, and close state transitions are deterministic.
- Input fixture data remains metadata-only.
- Resize bounds clamp columns and rows.
- Renderer reflow records are produced without rendering terminal output.

Allowed behavior:

- Show a synthetic session timeline in diagnostics.
- Reject all external process launch attempts.
- Keep Windows Terminal as the only live terminal route.

## Phase 2: Local Echo Process

Run a local echo-style process with no shell access and no user command execution.

Required evidence:

- Process lifecycle cleanup works after normal exit, interrupt, and close.
- Resize and input events reach the process without raw payload storage.
- Stdout/stderr capture is bounded.
- Failure state is visible in the readiness export.

Allowed behavior:

- Run a fixed local test binary or controlled command.
- Capture output for diagnostics.
- Keep user-selected terminal actions routed to Windows Terminal.

## Phase 3: TUI Canary

Run a single controlled TUI canary behind a developer-only setting.

Required evidence:

- Alternate screen, cursor state, scrollback, resize, and interruption are stable.
- Input forwarding handles key, paste-boundary, focus, mouse, and control-sequence metadata safely.
- Close and interrupt do not orphan child processes.
- Terminal-video and audio remain external.

Allowed behavior:

- Enable a canary action for maintainers.
- Log redacted readiness evidence.
- Keep external terminal fallback visible in the UI.

## Phase 4: Media Canary

Route only bounded terminal-frame fixtures through the embedded renderer.

Required evidence:

- Frame budget is enforced.
- Backpressure drops old frames deterministically.
- Audio clock is not blocked by frame rendering.
- Close and interrupt remain responsive under frame load.

Allowed behavior:

- Render fixture frames only.
- Keep mpv, tplay, viu, and Windows Terminal as production media paths.

## Canary Promotion And Rollback

Each phase promotes only after the previous phase has stable redacted evidence. A failure in any phase rolls back to the last phase with passing evidence and keeps Windows Terminal as the production route.

### Synthetic To Echo Process

Promotion requirements:

- `get_dx_agents_embedded_terminal_session_timeline` reports `process_spawned: false`, `allows_arbitrary_shell: false`, and `stores_payloads: false`.
- The synthetic timeline includes open, initial resize, keyboard, paste, focus, mouse, control-sequence, viewport resize, interrupt, and close events.
- Every timeline event is redacted.
- `echo_process_pilot_ready` is true.

Rollback triggers:

- Any raw input value, paste value, key text, mouse text, or control-sequence bytes appear in diagnostics.
- The timeline loses deterministic ordering.
- External terminal fallback is absent.

### Echo Process To TUI Canary

Promotion requirements:

- `run_dx_agents_embedded_terminal_echo_pilot` runs only `dx-agents echo-pilot --json`.
- The echo process exits successfully with the fixed `dx-agents-embedded-terminal-echo-ok` payload.
- The bridge records duration, exit code, stdout/stderr byte counts, and bounded redacted previews.
- Failed or skipped echo runs are visible in the terminal diagnostics panel.

Rollback triggers:

- The pilot accepts user command text or shell metacharacters.
- The process launch path changes to PowerShell, `cmd`, or any user-provided shell.
- The process hangs, leaks a child process, or returns unbounded output.

### TUI Canary To Media Canary

Promotion requirements:

- The TUI canary is behind a developer-only setting.
- The desktop bridge gate stays off unless `DX_AGENTS_TUI_CANARY` is explicitly set for the developer session and the bridge is restarted.
- Lifecycle evidence records open, resize, interrupt, close, and cleanup with bounded timeouts and output caps.
- Alternate screen, cursor state, scrollback, resize, interrupt, and close evidence stays stable across repeated runs.
- Terminal-video and audio still use external routes.
- The canary can be disabled without changing user terminal actions.

Rollback triggers:

- The gate registers or modifies a normal host action.
- Lifecycle evidence reports raw payload storage, arbitrary shell input, or a spawned process while the canary is only gated or armed.
- Close or interrupt leaves a child process alive.
- Resize causes stale cursor state, broken scrollback, or renderer reflow drift.
- Terminal-video, audio, or user-selected terminal actions route through embedded PTY before all production gates are ready.

## Promotion Rule

Embedded PTY can become a selectable production route only after:

- All required gates report ready.
- The readiness export is redacted and stable.
- The operator QA checklist passes.
- A maintainer intentionally flips the route from fallback to selectable.

Until then, the bridge must continue saying that Windows Terminal is the production route.
