# Tool Safety Audit Review Recovery Runbook

This runbook explains the recovery drill rows shown in the DX CLI Bridge after audit review escalation evidence is calculated. It is metadata-only operator guidance. It does not run tools, change tool configuration, alter approval settings, widen allowlists, duplicate raw audit or drill rows, or expose command, path, domain, environment, allowlist, or secret values.

## Recovery States

### blocked_recovery

The audit review has at least one blocked escalation alert. Pause release or promotion work, open the audit review alert runbook, resolve the blocked condition outside the bridge, then rerun the dry-run tool safety drill and refresh the audit review.

### warning_recovery

The audit review has warning escalation evidence but no blocked alert. Review the warning alert ids, confirm the latest redacted remediation action was expected, and rerun the dry-run audit path before treating the warning as stable.

### evidence_pending

The audit review still requires another metadata-only snapshot before the recovery drill can be treated as settled. Keep the retained summaries, then refresh audit history after the next dry-run safety drill.

### cleared_recovery

No blocked or warning escalation remains. Keep monitoring retained metadata-only audit summaries and continue using redacted status exports for operator evidence.

### runbook_missing_recovery

The recovery drill runbook is missing. Restore this runbook, rerun release-readiness checks, and refresh the audit review before treating recovery drill evidence as complete.

## Operator Rules

- Use recovery state, outcome, alert ids, counts, and redacted next-action text only.
- Keep raw drill rows in drill history and audit summaries in audit history.
- Treat `blocked_recovery` and `runbook_missing_recovery` as release-blocking.
- Treat `warning_recovery` as requiring operator review before promotion work continues.
- Treat `cleared_recovery` as safe to monitor, not as permission to skip future dry-run audits.
