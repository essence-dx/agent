# Tool Safety Audit Review Runbook

This runbook explains how to read tool safety audit history in the DX CLI Bridge. It is metadata-only operator guidance. It does not change tool configuration, invoke tools, change approval settings, expand allowlists, include raw row payloads, repeat raw drill row payloads, or expose command, path, domain, environment, allowlist, or secret values.

Use it with the audit summary and audit history panel:

- Run Safety drill to refresh the dry-run tool snapshot and write a redacted audit summary.
- Review the Audit trend row for ready, alert, blocked, and warning deltas.
- Treat ready-to-blocked or rising blocked deltas as a stop signal for live tool expansion.
- Keep tool access unchanged until the next audit history snapshot records the repair.

## Audit States

### empty

No audit history snapshots exist yet. Run the dry-run safety drill once so the bridge can retain the first metadata-only audit summary.

### single snapshot

Only one audit summary exists. Run another dry-run safety drill after the next tool configuration change so the bridge can compare drift.

### stable

The latest comparable summaries are unchanged. Keep monitoring the retained audit history and continue exporting only redacted evidence when operator review needs it.

### changed

The latest summary changed without a clear improving or worsening direction. Review the latest remediation action, confirm the change was expected, and rerun the dry-run safety drill after policy review.

### improving

The latest audit has fewer blockers, warnings, or alerts. Treat this as progress, not permission to widen access. Retain one more clean audit summary before considering broader live tool work.

### worsening

The latest audit moved toward ready-to-blocked, added blocked alerts, added warning alerts, or increased total alerts. Pause live tool expansion, use the latest redacted remediation action, repair the tool condition outside the bridge, and rerun the dry-run safety drill.

### redaction review

The latest audit says redaction review is required. Stop sharing exports, regenerate the drill after fixing redaction, and only continue when the audit confirms it stores no command, path, domain, environment, allowlist, or secret values.

### runbook missing

The audit review flow expects this runbook and the alert runbook to be present. Restore the missing runbook file, rerun release-readiness checks, and then refresh the audit history.

## Operator Checklist

- Confirm audit rows are summaries, not copied raw drill history rows.
- Confirm exports say redacted and do not contain command, path, domain, environment, allowlist, or secret values.
- Compare ready-to-blocked, blocked-alert, warning-alert, and total-alert deltas before live tool work.
- Use the alert runbook for direct drill alert remediation and this runbook for audit history trend remediation.
- Export audit history only when you need an auditable redacted snapshot for operator review.
