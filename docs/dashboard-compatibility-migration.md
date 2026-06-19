# Dashboard Compatibility Migration

Version: 8

This note is the cleanup gate for DX Agents dashboard compatibility aliases. Legacy aliases remain active until a later release verifies the compatibility, release-checklist, maintainer-handoff, final-operator-packet, evidence-soak, exact cutover-plan, retained cutover-plan history, cutover-plan alerts, cutover release packet, retained release-packet history, dry-run plan, rollback rehearsal, alias-removal evidence quorum, retained quorum history, quorum alerts, disabled no-op plan, maintainer signoff, parity inventory, and verifier evidence, then lands a separate alias-removal commit with rollback coverage. Version 8 adds the quorum, no-op plan, maintainer signoff, inventory, and verifier gates; it does not authorize alias removal.

## Compatibility Surface

Read/write shims:

- `dx_agents_token` writes alongside `zeroclaw_token` so existing dashboard sessions survive the product rename.
- `dx_agents_session_id` writes alongside `zeroclaw_session_id` for WebSocket session continuity.
- `dx_agents_chat_history_v1:` writes alongside `zeroclaw_chat_history_v1:` for persisted chat history.
- `dx_agents_chat_compact` writes alongside `zeroclaw_chat_compact`.
- `dx_agents_show_tool_activity` writes alongside `zeroclaw_show_tool_activity`.
- `dx_agents-theme` writes alongside `zeroclaw-theme`.
- `dx_agents-locale` writes alongside `zeroclaw-locale`.
- `dx_agents-sidebar-collapsed` writes alongside `zeroclaw-sidebar-collapsed`.
- `dx_agents_live_logs` writes alongside `zeroclaw_live_logs`.

Fallback-only aliases:

- `DX_AGENTS_GATEWAY_PORT` prefers DX naming while `ZEROCLAW_GATEWAY_PORT` remains readable.
- `__DX_AGENTS_BASE__` and `__DX_AGENTS_GATEWAY__` prefer DX naming while the old window globals remain readable.
- `dx-agents-unauthorized` is dispatched with `zeroclaw-unauthorized` for old listeners.
- `dx-agents.v1` is offered before `zeroclaw.v1` for dashboard WebSocket clients.

Exact candidate aliases for a future separate removal commit:

- `ZEROCLAW_CONFIG_DIR` -> `DX_AGENTS_CONFIG_DIR`
- `ZEROCLAW_WORKSPACE` -> `DX_AGENTS_WORKSPACE`
- `ZEROCLAW_PROVIDER` -> `DX_AGENTS_PROVIDER`
- `ZEROCLAW_MODEL` -> `DX_AGENTS_MODEL`
- `ZEROCLAW_PROVIDER_URL` -> `DX_AGENTS_PROVIDER_URL`
- `ZEROCLAW_API_KEY` -> `DX_AGENTS_API_KEY`
- `ZEROCLAW_CODEX_BASE_URL` -> `DX_AGENTS_CODEX_BASE_URL`
- `ZEROCLAW_CODEX_RESPONSES_URL` -> `DX_AGENTS_CODEX_RESPONSES_URL`
- `ZEROCLAW_CODEX_REASONING_EFFORT` -> `DX_AGENTS_CODEX_REASONING_EFFORT`
- `ZEROCLAW_ACP_TOKEN` -> `DX_AGENTS_ACP_TOKEN`
- `ZEROCLAW_ACP_GATEWAY_URL` -> `DX_AGENTS_ACP_GATEWAY_URL`
- `ZEROCLAW_GATEWAY_PORT` -> `DX_AGENTS_GATEWAY_PORT`
- `ZEROCLAW_GATEWAY_TOKEN` -> `DX_AGENTS_GATEWAY_TOKEN`
- `ZEROCLAW_ACP_BRIDGE_TOKEN` -> `DX_AGENTS_ACP_BRIDGE_TOKEN`
- `ZEROCLAW_ACP_PAIRING_CODE` -> `DX_AGENTS_ACP_PAIRING_CODE`
- `__ZEROCLAW_BASE__` -> `__DX_AGENTS_BASE__`
- `__ZEROCLAW_GATEWAY__` -> `__DX_AGENTS_GATEWAY__`
- `zeroclaw_token` -> `dx_agents_token`
- `zeroclaw_session_id` -> `dx_agents_session_id`
- `zeroclaw_chat_history_v1:` -> `dx_agents_chat_history_v1:`
- `zeroclaw_chat_compact` -> `dx_agents_chat_compact`
- `zeroclaw_show_tool_activity` -> `dx_agents_show_tool_activity`
- `zeroclaw-theme` -> `dx_agents-theme`
- `zeroclaw-locale` -> `dx_agents-locale`
- `zeroclaw-sidebar-collapsed` -> `dx_agents-sidebar-collapsed`
- `zeroclaw_live_logs` -> `dx_agents_live_logs`
- `zeroclaw-unauthorized` -> `dx-agents-unauthorized`
- `zeroclaw.v1` -> `dx-agents.v1`

