# Tool Safety Audit Review Recovery Alert Digest Release Gate Digest Alert Digest Alert Digest Runbook

This runbook explains the compact digest that rolls release gate digest alert digest alert rows into release-readiness evidence. It is metadata-only, redacted, and config-free operator guidance. It does not execute tools, change approval settings, widen allowlists, duplicate raw audit rows, duplicate recovery rows, duplicate release-gate rows, duplicate digest rows, duplicate alert rows, or reveal command, path, domain, environment, allowlist, or secret values.

## Digest States

### ready

The release gate digest alert digest alert digest is ready because all release gate digest alert digest alert rows are clear, safe to share, and backed by source-owned runbooks. Treat this as compact metadata-only evidence after the latest dry-run audit path has been refreshed.

### warning_review

The digest is warning-level. Review the alert count, warning count, top alert level, runbook coverage, and redacted recovery hint before continuing release or promotion work.

### blocked

The digest is blocked. Keep release or promotion work paused until blocked release gate digest alert digest alert rows are cleared, the dry-run safety path has been rerun, and audit history has been refreshed.

### release_gate_digest_alert_digest_runbook_missing

The upstream release gate digest alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating alert digest alert digest rollups as complete.

### release_gate_digest_alert_digest_alert_runbook_missing

The upstream release gate digest alert digest alert runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook.md`, rerun release-readiness checks, and refresh audit history before treating alert digest alert rows as complete.

### release_gate_digest_alert_digest_alert_digest_runbook_missing

This release gate digest alert digest alert digest runbook is missing. Restore `docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook.md`, rerun release-readiness checks, and refresh audit history before treating compact digest guidance as complete.

### redaction_review

The digest cannot be shared until redaction and metadata-only flags are fixed. Regenerate release gate digest alert digest alert evidence and confirm the digest reports no stored configuration values, no duplicated raw history rows, no duplicated alert payloads, and only redacted operator text.

### empty

No release gate digest alert digest alert rows were available. Run the tool safety audit, refresh audit history, and rerun release-readiness checks before using this digest as release evidence.

### review_required

The digest has an unknown or new state. Treat it as review-required until the source-owned state mapping, redaction behavior, and release-readiness checks are updated.

## Operator Rules

- Use schema version, state, severity, alert counts, top alert id, top alert level, runbook presence, safe-to-share flags, and redacted recovery hints only.
- Keep raw drill rows in drill history, audit summaries in audit history, recovery rows in recovery surfaces, release-gate rows in release-gate surfaces, digest details in the release gate digest surface, alert details in release gate digest alert rows, and alert digest details in the release gate digest alert digest surface.
- Treat `warning_review`, `blocked`, `release_gate_digest_alert_digest_runbook_missing`, `release_gate_digest_alert_digest_alert_runbook_missing`, `release_gate_digest_alert_digest_alert_digest_runbook_missing`, `redaction_review`, `empty`, and `review_required` as release-blocking until reviewed.
- Treat `ready` as evidence that the metadata-only safety path is clear, not as permission to skip future dry-run audits.
- Keep release gate digest alert digest alert digest guidance config-free: restore source files and rerun checks, but do not place secrets, command output, path output, domain output, environment output, or allowlist content in docs or status exports.
