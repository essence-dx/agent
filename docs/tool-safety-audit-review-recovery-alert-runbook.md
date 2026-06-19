# Tool Safety Audit Review Recovery Alert Runbook

This runbook explains the recovery digest alert rows shown in the DX CLI Bridge after audit recovery drill evidence is summarized. It is metadata-only operator guidance. It does not execute tools, change tool configuration, alter approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, or expose command, path, domain, environment, allowlist, or secret values.

## Alert IDs

### recovery_blocked

The recovery digest still has blocked audit recovery evidence. Pause release or promotion work, follow the audit recovery runbook, resolve the blocked condition outside the bridge, then rerun the dry-run safety path and refresh audit history.

### recovery_warning_review

The recovery digest has warning-level recovery evidence. Review the redacted warning ids and the latest redacted next action, confirm the remediation was expected, then rerun the dry-run audit path before continuing promotion work.

### recovery_pending_evidence

The recovery digest needs another metadata-only audit snapshot before it can be treated as cleared. Keep the retained summaries, then refresh audit history after the next dry-run safety drill.

### recovery_cleared

The recovery digest is cleared, redacted, and metadata-only. Keep monitoring retained summaries and continue using redacted status exports as operator evidence.

### recovery_runbook_missing

The audit recovery drill runbook is missing. Restore the source-owned recovery runbook, rerun release-readiness checks, and refresh audit history before treating recovery digest evidence as complete.

### recovery_alert_runbook_missing

This recovery alert runbook is missing. Restore this file, rerun release-readiness checks, and refresh audit history before treating recovery alert guidance as complete.

### recovery_redaction_review

The recovery digest cannot be used as shareable evidence until redaction and metadata-only flags are fixed. Regenerate recovery evidence and confirm no command, path, domain, environment, allowlist, or secret values are present.

## Operator Rules

- Use alert ids, severity, runbook presence, target names, counts, and redacted recovery hints only.
- Keep raw drill rows in drill history and audit summaries in audit history.
- Treat `blocked` recovery alerts as release-blocking until the dry-run evidence and recovery digest return to a safe state.
- Treat `warning` recovery alerts as operator-review states before promotion work continues.
- Treat `ok` recovery alerts as monitoring evidence, not permission to skip future dry-run audits.
