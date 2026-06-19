# Tool Safety Audit Review Recovery Alert Digest Release Gate Digest Alert Digest Alert Digest Alert Digest Runbook

This runbook explains the compact digest derived from release gate digest alert digest alert digest alert rows. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate release-gate rows, duplicate digest rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Digest States

### ready

The release gate digest alert digest alert digest alert digest is clear because all alert rows are metadata-only, redacted, and backed by source-owned runbooks. Treat this as release evidence only after the latest dry-run audit path has been refreshed.

### warning_review

The digest contains warning-level alert rows. Review the top alert id, top alert level, warning count, runbook coverage, and recovery hint before continuing release or promotion work.

### blocked

The digest contains blocked alert rows. Keep release or promotion work paused, resolve the blocked alert evidence, rerun the dry-run safety path, and refresh audit history before using this digest as release evidence.

### release_gate_digest_alert_digest_alert_digest_runbook_missing

The upstream release gate digest alert digest alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating this digest as complete.

### release_gate_digest_alert_digest_alert_digest_alert_runbook_missing

The release gate digest alert digest alert digest alert runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook.md`, rerun release-readiness checks, and refresh audit history before treating this digest as complete.

### release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing

This release gate digest alert digest alert digest alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating digest guidance as complete.

### redaction_review

The digest cannot be shared until redaction and metadata-only flags are fixed. Regenerate alert digest alert digest alert evidence and confirm the digest reports no stored configuration values, no duplicated raw history rows, no duplicated alert payloads, and only redacted operator text.

### empty

No release gate digest alert digest alert digest alert rows are available. Run the tool safety audit, refresh audit history, and rerun release-readiness checks before using this digest as release evidence.

### review_required

The digest state is unknown or new. Treat it as review-required until the source-owned state mapping, redaction behavior, and release-readiness checks are updated.

## Operator Rules

- Use digest state, severity, alert counts, top alert id, top alert level, runbook coverage, redacted recovery hint, and metadata-only flags only.
- Keep raw drill rows in drill history, audit summaries in audit history, recovery rows in recovery surfaces, release-gate rows in release-gate surfaces, compact digest details in release gate digest surfaces, and alert rows in alert surfaces.
- Treat `warning_review`, `blocked`, `release_gate_digest_alert_digest_alert_digest_runbook_missing`, `release_gate_digest_alert_digest_alert_digest_alert_runbook_missing`, `release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing`, `redaction_review`, `empty`, and `review_required` as release-blocking until reviewed.
- Treat `ready` as evidence that the metadata-only safety path is clear, not as permission to skip future dry-run audits.
- Keep release gate digest alert digest alert digest alert digest guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
