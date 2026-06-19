# Tool Safety Audit Review Recovery Alert Digest Release Gate Digest Runbook

This runbook explains the compact release gate digest derived from recovery alert digest release-gate evidence. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Digest States

### ready

The release gate digest is clear because the release gate is ready, all required runbooks are present, and the evidence is safe to share. Treat this as release evidence only after the latest dry-run audit path has been refreshed.

### warning_review

The release gate digest is blocking because the release gate has warning-level evidence. Review the redacted gate state, alert counts, runbook coverage counts, and recovery hint before continuing release or promotion work.

### blocked

The release gate digest is blocking because the release gate has blocked evidence or is not ready. Pause release or promotion work, resolve the blocked release-gate evidence, rerun the dry-run safety path, and refresh audit history.

### runbook_missing

One of the required upstream runbooks is missing. Restore the missing source-owned runbook, rerun release-readiness checks, and refresh audit history before treating the release gate digest as complete.

### release_gate_digest_runbook_missing

This release gate digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating digest guidance as complete.

### redaction_review

The release gate digest cannot be shared until redaction and metadata-only flags are fixed. Regenerate release-gate digest evidence and confirm the digest reports no stored configuration values, no duplicated raw history rows, and only redacted operator text.

## Operator Rules

- Use release gate digest state, severity, release-blocking status, safe-to-share status, gate state, alert counts, runbook coverage counts, and redacted recovery hints only.
- Keep raw drill rows in drill history, audit summaries in audit history, recovery alert rows in their existing metadata surfaces, and release-gate details in the release-gate surface.
- Treat `blocked`, `warning_review`, `runbook_missing`, `release_gate_digest_runbook_missing`, and `redaction_review` as release-blocking.
- Treat `ready` as evidence that the metadata-only safety path is clear, not as permission to skip future dry-run audits.
- Keep release gate digest guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
