# Tool Safety Alert Runbook

This runbook explains how to read tool safety history alerts in the DX CLI Bridge. It is metadata-only operator guidance. It does not change tool configuration, invoke tools, change approval settings, expand allowlists, or expose command, path, domain, environment, or secret values.

Use it with the dry-run safety drill and retained history panel:

- Run Safety drill to create or refresh the latest redacted snapshot.
- Review the History trend row for allowed, approval, denied, missing, and critical-blocker deltas.
- Review Safety alert rows before live tool work.
- Keep tool access unchanged until the next dry-run snapshot records the repair.

## Alert States

### blocked

A blocked alert means the latest retained snapshot has critical blockers, denied tools, missing tools, or a worsening blocked delta. Pause live tool expansion, inspect the latest redacted recovery hint, repair the underlying tool installation or policy outside the bridge, and rerun the dry-run safety drill.

### warning

A warning alert means the latest retained snapshot has more approval-required tools or another review-only posture change. Keep the approval boundary intact, confirm the change was expected, and rerun the dry-run safety drill after policy review.

### improving

An improving alert means blockers decreased between retained snapshots. Treat this as progress, not permission to widen access. Rerun the dry-run safety drill once more after the repair so the trend stays stable.

### stable

A stable alert means the latest comparable snapshots have no worsening deltas. Continue using the retained history as evidence, and keep watching for denied, missing, approval-required, and critical-blocker changes.

### empty

An empty-history alert means no retained tool safety snapshots exist yet. Run the dry-run safety drill to seed the history before depending on the bridge panel for tool readiness evidence.

## Operator Checklist

- Confirm alerts are based on dry-run or mock metadata, not live tool invocation.
- Confirm recovery hints are redacted before sharing exports.
- Repair missing or denied tools outside the bridge, then rerun the dry-run safety drill.
- Do not paste secrets, command values, allowlist entries, domains, or local paths into the runbook, changelog, or bridge notes.
- Export history only when you need an auditable redacted snapshot for operator review.
