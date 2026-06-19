# Tool Safety Audit Review Alert Runbook

This runbook explains the audit review alert rows shown in the DX CLI Bridge. It is metadata-only operator guidance. It does not change tool configuration, invoke tools, alter approval settings, expand allowlists, duplicate raw audit or drill rows, or expose command, path, domain, environment, allowlist, or secret values.

## Severity Levels

- `ok`: the alert is informational confirmation that the audit review is stable or improving.
- `info`: the alert needs more retained evidence before it can prove a durable trend.
- `warning`: the alert needs operator review before promotion work continues.
- `blocked`: the alert blocks release or promotion work until the redacted remediation action is resolved.

## Alert IDs

### audit_empty

No audit review history exists yet. Run the dry-run safety drill and refresh the audit review so the first redacted audit summary can be retained.

### audit_single_snapshot

Only one audit summary is retained. Keep the snapshot, then rerun the dry-run safety drill after the next meaningful tool configuration change to create comparable evidence.

### audit_stable

The latest comparable audit summaries are stable and metadata-only. Keep monitoring retained summaries and avoid copying raw drill rows into review notes.

### audit_improving

The latest audit reduced alert pressure. Retain one more clean summary before treating the improvement as stable release evidence.

### audit_changed

The audit changed without a clear improving or worsening direction. Review the latest redacted remediation action, confirm the change was expected, and rerun the dry-run safety drill.

### audit_ready_to_blocked

The audit moved from ready to blocked. Stop promotion work, open the audit review runbook, resolve the blocked condition outside the bridge, and rerun the dry-run drill.

### audit_runbook_missing

The audit review alert runbook or audit review runbook is missing. Restore the missing docs, rerun release-readiness, and refresh the audit review.

### audit_redaction_review

The audit review cannot be used as shareable evidence until redaction is fixed. Regenerate audit history and confirm no command, path, domain, environment, allowlist, or secret values are present.

### audit_worsening

The audit trend increased blocker, warning, or alert pressure. Treat the state as release-blocking until the audit returns to a stable redacted state.

### audit_review_required

The alert state is unknown or does not map to a named state. Follow the latest redacted remediation action and rerun the audit review.

## Operator Rules

- Use alert ids, severity, counts, and redacted remediation text only.
- Keep raw drill rows in drill history and audit summaries in audit history.
- Do not paste local values into docs, changelog entries, issue text, screenshots, or bridge notes.
- Treat `blocked` alerts as release-blocking until the dry-run evidence and audit digest return to a safe state.
