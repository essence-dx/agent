# Tool Safety Audit Review Recovery Alert Digest Release Gate Runbook

This runbook explains the release gate derived from the recovery alert digest in the DX CLI Bridge and redacted status exports. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Gate States

### ok

The release gate is clear because the recovery alert digest is ready, redacted, metadata-only, and backed by the required runbooks. Treat this as release evidence only after the latest dry-run audit path has been refreshed.

### warning_review

The release gate is blocking because the recovery alert digest has warning-level evidence. Review the redacted digest state, alert counts, top alert metadata, and recovery hint before continuing release or promotion work.

### blocked

The release gate is blocking because the recovery alert digest has blocked evidence or is not ready. Pause release or promotion work, resolve the blocked recovery alert evidence, rerun the dry-run safety path, and refresh audit history.

### runbook_missing

The recovery alert runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-runbook.md`, rerun release-readiness checks, and refresh audit history before treating the release gate as complete.

### digest_runbook_missing

The recovery alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating the release gate as complete.

### release_gate_runbook_missing

This release gate runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-runbook.md`, rerun release-readiness checks, and refresh audit history before treating release-gate guidance as complete.

### redaction_review

The release gate cannot be shared until redaction and metadata-only flags are fixed. Regenerate recovery alert digest evidence and confirm the gate reports no stored configuration values, no duplicated raw history rows, and only redacted operator text.

## Operator Rules

- Use release gate state, severity, release-blocking status, safe-to-share status, digest state, alert counts, runbook presence, safe target names, and redacted recovery hints only.
- Keep raw drill rows in drill history, audit summaries in audit history, and recovery alert rows in their existing metadata surfaces.
- Treat `blocked`, `warning_review`, `runbook_missing`, `digest_runbook_missing`, `release_gate_runbook_missing`, and `redaction_review` as release-blocking.
- Treat `ok` as evidence that the metadata-only safety path is clear, not as permission to skip future dry-run audits.
- Keep release-gate guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
