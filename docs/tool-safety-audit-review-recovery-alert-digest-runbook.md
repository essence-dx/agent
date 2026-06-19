# Tool Safety Audit Review Recovery Alert Digest Runbook

This runbook explains the compact recovery alert digest shown in the DX CLI Bridge and redacted status exports. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Digest States

### ok

The recovery alert digest is clear. Treat this as monitoring evidence only: keep the metadata-only audit workflow active and rerun the dry-run safety path before promotion work.

### warning_review

The digest contains warning-level recovery alert evidence. Review the redacted alert ids, count fields, top alert metadata, and recovery hint. Complete operator review before promotion work continues.

### blocked

The digest contains blocked recovery alert evidence. Pause release or promotion work, resolve the blocked recovery alert through the recovery alert runbook, rerun the dry-run safety path, and refresh audit history.

### runbook_missing

The recovery alert runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-runbook.md`, rerun release-readiness checks, and refresh audit history before treating recovery alert evidence as complete.

### digest_runbook_missing

This recovery alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating digest guidance as complete.

### redaction_review

The digest cannot be shared until redaction and metadata-only flags are fixed. Regenerate recovery alert evidence and confirm the digest reports no stored configuration values, no duplicated raw history rows, and only redacted operator text.

## Operator Rules

- Use digest state, severity, alert counts, runbook presence, safe target names, top alert id/title/level, and redacted recovery hints only.
- Keep raw drill rows in drill history, audit summaries in audit history, and recovery alert rows in their existing metadata surfaces.
- Treat `blocked` as release-blocking until dry-run recovery evidence returns to `ok`.
- Treat `warning_review` as an operator review gate, not a release pass.
- Treat `runbook_missing`, `digest_runbook_missing`, and `redaction_review` as evidence-quality blockers.
- Keep digest guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
