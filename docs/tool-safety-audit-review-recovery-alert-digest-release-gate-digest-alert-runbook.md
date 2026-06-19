# Tool Safety Audit Review Recovery Alert Digest Release Gate Digest Alert Runbook

This runbook explains the alert rows derived from release gate digest evidence. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate release-gate rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Alert States

### release_gate_digest_ready

The release gate digest alert is clear because the release gate digest is ready, safe to share, and backed by source-owned runbooks. Treat this as metadata-only release evidence after the latest dry-run audit path has been refreshed.

### release_gate_digest_warning_review

The release gate digest alert is warning-level. Review the redacted digest severity, alert counts, runbook coverage, and recovery hint before continuing release or promotion work.

### release_gate_digest_blocked

The release gate digest alert is blocked. Keep release or promotion work paused, resolve the blocked release-gate digest evidence, rerun the dry-run safety path, and refresh audit history.

### release_gate_digest_runbook_missing

The upstream release gate digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating digest alerts as complete.

### release_gate_digest_alert_runbook_missing

This release gate digest alert runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-runbook.md`, rerun release-readiness checks, and refresh audit history before treating alert guidance as complete.

### release_gate_digest_redaction_review

The release gate digest alert cannot be shared until redaction and metadata-only flags are fixed. Regenerate release-gate digest evidence and confirm the alert reports no stored configuration values, no duplicated raw history rows, and only redacted operator text.

### release_gate_digest_review_required

The release gate digest alert has an unknown or new state. Treat it as review-required until the source-owned state mapping, redaction behavior, and release-readiness checks are updated.

## Operator Rules

- Use alert id, level, title, detail, gate state, digest runbook presence, alert runbook presence, and redacted recovery hints only.
- Keep raw drill rows in drill history, audit summaries in audit history, recovery rows in recovery surfaces, release-gate details in release-gate surfaces, and digest details in the release gate digest surface.
- Treat `release_gate_digest_warning_review`, `release_gate_digest_blocked`, `release_gate_digest_runbook_missing`, `release_gate_digest_alert_runbook_missing`, `release_gate_digest_redaction_review`, and `release_gate_digest_review_required` as release-blocking until reviewed.
- Treat `release_gate_digest_ready` as evidence that the metadata-only safety path is clear, not as permission to skip future dry-run audits.
- Keep release gate digest alert guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