## Migration Gate

Legacy aliases remain active until all of these are true:

- `dx-agents dashboard-compatibility --json` and `get_dx_agents_dashboard_compatibility_status` report `ready: true`.
- The dashboard compatibility usage telemetry reports `decommission_ready: true` from `target/host-telemetry/dashboard-compatibility-usage.json`.
- `dx-agents dashboard-compatibility-history --archive --json` has retained at least two payload-free dashboard telemetry snapshots.
- `dx-agents dashboard-compatibility-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-aliases --json` reports no active legacy env aliases and keeps migration readers present.
- `dx-agents parity legacy-aliases-history --archive --json` has retained at least two payload-free snapshots.
- `dx-agents parity legacy-aliases-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-checklist --json` reports `removal_allowed: true`.
- `dx-agents parity legacy-alias-removal-checklist-history --archive --json` has retained at least two payload-free checklist snapshots.
- `dx-agents parity legacy-alias-removal-checklist-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-maintainer-handoff --json` reports `maintainer_signoff_allowed: true`.
- `dx-agents parity legacy-alias-removal-maintainer-handoff-history --archive --json` has retained at least two payload-free maintainer handoff snapshots.
- `dx-agents parity legacy-alias-removal-maintainer-handoff-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-operator-packet --json` reports `final_removal_allowed: true`.
- `dx-agents parity legacy-alias-removal-operator-packet-history --archive --json` has retained at least two payload-free final operator packet snapshots.
- `dx-agents parity legacy-alias-removal-operator-packet-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-evidence-soak --json` reports `cutover_candidate_ready: true`.
- `dx-agents parity legacy-alias-removal-evidence-soak-history --archive --json` has retained at least two payload-free evidence-soak snapshots.
- `dx-agents parity legacy-alias-removal-evidence-soak-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-cutover-plan --json` reports `cutover_allowed: true`, `changes_applied: false`, and a non-empty rollback action list.
- `dx-agents parity legacy-alias-removal-cutover-plan-history --archive --json` has retained at least two payload-free exact-alias cutover-plan snapshots.
- `dx-agents parity legacy-alias-removal-cutover-plan-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-cutover-release-packet --json` reports `packet_state: "clear"`, `release_note_state: "clear"`, and `rollback_state: "clear"`.
- `dx-agents parity legacy-alias-removal-cutover-release-packet-history --archive --json` has retained at least two payload-free release-packet snapshots.
- `dx-agents parity legacy-alias-removal-cutover-release-packet-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-dry-run --json` reports `dry_run_state: "clear"`, `dry_run_only: true`, `writes_changes: false`, and complete alias/affected-file/rollback coverage.
- `dx-agents parity legacy-alias-removal-rollback-rehearsal --json` reports `rehearsal_state: "clear"`, `rehearsal_only: true`, `writes_changes: false`, and no missing recovery paths.
- `dx-agents parity legacy-alias-removal-evidence-quorum --json` reports `quorum_met: true`, `writes_changes: false`, and clear dashboard telemetry, dry-run, rollback, release-packet, inventory, and verifier signals.
- `dx-agents parity legacy-alias-removal-evidence-quorum-history --archive --json` has retained at least two fresh, payload-free quorum snapshots.
- `dx-agents parity legacy-alias-removal-evidence-quorum-history-alerts --json` reports `alert_state: "clear"`.
- `dx-agents parity legacy-alias-removal-noop-plan --json` reports `plan_state: "clear"`, `disabled_by_default: true`, `execution_enabled: false`, `writes_changes: false`, `removes_aliases: false`, and complete affected-alias plus rollback-manifest coverage.
- `dx-agents parity legacy-alias-removal-maintainer-signoff --json` reports `signoff_allowed: true`, `execution_enabled: false`, `writes_changes: false`, and `removes_aliases: false`.
- `dx-agents parity inventory --json` includes `dashboard_compatibility_history`, `dashboard_compatibility_alerts`, `legacy_aliases`, `legacy_alias_history`, `legacy_alias_alerts`, `legacy_alias_removal_checklist`, `legacy_alias_removal_checklist_history`, `legacy_alias_removal_checklist_history_alerts`, `legacy_alias_removal_maintainer_handoff`, `legacy_alias_removal_maintainer_handoff_history`, `legacy_alias_removal_maintainer_handoff_history_alerts`, `legacy_alias_removal_operator_packet`, `legacy_alias_removal_operator_packet_history`, `legacy_alias_removal_operator_packet_history_alerts`, `legacy_alias_removal_evidence_soak`, `legacy_alias_removal_evidence_soak_history`, `legacy_alias_removal_evidence_soak_history_alerts`, `legacy_alias_removal_cutover_plan`, `legacy_alias_removal_cutover_plan_history`, `legacy_alias_removal_cutover_plan_history_alerts`, `legacy_alias_removal_cutover_release_packet`, `legacy_alias_removal_cutover_release_packet_history`, `legacy_alias_removal_cutover_release_packet_history_alerts`, `legacy_alias_removal_dry_run`, `legacy_alias_removal_rollback_rehearsal`, `legacy_alias_removal_evidence_quorum`, `legacy_alias_removal_evidence_quorum_history`, `legacy_alias_removal_evidence_quorum_history_alerts`, `legacy_alias_removal_noop_plan`, and `legacy_alias_removal_maintainer_signoff`.
- `dx-agents parity verify --json` passes legacy alias schema, surface, alert, release-checklist item, checklist-history surface, checklist-history alert, maintainer-handoff section, maintainer-handoff history surface, maintainer-handoff history alert, final-operator-packet section, packet-history surface, packet-history alert, evidence-soak section, evidence-soak-history surface, evidence-soak-history alert, cutover-plan candidate, cutover-plan prerequisite, cutover-plan rollback, cutover-plan history surface, cutover-plan history alert, cutover release-packet section, release-packet history surface, release-packet history alert, dry-run section, dry-run alias, dry-run rollback, rollback rehearsal section, quorum signal, quorum section, quorum-history surface, quorum-alert, no-op plan, maintainer-signoff requirement, maintainer-signoff action, dashboard telemetry history surface, dashboard telemetry alert, and redaction checks.
- Redacted bridge status export includes dashboard compatibility status without stored values.
- A release note describes rollback for users with old dashboard storage, environment aliases, browser globals, event aliases, and WebSocket subprotocols.
- The cleanup is committed separately from unrelated UI or protocol work.

## Current Decision

Do not remove the legacy aliases in this batch. DX Agents now has the required metadata-only readiness report, retained snapshot history, alert layer, release checklist, retained checklist history, checklist alerts, maintainer handoff, retained handoff history, handoff history alerts, final operator packet, retained packet history, packet history alerts, evidence soak, retained soak history, soak alerts, exact-alias cutover plan, retained cutover-plan history, cutover-plan history alerts, cutover release packet, retained release-packet history, release-packet history alerts, dry-run planning, rollback rehearsal, alias-removal evidence quorum, retained quorum history, quorum alerts, disabled no-op plan, maintainer signoff, parity inventory coverage, and verifier coverage, but the compatibility shims remain the safe default until repeated retained evidence and dashboard telemetry are clear.
