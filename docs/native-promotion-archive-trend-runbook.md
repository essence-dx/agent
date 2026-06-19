# Native Promotion Archive Trend Runbook

This runbook explains how to read the native promotion archive trend panel in the DX CLI Bridge. It is diagnostic-only. It does not change embedded terminal routing, and it does not replace the external fallbacks.

External fallbacks stay active:

- Windows Terminal for interactive TUI work.
- mpv for video playback.
- tplay for terminal video playback.
- viu for image preview.

## Trend States

### short

Fewer than two retained snapshots are available. Run Export diagnostics again after the next native promotion status change, then compare the new snapshot.

### stable

The newest retained snapshot has the same blocker count as the oldest retained sample. Review warning alerts and rollback text before treating the archive history as settled.

### improving

The newest retained snapshot has fewer blockers than the oldest retained sample. Continue collecting redacted snapshots and verify the underlying blocker rows before any promotion decision.

### worsening

The newest retained snapshot has more blockers, blocked alerts, or a redaction/diagnostic boundary problem. Pause native promotion work, inspect the blocked alerts, and keep Windows Terminal, mpv, tplay, and viu as active fallbacks.

## Alert Severity

- clean: no blocked or warning drift was detected for the compared snapshots.
- warning: operator review is needed before the archive history can be treated as stable.
- blocked: native promotion work should pause until the blocker or safety boundary is restored.

## Operator Checklist

- Confirm the latest snapshot is redacted and diagnostic-only.
- Check whether blocker count is stable, improving, or worsening.
- Review rollback text when it changes.
- Keep all external fallbacks available during review.
- Regenerate diagnostics after fixing blocker rows so the next trend point records the change.
