# Tool Safety Audit Review Recovery Alert Digest Release Gate Digest Alert Digest Alert Digest Alert Digest Alert Digest Alert Runbook

This runbook explains the metadata-only alert rows derived from the release gate digest alert digest alert digest alert digest alert digest state. It is redacted, config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate release-gate rows, duplicate digest rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Alert States

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_ready

The release gate digest alert digest alert digest alert digest alert digest is ready. Treat this as release evidence only when the upstream digest runbook, this alert runbook, and the latest dry-run audit path are all current.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_warning_review

Warning-level digest evidence needs operator review. Inspect only the redacted digest state, severity, alert id, alert level, runbook coverage, and recovery hint before continuing release or promotion work.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_blocked

Blocked digest evidence pauses release or promotion work. Resolve the blocked metadata, refresh the dry-run audit path, and rerun release-readiness checks before treating the alert row as clear.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing

The upstream release gate digest alert digest alert digest alert digest alert digest runbook is missing. Restore the source-owned runbook, rerun release-readiness checks, and refresh audit history before using this alert row as release evidence.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing

The upstream release gate digest alert digest alert digest alert digest alert digest alert runbook is missing. Restore the source-owned runbook, rerun release-readiness checks, and refresh audit history before using this alert row as release evidence.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing

The release gate digest alert digest alert digest alert digest alert digest runbook is missing. Restore the source-owned runbook, rerun release-readiness checks, and refresh audit history before using this alert row as release evidence.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing

This release gate digest alert digest alert digest alert digest alert digest alert runbook is missing. Restore this source-owned runbook, rerun release-readiness checks, and refresh audit history before treating derived alert rows as complete.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review

The alert row cannot be shared until metadata-only and redaction flags are corrected. Regenerate the digest-derived alert evidence after confirming it stores no configuration values and does not duplicate raw history rows.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_empty

No release gate digest alert digest alert digest alert digest alert digest evidence is available. Run the tool safety audit, refresh audit history, and rerun release-readiness checks before using this alert surface.

### release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_review_required

The digest state is unknown or newly introduced. Treat it as review-required until the source-owned state mapping, recovery hint, bridge rendering, and release-readiness checks are updated.

## Operator Rules

- Use only alert id, level, title, digest state, runbook presence, safe runbook target, redacted detail, redacted recovery hint, and metadata-only flags.
- Keep raw drill rows in drill history, audit summaries in audit history, recovery rows in recovery surfaces, release-gate rows in release-gate surfaces, digest details in digest surfaces, and these derived alert rows in the alert surface.
- Treat warning, blocked, missing-runbook, empty, review-required, and redaction states as release-blocking until reviewed and refreshed.
- Treat ready as evidence that the metadata-only alert surface is clear, not as permission to skip future dry-run audits.
- Keep this guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
