use std::{
    collections::HashSet,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;

const DEFAULT_DX_CLI_ROOT: &str = r"G:\Cli";
const CONTRACT_RELATIVE_PATH: &[&str] = &["target", "host-contract", "dx-host-contract.json"];
const INPUT_TOKEN: &str = "{{input}}";
const BRIDGE_SETTINGS_STORE: &str = "dx-cli-bridge-settings.json";
const BRIDGE_SETTINGS_KEY: &str = "settings";
const DASHBOARD_COMPATIBILITY_MIGRATION_PLAN: &str = "docs/dashboard-compatibility-migration.md";
const NATIVE_PROMOTION_ARCHIVE_TREND_RUNBOOK: &str =
    "docs/native-promotion-archive-trend-runbook.md";
const TOOL_SAFETY_ALERT_RUNBOOK: &str = "docs/tool-safety-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK: &str = "docs/tool-safety-audit-review-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK: &str =
    "docs/tool-safety-audit-review-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-alert-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK: &str =
    "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook.md";
const TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK:
    &str = "docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md";
const DASHBOARD_COMPATIBILITY_USAGE_FILE: &str = "dashboard-compatibility-usage.json";
const DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY: &str =
    "dx_agents_dashboard_compatibility_usage_v1";
const DX_CLI_IPC_COMMANDS: &[&str] = &[
    "get_dx_cli_host_contract",
    "get_dx_cli_host_status",
    "get_dx_cli_host_menu",
    "get_dx_cli_command_history",
    "get_dx_cli_bridge_settings",
    "save_dx_cli_bridge_settings",
    "plan_dx_cli_action",
    "run_dx_cli_json_action",
    "run_dx_cli_captured_action",
    "launch_dx_cli_terminal_action",
    "get_dx_agents_provider_health",
    "run_dx_agents_provider_smoke",
    "get_dx_agents_provider_smoke_history",
    "get_dx_agents_gateway_paircode",
    "get_dx_agents_gateway_pairing_drill",
    "get_dx_agents_continuation_status",
    "get_dx_agents_compact_status",
    "get_dx_agents_cron_preview",
    "get_dx_agents_cron_history",
    "get_dx_agents_cron_delivery_drill",
    "get_dx_agents_tool_safety_drill",
    "get_dx_agents_tool_safety_drill_history",
    "get_dx_agents_tool_safety_audit",
    "get_dx_agents_tool_safety_audit_history",
    "export_dx_agents_tool_safety_audit_history",
    "open_dx_agents_tool_safety_audit_history",
    "export_dx_agents_tool_safety_drill_history",
    "open_dx_agents_tool_safety_drill_history",
    "get_dx_agents_embedded_terminal_fixtures",
    "get_dx_agents_embedded_terminal_readiness",
    "get_dx_agents_embedded_terminal_session_timeline",
    "get_dx_agents_embedded_terminal_media_canary_evidence",
    "get_dx_agents_embedded_terminal_tui_canary_gate",
    "get_dx_agents_embedded_terminal_tui_canary_lifecycle",
    "get_dx_agents_embedded_terminal_tui_canary_renderer_evidence",
    "run_dx_agents_embedded_terminal_echo_pilot",
    "run_dx_agents_embedded_terminal_tui_canary_runner",
    "get_dx_agents_release_readiness",
    "get_dx_agents_dashboard_compatibility_status",
    "export_dx_agents_bridge_status",
    "get_dx_agents_bridge_status_exports",
    "open_dx_agents_bridge_status_export",
    "get_dx_cli_native_promotion_archives",
    "get_dx_cli_native_promotion_archive_diff",
    "open_dx_cli_native_promotion_archive",
    "open_dx_agents_continuation_target",
    "run_dx_cli_bridge_self_test",
];
const PROVIDER_SMOKE_MESSAGE: &str = "Reply with exactly: dx-agents-provider-ok";
const PROVIDER_SMOKE_HISTORY_FILE: &str = "dx-provider-smoke.jsonl";
const TOOL_SAFETY_DRILL_HISTORY_FILE: &str = "dx-tool-safety-drill.jsonl";
const TOOL_SAFETY_DRILL_HISTORY_LIMIT: usize = 50;
const TOOL_SAFETY_AUDIT_HISTORY_FILE: &str = "dx-tool-safety-audit.jsonl";
const TOOL_SAFETY_AUDIT_HISTORY_LIMIT: usize = 50;
const TUI_CANARY_ENV: &str = "DX_AGENTS_TUI_CANARY";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliHostContract {
    pub schema_version: String,
    pub host_name: String,
    pub workspace: String,
    pub launcher: DxCliLauncher,
    pub health: DxCliHealth,
    pub embedding: serde_json::Value,
    #[serde(default)]
    pub action_groups: Vec<DxCliActionGroup>,
    pub actions: Vec<DxCliAction>,
    pub media_routes: serde_json::Value,
    #[serde(default)]
    pub media_viewer: Option<serde_json::Value>,
    #[serde(default)]
    pub telemetry: Option<DxCliTelemetry>,
    #[serde(default)]
    pub settings: Option<DxCliSettingsContract>,
    pub terminal_readiness: serde_json::Value,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliTelemetry {
    pub supported: bool,
    pub schema_version: String,
    pub storage: String,
    pub history_path: String,
    pub max_entries: usize,
    pub redaction: serde_json::Value,
    pub event_fields: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliSettingsContract {
    pub supported: bool,
    pub schema_version: String,
    pub persistence: String,
    pub namespace: String,
    pub defaults: DxCliBridgeSettings,
    pub constraints: DxCliSettingsConstraints,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxCliBridgeSettings {
    pub media_input: String,
    pub preferred_terminal_surface: String,
    pub command_history_limit: usize,
    pub safe_launch_policy: String,
    pub provider_health_mode: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliSettingsConstraints {
    pub terminal_surfaces: Vec<String>,
    pub command_history_limit_min: usize,
    pub command_history_limit_max: usize,
    pub provider_health_modes: Vec<String>,
    pub safe_launch_policies: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliLauncher {
    pub cwd: String,
    pub powershell_script: String,
    pub release_binary: String,
    pub release_binary_exists: bool,
    pub preferred_invocation: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliHealth {
    pub score: u8,
    pub summary: String,
    pub prepared_engines_ready: usize,
    pub prepared_engines_total: usize,
    pub sample_artifacts_ready: usize,
    pub sample_artifacts_total: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliActionGroup {
    pub id: String,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliAction {
    pub id: String,
    pub group: String,
    pub label: String,
    pub description: String,
    pub argv: Vec<String>,
    pub cwd: String,
    pub output: String,
    pub requires_terminal: bool,
    pub accepts_input: bool,
    pub input_placeholder: Option<String>,
    pub enabled: bool,
    pub required_tools: Vec<String>,
    pub required_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliHostStatus {
    pub root: String,
    pub contract_path: String,
    pub contract_exists: bool,
    pub contract_parse_ok: bool,
    pub group_count: usize,
    pub action_count: usize,
    pub terminal_action_count: usize,
    pub json_action_count: usize,
    pub quick_terminal_actions: Vec<DxCliQuickAction>,
    pub quick_captured_actions: Vec<DxCliQuickAction>,
    pub health_score: Option<u8>,
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliQuickAction {
    pub action_id: String,
    pub label: String,
    pub group: String,
    pub accepts_input: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliHostMenu {
    pub workspace: String,
    pub group_count: usize,
    pub action_count: usize,
    pub enabled_action_count: usize,
    pub groups: Vec<DxCliMenuGroup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliMenuGroup {
    pub id: String,
    pub label: String,
    pub description: String,
    pub action_count: usize,
    pub enabled_action_count: usize,
    pub terminal_action_count: usize,
    pub json_action_count: usize,
    pub input_action_count: usize,
    pub actions: Vec<DxCliMenuAction>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliMenuAction {
    pub action_id: String,
    pub label: String,
    pub description: String,
    pub output: String,
    pub requires_terminal: bool,
    pub accepts_input: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliActionPlan {
    pub action_id: String,
    pub label: String,
    pub argv: Vec<String>,
    pub cwd: String,
    pub output: String,
    pub requires_terminal: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliLaunchResult {
    pub action_id: String,
    pub label: String,
    pub pid: u32,
    pub argv: Vec<String>,
    pub cwd: String,
    pub launched_in_terminal: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliRunResult {
    pub action_id: String,
    pub label: String,
    pub output: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub argv: Vec<String>,
    pub cwd: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliCommandHistory {
    pub history_path: String,
    pub count: usize,
    pub entries: Vec<DxCliCommandRunRecord>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxCliCommandRunRecord {
    pub schema_version: String,
    pub recorded_at_ms: u64,
    pub action_id: String,
    pub label: String,
    pub surface: String,
    pub status: String,
    pub output: String,
    pub duration_ms: u64,
    pub exit_code: Option<i32>,
    pub pid: Option<u32>,
    pub launched_in_terminal: Option<bool>,
    pub input_supplied: bool,
    pub argv_program: Option<String>,
    pub argv_arg_count: usize,
    pub cwd: String,
    pub error_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsDashboardCommand {
    pub label: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub json: Option<serde_json::Value>,
    pub json_error: Option<String>,
    pub recovery_hint: Option<String>,
    pub argv: Vec<String>,
    pub cwd: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxAgentsProviderSmokeHistory {
    pub history_path: String,
    pub count: usize,
    pub entries: Vec<DxAgentsProviderSmokeRecord>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxAgentsProviderSmokeRecord {
    pub schema_version: String,
    pub recorded_at_ms: u64,
    pub provider: String,
    pub model: Option<String>,
    pub status: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub stdout_summary: String,
    pub stderr_summary: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DxAgentsToolSafetyDrillHistory {
    pub history_path: String,
    pub count: usize,
    pub retention_limit: usize,
    pub latest_blocked_recovery_hint: Option<String>,
    pub trend: DxAgentsToolSafetyDrillTrend,
    pub alerts: Vec<DxAgentsToolSafetyDrillAlert>,
    pub entries: Vec<DxAgentsToolSafetyDrillRecord>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyDrillRecord {
    pub schema_version: String,
    pub recorded_at_ms: u64,
    pub source: String,
    pub mode: String,
    pub status: String,
    pub ready: bool,
    pub command_success: bool,
    pub exit_code: Option<i32>,
    pub allowed_count: usize,
    pub approval_required_count: usize,
    pub denied_count: usize,
    pub missing_count: usize,
    pub critical_blocker_count: usize,
    pub autonomy_level: Option<String>,
    pub auto_approve_count: usize,
    pub always_ask_count: usize,
    pub tool_filter_group_count: usize,
    pub mcp_server_count: usize,
    pub redaction_ok: bool,
    pub recovery_hint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyDrillTrend {
    pub state: String,
    pub snapshot_count: usize,
    pub latest_recorded_at_ms: Option<u64>,
    pub previous_recorded_at_ms: Option<u64>,
    pub allowed_delta: i64,
    pub approval_required_delta: i64,
    pub denied_delta: i64,
    pub missing_delta: i64,
    pub critical_blocker_delta: i64,
    pub recovery_hint: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyDrillAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsToolSafetyDrillHistoryExport {
    pub exported_at_ms: u64,
    pub export_path: String,
    pub history: DxAgentsToolSafetyDrillHistory,
    pub redacted: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditSummary {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub ready: bool,
    pub state: String,
    pub latest_drill_status: Option<String>,
    pub latest_drill_mode: Option<String>,
    pub history_snapshot_count: usize,
    pub history_trend: String,
    pub alert_count: usize,
    pub blocked_alert_count: usize,
    pub warning_alert_count: usize,
    pub ok_alert_count: usize,
    pub info_alert_count: usize,
    pub alert_ids: Vec<String>,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub latest_recovery_hint_available: bool,
    pub next_remediation_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditHistory {
    pub history_path: String,
    pub count: usize,
    pub retention_limit: usize,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub digest: DxAgentsToolSafetyAuditDigest,
    pub alerts: Vec<DxAgentsToolSafetyAuditReviewAlert>,
    pub escalation_evidence: DxAgentsToolSafetyAuditEscalationEvidence,
    pub recovery_drill: DxAgentsToolSafetyAuditRecoveryDrill,
    pub recovery_digest: DxAgentsToolSafetyAuditRecoveryDigest,
    pub recovery_alerts: Vec<DxAgentsToolSafetyAuditRecoveryAlert>,
    pub recovery_alert_digest: DxAgentsToolSafetyAuditRecoveryAlertDigest,
    pub recovery_alert_digest_release_gate: DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGate,
    pub recovery_alert_digest_release_gate_digest:
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest,
    pub recovery_alert_digest_release_gate_digest_alerts:
        Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert>,
    pub recovery_alert_digest_release_gate_digest_alert_digest:
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alerts:
        Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert>,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest:
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts:
        Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert>,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest:
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigest,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts:
        Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert>,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest:
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigest,
    pub recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts:
        Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert>,
    pub trend: DxAgentsToolSafetyAuditTrend,
    pub latest_remediation_action: Option<String>,
    pub entries: Vec<DxAgentsToolSafetyAuditSummary>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditReviewAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub ok_count: usize,
    pub top_alert_id: Option<String>,
    pub top_alert_title: Option<String>,
    pub top_alert_level: Option<String>,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub digest_runbook_present: bool,
    pub digest_runbook_target: String,
    pub review_required: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGate {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub digest_state: String,
    pub digest_ready: bool,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub digest_runbook_present: bool,
    pub digest_runbook_target: String,
    pub release_gate_runbook_present: bool,
    pub release_gate_runbook_target: String,
    pub review_required: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub gate_state: String,
    pub gate_ready: bool,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub runbook_count: usize,
    pub runbook_present_count: usize,
    pub missing_runbook_count: usize,
    pub runbook_present: bool,
    pub digest_runbook_present: bool,
    pub release_gate_runbook_present: bool,
    pub release_gate_digest_runbook_present: bool,
    pub release_gate_digest_runbook_target: String,
    pub all_runbooks_present: bool,
    pub review_required: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub gate_state: String,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub ok_count: usize,
    pub info_count: usize,
    pub top_alert_id: Option<String>,
    pub top_alert_level: Option<String>,
    pub digest_runbook_present: bool,
    pub digest_runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub alert_digest_runbook_present: bool,
    pub alert_digest_runbook_target: String,
    pub runbook_count: usize,
    pub runbook_present_count: usize,
    pub missing_runbook_count: usize,
    pub all_runbooks_present: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub digest_state: String,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub ok_count: usize,
    pub info_count: usize,
    pub top_alert_id: Option<String>,
    pub top_alert_level: Option<String>,
    pub alert_digest_runbook_present: bool,
    pub alert_digest_runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub alert_digest_alert_digest_runbook_present: bool,
    pub alert_digest_alert_digest_runbook_target: String,
    pub runbook_count: usize,
    pub runbook_present_count: usize,
    pub missing_runbook_count: usize,
    pub all_runbooks_present: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert {
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub digest_state: String,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigest
{
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub ok_count: usize,
    pub info_count: usize,
    pub top_alert_id: Option<String>,
    pub top_alert_level: Option<String>,
    pub alert_digest_alert_digest_runbook_present: bool,
    pub alert_digest_alert_digest_runbook_target: String,
    pub alert_digest_alert_digest_alert_runbook_present: bool,
    pub alert_digest_alert_digest_alert_runbook_target: String,
    pub alert_digest_alert_digest_alert_digest_runbook_present: bool,
    pub alert_digest_alert_digest_alert_digest_runbook_target: String,
    pub runbook_count: usize,
    pub runbook_present_count: usize,
    pub missing_runbook_count: usize,
    pub all_runbooks_present: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert
{
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub digest_state: String,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigest
{
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub release_blocking: bool,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub ok_count: usize,
    pub info_count: usize,
    pub top_alert_id: Option<String>,
    pub top_alert_level: Option<String>,
    pub alert_digest_alert_digest_alert_digest_alert_digest_runbook_present: bool,
    pub alert_digest_alert_digest_alert_digest_alert_digest_runbook_target: String,
    pub alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present: bool,
    pub alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_target: String,
    pub alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present: bool,
    pub alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target: String,
    pub runbook_count: usize,
    pub runbook_present_count: usize,
    pub missing_runbook_count: usize,
    pub all_runbooks_present: bool,
    pub safe_to_share: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert
{
    pub id: String,
    pub level: String,
    pub title: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
    pub digest_state: String,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditEscalationEvidence {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub severity: String,
    pub alert_count: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    pub ok_count: usize,
    pub blocked_alert_ids: Vec<String>,
    pub warning_alert_ids: Vec<String>,
    pub top_alert_id: Option<String>,
    pub top_alert_title: Option<String>,
    pub recovery_hint: Option<String>,
    pub next_action: String,
    pub runbook_target: String,
    pub alert_runbook_target: String,
    pub review_required: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryDrill {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub outcome: String,
    pub escalation_state: String,
    pub severity: String,
    pub blocked_alert_ids: Vec<String>,
    pub warning_alert_ids: Vec<String>,
    pub planned_steps: Vec<String>,
    pub cleared: bool,
    pub review_required: bool,
    pub dry_run_only: bool,
    pub invokes_tools: bool,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditRecoveryDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub outcome: String,
    pub recovery_state: String,
    pub escalation_state: String,
    pub severity: String,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub planned_step_count: usize,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub cleared: bool,
    pub review_required: bool,
    pub dry_run_only: bool,
    pub invokes_tools: bool,
    pub metadata_only: bool,
    pub redacted: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub next_action: String,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditDigest {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub state: String,
    pub ready: bool,
    pub review_required: bool,
    pub audit_count: usize,
    pub latest_audit_state: Option<String>,
    pub latest_ready: Option<bool>,
    pub trend_state: String,
    pub ready_delta: i64,
    pub alert_delta: i64,
    pub blocked_alert_delta: i64,
    pub warning_alert_delta: i64,
    pub runbook_present: bool,
    pub runbook_target: String,
    pub alert_runbook_present: bool,
    pub alert_runbook_target: String,
    pub latest_remediation_action: Option<String>,
    pub next_action: String,
    pub redacted: bool,
    pub metadata_only: bool,
    pub stores_config_values: bool,
    pub duplicates_history_rows: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct DxAgentsToolSafetyAuditTrend {
    pub state: String,
    pub snapshot_count: usize,
    pub latest_generated_at_ms: Option<u64>,
    pub previous_generated_at_ms: Option<u64>,
    pub ready_delta: i64,
    pub alert_delta: i64,
    pub blocked_alert_delta: i64,
    pub warning_alert_delta: i64,
    pub recovery_hint: Option<String>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsToolSafetyAuditHistoryExport {
    pub exported_at_ms: u64,
    pub export_path: String,
    pub history: DxAgentsToolSafetyAuditHistory,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsDashboardCompatibilityStatus {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub ready: bool,
    pub source: String,
    pub contract_schema_version: Option<String>,
    pub product_name: Option<String>,
    pub package_name: Option<String>,
    pub alias_category_count: usize,
    pub alias_count: usize,
    pub env_alias_count: usize,
    pub window_global_count: usize,
    pub storage_alias_count: usize,
    pub event_alias_count: usize,
    pub websocket_protocol_count: usize,
    pub legacy_readable: bool,
    pub legacy_writable: bool,
    pub exposes_stored_values: bool,
    pub cleanup_gate_ready: bool,
    pub decommission_ready: bool,
    pub migration_plan_path: String,
    pub usage_telemetry: DxAgentsDashboardCompatibilityUsageStatus,
    pub drift_checks: Vec<DxAgentsDashboardCompatibilityDrift>,
    pub next_action: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsDashboardCompatibilityUsageStatus {
    pub schema_version: String,
    pub supported: bool,
    pub state: String,
    pub telemetry_path: String,
    pub browser_storage_key: String,
    pub primary_usage_count: u64,
    pub legacy_usage_count: u64,
    pub legacy_read_count: u64,
    pub legacy_write_count: u64,
    pub legacy_remove_count: u64,
    pub migration_count: u64,
    pub decommission_ready: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsDashboardCompatibilityDrift {
    pub id: String,
    pub label: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsBridgeStatusExport {
    pub exported_at_ms: u64,
    pub export_path: String,
    pub self_test: DxCliBridgeSelfTest,
    pub command_history: DxCliCommandHistory,
    pub provider_health: DxAgentsDashboardCommand,
    pub provider_failover_drill: DxAgentsDashboardCommand,
    pub gateway_pairing_drill: DxAgentsDashboardCommand,
    pub provider_smoke_history: DxAgentsProviderSmokeHistory,
    pub compact_status: DxAgentsDashboardCommand,
    pub cron_preview: DxAgentsDashboardCommand,
    pub cron_history: DxAgentsDashboardCommand,
    pub cron_delivery_drill: DxAgentsDashboardCommand,
    pub tool_safety_drill: DxAgentsDashboardCommand,
    pub tool_safety_history: DxAgentsToolSafetyDrillHistory,
    pub tool_safety_audit: DxAgentsToolSafetyAuditSummary,
    pub tool_safety_audit_digest: DxAgentsToolSafetyAuditDigest,
    pub tool_safety_audit_history: DxAgentsToolSafetyAuditHistory,
    pub continuation_status: DxAgentsDashboardCommand,
    pub session_tool_routing: DxAgentsDashboardCommand,
    pub memory_skill_learning: DxAgentsDashboardCommand,
    pub native_promotion_status: serde_json::Value,
    pub native_promotion_archive_diff: DxCliNativePromotionArchiveDiffSummary,
    pub dashboard_compatibility: DxAgentsDashboardCompatibilityStatus,
    pub media_canary_evidence: DxAgentsEmbeddedTerminalMediaCanaryEvidence,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsBridgeStatusExportList {
    pub export_dir: String,
    pub count: usize,
    pub entries: Vec<DxAgentsBridgeStatusExportEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsBridgeStatusExportEntry {
    pub file_name: String,
    pub path: String,
    pub modified_at_ms: u64,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveList {
    pub archive_dir: String,
    pub count: usize,
    pub entries: Vec<DxCliNativePromotionArchiveEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveEntry {
    pub file_name: String,
    pub path: String,
    pub modified_at_ms: u64,
    pub size_bytes: u64,
    pub archived_at_ms: u64,
    pub retained_count: usize,
    pub retention_limit: usize,
    pub blocker_count: usize,
    pub surface_count: usize,
    pub diagnostic_only: bool,
    pub production_ready: bool,
    pub redacted: bool,
    pub rollback_summary: String,
    pub next_surface: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveDiffSummary {
    pub archive_dir: String,
    pub available: bool,
    pub snapshot_count: usize,
    pub latest_file_name: Option<String>,
    pub previous_file_name: Option<String>,
    pub blocker_delta: i64,
    pub retention_delta: i64,
    pub production_ready_changed: bool,
    pub next_surface_changed: bool,
    pub rollback_changed: bool,
    pub redacted: bool,
    pub diagnostic_only: bool,
    pub alert_level: String,
    pub summary: String,
    pub recovery_hint: Option<String>,
    pub changes: Vec<DxCliNativePromotionArchiveDiffChange>,
    pub alerts: Vec<DxCliNativePromotionArchiveDriftAlert>,
    pub trend_history: DxCliNativePromotionArchiveTrendHistory,
    pub runbook: DxCliNativePromotionArchiveTrendRunbook,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveDiffChange {
    pub id: String,
    pub label: String,
    pub before: String,
    pub after: String,
    pub delta: Option<i64>,
    pub changed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveDriftAlert {
    pub id: String,
    pub severity: String,
    pub label: String,
    pub detail: String,
    pub recovery_hint: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveTrendHistory {
    pub available: bool,
    pub trend: String,
    pub sample_count: usize,
    pub latest_file_name: Option<String>,
    pub oldest_file_name: Option<String>,
    pub latest_blocker_count: Option<usize>,
    pub oldest_blocker_count: Option<usize>,
    pub blocker_delta_total: i64,
    pub warning_alert_count: usize,
    pub blocked_alert_count: usize,
    pub redacted: bool,
    pub diagnostic_only: bool,
    pub summary: String,
    pub recovery_hint: Option<String>,
    pub points: Vec<DxCliNativePromotionArchiveTrendPoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveTrendRunbook {
    pub path: String,
    pub title: String,
    pub diagnostic_only: bool,
    pub external_fallbacks: Vec<String>,
    pub safety_summary: String,
    pub guidance: Vec<DxCliNativePromotionArchiveTrendRunbookGuidance>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveTrendRunbookGuidance {
    pub state: String,
    pub severity: String,
    pub meaning: String,
    pub operator_action: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliNativePromotionArchiveTrendPoint {
    pub file_name: String,
    pub blocker_count: usize,
    pub alert_level: String,
    pub production_ready: bool,
    pub redacted: bool,
    pub diagnostic_only: bool,
    pub next_surface: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsReleaseReadiness {
    pub generated_at_ms: u64,
    pub score: u8,
    pub ready: bool,
    pub items: Vec<DxAgentsReleaseReadinessItem>,
    pub next_action: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsReleaseReadinessItem {
    pub id: String,
    pub label: String,
    pub status: String,
    pub detail: String,
    pub recovery_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalFixtures {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub redaction_policy: String,
    pub input_events: Vec<DxAgentsEmbeddedTerminalInputFixture>,
    pub resize_events: Vec<DxAgentsEmbeddedTerminalResizeFixture>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalInputFixture {
    pub event_id: String,
    pub kind: String,
    pub timestamp_ms: u64,
    pub modifier_count: u8,
    pub payload_bytes: usize,
    pub redacted: bool,
    pub stores_payload: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalResizeFixture {
    pub event_id: String,
    pub kind: String,
    pub columns: u16,
    pub rows: u16,
    pub debounced: bool,
    pub renderer_reflow_required: bool,
    pub pty_resize_required: bool,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalReadiness {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub contract_path: String,
    pub ready: bool,
    pub embedded_pty_production_ready: bool,
    pub input_contract_present: bool,
    pub resize_contract_present: bool,
    pub media_session_contract_present: bool,
    pub evidence: Vec<DxAgentsEmbeddedTerminalReadinessEvidence>,
    pub fixtures: DxAgentsEmbeddedTerminalFixtures,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalReadinessEvidence {
    pub id: String,
    pub label: String,
    pub status: String,
    pub detail: String,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalSessionTimeline {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub phase: String,
    pub contract_path: String,
    pub process_spawned: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub external_terminal_fallback: bool,
    pub readiness_ready: bool,
    pub echo_process_pilot_ready: bool,
    pub readiness_evidence: Vec<DxAgentsEmbeddedTerminalReadinessEvidence>,
    pub events: Vec<DxAgentsEmbeddedTerminalSessionEvent>,
    pub next_phase: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalSessionEvent {
    pub event_id: String,
    pub step: u8,
    pub kind: String,
    pub status: String,
    pub source: String,
    pub detail: String,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalMediaCanaryEvidence {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub status: String,
    pub process_spawned: bool,
    pub media_session_contract_present: bool,
    pub media_session_enabled: bool,
    pub external_terminal_fallback: bool,
    pub production_routing_enabled: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub sample_count: usize,
    pub samples: Vec<DxAgentsEmbeddedTerminalMediaCanarySample>,
    pub fallback_actions: Vec<String>,
    pub operator_export_ready: bool,
    pub rollback_state: String,
    pub next_phase: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalMediaCanarySample {
    pub step: u8,
    pub sample_id: String,
    pub media_kind: String,
    pub phase: String,
    pub status: String,
    pub source: String,
    pub fallback_action: String,
    pub max_frame_rate: Option<u16>,
    pub max_pending_frames: Option<u16>,
    pub max_audio_buffer_ms: Option<u16>,
    pub dimensions: Option<String>,
    pub redacted: bool,
    pub stores_payload: bool,
    pub process_spawned: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalEchoPilot {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub status: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub process_spawned: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub stdout_preview: String,
    pub stderr_preview: String,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub json: Option<serde_json::Value>,
    pub argv: Vec<String>,
    pub cwd: String,
    pub timeline_phase: String,
    pub timeline_next_phase: String,
    pub skipped_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryGate {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub enabled: bool,
    pub default_enabled: bool,
    pub developer_only: bool,
    pub env_var: String,
    pub env_value_present: bool,
    pub mode: String,
    pub production_terminal_surface: String,
    pub preferred_terminal_surface: String,
    pub normal_terminal_actions_unchanged: bool,
    pub normal_terminal_action_count: usize,
    pub registers_host_action: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub guardrails: Vec<String>,
    pub next_phase: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryLifecycle {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub status: String,
    pub gate_enabled: bool,
    pub process_spawned: bool,
    pub process_kind: String,
    pub max_duration_ms: u64,
    pub stdout_limit_bytes: usize,
    pub stderr_limit_bytes: usize,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub external_terminal_fallback: bool,
    pub events: Vec<DxAgentsEmbeddedTerminalTuiCanaryLifecycleEvent>,
    pub rollback_triggers: Vec<String>,
    pub next_phase: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryRunner {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub status: String,
    pub success: bool,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub process_spawned: bool,
    pub gate_enabled: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub stdout_preview: String,
    pub stderr_preview: String,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub stdout_limit_bytes: usize,
    pub stderr_limit_bytes: usize,
    pub max_duration_ms: u64,
    pub json: Option<serde_json::Value>,
    pub argv: Vec<String>,
    pub cwd: String,
    pub lifecycle_status: String,
    pub lifecycle_next_phase: String,
    pub contract_present: bool,
    pub contract_accepted: bool,
    pub contract_source: String,
    pub contract_diagnostics: Vec<String>,
    pub contract_fixed_command: Vec<String>,
    pub contract_result_states: Vec<String>,
    pub skip_reasons: Vec<String>,
    pub skipped_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryRendererEvidence {
    pub schema_version: String,
    pub generated_at_ms: u64,
    pub status: String,
    pub gate_enabled: bool,
    pub process_spawned: bool,
    pub source_contract_present: bool,
    pub source_contract_accepted: bool,
    pub source_contract_source: String,
    pub source_contract_diagnostics: Vec<String>,
    pub renderer_contract_present: bool,
    pub renderer_enabled: bool,
    pub external_terminal_fallback: bool,
    pub production_routing_enabled: bool,
    pub allows_arbitrary_shell: bool,
    pub stores_payloads: bool,
    pub snapshot_count: usize,
    pub snapshots: Vec<DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot>,
    pub rollback_state: String,
    pub drift_checks: Vec<String>,
    pub next_phase: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot {
    pub step: u8,
    pub snapshot_id: String,
    pub phase: String,
    pub status: String,
    pub source: String,
    pub terminal_state: String,
    pub columns: u16,
    pub rows: u16,
    pub cursor_row: u16,
    pub cursor_column: u16,
    pub scrollback_lines: u16,
    pub alternate_screen: bool,
    pub cursor_visible: bool,
    pub renderer_reflow_required: bool,
    pub process_spawned: bool,
    pub redacted: bool,
    pub stores_payload: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
struct TuiCanaryRunnerContract {
    args: Vec<String>,
    fixed_command: Vec<String>,
    stdout_limit_bytes: usize,
    stderr_limit_bytes: usize,
    max_duration_ms: u64,
    present: bool,
    accepted: bool,
    source: String,
    diagnostics: Vec<String>,
    result_states: Vec<String>,
    skip_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsEmbeddedTerminalTuiCanaryLifecycleEvent {
    pub step: u8,
    pub event_id: String,
    pub phase: String,
    pub status: String,
    pub timeout_ms: u64,
    pub cleanup_required: bool,
    pub detail: String,
    pub redacted: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliBridgeSelfTest {
    pub ok: bool,
    pub command_count: usize,
    pub commands: Vec<DxCliBridgeSelfTestCommand>,
    pub diagnostics: Vec<DxCliBridgeSelfTestDiagnostic>,
    pub repo_dir: String,
    pub cli_program: String,
    pub contract_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliBridgeSelfTestCommand {
    pub name: String,
    pub available: bool,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxCliBridgeSelfTestDiagnostic {
    pub id: String,
    pub status: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DxAgentsOpenPathResult {
    pub target: String,
    pub path: String,
    pub opened: bool,
}

#[tauri::command]
pub fn get_dx_cli_host_contract() -> Result<serde_json::Value, String> {
    load_contract_value()
}

#[tauri::command]
pub fn get_dx_cli_host_status() -> DxCliHostStatus {
    let root = dx_cli_root();
    let contract_path = contract_path(&root);
    let contract = load_contract_from_path(&contract_path).ok();

    DxCliHostStatus {
        root: root.display().to_string(),
        contract_path: contract_path.display().to_string(),
        contract_exists: contract_path.is_file(),
        contract_parse_ok: contract.is_some(),
        group_count: contract
            .as_ref()
            .map(|contract| host_menu(contract).groups.len())
            .unwrap_or_default(),
        action_count: contract
            .as_ref()
            .map(|contract| contract.actions.len())
            .unwrap_or_default(),
        terminal_action_count: count_actions(contract.as_ref(), |action| action.requires_terminal),
        json_action_count: count_actions(contract.as_ref(), |action| action.output == "json"),
        quick_terminal_actions: contract
            .as_ref()
            .map(quick_terminal_actions)
            .unwrap_or_default(),
        quick_captured_actions: contract
            .as_ref()
            .map(quick_captured_actions)
            .unwrap_or_default(),
        health_score: contract.as_ref().map(|contract| contract.health.score),
        ready: contract
            .as_ref()
            .map(|contract| contract.health.score >= 98)
            .unwrap_or(false),
    }
}

#[tauri::command]
pub fn get_dx_cli_host_menu() -> Result<DxCliHostMenu, String> {
    let contract = load_contract()?;
    Ok(host_menu(&contract))
}

#[tauri::command]
pub fn get_dx_cli_bridge_settings<R: Runtime>(
    app: AppHandle<R>,
) -> Result<DxCliBridgeSettings, String> {
    let contract = load_contract()?;
    let defaults = default_bridge_settings_for_contract(&contract);
    let store = app
        .store(BRIDGE_SETTINGS_STORE)
        .map_err(|e| e.to_string())?;
    let saved = store
        .get(BRIDGE_SETTINGS_KEY)
        .and_then(|value| serde_json::from_value::<DxCliBridgeSettings>(value.clone()).ok());

    Ok(normalize_bridge_settings(
        &contract,
        saved.unwrap_or(defaults),
    ))
}

#[tauri::command]
pub fn save_dx_cli_bridge_settings<R: Runtime>(
    app: AppHandle<R>,
    settings: DxCliBridgeSettings,
) -> Result<DxCliBridgeSettings, String> {
    let contract = load_contract()?;
    let settings = normalize_bridge_settings(&contract, settings);
    let store = app
        .store(BRIDGE_SETTINGS_STORE)
        .map_err(|e| e.to_string())?;
    store.set(
        BRIDGE_SETTINGS_KEY,
        serde_json::to_value(&settings).map_err(|error| error.to_string())?,
    );
    store.save().map_err(|e| e.to_string())?;
    Ok(settings)
}

#[tauri::command]
pub fn get_dx_cli_command_history(limit: Option<usize>) -> Result<DxCliCommandHistory, String> {
    let contract = load_contract()?;
    read_command_history(&contract, limit.unwrap_or(8).clamp(1, 50))
}

#[tauri::command]
pub fn plan_dx_cli_action(
    action_id: String,
    input: Option<String>,
) -> Result<DxCliActionPlan, String> {
    let contract = load_contract()?;
    let action = find_action(&contract, &action_id)?;
    let argv = argv_for_action(action, input.as_deref())?;

    Ok(DxCliActionPlan {
        action_id: action.id.clone(),
        label: action.label.clone(),
        argv,
        cwd: action.cwd.clone(),
        output: action.output.clone(),
        requires_terminal: action.requires_terminal,
    })
}

#[tauri::command]
pub async fn run_dx_cli_json_action(
    action_id: String,
    input: Option<String>,
) -> Result<serde_json::Value, String> {
    tauri::async_runtime::spawn_blocking(move || run_json_action(action_id, input))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn run_dx_cli_captured_action(
    action_id: String,
    input: Option<String>,
) -> Result<DxCliRunResult, String> {
    tauri::async_runtime::spawn_blocking(move || run_captured_action(action_id, input))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn launch_dx_cli_terminal_action(
    action_id: String,
    input: Option<String>,
) -> Result<DxCliLaunchResult, String> {
    tauri::async_runtime::spawn_blocking(move || launch_terminal_action(action_id, input))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_provider_health(
    mode: Option<String>,
    provider: Option<String>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let args = dx_agents_provider_health_args(mode.as_deref(), provider.as_deref());
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Provider health", &arg_refs)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn run_dx_agents_provider_smoke(
    provider: Option<String>,
    model: Option<String>,
    timeout_secs: Option<u64>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let provider_name = provider
            .as_deref()
            .and_then(trimmed_non_empty)
            .unwrap_or("default")
            .to_string();
        let model_name = model
            .as_deref()
            .and_then(trimmed_non_empty)
            .map(str::to_string);
        let args =
            dx_agents_provider_smoke_args(provider.as_deref(), model.as_deref(), timeout_secs);
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        let started = Instant::now();
        let command = run_dx_agents_dashboard_command("Provider live smoke", &arg_refs)?;
        let command = redact_dashboard_command(command);
        let record = provider_smoke_record(
            &provider_name,
            model_name,
            &command,
            started.elapsed().as_millis() as u64,
        );
        let _ = append_provider_smoke_record(&provider_smoke_history_path(), &record);
        Ok(command)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn get_dx_agents_provider_smoke_history(
    limit: Option<usize>,
) -> Result<DxAgentsProviderSmokeHistory, String> {
    read_provider_smoke_history(
        &provider_smoke_history_path(),
        limit.unwrap_or(6).clamp(1, 50),
    )
}

#[tauri::command]
pub async fn get_dx_agents_gateway_paircode() -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let args = dx_agents_gateway_paircode_args();
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Gateway pairing code", &arg_refs)
            .map(redact_dashboard_command)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_gateway_pairing_drill(
    mode: Option<String>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let args = dx_agents_gateway_pairing_drill_args(mode.as_deref());
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Gateway pairing allowlist drill", &arg_refs)
            .map(redact_dashboard_command)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_continuation_status(
    limit: Option<usize>,
) -> Result<DxAgentsDashboardCommand, String> {
    let limit = limit.unwrap_or(5).clamp(1, 20).to_string();
    tauri::async_runtime::spawn_blocking(move || {
        run_dx_agents_dashboard_command(
            "Continuation journal",
            &["workloop", "status", "--limit", &limit, "--json"],
        )
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_compact_status() -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let args = dx_agents_compact_status_args();
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Compact status", &arg_refs)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_cron_preview(
    limit: Option<usize>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let args = dx_agents_cron_preview_args(limit);
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Cron delivery preview", &arg_refs)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_cron_history(
    limit: Option<usize>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let args = dx_agents_cron_history_args(limit);
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Cron run history", &arg_refs)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_cron_delivery_drill(
    mode: Option<String>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let args = dx_agents_cron_delivery_drill_args(mode.as_deref());
        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        run_dx_agents_dashboard_command("Cron delivery recovery drill", &arg_refs)
            .map(redact_dashboard_command)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn get_dx_agents_tool_safety_drill(
    mode: Option<String>,
) -> Result<DxAgentsDashboardCommand, String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_tool_safety_drill_dashboard_command(
            "Tool configuration safety drill",
            mode.as_deref(),
            "bridge_command",
        )
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn get_dx_agents_tool_safety_drill_history(
    limit: Option<usize>,
) -> Result<DxAgentsToolSafetyDrillHistory, String> {
    read_tool_safety_drill_history(
        &tool_safety_drill_history_path(),
        limit.unwrap_or(8).clamp(1, TOOL_SAFETY_DRILL_HISTORY_LIMIT),
    )
}

#[tauri::command]
pub fn get_dx_agents_tool_safety_audit() -> Result<DxAgentsToolSafetyAuditSummary, String> {
    let history = read_tool_safety_drill_history(
        &tool_safety_drill_history_path(),
        TOOL_SAFETY_DRILL_HISTORY_LIMIT,
    )?;
    let audit = tool_safety_audit_summary(
        &history,
        dx_agents_repo_dir()
            .join(TOOL_SAFETY_ALERT_RUNBOOK)
            .is_file(),
    );
    let _ = append_tool_safety_audit_record(
        &tool_safety_audit_history_path(),
        &audit,
        TOOL_SAFETY_AUDIT_HISTORY_LIMIT,
    );
    Ok(audit)
}

#[tauri::command]
pub fn get_dx_agents_tool_safety_audit_history(
    limit: Option<usize>,
) -> Result<DxAgentsToolSafetyAuditHistory, String> {
    read_tool_safety_audit_history(
        &tool_safety_audit_history_path(),
        limit.unwrap_or(8).clamp(1, TOOL_SAFETY_AUDIT_HISTORY_LIMIT),
    )
}

#[tauri::command]
pub async fn export_dx_agents_tool_safety_audit_history()
-> Result<DxAgentsToolSafetyAuditHistoryExport, String> {
    tauri::async_runtime::spawn_blocking(export_tool_safety_audit_history)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn open_dx_agents_tool_safety_audit_history() -> Result<DxAgentsOpenPathResult, String> {
    tauri::async_runtime::spawn_blocking(open_tool_safety_audit_history)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn export_dx_agents_tool_safety_drill_history()
-> Result<DxAgentsToolSafetyDrillHistoryExport, String> {
    tauri::async_runtime::spawn_blocking(export_tool_safety_drill_history)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn open_dx_agents_tool_safety_drill_history() -> Result<DxAgentsOpenPathResult, String> {
    tauri::async_runtime::spawn_blocking(open_tool_safety_drill_history)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn get_dx_agents_release_readiness() -> DxAgentsReleaseReadiness {
    release_readiness_report()
}

#[tauri::command]
pub fn get_dx_agents_dashboard_compatibility_status()
-> Result<DxAgentsDashboardCompatibilityStatus, String> {
    let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
    Ok(dashboard_compatibility_status(
        &value,
        &dx_agents_repo_dir(),
    ))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_fixtures() -> DxAgentsEmbeddedTerminalFixtures {
    embedded_terminal_fixtures()
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_readiness()
-> Result<DxAgentsEmbeddedTerminalReadiness, String> {
    let path = contract_path(&dx_cli_root());
    let value = load_contract_value_from_path(&path)?;
    Ok(embedded_terminal_readiness_export(&path, &value))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_session_timeline()
-> Result<DxAgentsEmbeddedTerminalSessionTimeline, String> {
    let path = contract_path(&dx_cli_root());
    let value = load_contract_value_from_path(&path)?;
    Ok(embedded_terminal_session_timeline(&path, &value))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_media_canary_evidence()
-> Result<DxAgentsEmbeddedTerminalMediaCanaryEvidence, String> {
    let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
    Ok(embedded_terminal_media_canary_evidence(&value))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_tui_canary_gate()
-> Result<DxAgentsEmbeddedTerminalTuiCanaryGate, String> {
    let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
    let env_value = env::var(TUI_CANARY_ENV).ok();
    Ok(embedded_terminal_tui_canary_gate(
        &value,
        env_value.as_deref(),
    ))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_tui_canary_lifecycle()
-> Result<DxAgentsEmbeddedTerminalTuiCanaryLifecycle, String> {
    let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
    let env_value = env::var(TUI_CANARY_ENV).ok();
    Ok(embedded_terminal_tui_canary_lifecycle(
        &value,
        env_value.as_deref(),
    ))
}

#[tauri::command]
pub fn get_dx_agents_embedded_terminal_tui_canary_renderer_evidence()
-> Result<DxAgentsEmbeddedTerminalTuiCanaryRendererEvidence, String> {
    let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
    let env_value = env::var(TUI_CANARY_ENV).ok();
    Ok(embedded_terminal_tui_canary_renderer_evidence(
        &value,
        env_value.as_deref(),
    ))
}

#[tauri::command]
pub async fn run_dx_agents_embedded_terminal_echo_pilot()
-> Result<DxAgentsEmbeddedTerminalEchoPilot, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let path = contract_path(&dx_cli_root());
        let value = load_contract_value_from_path(&path)?;
        run_embedded_terminal_echo_pilot(&path, &value)
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn run_dx_agents_embedded_terminal_tui_canary_runner()
-> Result<DxAgentsEmbeddedTerminalTuiCanaryRunner, String> {
    tauri::async_runtime::spawn_blocking(|| {
        let value = load_contract_value_from_path(&contract_path(&dx_cli_root()))?;
        let env_value = env::var(TUI_CANARY_ENV).ok();
        run_embedded_terminal_tui_canary_runner(&value, env_value.as_deref())
    })
    .await
    .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn export_dx_agents_bridge_status() -> Result<DxAgentsBridgeStatusExport, String> {
    tauri::async_runtime::spawn_blocking(export_bridge_status)
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn get_dx_agents_bridge_status_exports(
    limit: Option<usize>,
) -> Result<DxAgentsBridgeStatusExportList, String> {
    list_bridge_status_exports(limit.unwrap_or(6).clamp(1, 50))
}

#[tauri::command]
pub fn get_dx_cli_native_promotion_archives(
    limit: Option<usize>,
) -> Result<DxCliNativePromotionArchiveList, String> {
    list_native_promotion_archives(limit.unwrap_or(6).clamp(1, 50))
}

#[tauri::command]
pub fn get_dx_cli_native_promotion_archive_diff()
-> Result<DxCliNativePromotionArchiveDiffSummary, String> {
    native_promotion_archive_diff()
}

#[tauri::command]
pub async fn open_dx_agents_bridge_status_export(
    file_name: String,
) -> Result<DxAgentsOpenPathResult, String> {
    tauri::async_runtime::spawn_blocking(move || open_bridge_status_export(&file_name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub async fn open_dx_cli_native_promotion_archive(
    file_name: String,
) -> Result<DxAgentsOpenPathResult, String> {
    tauri::async_runtime::spawn_blocking(move || open_native_promotion_archive(&file_name))
        .await
        .map_err(|error| error.to_string())?
}

#[tauri::command]
pub fn run_dx_cli_bridge_self_test() -> DxCliBridgeSelfTest {
    dx_cli_bridge_self_test()
}

#[tauri::command]
pub async fn open_dx_agents_continuation_target(
    target: String,
) -> Result<DxAgentsOpenPathResult, String> {
    tauri::async_runtime::spawn_blocking(move || open_continuation_target(&target))
        .await
        .map_err(|error| error.to_string())?
}

fn open_continuation_target(target: &str) -> Result<DxAgentsOpenPathResult, String> {
    let path = continuation_target_path(target)?;
    if !path.is_file() {
        return Err(format!(
            "Continuation target `{}` does not exist at {}.",
            target,
            path.display()
        ));
    }

    open_path_with_default_app(&path)?;

    Ok(DxAgentsOpenPathResult {
        target: target.trim().to_ascii_lowercase(),
        path: path.display().to_string(),
        opened: true,
    })
}

fn continuation_target_path(target: &str) -> Result<PathBuf, String> {
    match target.trim().to_ascii_lowercase().as_str() {
        "todo" => Ok(dx_agents_repo_dir().join("TODO.md")),
        "changelog" => Ok(dx_agents_repo_dir().join("CHANGELOG.md")),
        "journal" => continuation_journal_path(),
        "native-promotion-runbook" | "native_promotion_runbook" | "trend-runbook" => {
            Ok(dx_agents_repo_dir().join(NATIVE_PROMOTION_ARCHIVE_TREND_RUNBOOK))
        }
        "tool-safety-runbook" | "tool_safety_runbook" | "tool-safety-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_ALERT_RUNBOOK))
        }
        "tool-safety-audit-runbook"
        | "tool_safety_audit_runbook"
        | "tool-safety-audit-review-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK))
        }
        "tool-safety-audit-alert-runbook"
        | "tool_safety_audit_alert_runbook"
        | "tool-safety-audit-review-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK))
        }
        "tool-safety-audit-recovery-runbook"
        | "tool_safety_audit_recovery_runbook"
        | "tool-safety-audit-review-recovery-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK))
        }
        "tool-safety-audit-recovery-alert-runbook"
        | "tool_safety_audit_recovery_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK))
        }
        "tool-safety-audit-recovery-alert-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-runbook" => {
            Ok(dx_agents_repo_dir().join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-runbook" => {
            Ok(dx_agents_repo_dir()
                .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook" => {
            Ok(dx_agents_repo_dir()
                .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
            ))
        }
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        | "tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook"
        | "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook" => {
            Ok(dx_agents_repo_dir().join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
            ))
        }
        other => Err(format!(
            "Unknown continuation target `{other}`. Expected todo, changelog, journal, native-promotion-runbook, tool-safety-runbook, tool-safety-audit-runbook, tool-safety-audit-alert-runbook, tool-safety-audit-recovery-runbook, tool-safety-audit-recovery-alert-runbook, tool-safety-audit-recovery-alert-digest-runbook, tool-safety-audit-recovery-alert-digest-release-gate-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook, tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook, or tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook."
        )),
    }
}

fn continuation_journal_path() -> Result<PathBuf, String> {
    let result = run_dx_agents_dashboard_command(
        "Continuation journal path",
        &["workloop", "journal-path"],
    )?;
    if !result.success {
        return Err(format!(
            "Failed to resolve continuation journal path: {}",
            result.stderr.trim()
        ));
    }

    let path = result.stdout.trim();
    if path.is_empty() {
        return Err("Continuation journal path command returned an empty path.".to_string());
    }

    Ok(PathBuf::from(path))
}

fn open_path_with_default_app(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut command = Command::new("powershell");
        command.args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-Command",
            "Start-Process -LiteralPath $args[0]",
        ]);
        command.arg(path);
        command
    };

    #[cfg(target_os = "macos")]
    let mut command = {
        let mut command = Command::new("open");
        command.arg(path);
        command
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = {
        let mut command = Command::new("xdg-open");
        command.arg(path);
        command
    };

    command
        .spawn()
        .map(|_| ())
        .map_err(|error| format!("Failed to open {}: {error}", path.display()))
}

fn dx_cli_bridge_self_test() -> DxCliBridgeSelfTest {
    let repo_dir = dx_agents_repo_dir();
    let cli_program = dx_agents_cli_program();
    let root = dx_cli_root();
    let contract = contract_path(&root);
    let commands = DX_CLI_IPC_COMMANDS
        .iter()
        .map(|name| DxCliBridgeSelfTestCommand {
            name: (*name).to_string(),
            available: true,
            kind: "ipc".to_string(),
        })
        .collect::<Vec<_>>();
    let mut diagnostics = Vec::new();

    let duplicates = duplicate_command_names(DX_CLI_IPC_COMMANDS);
    if duplicates.is_empty() {
        diagnostics.push(self_test_diagnostic(
            "ipc_inventory",
            "ok",
            format!("{} desktop IPC commands are declared.", commands.len()),
        ));
    } else {
        diagnostics.push(self_test_diagnostic(
            "ipc_inventory",
            "error",
            format!(
                "Duplicate desktop IPC command declarations: {}",
                duplicates.join(", ")
            ),
        ));
    }

    diagnostics.push(if repo_dir.is_dir() {
        self_test_diagnostic(
            "repo_dir",
            "ok",
            format!("DX Agents repo directory exists at {}.", repo_dir.display()),
        )
    } else {
        self_test_diagnostic(
            "repo_dir",
            "warn",
            format!(
                "DX Agents repo directory was not found at {}.",
                repo_dir.display()
            ),
        )
    });

    diagnostics.push(if cli_program.is_file() || cli_program == PathBuf::from(dx_agents_cli_file_name()) {
        self_test_diagnostic(
            "cli_program",
            "ok",
            format!("DX Agents CLI target resolved to {}.", cli_program.display()),
        )
    } else {
        self_test_diagnostic(
            "cli_program",
            "warn",
            format!(
                "DX Agents CLI target is not built yet at {}; dashboard commands can still use PATH fallback.",
                cli_program.display()
            ),
        )
    });

    diagnostics.push(if contract.is_file() {
        match load_contract_from_path(&contract) {
            Ok(contract_value) => self_test_diagnostic(
                "host_contract",
                "ok",
                format!(
                    "Host contract parsed with {} actions.",
                    contract_value.actions.len()
                ),
            ),
            Err(error) => self_test_diagnostic(
                "host_contract",
                "warn",
                format!("Host contract exists but did not parse: {error}"),
            ),
        }
    } else {
        self_test_diagnostic(
            "host_contract",
            "warn",
            format!("Host contract was not found at {}.", contract.display()),
        )
    });

    diagnostics.push(if contract.is_file() {
        match load_contract_value_from_path(&contract)
            .map(|value| dashboard_compatibility_status(&value, &repo_dir))
        {
            Ok(status) => self_test_diagnostic(
                "dashboard_compatibility",
                if status.ready { "ok" } else { "warn" },
                format!(
                    "{} aliases across {} categories from {}; drift checks: {}.",
                    status.alias_count,
                    status.alias_category_count,
                    status.source,
                    status
                        .drift_checks
                        .iter()
                        .map(|check| format!("{}={}", check.id, check.status))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
            Err(error) => self_test_diagnostic(
                "dashboard_compatibility",
                "warn",
                format!("Dashboard compatibility status unavailable: {error}"),
            ),
        }
    } else {
        self_test_diagnostic(
            "dashboard_compatibility",
            "warn",
            format!(
                "Host contract was not found at {}; compatibility drift cannot be checked.",
                contract.display()
            ),
        )
    });

    let ok = diagnostics
        .iter()
        .all(|diagnostic| diagnostic.status != "error");

    DxCliBridgeSelfTest {
        ok,
        command_count: commands.len(),
        commands,
        diagnostics,
        repo_dir: repo_dir.display().to_string(),
        cli_program: cli_program.display().to_string(),
        contract_path: contract.display().to_string(),
    }
}

fn self_test_diagnostic(
    id: impl Into<String>,
    status: impl Into<String>,
    detail: impl Into<String>,
) -> DxCliBridgeSelfTestDiagnostic {
    DxCliBridgeSelfTestDiagnostic {
        id: id.into(),
        status: status.into(),
        detail: detail.into(),
    }
}

fn duplicate_command_names(commands: &[&str]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for command in commands {
        if !seen.insert(*command) {
            duplicates.push((*command).to_string());
        }
    }

    duplicates.sort();
    duplicates.dedup();
    duplicates
}

fn dx_agents_provider_health_args(mode: Option<&str>, provider: Option<&str>) -> Vec<String> {
    let mut args = vec!["models".to_string(), "health".to_string()];

    if let Some(provider) = provider.and_then(trimmed_non_empty) {
        args.push("--provider".to_string());
        args.push(provider.to_string());
    }

    args.push("--mode".to_string());
    args.push(normalize_provider_health_mode(mode).to_string());
    args.push("--json".to_string());
    args
}

fn dx_agents_provider_smoke_args(
    provider: Option<&str>,
    model: Option<&str>,
    _timeout_secs: Option<u64>,
) -> Vec<String> {
    let mut args = vec!["agent".to_string(), "-a".to_string(), "dx".to_string()];

    if let Some(provider) = provider.and_then(trimmed_non_empty) {
        args.push("-p".to_string());
        args.push(provider.to_string());
    }

    if let Some(model) = model.and_then(trimmed_non_empty) {
        args.push("--model".to_string());
        args.push(model.to_string());
    }

    args.push("--message".to_string());
    args.push(PROVIDER_SMOKE_MESSAGE.to_string());
    args
}

fn dx_agents_provider_failover_drill_args(mode: Option<&str>) -> Vec<String> {
    vec![
        "models".to_string(),
        "failover-drill".to_string(),
        "--mode".to_string(),
        normalize_provider_failover_drill_mode(mode).to_string(),
        "--json".to_string(),
    ]
}

fn normalize_provider_failover_drill_mode(mode: Option<&str>) -> &'static str {
    let Some(mode) = mode.map(str::trim) else {
        return "dry-run";
    };

    if mode.eq_ignore_ascii_case("mock") {
        "mock"
    } else {
        "dry-run"
    }
}

fn normalize_provider_health_mode(mode: Option<&str>) -> &'static str {
    let Some(mode) = mode.map(str::trim) else {
        return "dry-run";
    };

    if mode.eq_ignore_ascii_case("mock") {
        "mock"
    } else if mode.eq_ignore_ascii_case("live") {
        "live"
    } else {
        "dry-run"
    }
}

fn trimmed_non_empty(value: &str) -> Option<&str> {
    let value = value.trim();
    (!value.is_empty()).then_some(value)
}

fn dx_agents_compact_status_args() -> Vec<String> {
    vec![
        "status".to_string(),
        "--compact".to_string(),
        "--json".to_string(),
    ]
}

fn dx_agents_cron_preview_args(limit: Option<usize>) -> Vec<String> {
    vec![
        "cron".to_string(),
        "preview".to_string(),
        "--limit".to_string(),
        limit.unwrap_or(5).clamp(1, 25).to_string(),
        "--json".to_string(),
    ]
}

fn dx_agents_cron_history_args(limit: Option<usize>) -> Vec<String> {
    vec![
        "cron".to_string(),
        "history".to_string(),
        "--limit".to_string(),
        limit.unwrap_or(3).clamp(1, 25).to_string(),
        "--json".to_string(),
    ]
}

fn dx_agents_cron_delivery_drill_args(mode: Option<&str>) -> Vec<String> {
    vec![
        "cron".to_string(),
        "delivery-drill".to_string(),
        "--mode".to_string(),
        normalize_cron_delivery_drill_mode(mode).to_string(),
        "--json".to_string(),
    ]
}

fn normalize_cron_delivery_drill_mode(mode: Option<&str>) -> &'static str {
    let Some(mode) = mode.map(str::trim) else {
        return "dry-run";
    };

    if mode.eq_ignore_ascii_case("mock") {
        "mock"
    } else {
        "dry-run"
    }
}

fn dx_agents_tool_safety_drill_args(mode: Option<&str>) -> Vec<String> {
    vec![
        "tools".to_string(),
        "safety-drill".to_string(),
        "--mode".to_string(),
        normalize_tool_safety_drill_mode(mode).to_string(),
        "--json".to_string(),
    ]
}

fn normalize_tool_safety_drill_mode(mode: Option<&str>) -> &'static str {
    let Some(mode) = mode.map(str::trim) else {
        return "dry-run";
    };

    if mode.eq_ignore_ascii_case("mock") {
        "mock"
    } else {
        "dry-run"
    }
}

fn dx_agents_session_tool_routing_args() -> Vec<String> {
    vec![
        "sessions".to_string(),
        "tool-routing".to_string(),
        "--json".to_string(),
    ]
}

fn dx_agents_memory_skill_learning_args() -> Vec<String> {
    vec![
        "memory".to_string(),
        "learning-loop".to_string(),
        "--json".to_string(),
    ]
}

fn dx_agents_gateway_paircode_args() -> Vec<String> {
    vec![
        "gateway".to_string(),
        "get-paircode".to_string(),
        "--new".to_string(),
    ]
}

fn dx_agents_gateway_pairing_drill_args(mode: Option<&str>) -> Vec<String> {
    vec![
        "gateway".to_string(),
        "pairing-drill".to_string(),
        "--mode".to_string(),
        normalize_gateway_pairing_drill_mode(mode).to_string(),
        "--json".to_string(),
    ]
}

fn normalize_gateway_pairing_drill_mode(mode: Option<&str>) -> &'static str {
    let Some(mode) = mode.map(str::trim) else {
        return "dry-run";
    };

    if mode.eq_ignore_ascii_case("mock") {
        "mock"
    } else {
        "dry-run"
    }
}

fn dx_agents_embedded_terminal_echo_pilot_args() -> Vec<String> {
    vec!["echo-pilot".to_string(), "--json".to_string()]
}

fn dx_agents_embedded_terminal_tui_canary_runner_args() -> Vec<String> {
    vec!["tui-canary".to_string(), "--json".to_string()]
}

fn tui_canary_runner_contract(contract: &serde_json::Value) -> TuiCanaryRunnerContract {
    let fallback_args = dx_agents_embedded_terminal_tui_canary_runner_args();
    let fallback_fixed_command = vec![
        "dx-agent".to_string(),
        "tui-canary".to_string(),
        "--json".to_string(),
    ];
    let fallback_result_states = vec![
        "skipped".to_string(),
        "success".to_string(),
        "failed".to_string(),
    ];
    let fallback = || TuiCanaryRunnerContract {
        args: fallback_args.clone(),
        fixed_command: fallback_fixed_command.clone(),
        stdout_limit_bytes: 16 * 1024,
        stderr_limit_bytes: 16 * 1024,
        max_duration_ms: 15_000,
        present: false,
        accepted: false,
        source: "local_fallback".to_string(),
        diagnostics: vec!["source_owned_runner_contract_missing".to_string()],
        result_states: fallback_result_states.clone(),
        skip_reasons: Vec::new(),
    };

    let Some((runner, source)) = contract
        .pointer("/embedded_terminal_tui_canary_runner")
        .map(|runner| (runner, "host_contract.embedded_terminal_tui_canary_runner"))
        .or_else(|| {
            contract
                .pointer("/embedded_terminal_tui_canary/runner")
                .map(|runner| (runner, "legacy_canary_runner_fallback"))
        })
    else {
        return fallback();
    };

    let declared_argv = runner
        .get("fixed_command")
        .and_then(tui_canary_fixed_command_argv)
        .unwrap_or_default();
    let declared_argv_text = declared_argv.iter().map(String::as_str).collect::<Vec<_>>();
    let accepted_argv = matches!(
        declared_argv_text.as_slice(),
        ["dx-agent", "tui-canary", "--json"] | ["tui-canary", "--json"]
    );
    let shell_free = runner
        .get("shell_free")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let accepts_user_input = runner
        .get("accepts_user_input")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let allows_arbitrary_shell = runner
        .get("allows_arbitrary_shell")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let registers_host_action = runner
        .get("registers_host_action")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let redacts_output = runner
        .get("redacts_output")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let supported = runner
        .get("supported")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let schema_ok = runner
        .get("schema_version")
        .and_then(serde_json::Value::as_str)
        .is_none_or(|schema| schema == "dx.embedded_terminal_tui_canary_runner.v1");
    let result_schema_ok = runner
        .get("result_schema_version")
        .and_then(serde_json::Value::as_str)
        .is_none_or(|schema| schema == "dx.embedded_terminal_tui_canary_runner.v1");
    let expected_message_ok = runner
        .get("expected_message")
        .and_then(serde_json::Value::as_str)
        .is_none_or(|message| message == "dx-agent-tui-canary-ok");
    let expected_runner_ok = runner
        .get("expected_runner")
        .and_then(serde_json::Value::as_str)
        .is_none_or(|runner| runner == "developer_tui_canary");
    let stores_payloads = runner
        .get("stores_payloads")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let production_routing_enabled = runner
        .get("production_routing_enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let external_terminal_fallback = runner
        .get("external_terminal_fallback")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(true);
    let result_states = runner
        .get("result_states")
        .and_then(serde_json::Value::as_array)
        .and_then(|values| json_string_array(values))
        .unwrap_or_else(|| fallback_result_states.clone());
    let mut diagnostics = Vec::new();

    if !accepted_argv {
        diagnostics.push("fixed_command_drift".to_string());
    }
    if !shell_free {
        diagnostics.push("shell_free_not_declared".to_string());
    }
    if accepts_user_input {
        diagnostics.push("accepts_user_input_not_allowed".to_string());
    }
    if allows_arbitrary_shell {
        diagnostics.push("arbitrary_shell_not_allowed".to_string());
    }
    if registers_host_action {
        diagnostics.push("host_action_registration_not_allowed".to_string());
    }
    if !redacts_output {
        diagnostics.push("output_redaction_not_declared".to_string());
    }
    if !supported {
        diagnostics.push("runner_contract_not_supported".to_string());
    }
    if !schema_ok {
        diagnostics.push("runner_contract_schema_drift".to_string());
    }
    if !result_schema_ok {
        diagnostics.push("result_schema_drift".to_string());
    }
    if !expected_message_ok {
        diagnostics.push("expected_message_drift".to_string());
    }
    if !expected_runner_ok {
        diagnostics.push("expected_runner_drift".to_string());
    }
    if stores_payloads {
        diagnostics.push("payload_storage_not_allowed".to_string());
    }
    if production_routing_enabled {
        diagnostics.push("production_routing_not_allowed".to_string());
    }
    if !external_terminal_fallback {
        diagnostics.push("external_terminal_fallback_missing".to_string());
    }
    for state in ["skipped", "success", "failed"] {
        if !result_states.iter().any(|candidate| candidate == state) {
            diagnostics.push(format!("result_state_missing_{state}"));
        }
    }

    let accepted = diagnostics.is_empty();
    let mut contract = TuiCanaryRunnerContract {
        args: fallback_args,
        fixed_command: fallback_fixed_command,
        stdout_limit_bytes: 16 * 1024,
        stderr_limit_bytes: 16 * 1024,
        max_duration_ms: 15_000,
        present: true,
        accepted,
        source: if accepted {
            source.to_string()
        } else {
            "local_fallback_due_to_contract_drift".to_string()
        },
        diagnostics,
        result_states,
        skip_reasons: tui_canary_contract_skip_reasons(runner),
    };

    if accepted {
        contract.fixed_command = declared_argv.clone();
        contract.args = if declared_argv.first().is_some_and(|arg| arg == "dx-agent") {
            declared_argv.into_iter().skip(1).collect()
        } else {
            declared_argv
        };
        contract.stdout_limit_bytes = json_usize(runner.get("stdout_limit_bytes"))
            .unwrap_or(16 * 1024)
            .clamp(1024, 64 * 1024);
        contract.stderr_limit_bytes = json_usize(runner.get("stderr_limit_bytes"))
            .unwrap_or(16 * 1024)
            .clamp(1024, 64 * 1024);
        contract.max_duration_ms = json_u64(runner.get("max_duration_ms"))
            .unwrap_or(15_000)
            .clamp(1_000, 30_000);
    }

    contract
}

fn tui_canary_fixed_command_argv(value: &serde_json::Value) -> Option<Vec<String>> {
    if let Some(array) = value.as_array() {
        return json_string_array(array);
    }
    value
        .get("argv")?
        .as_array()
        .and_then(|values| json_string_array(values))
}

fn json_string_array(values: &[serde_json::Value]) -> Option<Vec<String>> {
    values
        .iter()
        .map(|value| value.as_str().map(str::to_string))
        .collect()
}

fn json_string_values(value: Option<&serde_json::Value>) -> Vec<String> {
    value
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn json_str<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(serde_json::Value::as_str)
}

fn json_bool(value: &serde_json::Value, key: &str, default: bool) -> bool {
    value
        .get(key)
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(default)
}

fn json_usize(value: Option<&serde_json::Value>) -> Option<usize> {
    value?
        .as_u64()
        .and_then(|value| usize::try_from(value).ok())
}

fn json_u64(value: Option<&serde_json::Value>) -> Option<u64> {
    value?.as_u64()
}

fn tui_canary_contract_skip_reasons(runner: &serde_json::Value) -> Vec<String> {
    runner
        .get("skip_reasons")
        .and_then(serde_json::Value::as_array)
        .and_then(|values| json_string_array(values))
        .unwrap_or_default()
}

fn redact_dashboard_command(mut command: DxAgentsDashboardCommand) -> DxAgentsDashboardCommand {
    command.stdout = redact_sensitive_text(&command.stdout);
    command.stderr = redact_sensitive_text(&command.stderr);
    command.json_error = command.json_error.as_deref().map(redact_sensitive_text);
    command.recovery_hint = command.recovery_hint.as_deref().map(redact_sensitive_text);
    command
}

fn provider_smoke_record(
    provider: &str,
    model: Option<String>,
    command: &DxAgentsDashboardCommand,
    duration_ms: u64,
) -> DxAgentsProviderSmokeRecord {
    DxAgentsProviderSmokeRecord {
        schema_version: "dx.provider_smoke.v1".to_string(),
        recorded_at_ms: now_ms(),
        provider: provider.to_string(),
        model,
        status: if command.success { "success" } else { "failed" }.to_string(),
        success: command.success,
        exit_code: command.exit_code,
        duration_ms,
        stdout_summary: summarize_error(&command.stdout),
        stderr_summary: summarize_error(&command.stderr),
    }
}

fn append_provider_smoke_record(
    path: &Path,
    record: &DxAgentsProviderSmokeRecord,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create provider smoke history directory: {error}")
        })?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| {
            format!(
                "failed to open provider smoke history {}: {error}",
                path.display()
            )
        })?;
    serde_json::to_writer(&mut file, record)
        .map_err(|error| format!("failed to serialize provider smoke history: {error}"))?;
    writeln!(file)
        .map_err(|error| format!("failed to append provider smoke history newline: {error}"))?;
    Ok(())
}

fn read_provider_smoke_history(
    path: &Path,
    limit: usize,
) -> Result<DxAgentsProviderSmokeHistory, String> {
    if !path.is_file() {
        return Ok(DxAgentsProviderSmokeHistory {
            history_path: path.display().to_string(),
            count: 0,
            entries: Vec::new(),
        });
    }

    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read provider smoke history {}: {error}",
            path.display()
        )
    })?;
    let entries = source
        .lines()
        .rev()
        .filter_map(|line| serde_json::from_str::<DxAgentsProviderSmokeRecord>(line).ok())
        .take(limit)
        .collect::<Vec<_>>();

    Ok(DxAgentsProviderSmokeHistory {
        history_path: path.display().to_string(),
        count: entries.len(),
        entries,
    })
}

fn provider_smoke_history_path() -> PathBuf {
    host_telemetry_dir().join(PROVIDER_SMOKE_HISTORY_FILE)
}

fn run_tool_safety_drill_dashboard_command(
    label: &str,
    mode: Option<&str>,
    source: &str,
) -> Result<DxAgentsDashboardCommand, String> {
    let args = dx_agents_tool_safety_drill_args(mode);
    let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
    let command =
        run_dx_agents_dashboard_command(label, &arg_refs).map(redact_dashboard_command)?;
    let record = tool_safety_drill_record(source, &command);
    let _ = append_tool_safety_drill_record(
        &tool_safety_drill_history_path(),
        &record,
        TOOL_SAFETY_DRILL_HISTORY_LIMIT,
    );
    Ok(command)
}

fn tool_safety_drill_record(
    source: &str,
    command: &DxAgentsDashboardCommand,
) -> DxAgentsToolSafetyDrillRecord {
    let json = command.json.as_ref();
    let summary = json.and_then(|value| value.get("summary"));
    let approval = json.and_then(|value| value.get("approval"));
    let allowlists = json.and_then(|value| value.get("allowlists"));
    let redaction = json.and_then(|value| value.get("redaction"));
    let ready = json.is_some_and(|value| json_bool(value, "ready", false));
    let status = if !command.success {
        "failed"
    } else if ready {
        "ready"
    } else {
        "blocked"
    };
    let redaction_ok = redaction.is_some_and(|value| {
        !json_bool(value, "exports_secret_values", true)
            && !json_bool(value, "exports_allowlist_values", true)
            && !json_bool(value, "exports_command_values", true)
            && !json_bool(value, "exports_path_values", true)
    });
    let recovery_hint = json
        .and_then(|value| json_str(value, "recovery_hint"))
        .map(str::to_string)
        .or_else(|| command.recovery_hint.clone())
        .map(|hint| redact_sensitive_text(&hint));

    DxAgentsToolSafetyDrillRecord {
        schema_version: "dx.tool_safety_drill_history.v1".to_string(),
        recorded_at_ms: now_ms(),
        source: source.to_string(),
        mode: json
            .and_then(|value| json_str(value, "mode"))
            .unwrap_or("unknown")
            .to_string(),
        status: status.to_string(),
        ready,
        command_success: command.success,
        exit_code: command.exit_code,
        allowed_count: json_usize(summary.and_then(|value| value.get("allowed_count")))
            .unwrap_or_default(),
        approval_required_count: json_usize(
            summary.and_then(|value| value.get("approval_required_count")),
        )
        .unwrap_or_default(),
        denied_count: json_usize(summary.and_then(|value| value.get("denied_count")))
            .unwrap_or_default(),
        missing_count: json_usize(summary.and_then(|value| value.get("missing_count")))
            .unwrap_or_default(),
        critical_blocker_count: json_usize(
            summary.and_then(|value| value.get("critical_blocker_count")),
        )
        .unwrap_or_default(),
        autonomy_level: approval
            .and_then(|value| json_str(value, "autonomy_level"))
            .map(str::to_string),
        auto_approve_count: json_usize(approval.and_then(|value| value.get("auto_approve_count")))
            .unwrap_or_default(),
        always_ask_count: json_usize(approval.and_then(|value| value.get("always_ask_count")))
            .unwrap_or_default(),
        tool_filter_group_count: json_usize(
            approval.and_then(|value| value.get("tool_filter_group_count")),
        )
        .unwrap_or_default(),
        mcp_server_count: json_usize(allowlists.and_then(|value| value.get("mcp_server_count")))
            .unwrap_or_default(),
        redaction_ok,
        recovery_hint,
    }
}

fn append_tool_safety_drill_record(
    path: &Path,
    record: &DxAgentsToolSafetyDrillRecord,
    retention_limit: usize,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety drill history directory: {error}")
        })?;
    }

    let mut records = read_tool_safety_drill_records(path)?;
    records.push(record.clone());
    records.sort_by_key(|record| std::cmp::Reverse(record.recorded_at_ms));
    records.truncate(retention_limit.max(1));

    let mut file = fs::File::create(path).map_err(|error| {
        format!(
            "failed to write tool safety drill history {}: {error}",
            path.display()
        )
    })?;
    for record in records.iter().rev() {
        serde_json::to_writer(&mut file, record)
            .map_err(|error| format!("failed to serialize tool safety drill history: {error}"))?;
        writeln!(file).map_err(|error| {
            format!("failed to append tool safety drill history newline: {error}")
        })?;
    }

    Ok(())
}

fn read_tool_safety_drill_history(
    path: &Path,
    limit: usize,
) -> Result<DxAgentsToolSafetyDrillHistory, String> {
    let mut records = read_tool_safety_drill_records(path)?;
    records.sort_by_key(|record| std::cmp::Reverse(record.recorded_at_ms));
    let trend = tool_safety_drill_trend(&records);
    let alerts = tool_safety_drill_alerts(&trend, &records);
    let latest_blocked_recovery_hint = records
        .iter()
        .find(|record| record.status == "blocked" && record.recovery_hint.is_some())
        .and_then(|record| record.recovery_hint.clone());
    records.truncate(limit);

    Ok(DxAgentsToolSafetyDrillHistory {
        history_path: path.display().to_string(),
        count: records.len(),
        retention_limit: TOOL_SAFETY_DRILL_HISTORY_LIMIT,
        latest_blocked_recovery_hint,
        trend,
        alerts,
        entries: records,
    })
}

fn read_tool_safety_drill_records(
    path: &Path,
) -> Result<Vec<DxAgentsToolSafetyDrillRecord>, String> {
    if !path.is_file() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read tool safety drill history {}: {error}",
            path.display()
        )
    })?;
    Ok(source
        .lines()
        .filter_map(|line| serde_json::from_str::<DxAgentsToolSafetyDrillRecord>(line).ok())
        .collect())
}

fn tool_safety_drill_trend(
    records: &[DxAgentsToolSafetyDrillRecord],
) -> DxAgentsToolSafetyDrillTrend {
    let Some(latest) = records.first() else {
        return DxAgentsToolSafetyDrillTrend {
            state: "empty".to_string(),
            snapshot_count: 0,
            latest_recorded_at_ms: None,
            previous_recorded_at_ms: None,
            allowed_delta: 0,
            approval_required_delta: 0,
            denied_delta: 0,
            missing_delta: 0,
            critical_blocker_delta: 0,
            recovery_hint: Some(
                "Run the tool safety drill to create the first redacted snapshot.".to_string(),
            ),
            summary: "No retained tool safety drill snapshots.".to_string(),
        };
    };

    let Some(previous) = records.get(1) else {
        return DxAgentsToolSafetyDrillTrend {
            state: "single_snapshot".to_string(),
            snapshot_count: records.len(),
            latest_recorded_at_ms: Some(latest.recorded_at_ms),
            previous_recorded_at_ms: None,
            allowed_delta: 0,
            approval_required_delta: 0,
            denied_delta: 0,
            missing_delta: 0,
            critical_blocker_delta: 0,
            recovery_hint: latest.recovery_hint.clone(),
            summary: "Only one retained snapshot; run the drill again to calculate trend deltas."
                .to_string(),
        };
    };

    let allowed_delta = count_delta(latest.allowed_count, previous.allowed_count);
    let approval_required_delta = count_delta(
        latest.approval_required_count,
        previous.approval_required_count,
    );
    let denied_delta = count_delta(latest.denied_count, previous.denied_count);
    let missing_delta = count_delta(latest.missing_count, previous.missing_count);
    let critical_blocker_delta = count_delta(
        latest.critical_blocker_count,
        previous.critical_blocker_count,
    );
    let state = if critical_blocker_delta > 0 || denied_delta > 0 || missing_delta > 0 {
        "worsening"
    } else if critical_blocker_delta < 0 || denied_delta < 0 || missing_delta < 0 {
        "improving"
    } else if approval_required_delta != 0 || allowed_delta != 0 {
        "changed"
    } else {
        "stable"
    };
    let recovery_hint = if state == "worsening" {
        Some(
            "Inspect the latest blocked row recovery hints before expanding tool access."
                .to_string(),
        )
    } else {
        latest.recovery_hint.clone()
    };

    DxAgentsToolSafetyDrillTrend {
        state: state.to_string(),
        snapshot_count: records.len(),
        latest_recorded_at_ms: Some(latest.recorded_at_ms),
        previous_recorded_at_ms: Some(previous.recorded_at_ms),
        allowed_delta,
        approval_required_delta,
        denied_delta,
        missing_delta,
        critical_blocker_delta,
        recovery_hint,
        summary: format!(
            "Latest vs previous: allowed {allowed_delta:+}, approval {approval_required_delta:+}, denied {denied_delta:+}, missing {missing_delta:+}, critical blockers {critical_blocker_delta:+}."
        ),
    }
}

fn tool_safety_drill_alerts(
    trend: &DxAgentsToolSafetyDrillTrend,
    records: &[DxAgentsToolSafetyDrillRecord],
) -> Vec<DxAgentsToolSafetyDrillAlert> {
    let Some(latest) = records.first() else {
        return vec![DxAgentsToolSafetyDrillAlert {
            id: "empty_history".to_string(),
            level: "info".to_string(),
            title: "No tool safety history".to_string(),
            detail: "Run the dry-run safety drill to create the first redacted metadata snapshot."
                .to_string(),
            recovery_hint: trend.recovery_hint.clone(),
        }];
    };

    let mut alerts = Vec::new();
    if trend.critical_blocker_delta > 0 || latest.critical_blocker_count > 0 {
        alerts.push(DxAgentsToolSafetyDrillAlert {
            id: "critical_blockers".to_string(),
            level: "blocked".to_string(),
            title: "Critical tool blockers detected".to_string(),
            detail: format!(
                "{} critical blockers retained, delta {:+}.",
                latest.critical_blocker_count, trend.critical_blocker_delta
            ),
            recovery_hint: latest
                .recovery_hint
                .clone()
                .or_else(|| trend.recovery_hint.clone()),
        });
    }
    if trend.denied_delta > 0 || latest.denied_count > 0 {
        alerts.push(DxAgentsToolSafetyDrillAlert {
            id: "denied_tools".to_string(),
            level: "blocked".to_string(),
            title: "Denied tool access changed".to_string(),
            detail: format!(
                "{} denied tools retained, delta {:+}.",
                latest.denied_count, trend.denied_delta
            ),
            recovery_hint: latest
                .recovery_hint
                .clone()
                .or_else(|| trend.recovery_hint.clone()),
        });
    }
    if trend.missing_delta > 0 || latest.missing_count > 0 {
        alerts.push(DxAgentsToolSafetyDrillAlert {
            id: "missing_tools".to_string(),
            level: "blocked".to_string(),
            title: "Required tools are missing".to_string(),
            detail: format!(
                "{} missing tools retained, delta {:+}.",
                latest.missing_count, trend.missing_delta
            ),
            recovery_hint: latest
                .recovery_hint
                .clone()
                .or_else(|| trend.recovery_hint.clone()),
        });
    }
    if trend.approval_required_delta > 0 || latest.approval_required_count > 0 {
        alerts.push(DxAgentsToolSafetyDrillAlert {
            id: "approval_required_tools".to_string(),
            level: "warning".to_string(),
            title: "More tools require approval".to_string(),
            detail: format!(
                "{} approval-required tools retained, delta {:+}.",
                latest.approval_required_count, trend.approval_required_delta
            ),
            recovery_hint: latest
                .recovery_hint
                .clone()
                .or_else(|| trend.recovery_hint.clone()),
        });
    }

    if alerts.is_empty() {
        alerts.push(DxAgentsToolSafetyDrillAlert {
            id: match trend.state.as_str() {
                "improving" => "improving_history",
                "single_snapshot" => "single_snapshot",
                _ => "stable_history",
            }
            .to_string(),
            level: if trend.state == "single_snapshot" {
                "info"
            } else {
                "ok"
            }
            .to_string(),
            title: if trend.state == "improving" {
                "Tool safety trend is improving"
            } else if trend.state == "single_snapshot" {
                "Need another snapshot"
            } else {
                "Tool safety trend is stable"
            }
            .to_string(),
            detail: trend.summary.clone(),
            recovery_hint: trend.recovery_hint.clone(),
        });
    }

    alerts
}

fn tool_safety_audit_summary(
    history: &DxAgentsToolSafetyDrillHistory,
    runbook_present: bool,
) -> DxAgentsToolSafetyAuditSummary {
    let latest = history.entries.first();
    let blocked_alert_count = history
        .alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_alert_count = history
        .alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let ok_alert_count = history
        .alerts
        .iter()
        .filter(|alert| alert.level == "ok")
        .count();
    let info_alert_count = history
        .alerts
        .iter()
        .filter(|alert| alert.level == "info")
        .count();
    let redacted = history.entries.iter().all(|record| record.redaction_ok);
    let latest_recovery_hint_available = latest
        .and_then(|record| record.recovery_hint.as_ref())
        .is_some()
        || history.latest_blocked_recovery_hint.is_some()
        || history
            .alerts
            .iter()
            .any(|alert| alert.recovery_hint.is_some());
    let (state, ready, next_remediation_action) = if history.count == 0 {
        (
            "missing_history",
            false,
            "Run the dry-run safety drill to create the first redacted metadata snapshot.",
        )
    } else if !redacted {
        (
            "redaction_review",
            false,
            "Regenerate the tool safety drill after fixing redaction so the audit can stay metadata-only.",
        )
    } else if !runbook_present {
        (
            "runbook_missing",
            false,
            "Restore the tool safety alert runbook, then rerun release-readiness checks.",
        )
    } else if blocked_alert_count > 0 {
        (
            "blocked",
            false,
            "Open the tool safety runbook, repair the blocked tool condition outside the bridge, then rerun the dry-run safety drill.",
        )
    } else if warning_alert_count > 0 {
        (
            "warning",
            false,
            "Review approval-required tool changes against the runbook, keep approval boundaries intact, then rerun the dry-run safety drill.",
        )
    } else if history.trend.state == "single_snapshot" {
        (
            "needs_second_snapshot",
            false,
            "Run the dry-run safety drill again after the next tool configuration change to produce comparable trend evidence.",
        )
    } else {
        (
            "ready",
            true,
            "Keep monitoring retained tool safety history and export diagnostics only when redacted operator evidence is needed.",
        )
    };
    let summary = format!(
        "Tool safety audit {state}: {} retained snapshots, {} alerts, trend {}, runbook {}.",
        history.count,
        history.alerts.len(),
        history.trend.state,
        if runbook_present {
            "present"
        } else {
            "missing"
        }
    );

    DxAgentsToolSafetyAuditSummary {
        schema_version: "dx.tool_safety_audit.v1".to_string(),
        generated_at_ms: now_ms(),
        ready,
        state: state.to_string(),
        latest_drill_status: latest.map(|record| record.status.clone()),
        latest_drill_mode: latest.map(|record| record.mode.clone()),
        history_snapshot_count: history.count,
        history_trend: history.trend.state.clone(),
        alert_count: history.alerts.len(),
        blocked_alert_count,
        warning_alert_count,
        ok_alert_count,
        info_alert_count,
        alert_ids: history
            .alerts
            .iter()
            .map(|alert| alert.id.clone())
            .collect(),
        runbook_present,
        runbook_target: "tool-safety-runbook".to_string(),
        redacted,
        stores_config_values: false,
        duplicates_history_rows: false,
        latest_recovery_hint_available,
        next_remediation_action: redact_sensitive_text(next_remediation_action),
        summary: redact_sensitive_text(&summary),
    }
}

fn append_tool_safety_audit_record(
    path: &Path,
    record: &DxAgentsToolSafetyAuditSummary,
    retention_limit: usize,
) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety audit history directory: {error}")
        })?;
    }

    let mut records = read_tool_safety_audit_records(path)?;
    records.push(record.clone());
    records.sort_by_key(|record| std::cmp::Reverse(record.generated_at_ms));
    records.truncate(retention_limit.max(1));

    let mut file = fs::File::create(path).map_err(|error| {
        format!(
            "failed to write tool safety audit history {}: {error}",
            path.display()
        )
    })?;
    for record in records.iter().rev() {
        serde_json::to_writer(&mut file, record)
            .map_err(|error| format!("failed to serialize tool safety audit history: {error}"))?;
        writeln!(file).map_err(|error| {
            format!("failed to append tool safety audit history newline: {error}")
        })?;
    }

    Ok(())
}

fn read_tool_safety_audit_history(
    path: &Path,
    limit: usize,
) -> Result<DxAgentsToolSafetyAuditHistory, String> {
    let mut records = read_tool_safety_audit_records(path)?;
    records.sort_by_key(|record| std::cmp::Reverse(record.generated_at_ms));
    let trend = tool_safety_audit_trend(&records);
    let latest_remediation_action = records
        .iter()
        .find(|record| !record.ready)
        .map(|record| redact_sensitive_text(&record.next_remediation_action));
    records.truncate(limit);
    let runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK)
        .is_file();
    let runbook_target = "tool-safety-audit-runbook";
    let digest = tool_safety_audit_digest_from_parts(
        records.len(),
        runbook_present,
        runbook_target,
        &trend,
        latest_remediation_action.as_deref(),
        &records,
    );
    let alerts = tool_safety_audit_review_alerts(&digest);
    let escalation_evidence = tool_safety_audit_escalation_evidence(&digest, &alerts);
    let recovery_drill = tool_safety_audit_recovery_drill(&escalation_evidence);
    let recovery_digest = tool_safety_audit_recovery_digest(&recovery_drill);
    let recovery_alerts = tool_safety_audit_recovery_digest_alerts(&recovery_digest);
    let recovery_alert_digest = tool_safety_audit_recovery_alert_digest(&recovery_alerts);
    let recovery_alert_digest_release_gate =
        tool_safety_audit_recovery_alert_digest_release_gate(&recovery_alert_digest);
    let recovery_alert_digest_release_gate_digest =
        tool_safety_audit_recovery_alert_digest_release_gate_digest(
            &recovery_alert_digest_release_gate,
        );
    let recovery_alert_digest_release_gate_digest_alerts =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
            &recovery_alert_digest_release_gate_digest,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
            &recovery_alert_digest_release_gate_digest_alerts,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alerts =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
            &recovery_alert_digest_release_gate_digest_alert_digest,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
            &recovery_alert_digest_release_gate_digest_alert_digest_alerts,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
            &recovery_alert_digest_release_gate_digest_alert_digest_alert_digest,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
            &recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
            &recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
            &recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts,
        );
    let recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts =
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
            &recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest,
        );

    Ok(DxAgentsToolSafetyAuditHistory {
        history_path: path.display().to_string(),
        count: records.len(),
        retention_limit: TOOL_SAFETY_AUDIT_HISTORY_LIMIT,
        runbook_present,
        runbook_target: runbook_target.to_string(),
        digest,
        alerts,
        escalation_evidence,
        recovery_drill,
        recovery_digest,
        recovery_alerts,
        recovery_alert_digest,
        recovery_alert_digest_release_gate,
        recovery_alert_digest_release_gate_digest,
        recovery_alert_digest_release_gate_digest_alerts,
        recovery_alert_digest_release_gate_digest_alert_digest,
        recovery_alert_digest_release_gate_digest_alert_digest_alerts,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest,
        recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts,
        trend,
        latest_remediation_action,
        entries: records,
    })
}

fn read_tool_safety_audit_records(
    path: &Path,
) -> Result<Vec<DxAgentsToolSafetyAuditSummary>, String> {
    if !path.is_file() {
        return Ok(Vec::new());
    }

    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read tool safety audit history {}: {error}",
            path.display()
        )
    })?;
    Ok(source
        .lines()
        .filter_map(|line| serde_json::from_str::<DxAgentsToolSafetyAuditSummary>(line).ok())
        .collect())
}

fn tool_safety_audit_trend(
    records: &[DxAgentsToolSafetyAuditSummary],
) -> DxAgentsToolSafetyAuditTrend {
    let Some(latest) = records.first() else {
        return DxAgentsToolSafetyAuditTrend {
            state: "empty".to_string(),
            snapshot_count: 0,
            latest_generated_at_ms: None,
            previous_generated_at_ms: None,
            ready_delta: 0,
            alert_delta: 0,
            blocked_alert_delta: 0,
            warning_alert_delta: 0,
            recovery_hint: Some(
                "Run the tool safety audit to create the first metadata-only review snapshot."
                    .to_string(),
            ),
            summary: "No retained tool safety audit summaries.".to_string(),
        };
    };

    let Some(previous) = records.get(1) else {
        return DxAgentsToolSafetyAuditTrend {
            state: "single_snapshot".to_string(),
            snapshot_count: records.len(),
            latest_generated_at_ms: Some(latest.generated_at_ms),
            previous_generated_at_ms: None,
            ready_delta: 0,
            alert_delta: 0,
            blocked_alert_delta: 0,
            warning_alert_delta: 0,
            recovery_hint: Some(
                "Retain at least two audit summaries to classify readiness drift.".to_string(),
            ),
            summary: "Only one retained tool safety audit summary.".to_string(),
        };
    };

    let ready_delta = bool_delta(latest.ready, previous.ready);
    let alert_delta = count_delta(latest.alert_count, previous.alert_count);
    let blocked_alert_delta = count_delta(latest.blocked_alert_count, previous.blocked_alert_count);
    let warning_alert_delta = count_delta(latest.warning_alert_count, previous.warning_alert_count);
    let state = if !latest.ready && previous.ready {
        "worsening"
    } else if latest.ready && !previous.ready {
        "improving"
    } else if blocked_alert_delta > 0 || warning_alert_delta > 0 {
        "worsening"
    } else if blocked_alert_delta < 0 || warning_alert_delta < 0 || alert_delta < 0 {
        "improving"
    } else if alert_delta != 0 || latest.state != previous.state {
        "changed"
    } else {
        "stable"
    };
    let recovery_hint = match state {
        "worsening" => Some(latest.next_remediation_action.clone()),
        "single_snapshot" => {
            Some("Retain at least two audit summaries to classify readiness drift.".to_string())
        }
        _ => None,
    };

    DxAgentsToolSafetyAuditTrend {
        state: state.to_string(),
        snapshot_count: records.len(),
        latest_generated_at_ms: Some(latest.generated_at_ms),
        previous_generated_at_ms: Some(previous.generated_at_ms),
        ready_delta,
        alert_delta,
        blocked_alert_delta,
        warning_alert_delta,
        recovery_hint: recovery_hint.map(|hint| redact_sensitive_text(&hint)),
        summary: format!(
            "Latest vs previous: ready {ready_delta:+}, alerts {alert_delta:+}, blocked {blocked_alert_delta:+}, warnings {warning_alert_delta:+}."
        ),
    }
}

fn tool_safety_audit_digest_from_parts(
    audit_count: usize,
    runbook_present: bool,
    runbook_target: &str,
    trend: &DxAgentsToolSafetyAuditTrend,
    latest_remediation_action: Option<&str>,
    entries: &[DxAgentsToolSafetyAuditSummary],
) -> DxAgentsToolSafetyAuditDigest {
    let latest = entries.first();
    let latest_ready = latest.map(|entry| entry.ready);
    let latest_audit_state = latest.map(|entry| entry.state.clone());
    let redacted = entries.iter().all(|entry| {
        entry.redacted && !entry.stores_config_values && !entry.duplicates_history_rows
    });
    let alert_runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK)
        .is_file();
    let alert_runbook_target = "tool-safety-audit-alert-runbook";
    let state = if audit_count == 0 {
        "empty"
    } else if !runbook_present || !alert_runbook_present {
        "runbook_missing"
    } else if trend.ready_delta < 0 {
        "ready_to_blocked"
    } else if !redacted || latest.is_some_and(|entry| !entry.redacted) {
        "redaction_review"
    } else if trend.state == "worsening" {
        "worsening"
    } else if latest.is_some_and(|entry| !entry.ready) {
        latest
            .map(|entry| entry.state.as_str())
            .unwrap_or("needs_review")
    } else {
        trend.state.as_str()
    };
    let ready = state == "stable"
        && latest_ready == Some(true)
        && runbook_present
        && alert_runbook_present
        && redacted;
    let latest_remediation_action = latest_remediation_action.map(redact_sensitive_text);
    let next_action = match state {
        "empty" => "Run the tool safety audit to create the first metadata-only review snapshot.",
        "runbook_missing" => {
            "Restore the missing audit review runbooks, then rerun release-readiness checks."
        }
        "ready_to_blocked" => {
            "Open the audit review runbook, resolve the latest remediation action, then rerun the dry-run drill and audit."
        }
        "redaction_review" => {
            "Regenerate audit history after fixing redaction so the digest stays metadata-only."
        }
        "worsening" => {
            "Treat worsening audit drift as release-blocking until the latest audit is stable and redacted."
        }
        "stable" => {
            "Keep monitoring retained audit history and export diagnostics only when review evidence is needed."
        }
        "single_snapshot" => "Retain another audit summary before relying on trend classification.",
        "improving" => {
            "Preserve the improving audit evidence and rerun once more before calling it stable."
        }
        _ => "Follow the latest redacted remediation action and rerun the audit review.",
    };
    let summary = format!(
        "Tool safety audit digest {state}: {audit_count} retained audits, trend {}, ready {}, runbook {}, alert runbook {}.",
        trend.state,
        if ready { "yes" } else { "no" },
        if runbook_present {
            "present"
        } else {
            "missing"
        },
        if alert_runbook_present {
            "present"
        } else {
            "missing"
        }
    );

    DxAgentsToolSafetyAuditDigest {
        schema_version: "dx.tool_safety_audit_digest.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        review_required: !ready,
        audit_count,
        latest_audit_state,
        latest_ready,
        trend_state: trend.state.clone(),
        ready_delta: trend.ready_delta,
        alert_delta: trend.alert_delta,
        blocked_alert_delta: trend.blocked_alert_delta,
        warning_alert_delta: trend.warning_alert_delta,
        runbook_present,
        runbook_target: runbook_target.to_string(),
        alert_runbook_present,
        alert_runbook_target: alert_runbook_target.to_string(),
        latest_remediation_action,
        next_action: redact_sensitive_text(next_action),
        redacted,
        metadata_only: true,
        stores_config_values: false,
        duplicates_history_rows: false,
        summary: redact_sensitive_text(&summary),
    }
}

fn tool_safety_audit_review_alerts(
    digest: &DxAgentsToolSafetyAuditDigest,
) -> Vec<DxAgentsToolSafetyAuditReviewAlert> {
    let mut alerts = Vec::new();
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "empty" => (
            "audit_empty",
            "info",
            "No audit review history",
            "No retained audit summaries exist yet.",
            Some("Run the dry-run safety drill to create the first metadata-only audit summary."),
        ),
        "single_snapshot" => (
            "audit_single_snapshot",
            "info",
            "Need another audit snapshot",
            "Only one audit summary is retained, so trend classification is not durable yet.",
            Some("Run the dry-run safety drill again after the next tool configuration change."),
        ),
        "stable" => (
            "audit_stable",
            "ok",
            "Audit review is stable",
            "The latest comparable audit summaries are stable and metadata-only.",
            None,
        ),
        "improving" => (
            "audit_improving",
            "ok",
            "Audit review is improving",
            "The latest audit summary reduced alert pressure compared with the previous snapshot.",
            Some("Retain one more clean audit summary before treating the flow as stable."),
        ),
        "changed" => (
            "audit_changed",
            "warning",
            "Audit review changed",
            "The latest audit summary changed without a clear improving or worsening direction.",
            Some(
                "Review the latest remediation action, confirm the change was expected, and rerun the dry-run safety drill.",
            ),
        ),
        "ready_to_blocked" => (
            "audit_ready_to_blocked",
            "blocked",
            "Audit moved from ready to blocked",
            "The latest audit review moved from ready toward a blocked state.",
            Some(
                "Open the audit review runbook, repair the blocked tool condition, then rerun the dry-run safety drill.",
            ),
        ),
        "runbook_missing" => (
            "audit_runbook_missing",
            "blocked",
            "Audit review runbook is missing",
            "The audit review runbooks are required before this flow can be considered release-ready.",
            Some(
                "Restore the missing audit review runbook docs and rerun release-readiness checks.",
            ),
        ),
        "redaction_review" => (
            "audit_redaction_review",
            "blocked",
            "Audit review needs redaction",
            "The latest audit review cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate the audit after fixing redaction and confirm no command, path, domain, environment, allowlist, or secret values are present.",
            ),
        ),
        "worsening" => (
            "audit_worsening",
            "blocked",
            "Audit review is worsening",
            "The latest audit trend increased blocker, warning, or alert pressure.",
            Some(
                "Treat worsening audit drift as release-blocking until the latest audit is stable and redacted.",
            ),
        ),
        other => (
            "audit_review_required",
            "warning",
            "Audit review needs attention",
            if other.is_empty() {
                "The latest audit review state is unknown."
            } else {
                "The latest audit review state needs operator review."
            },
            Some("Follow the latest redacted remediation action and rerun the audit review."),
        ),
    };
    alerts.push(DxAgentsToolSafetyAuditReviewAlert {
        id: id.to_string(),
        level: level.to_string(),
        title: title.to_string(),
        detail: redact_audit_review_alert_text(detail),
        recovery_hint: recovery_hint
            .or(digest.latest_remediation_action.as_deref())
            .map(redact_audit_review_alert_text),
    });

    if (!digest.runbook_present || !digest.alert_runbook_present)
        && !alerts
            .iter()
            .any(|alert| alert.id == "audit_runbook_missing")
    {
        alerts.push(DxAgentsToolSafetyAuditReviewAlert {
            id: "audit_runbook_missing".to_string(),
            level: "blocked".to_string(),
            title: "Audit review runbook is missing".to_string(),
            detail: "The audit review runbooks are required before this flow can be considered release-ready."
                .to_string(),
            recovery_hint: Some(
                "Restore the missing audit review runbook docs and rerun release-readiness checks."
                    .to_string(),
            ),
        });
    }
    if (!digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts
            .iter()
            .any(|alert| alert.id == "audit_redaction_review")
    {
        alerts.push(DxAgentsToolSafetyAuditReviewAlert {
            id: "audit_redaction_review".to_string(),
            level: "blocked".to_string(),
            title: "Audit review needs redaction".to_string(),
            detail:
                "The digest must stay metadata-only and must not duplicate raw audit or drill rows."
                    .to_string(),
            recovery_hint: Some("Regenerate the audit after fixing redaction.".to_string()),
        });
    }
    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_escalation_evidence(
    digest: &DxAgentsToolSafetyAuditDigest,
    alerts: &[DxAgentsToolSafetyAuditReviewAlert],
) -> DxAgentsToolSafetyAuditEscalationEvidence {
    let blocked_alerts = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .collect::<Vec<_>>();
    let warning_alerts = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .collect::<Vec<_>>();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let top_alert = blocked_alerts
        .first()
        .copied()
        .or_else(|| warning_alerts.first().copied())
        .or_else(|| alerts.first());
    let severity = if !blocked_alerts.is_empty() {
        "blocked"
    } else if !warning_alerts.is_empty() {
        "warning"
    } else {
        "ok"
    };
    let state = match severity {
        "blocked" => "blocked_escalation",
        "warning" => "warning_escalation",
        _ => "no_escalation",
    };
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .or(digest.latest_remediation_action.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match severity {
        "blocked" => "Pause release or promotion work until blocked audit review alerts clear.",
        "warning" => {
            "Review warning audit review alerts, confirm the change was expected, then rerun the dry-run audit path."
        }
        _ => "No blocked or warning audit review alerts require escalation.",
    };
    let summary = format!(
        "Audit review escalation {state}: {} blocked, {} warning, {} info, {} ok alerts.",
        blocked_alerts.len(),
        warning_alerts.len(),
        info_count,
        ok_count
    );

    DxAgentsToolSafetyAuditEscalationEvidence {
        schema_version: "dx.tool_safety_audit_escalation.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        severity: severity.to_string(),
        alert_count: alerts.len(),
        blocked_count: blocked_alerts.len(),
        warning_count: warning_alerts.len(),
        info_count,
        ok_count,
        blocked_alert_ids: blocked_alerts
            .iter()
            .map(|alert| alert.id.clone())
            .collect(),
        warning_alert_ids: warning_alerts
            .iter()
            .map(|alert| alert.id.clone())
            .collect(),
        top_alert_id: top_alert.map(|alert| alert.id.clone()),
        top_alert_title: top_alert.map(|alert| alert.title.clone()),
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        runbook_target: digest.runbook_target.clone(),
        alert_runbook_target: digest.alert_runbook_target.clone(),
        review_required: severity != "ok" || digest.review_required,
        metadata_only: true,
        redacted: digest.redacted,
        stores_config_values: false,
        duplicates_history_rows: false,
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_drill(
    escalation: &DxAgentsToolSafetyAuditEscalationEvidence,
) -> DxAgentsToolSafetyAuditRecoveryDrill {
    let runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK)
        .is_file();
    let runbook_target = "tool-safety-audit-recovery-runbook";
    let (state, outcome, planned_steps, cleared, review_required, next_action) = if !runbook_present
    {
        (
            "runbook_missing_recovery",
            "blocked",
            vec![
                "Restore the audit recovery drill runbook.".to_string(),
                "Rerun release-readiness checks before treating recovery evidence as complete."
                    .to_string(),
            ],
            false,
            true,
            "Restore the audit recovery drill runbook before promotion work continues.",
        )
    } else {
        match escalation.severity.as_str() {
            "blocked" => (
                "blocked_recovery",
                "blocked",
                vec![
                    "Open the audit review alert runbook.".to_string(),
                    "Resolve the blocked audit review condition outside the bridge.".to_string(),
                    "Rerun the dry-run tool safety drill and refresh the audit review.".to_string(),
                ],
                false,
                true,
                "Keep release or promotion work paused until blocked audit escalation clears.",
            ),
            "warning" => (
                "warning_recovery",
                "review",
                vec![
                    "Review the warning audit review alert ids.".to_string(),
                    "Confirm the latest remediation action was expected.".to_string(),
                    "Rerun the dry-run audit path before treating the warning as stable."
                        .to_string(),
                ],
                false,
                true,
                "Complete operator review before promotion work continues.",
            ),
            _ if escalation.review_required => (
                "evidence_pending",
                "pending",
                vec![
                    "Retain another metadata-only audit summary.".to_string(),
                    "Refresh audit history after the next dry-run safety drill.".to_string(),
                ],
                false,
                true,
                "Collect one more metadata-only audit snapshot before treating recovery as cleared.",
            ),
            _ => (
                "cleared_recovery",
                "cleared",
                vec![
                    "Keep the current metadata-only audit evidence.".to_string(),
                    "Continue monitoring retained audit summaries.".to_string(),
                ],
                true,
                false,
                "No audit escalation recovery drill is required.",
            ),
        }
    };
    let planned_steps = planned_steps
        .into_iter()
        .map(|step| redact_audit_review_alert_text(&step))
        .collect::<Vec<_>>();
    let summary = format!(
        "Audit recovery drill {state}: outcome {outcome}, blocked {}, warning {}, dry-run only.",
        escalation.blocked_count, escalation.warning_count
    );

    DxAgentsToolSafetyAuditRecoveryDrill {
        schema_version: "dx.tool_safety_audit_recovery_drill.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        outcome: outcome.to_string(),
        escalation_state: escalation.state.clone(),
        severity: escalation.severity.clone(),
        blocked_alert_ids: escalation
            .blocked_alert_ids
            .iter()
            .map(|id| redact_audit_review_alert_text(id))
            .collect(),
        warning_alert_ids: escalation
            .warning_alert_ids
            .iter()
            .map(|id| redact_audit_review_alert_text(id))
            .collect(),
        planned_steps,
        cleared,
        review_required,
        dry_run_only: true,
        invokes_tools: false,
        runbook_present,
        runbook_target: runbook_target.to_string(),
        metadata_only: true,
        redacted: escalation.redacted,
        stores_config_values: false,
        duplicates_history_rows: false,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_digest(
    drill: &DxAgentsToolSafetyAuditRecoveryDrill,
) -> DxAgentsToolSafetyAuditRecoveryDigest {
    let ready = drill.cleared
        && drill.runbook_present
        && drill.dry_run_only
        && !drill.invokes_tools
        && drill.metadata_only
        && drill.redacted
        && !drill.stores_config_values
        && !drill.duplicates_history_rows;
    let state = if !drill.runbook_present {
        "runbook_missing"
    } else {
        match drill.outcome.as_str() {
            "blocked" => "blocked",
            "review" => "warning_review",
            "pending" => "pending_evidence",
            "cleared" if ready => "cleared",
            "cleared" => "cleared_review",
            _ => "review_required",
        }
    };
    let review_required = !ready || drill.review_required;
    let next_action = if ready {
        "Recovery digest is clear; keep monitoring metadata-only audit summaries."
    } else {
        drill.next_action.as_str()
    };
    let summary = format!(
        "Audit recovery digest {state}: outcome {}, ready {}, blocked {}, warning {}, runbook {}.",
        drill.outcome,
        ready,
        drill.blocked_alert_ids.len(),
        drill.warning_alert_ids.len(),
        if drill.runbook_present {
            "present"
        } else {
            "missing"
        }
    );

    DxAgentsToolSafetyAuditRecoveryDigest {
        schema_version: "dx.tool_safety_audit_recovery_digest.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        outcome: drill.outcome.clone(),
        recovery_state: drill.state.clone(),
        escalation_state: drill.escalation_state.clone(),
        severity: drill.severity.clone(),
        blocked_count: drill.blocked_alert_ids.len(),
        warning_count: drill.warning_alert_ids.len(),
        planned_step_count: drill.planned_steps.len(),
        runbook_present: drill.runbook_present,
        runbook_target: drill.runbook_target.clone(),
        cleared: drill.cleared,
        review_required,
        dry_run_only: drill.dry_run_only,
        invokes_tools: drill.invokes_tools,
        metadata_only: drill.metadata_only,
        redacted: drill.redacted,
        stores_config_values: drill.stores_config_values,
        duplicates_history_rows: drill.duplicates_history_rows,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlert> {
    let mut alerts = Vec::new();
    let runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK)
        .is_file();
    let runbook_target = "tool-safety-audit-recovery-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "runbook_missing" => (
            "recovery_runbook_missing",
            "blocked",
            "Recovery runbook is missing",
            "The recovery digest cannot be release-ready until the source-owned recovery runbook is present.",
            Some(
                "Restore the recovery runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "blocked" => (
            "recovery_blocked",
            "blocked",
            "Recovery remains blocked",
            "The recovery digest still has blocked audit recovery evidence.",
            Some(
                "Keep promotion work paused, follow the recovery runbook, then rerun the dry-run safety drill.",
            ),
        ),
        "warning_review" => (
            "recovery_warning_review",
            "warning",
            "Recovery needs warning review",
            "The recovery digest has warning-level recovery evidence that needs operator review.",
            Some(
                "Review warning recovery evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "pending_evidence" => (
            "recovery_pending_evidence",
            "warning",
            "Recovery evidence is pending",
            "The recovery digest needs another metadata-only audit snapshot before it can be treated as cleared.",
            Some(
                "Retain another metadata-only audit summary and refresh audit history after the next dry-run safety drill.",
            ),
        ),
        "cleared" => (
            "recovery_cleared",
            "ok",
            "Recovery digest is clear",
            "The recovery digest is cleared, metadata-only, redacted, and ready for monitoring.",
            None,
        ),
        "cleared_review" => (
            "recovery_cleared_review",
            "warning",
            "Cleared recovery needs review",
            "The recovery digest outcome is cleared, but readiness flags still require review.",
            Some("Review recovery digest readiness flags and rerun the dry-run audit path."),
        ),
        other => (
            "recovery_review_required",
            "warning",
            "Recovery digest needs review",
            if other.is_empty() {
                "The recovery digest state is unknown."
            } else {
                "The recovery digest state needs operator review."
            },
            Some("Follow the redacted recovery digest action and refresh audit history."),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(DxAgentsToolSafetyAuditRecoveryAlert {
        id: id.to_string(),
        level: level.to_string(),
        title: title.to_string(),
        detail: redact_audit_review_alert_text(detail),
        recovery_hint: recovery_hint
            .or(fallback_hint)
            .map(redact_audit_review_alert_text),
        runbook_present,
        runbook_target: runbook_target.to_string(),
        metadata_only: true,
        redacted: digest.redacted,
        stores_config_values: false,
        duplicates_history_rows: false,
    });

    if !runbook_present
        && !alerts
            .iter()
            .any(|alert| alert.id == "recovery_alert_runbook_missing")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlert {
            id: "recovery_alert_runbook_missing".to_string(),
            level: "blocked".to_string(),
            title: "Recovery alert runbook is missing".to_string(),
            detail:
                "The recovery digest alert runbook is required before recovery alerts are release-ready."
                    .to_string(),
            recovery_hint: Some(
                "Restore the recovery digest alert runbook and rerun release-readiness checks."
                    .to_string(),
            ),
            runbook_present,
            runbook_target: runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    if (!digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows
        || digest.invokes_tools)
        && !alerts
            .iter()
            .any(|alert| alert.id == "recovery_redaction_review")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlert {
            id: "recovery_redaction_review".to_string(),
            level: "blocked".to_string(),
            title: "Recovery digest needs redaction".to_string(),
            detail:
                "The recovery digest must stay metadata-only, redacted, and must not invoke tools or duplicate raw rows."
                    .to_string(),
            recovery_hint: Some(
                "Regenerate recovery evidence after fixing metadata-only and redaction flags.".to_string(),
            ),
            runbook_present,
            runbook_target: runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_recovery_alert_digest(
    alerts: &[DxAgentsToolSafetyAuditRecoveryAlert],
) -> DxAgentsToolSafetyAuditRecoveryAlertDigest {
    let alert_count = alerts.len();
    let blocked_count = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_count = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let runbook_present = alerts.iter().all(|alert| alert.runbook_present);
    let metadata_only = alerts.iter().all(|alert| alert.metadata_only);
    let redacted = alerts.iter().all(|alert| alert.redacted);
    let stores_config_values = alerts.iter().any(|alert| alert.stores_config_values);
    let duplicates_history_rows = alerts.iter().any(|alert| alert.duplicates_history_rows);
    let top_alert = alerts
        .iter()
        .find(|alert| alert.level == "blocked")
        .or_else(|| alerts.iter().find(|alert| alert.level == "warning"))
        .or_else(|| alerts.iter().find(|alert| alert.level == "ok"))
        .or_else(|| alerts.first());
    let runbook_target = top_alert
        .map(|alert| alert.runbook_target.clone())
        .unwrap_or_else(|| "tool-safety-audit-recovery-alert-runbook".to_string());
    let digest_runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK)
        .is_file();
    let digest_runbook_target = "tool-safety-audit-recovery-alert-digest-runbook";
    let safety_blocked =
        !metadata_only || !redacted || stores_config_values || duplicates_history_rows;
    let severity = if alert_count == 0 {
        "info"
    } else if !runbook_present || !digest_runbook_present || safety_blocked || blocked_count > 0 {
        "blocked"
    } else if warning_count > 0 {
        "warning"
    } else {
        "ok"
    };
    let state = if alert_count == 0 {
        "empty"
    } else if !digest_runbook_present {
        "digest_runbook_missing"
    } else if !runbook_present {
        "runbook_missing"
    } else if safety_blocked {
        "redaction_review"
    } else {
        match severity {
            "blocked" => "blocked",
            "warning" => "warning_review",
            _ => "ok",
        }
    };
    let ready = alert_count > 0
        && state == "ok"
        && runbook_present
        && digest_runbook_present
        && metadata_only
        && redacted
        && !stores_config_values
        && !duplicates_history_rows;
    let review_required = !ready;
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match state {
        "ok" => "Recovery alert digest is clear; keep monitoring metadata-only audit evidence.",
        "runbook_missing" => {
            "Restore the recovery alert runbook, rerun release-readiness checks, and refresh audit history."
        }
        "digest_runbook_missing" => {
            "Restore the recovery alert digest runbook, rerun release-readiness checks, and refresh audit history."
        }
        "redaction_review" => {
            "Regenerate recovery alert evidence after fixing metadata-only and redaction flags."
        }
        "blocked" => "Keep promotion work paused until blocked recovery alerts clear.",
        "warning_review" => {
            "Complete operator review for warning recovery alerts before promotion work continues."
        }
        _ => "Review recovery alert digest state and refresh audit history.",
    };
    let summary = format!(
        "Audit recovery alert digest {state}: severity {severity}, alerts {}, blocked {}, warning {}, ok {}, runbook {}, digest runbook {}.",
        alerts.len(),
        blocked_count,
        warning_count,
        ok_count,
        if runbook_present {
            "present"
        } else {
            "missing"
        },
        if digest_runbook_present {
            "present"
        } else {
            "missing"
        }
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigest {
        schema_version: "dx.tool_safety_audit_recovery_alert_digest.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        severity: severity.to_string(),
        alert_count,
        blocked_count,
        warning_count,
        info_count,
        ok_count,
        top_alert_id: top_alert.map(|alert| redact_audit_review_alert_text(&alert.id)),
        top_alert_title: top_alert.map(|alert| redact_audit_review_alert_text(&alert.title)),
        top_alert_level: top_alert.map(|alert| alert.level.clone()),
        runbook_present,
        runbook_target: redact_audit_review_alert_text(&runbook_target),
        digest_runbook_present,
        digest_runbook_target: digest_runbook_target.to_string(),
        review_required,
        metadata_only,
        redacted,
        stores_config_values,
        duplicates_history_rows,
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigest,
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGate {
    let release_gate_runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK)
        .is_file();
    let release_gate_runbook_target =
        "tool-safety-audit-recovery-alert-digest-release-gate-runbook";
    let safe_to_share = digest.metadata_only
        && digest.redacted
        && !digest.stores_config_values
        && !digest.duplicates_history_rows;
    let state = if !safe_to_share {
        "redaction_review"
    } else if !release_gate_runbook_present {
        "release_gate_runbook_missing"
    } else if !digest.digest_runbook_present {
        "digest_runbook_missing"
    } else if !digest.runbook_present {
        "runbook_missing"
    } else if digest.ready {
        "ok"
    } else if digest.severity == "warning" {
        "warning_review"
    } else {
        "blocked"
    };
    let ready = state == "ok" && digest.ready && safe_to_share;
    let release_blocking = !ready;
    let review_required = release_blocking;
    let severity = match state {
        "ok" => "ok",
        "warning_review" => "warning",
        _ => "blocked",
    };
    let next_action = match state {
        "ok" => {
            "Recovery alert digest release gate is clear; keep metadata-only monitoring active."
        }
        "warning_review" => {
            "Complete recovery alert digest operator review before release or promotion continues."
        }
        "runbook_missing" => {
            "Restore the recovery alert runbook, rerun release-readiness checks, and refresh audit history."
        }
        "digest_runbook_missing" => {
            "Restore the recovery alert digest runbook, rerun release-readiness checks, and refresh audit history."
        }
        "release_gate_runbook_missing" => {
            "Restore the recovery alert digest release gate runbook, rerun release-readiness checks, and refresh audit history."
        }
        "redaction_review" => {
            "Regenerate recovery alert digest evidence after fixing metadata-only and redaction flags."
        }
        _ => {
            "Keep release or promotion work paused until blocked recovery alert digest evidence clears."
        }
    };
    let summary = format!(
        "Recovery alert digest release gate {state}: digest {}, severity {}, alerts {}, blocked {}, warning {}, release blocking {}, runbook {}.",
        digest.state,
        severity,
        digest.alert_count,
        digest.blocked_count,
        digest.warning_count,
        if release_blocking { "yes" } else { "no" },
        if release_gate_runbook_present {
            "present"
        } else {
            "missing"
        }
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGate {
        schema_version: "dx.tool_safety_audit_recovery_alert_digest_release_gate.v1".to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        digest_state: redact_audit_review_alert_text(&digest.state),
        digest_ready: digest.ready,
        alert_count: digest.alert_count,
        blocked_count: digest.blocked_count,
        warning_count: digest.warning_count,
        runbook_present: digest.runbook_present,
        runbook_target: redact_audit_review_alert_text(&digest.runbook_target),
        digest_runbook_present: digest.digest_runbook_present,
        digest_runbook_target: redact_audit_review_alert_text(&digest.digest_runbook_target),
        release_gate_runbook_present,
        release_gate_runbook_target: release_gate_runbook_target.to_string(),
        review_required,
        safe_to_share,
        metadata_only: digest.metadata_only,
        redacted: digest.redacted,
        stores_config_values: digest.stores_config_values,
        duplicates_history_rows: digest.duplicates_history_rows,
        recovery_hint: digest
            .recovery_hint
            .as_deref()
            .map(redact_audit_review_alert_text),
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest(
    gate: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGate,
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest {
    let release_gate_digest_runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK)
        .is_file();
    let release_gate_digest_runbook_target =
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook";
    let runbook_present_count = [
        gate.runbook_present,
        gate.digest_runbook_present,
        gate.release_gate_runbook_present,
        release_gate_digest_runbook_present,
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let runbook_count = 4;
    let missing_runbook_count = runbook_count - runbook_present_count;
    let all_runbooks_present = missing_runbook_count == 0;
    let safe_to_share = gate.safe_to_share
        && gate.metadata_only
        && gate.redacted
        && !gate.stores_config_values
        && !gate.duplicates_history_rows;
    let state = if !safe_to_share {
        "redaction_review"
    } else if !release_gate_digest_runbook_present {
        "release_gate_digest_runbook_missing"
    } else if !all_runbooks_present {
        "runbook_missing"
    } else if gate.ready {
        "ready"
    } else if gate.severity == "warning" {
        "warning_review"
    } else {
        "blocked"
    };
    let ready = state == "ready" && gate.ready;
    let release_blocking = !ready;
    let review_required = release_blocking;
    let severity = match state {
        "ready" => "ok",
        "warning_review" => "warning",
        _ => "blocked",
    };
    let next_action = match state {
        "ready" => "Release gate digest is clear; keep metadata-only monitoring active.",
        "warning_review" => {
            "Complete release gate operator review before release or promotion continues."
        }
        "runbook_missing" => {
            "Restore missing release gate runbook inputs, rerun release-readiness checks, and refresh audit history."
        }
        "release_gate_digest_runbook_missing" => {
            "Restore the recovery alert digest release gate digest runbook, rerun release-readiness checks, and refresh audit history."
        }
        "redaction_review" => {
            "Regenerate release gate digest evidence after fixing metadata-only and redaction flags."
        }
        _ => "Keep release or promotion work paused until blocked release gate evidence clears.",
    };
    let summary = format!(
        "Recovery alert digest release gate digest {state}: gate {}, severity {}, alerts {}, blocked {}, warning {}, runbooks {}/{} present.",
        gate.state,
        severity,
        gate.alert_count,
        gate.blocked_count,
        gate.warning_count,
        runbook_present_count,
        runbook_count
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest {
        schema_version: "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest.v1"
            .to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        gate_state: redact_audit_review_alert_text(&gate.state),
        gate_ready: gate.ready,
        alert_count: gate.alert_count,
        blocked_count: gate.blocked_count,
        warning_count: gate.warning_count,
        runbook_count,
        runbook_present_count,
        missing_runbook_count,
        runbook_present: gate.runbook_present,
        digest_runbook_present: gate.digest_runbook_present,
        release_gate_runbook_present: gate.release_gate_runbook_present,
        release_gate_digest_runbook_present,
        release_gate_digest_runbook_target: release_gate_digest_runbook_target.to_string(),
        all_runbooks_present,
        review_required,
        safe_to_share,
        metadata_only: gate.metadata_only,
        redacted: gate.redacted,
        stores_config_values: gate.stores_config_values,
        duplicates_history_rows: gate.duplicates_history_rows,
        recovery_hint: gate
            .recovery_hint
            .as_deref()
            .map(redact_audit_review_alert_text),
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert> {
    let mut alerts = Vec::new();
    let runbook_present = digest.release_gate_digest_runbook_present;
    let runbook_target = digest.release_gate_digest_runbook_target.clone();
    let alert_runbook_present = dx_agents_repo_dir()
        .join(TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK)
        .is_file();
    let alert_runbook_target =
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "ready" => (
            "release_gate_digest_ready",
            "ok",
            "Release gate digest is ready",
            "The release gate digest is ready, metadata-only, redacted, and backed by required runbooks.",
            None,
        ),
        "warning_review" => (
            "release_gate_digest_warning_review",
            "warning",
            "Release gate digest needs warning review",
            "The release gate digest has warning-level evidence that needs operator review.",
            Some(
                "Review warning release-gate evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "blocked" => (
            "release_gate_digest_blocked",
            "blocked",
            "Release gate digest is blocked",
            "The release gate digest is blocking release or promotion work.",
            Some(
                "Keep promotion work paused, resolve blocked release-gate evidence, then rerun the dry-run safety drill.",
            ),
        ),
        "runbook_missing" => (
            "release_gate_digest_runbook_missing",
            "blocked",
            "Release gate digest runbook input is missing",
            "The release gate digest cannot be release-ready until every required source-owned runbook is present.",
            Some(
                "Restore missing release-gate digest runbook inputs, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_runbook_missing" => (
            "release_gate_digest_runbook_missing",
            "blocked",
            "Release gate digest runbook is missing",
            "The release gate digest runbook is required before release-gate digest evidence is complete.",
            Some(
                "Restore the release gate digest runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "redaction_review" => (
            "release_gate_digest_redaction_review",
            "blocked",
            "Release gate digest needs redaction",
            "The release gate digest cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate release-gate digest evidence after fixing metadata-only and redaction flags.",
            ),
        ),
        other => (
            "release_gate_digest_review_required",
            "warning",
            "Release gate digest needs review",
            if other.is_empty() {
                "The release gate digest state is unknown."
            } else {
                "The release gate digest state needs operator review."
            },
            Some("Follow the redacted release-gate digest action and refresh audit history."),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert {
            id: id.to_string(),
            level: level.to_string(),
            title: title.to_string(),
            detail: redact_audit_review_alert_text(detail),
            recovery_hint: recovery_hint
                .or(fallback_hint)
                .map(redact_audit_review_alert_text),
            gate_state: redact_audit_review_alert_text(&digest.gate_state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: redact_audit_review_alert_text(alert_runbook_target),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        },
    );

    if !alert_runbook_present
        && !alerts
            .iter()
            .any(|alert| alert.id == "release_gate_digest_alert_runbook_missing")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert {
            id: "release_gate_digest_alert_runbook_missing".to_string(),
            level: "blocked".to_string(),
            title: "Release gate digest alert runbook is missing".to_string(),
            detail:
                "The release gate digest alert runbook is required before release-gate digest alerts are release-ready."
                    .to_string(),
            recovery_hint: Some(
                "Restore the release gate digest alert runbook and rerun release-readiness checks."
                    .to_string(),
            ),
            gate_state: redact_audit_review_alert_text(&digest.gate_state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    if (!digest.safe_to_share
        || !digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts
            .iter()
            .any(|alert| alert.id == "release_gate_digest_redaction_review")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert {
            id: "release_gate_digest_redaction_review".to_string(),
            level: "blocked".to_string(),
            title: "Release gate digest needs redaction".to_string(),
            detail:
                "The release gate digest must stay metadata-only, redacted, and must not duplicate raw rows."
                    .to_string(),
            recovery_hint: Some(
                "Regenerate release-gate digest evidence after fixing metadata-only and redaction flags."
                    .to_string(),
            ),
            gate_state: redact_audit_review_alert_text(&digest.gate_state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: redact_audit_review_alert_text(alert_runbook_target),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
    alerts: &[DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlert],
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest {
    let alert_count = alerts.len();
    let blocked_count = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_count = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let top_alert = alerts.first();
    let digest_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.runbook_present);
    let alert_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.alert_runbook_present);
    let alert_digest_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK,
        )
        .is_file();
    let digest_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook".to_string()
        });
    let alert_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.alert_runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook".to_string()
        });
    let alert_digest_runbook_target =
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook";
    let runbook_present_count = [
        digest_runbook_present,
        alert_runbook_present,
        alert_digest_runbook_present,
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let runbook_count = 3;
    let missing_runbook_count = runbook_count - runbook_present_count;
    let all_runbooks_present = missing_runbook_count == 0;
    let metadata_only = alerts.iter().all(|alert| alert.metadata_only);
    let redacted = alerts.iter().all(|alert| alert.redacted);
    let stores_config_values = alerts.iter().any(|alert| alert.stores_config_values);
    let duplicates_history_rows = alerts.iter().any(|alert| alert.duplicates_history_rows);
    let safe_to_share = alert_count > 0
        && metadata_only
        && redacted
        && !stores_config_values
        && !duplicates_history_rows;
    let state = if alert_count == 0 {
        "empty"
    } else if !safe_to_share {
        "redaction_review"
    } else if !alert_digest_runbook_present {
        "release_gate_digest_alert_digest_runbook_missing"
    } else if !alert_runbook_present {
        "release_gate_digest_alert_runbook_missing"
    } else if !digest_runbook_present {
        "release_gate_digest_runbook_missing"
    } else if blocked_count > 0 {
        "blocked"
    } else if warning_count > 0 {
        "warning_review"
    } else if ok_count == alert_count {
        "ready"
    } else {
        "review_required"
    };
    let ready = state == "ready";
    let release_blocking = !ready;
    let severity = match state {
        "ready" => "ok",
        "warning_review" | "review_required" => "warning",
        _ => "blocked",
    };
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match state {
        "ready" => "Release gate digest alerts are clear; keep metadata-only monitoring active.",
        "warning_review" => {
            "Review warning release-gate digest alerts before release or promotion continues."
        }
        "blocked" => {
            "Keep release or promotion work paused until blocked release-gate digest alerts clear."
        }
        "release_gate_digest_runbook_missing" => {
            "Restore the release gate digest runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_runbook_missing" => {
            "Restore the release gate digest alert runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest runbook and rerun release-readiness checks."
        }
        "redaction_review" => {
            "Regenerate release gate digest alert evidence after fixing metadata-only and redaction flags."
        }
        "empty" => "Run the tool safety audit to produce release gate digest alerts.",
        _ => "Review release gate digest alert evidence and refresh audit history.",
    };
    let summary = format!(
        "Release gate digest alert digest {state}: {alert_count} alerts, severity {severity}, blocked {blocked_count}, warning {warning_count}, runbooks {runbook_present_count}/{runbook_count} present."
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest {
        schema_version:
            "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest.v1"
                .to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        alert_count,
        blocked_count,
        warning_count,
        ok_count,
        info_count,
        top_alert_id: top_alert.map(|alert| redact_audit_review_alert_text(&alert.id)),
        top_alert_level: top_alert.map(|alert| redact_audit_review_alert_text(&alert.level)),
        digest_runbook_present,
        digest_runbook_target,
        alert_runbook_present,
        alert_runbook_target,
        alert_digest_runbook_present,
        alert_digest_runbook_target: alert_digest_runbook_target.to_string(),
        runbook_count,
        runbook_present_count,
        missing_runbook_count,
        all_runbooks_present,
        safe_to_share,
        metadata_only,
        redacted,
        stores_config_values,
        duplicates_history_rows,
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert> {
    let mut alerts = Vec::new();
    let runbook_present = digest.alert_digest_runbook_present;
    let runbook_target = digest.alert_digest_runbook_target.clone();
    let alert_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        )
        .is_file();
    let alert_runbook_target =
        "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "ready" => (
            "release_gate_digest_alert_digest_ready",
            "ok",
            "Release gate digest alert digest is ready",
            "The release gate digest alert digest is ready, metadata-only, redacted, and backed by required runbooks.",
            None,
        ),
        "warning_review" => (
            "release_gate_digest_alert_digest_warning_review",
            "warning",
            "Release gate digest alert digest needs warning review",
            "The release gate digest alert digest has warning-level evidence that needs operator review.",
            Some(
                "Review warning release-gate digest alert digest evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "blocked" => (
            "release_gate_digest_alert_digest_blocked",
            "blocked",
            "Release gate digest alert digest is blocked",
            "The release gate digest alert digest is blocking release or promotion work.",
            Some(
                "Keep promotion work paused, resolve blocked release-gate digest alert digest evidence, then rerun the dry-run safety drill.",
            ),
        ),
        "release_gate_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_digest_runbook_missing",
            "blocked",
            "Release gate digest runbook is missing",
            "The upstream release gate digest runbook is required before alert digest evidence is complete.",
            Some(
                "Restore the release gate digest runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_runbook_missing",
            "blocked",
            "Release gate digest alert runbook is missing",
            "The release gate digest alert runbook is required before alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest runbook is missing",
            "The release gate digest alert digest runbook is required before alert digest alerts are release-ready.",
            Some(
                "Restore the release gate digest alert digest runbook and rerun release-readiness checks.",
            ),
        ),
        "redaction_review" => (
            "release_gate_digest_alert_digest_redaction_review",
            "blocked",
            "Release gate digest alert digest needs redaction",
            "The release gate digest alert digest cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate release-gate digest alert digest evidence after fixing metadata-only and redaction flags.",
            ),
        ),
        "empty" => (
            "release_gate_digest_alert_digest_empty",
            "blocked",
            "Release gate digest alert digest is empty",
            "No release gate digest alert evidence is available for release readiness.",
            Some(
                "Run the tool safety audit, refresh audit history, and rerun release-readiness checks.",
            ),
        ),
        _ => (
            "release_gate_digest_alert_digest_review_required",
            "warning",
            "Release gate digest alert digest needs review",
            "The release gate digest alert digest state needs operator review.",
            Some("Follow the redacted alert digest action and refresh audit history."),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert {
            id: id.to_string(),
            level: level.to_string(),
            title: title.to_string(),
            detail: redact_audit_review_alert_text(detail),
            recovery_hint: recovery_hint
                .or(fallback_hint)
                .map(redact_audit_review_alert_text),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        },
    );

    if !alert_runbook_present
        && !alerts
            .iter()
            .any(|alert| alert.id == "release_gate_digest_alert_digest_alerts_runbook_missing")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert {
            id: "release_gate_digest_alert_digest_alerts_runbook_missing".to_string(),
            level: "blocked".to_string(),
            title: "Release gate digest alert digest alert runbook is missing".to_string(),
            detail:
                "The release gate digest alert digest alert runbook is required before alert digest alerts are release-ready."
                    .to_string(),
            recovery_hint: Some(
                "Restore the release gate digest alert digest alert runbook and rerun release-readiness checks."
                    .to_string(),
            ),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    if (!digest.safe_to_share
        || !digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts
            .iter()
            .any(|alert| alert.id == "release_gate_digest_alert_digest_redaction_review")
    {
        alerts.push(DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert {
            id: "release_gate_digest_alert_digest_redaction_review".to_string(),
            level: "blocked".to_string(),
            title: "Release gate digest alert digest needs redaction".to_string(),
            detail:
                "The release gate digest alert digest must stay metadata-only, redacted, and must not duplicate raw rows."
                    .to_string(),
            recovery_hint: Some(
                "Regenerate alert digest evidence after fixing metadata-only and redaction flags."
                    .to_string(),
            ),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        });
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
    alerts: &[DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlert],
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest {
    let alert_count = alerts.len();
    let blocked_count = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_count = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let top_alert = alerts.first();
    let alert_digest_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.runbook_present);
    let alert_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.alert_runbook_present);
    let alert_digest_alert_digest_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        )
        .is_file();
    let alert_digest_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook"
                .to_string()
        });
    let alert_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.alert_runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook"
                .to_string()
        });
    let alert_digest_alert_digest_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook";
    let runbook_present_count = [
        alert_digest_runbook_present,
        alert_runbook_present,
        alert_digest_alert_digest_runbook_present,
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let runbook_count = 3;
    let missing_runbook_count = runbook_count - runbook_present_count;
    let all_runbooks_present = missing_runbook_count == 0;
    let metadata_only = alerts.iter().all(|alert| alert.metadata_only);
    let redacted = alerts.iter().all(|alert| alert.redacted);
    let stores_config_values = alerts.iter().any(|alert| alert.stores_config_values);
    let duplicates_history_rows = alerts.iter().any(|alert| alert.duplicates_history_rows);
    let safe_to_share = alert_count > 0
        && metadata_only
        && redacted
        && !stores_config_values
        && !duplicates_history_rows;
    let state = if alert_count == 0 {
        "empty"
    } else if !safe_to_share {
        "redaction_review"
    } else if !alert_digest_alert_digest_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_runbook_missing"
    } else if !alert_runbook_present {
        "release_gate_digest_alert_digest_alert_runbook_missing"
    } else if !alert_digest_runbook_present {
        "release_gate_digest_alert_digest_runbook_missing"
    } else if blocked_count > 0 {
        "blocked"
    } else if warning_count > 0 {
        "warning_review"
    } else if ok_count == alert_count {
        "ready"
    } else {
        "review_required"
    };
    let ready = state == "ready";
    let release_blocking = !ready;
    let severity = match state {
        "ready" => "ok",
        "warning_review" | "review_required" => "warning",
        _ => "blocked",
    };
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match state {
        "ready" => {
            "Release gate digest alert digest alerts are clear; keep metadata-only monitoring active."
        }
        "warning_review" => {
            "Review warning release-gate digest alert digest alerts before release or promotion continues."
        }
        "blocked" => {
            "Keep release or promotion work paused until blocked release-gate digest alert digest alerts clear."
        }
        "release_gate_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_runbook_missing" => {
            "Restore the release gate digest alert digest alert runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest runbook and rerun release-readiness checks."
        }
        "redaction_review" => {
            "Regenerate release gate digest alert digest alert evidence after fixing metadata-only and redaction flags."
        }
        "empty" => "Run the tool safety audit to produce release gate digest alert digest alerts.",
        _ => "Review release gate digest alert digest alert evidence and refresh audit history.",
    };
    let summary = format!(
        "Release gate digest alert digest alert digest {state}: {alert_count} alerts, severity {severity}, blocked {blocked_count}, warning {warning_count}, runbooks {runbook_present_count}/{runbook_count} present."
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest {
        schema_version:
            "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest.v1"
                .to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        alert_count,
        blocked_count,
        warning_count,
        ok_count,
        info_count,
        top_alert_id: top_alert.map(|alert| redact_audit_review_alert_text(&alert.id)),
        top_alert_level: top_alert.map(|alert| redact_audit_review_alert_text(&alert.level)),
        alert_digest_runbook_present,
        alert_digest_runbook_target,
        alert_runbook_present,
        alert_runbook_target,
        alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_runbook_target: alert_digest_alert_digest_runbook_target
            .to_string(),
        runbook_count,
        runbook_present_count,
        missing_runbook_count,
        all_runbooks_present,
        safe_to_share,
        metadata_only,
        redacted,
        stores_config_values,
        duplicates_history_rows,
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert> {
    let mut alerts = Vec::new();
    let runbook_present = digest.alert_digest_alert_digest_runbook_present;
    let runbook_target = digest.alert_digest_alert_digest_runbook_target.clone();
    let alert_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        )
        .is_file();
    let alert_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "ready" => (
            "release_gate_digest_alert_digest_alert_digest_ready",
            "ok",
            "Release gate digest alert digest alert digest is ready",
            "The release gate digest alert digest alert digest is ready, metadata-only, redacted, and backed by required runbooks.",
            None,
        ),
        "warning_review" => (
            "release_gate_digest_alert_digest_alert_digest_warning_review",
            "warning",
            "Release gate digest alert digest alert digest needs warning review",
            "The release gate digest alert digest alert digest has warning-level evidence that needs operator review.",
            Some(
                "Review warning release-gate digest alert digest alert digest evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "blocked" => (
            "release_gate_digest_alert_digest_alert_digest_blocked",
            "blocked",
            "Release gate digest alert digest alert digest is blocked",
            "The release gate digest alert digest alert digest is blocking release or promotion work.",
            Some(
                "Keep promotion work paused, resolve blocked release-gate digest alert digest alert digest evidence, then rerun the dry-run safety drill.",
            ),
        ),
        "release_gate_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest runbook is missing",
            "The upstream release gate digest alert digest runbook is required before alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert runbook is missing",
            "The upstream release gate digest alert digest alert runbook is required before alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest alert runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest runbook is missing",
            "The release gate digest alert digest alert digest runbook is required before alert digest alert digest alerts are release-ready.",
            Some(
                "Restore the release gate digest alert digest alert digest runbook and rerun release-readiness checks.",
            ),
        ),
        "redaction_review" => (
            "release_gate_digest_alert_digest_alert_digest_redaction_review",
            "blocked",
            "Release gate digest alert digest alert digest needs redaction",
            "The release gate digest alert digest alert digest cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate release-gate digest alert digest alert digest evidence after fixing metadata-only and redaction flags.",
            ),
        ),
        "empty" => (
            "release_gate_digest_alert_digest_alert_digest_empty",
            "blocked",
            "Release gate digest alert digest alert digest is empty",
            "No release gate digest alert digest alert evidence is available for release readiness.",
            Some(
                "Run the tool safety audit, refresh audit history, and rerun release-readiness checks.",
            ),
        ),
        _ => (
            "release_gate_digest_alert_digest_alert_digest_review_required",
            "warning",
            "Release gate digest alert digest alert digest needs review",
            "The release gate digest alert digest alert digest state needs operator review.",
            Some("Follow the redacted alert digest alert digest action and refresh audit history."),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert {
            id: id.to_string(),
            level: level.to_string(),
            title: title.to_string(),
            detail: redact_audit_review_alert_text(detail),
            recovery_hint: recovery_hint
                .or(fallback_hint)
                .map(redact_audit_review_alert_text),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        },
    );

    if !alert_runbook_present
        && !alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_alerts_runbook_missing"
        })
    {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_alerts_runbook_missing"
                    .to_string(),
                level: "blocked".to_string(),
                title: "Release gate digest alert digest alert digest alert runbook is missing"
                    .to_string(),
                detail:
                    "The release gate digest alert digest alert digest alert runbook is required before alert digest alert digest alerts are release-ready."
                        .to_string(),
                recovery_hint: Some(
                    "Restore the release gate digest alert digest alert digest alert runbook and rerun release-readiness checks."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    if (!digest.safe_to_share
        || !digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_redaction_review"
        })
    {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_redaction_review".to_string(),
                level: "blocked".to_string(),
                title: "Release gate digest alert digest alert digest needs redaction".to_string(),
                detail:
                    "The release gate digest alert digest alert digest must stay metadata-only, redacted, and must not duplicate raw rows."
                        .to_string(),
                recovery_hint: Some(
                    "Regenerate alert digest alert digest evidence after fixing metadata-only and redaction flags."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
    alerts: &[DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlert],
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigest {
    let alert_count = alerts.len();
    let blocked_count = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_count = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let top_alert = alerts.first();
    let alert_digest_alert_digest_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.runbook_present);
    let alert_digest_alert_digest_alert_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.alert_runbook_present);
    let alert_digest_alert_digest_alert_digest_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        )
        .is_file();
    let alert_digest_alert_digest_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook"
                .to_string()
        });
    let alert_digest_alert_digest_alert_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.alert_runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook"
                .to_string()
        });
    let alert_digest_alert_digest_alert_digest_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook";
    let runbook_present_count = [
        alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_alert_runbook_present,
        alert_digest_alert_digest_alert_digest_runbook_present,
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let runbook_count = 3;
    let missing_runbook_count = runbook_count - runbook_present_count;
    let all_runbooks_present = missing_runbook_count == 0;
    let metadata_only = alerts.iter().all(|alert| alert.metadata_only);
    let redacted = alerts.iter().all(|alert| alert.redacted);
    let stores_config_values = alerts.iter().any(|alert| alert.stores_config_values);
    let duplicates_history_rows = alerts.iter().any(|alert| alert.duplicates_history_rows);
    let safe_to_share = alert_count > 0
        && metadata_only
        && redacted
        && !stores_config_values
        && !duplicates_history_rows;
    let state = if alert_count == 0 {
        "empty"
    } else if !safe_to_share {
        "redaction_review"
    } else if !alert_digest_alert_digest_alert_digest_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
    } else if !alert_digest_alert_digest_alert_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing"
    } else if !alert_digest_alert_digest_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_runbook_missing"
    } else if blocked_count > 0 {
        "blocked"
    } else if warning_count > 0 {
        "warning_review"
    } else if ok_count == alert_count {
        "ready"
    } else {
        "review_required"
    };
    let ready = state == "ready";
    let release_blocking = !ready;
    let severity = match state {
        "ready" => "ok",
        "warning_review" | "review_required" => "warning",
        _ => "blocked",
    };
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match state {
        "ready" => {
            "Release gate digest alert digest alert digest alerts are clear; keep metadata-only monitoring active."
        }
        "warning_review" => {
            "Review warning release-gate digest alert digest alert digest alerts before release or promotion continues."
        }
        "blocked" => {
            "Keep release or promotion work paused until blocked release-gate digest alert digest alert digest alerts clear."
        }
        "release_gate_digest_alert_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest alert runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest alert digest runbook and rerun release-readiness checks."
        }
        "redaction_review" => {
            "Regenerate release gate digest alert digest alert digest alert evidence after fixing metadata-only and redaction flags."
        }
        "empty" => {
            "Run the tool safety audit to produce release gate digest alert digest alert digest alerts."
        }
        _ => {
            "Review release gate digest alert digest alert digest alert evidence and refresh audit history."
        }
    };
    let summary = format!(
        "Release gate digest alert digest alert digest alert digest {state}: {alert_count} alerts, severity {severity}, blocked {blocked_count}, warning {warning_count}, runbooks {runbook_present_count}/{runbook_count} present."
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigest {
        schema_version:
            "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest.v1"
                .to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        alert_count,
        blocked_count,
        warning_count,
        ok_count,
        info_count,
        top_alert_id: top_alert.map(|alert| redact_audit_review_alert_text(&alert.id)),
        top_alert_level: top_alert.map(|alert| redact_audit_review_alert_text(&alert.level)),
        alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_runbook_target,
        alert_digest_alert_digest_alert_runbook_present,
        alert_digest_alert_digest_alert_runbook_target,
        alert_digest_alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_alert_digest_runbook_target:
            alert_digest_alert_digest_alert_digest_runbook_target.to_string(),
        runbook_count,
        runbook_present_count,
        missing_runbook_count,
        all_runbooks_present,
        safe_to_share,
        metadata_only,
        redacted,
        stores_config_values,
        duplicates_history_rows,
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert>{
    let mut alerts = Vec::new();
    let runbook_present = digest.alert_digest_alert_digest_alert_digest_runbook_present;
    let runbook_target = digest
        .alert_digest_alert_digest_alert_digest_runbook_target
        .clone();
    let alert_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        )
        .is_file();
    let alert_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "ready" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_ready",
            "ok",
            "Release gate digest alert digest alert digest alert digest is ready",
            "The release gate digest alert digest alert digest alert digest is ready, metadata-only, redacted, and backed by required runbooks.",
            None,
        ),
        "warning_review" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_warning_review",
            "warning",
            "Release gate digest alert digest alert digest alert digest needs warning review",
            "The release gate digest alert digest alert digest alert digest has warning-level evidence that needs operator review.",
            Some(
                "Review warning release-gate digest alert digest alert digest alert digest evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "blocked" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_blocked",
            "blocked",
            "Release gate digest alert digest alert digest alert digest is blocked",
            "The release gate digest alert digest alert digest alert digest is blocking release or promotion work.",
            Some(
                "Keep promotion work paused, resolve blocked release-gate digest alert digest alert digest alert digest evidence, then rerun the dry-run safety drill.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest runbook is missing",
            "The upstream release gate digest alert digest alert digest runbook is required before alert digest alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest alert digest runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest alert runbook is missing",
            "The upstream release gate digest alert digest alert digest alert runbook is required before alert digest alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest alert digest alert runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest alert digest runbook is missing",
            "The release gate digest alert digest alert digest alert digest runbook is required before alert digest alert digest alert digest alerts are release-ready.",
            Some(
                "Restore the release gate digest alert digest alert digest alert digest runbook and rerun release-readiness checks.",
            ),
        ),
        "redaction_review" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review",
            "blocked",
            "Release gate digest alert digest alert digest alert digest needs redaction",
            "The release gate digest alert digest alert digest alert digest cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate release-gate digest alert digest alert digest alert digest evidence after fixing metadata-only and redaction flags.",
            ),
        ),
        "empty" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_empty",
            "blocked",
            "Release gate digest alert digest alert digest alert digest is empty",
            "No release gate digest alert digest alert digest alert evidence is available for release readiness.",
            Some(
                "Run the tool safety audit, refresh audit history, and rerun release-readiness checks.",
            ),
        ),
        _ => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_review_required",
            "warning",
            "Release gate digest alert digest alert digest alert digest needs review",
            "The release gate digest alert digest alert digest alert digest state needs operator review.",
            Some(
                "Follow the redacted alert digest alert digest alert digest action and refresh audit history.",
            ),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert {
            id: id.to_string(),
            level: level.to_string(),
            title: title.to_string(),
            detail: redact_audit_review_alert_text(detail),
            recovery_hint: recovery_hint
                .or(fallback_hint)
                .map(redact_audit_review_alert_text),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        },
    );

    if !alert_runbook_present && !alerts.iter().any(|alert| {
        alert.id
            == "release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
    }) {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
                    .to_string(),
                level: "blocked".to_string(),
                title:
                    "Release gate digest alert digest alert digest alert digest alert runbook is missing"
                        .to_string(),
                detail:
                    "The release gate digest alert digest alert digest alert digest alert runbook is required before alert digest alert digest alert digest alerts are release-ready."
                        .to_string(),
                recovery_hint: Some(
                    "Restore the release gate digest alert digest alert digest alert digest alert runbook and rerun release-readiness checks."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    if (!digest.safe_to_share
        || !digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review"
        })
    {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review"
                    .to_string(),
                level: "blocked".to_string(),
                title: "Release gate digest alert digest alert digest alert digest needs redaction"
                    .to_string(),
                detail:
                    "The release gate digest alert digest alert digest alert digest must stay metadata-only, redacted, and must not duplicate raw rows."
                        .to_string(),
                recovery_hint: Some(
                    "Regenerate alert digest alert digest alert digest evidence after fixing metadata-only and redaction flags."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
    alerts: &[DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlert],
) -> DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigest{
    let alert_count = alerts.len();
    let blocked_count = alerts
        .iter()
        .filter(|alert| alert.level == "blocked")
        .count();
    let warning_count = alerts
        .iter()
        .filter(|alert| alert.level == "warning")
        .count();
    let ok_count = alerts.iter().filter(|alert| alert.level == "ok").count();
    let info_count = alerts.iter().filter(|alert| alert.level == "info").count();
    let top_alert = alerts.first();
    let alert_digest_alert_digest_alert_digest_alert_digest_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.runbook_present);
    let alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present =
        !alerts.is_empty() && alerts.iter().all(|alert| alert.alert_runbook_present);
    let alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present =
        dx_agents_repo_dir()
            .join(
                TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
            )
            .is_file();
    let alert_digest_alert_digest_alert_digest_alert_digest_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook"
                .to_string()
        });
    let alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_target = top_alert
        .map(|alert| redact_audit_review_alert_text(&alert.alert_runbook_target))
        .unwrap_or_else(|| {
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
                .to_string()
        });
    let alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook";
    let runbook_present_count = [
        alert_digest_alert_digest_alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present,
    ]
    .into_iter()
    .filter(|present| *present)
    .count();
    let runbook_count = 3;
    let missing_runbook_count = runbook_count - runbook_present_count;
    let all_runbooks_present = missing_runbook_count == 0;
    let metadata_only = alerts.iter().all(|alert| alert.metadata_only);
    let redacted = alerts.iter().all(|alert| alert.redacted);
    let stores_config_values = alerts.iter().any(|alert| alert.stores_config_values);
    let duplicates_history_rows = alerts.iter().any(|alert| alert.duplicates_history_rows);
    let safe_to_share = alert_count > 0
        && metadata_only
        && redacted
        && !stores_config_values
        && !duplicates_history_rows;
    let state = if alert_count == 0 {
        "empty"
    } else if !safe_to_share {
        "redaction_review"
    } else if !alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
    } else if !alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing"
    } else if !alert_digest_alert_digest_alert_digest_alert_digest_runbook_present {
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
    } else if blocked_count > 0 {
        "blocked"
    } else if warning_count > 0 {
        "warning_review"
    } else if ok_count == alert_count {
        "ready"
    } else {
        "review_required"
    };
    let ready = state == "ready";
    let release_blocking = !ready;
    let severity = match state {
        "ready" => "ok",
        "warning_review" | "review_required" => "warning",
        _ => "blocked",
    };
    let recovery_hint = top_alert
        .and_then(|alert| alert.recovery_hint.as_deref())
        .map(redact_audit_review_alert_text);
    let next_action = match state {
        "ready" => {
            "Release gate digest alert digest alert digest alert digest alerts are clear; keep metadata-only monitoring active."
        }
        "warning_review" => {
            "Review warning release-gate digest alert digest alert digest alert digest alerts before release or promotion continues."
        }
        "blocked" => {
            "Keep release or promotion work paused until blocked release-gate digest alert digest alert digest alert digest alerts clear."
        }
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest alert digest alert runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest alert digest alert digest runbook and rerun release-readiness checks."
        }
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => {
            "Restore the release gate digest alert digest alert digest alert digest runbook and rerun release-readiness checks."
        }
        "redaction_review" => {
            "Regenerate release gate digest alert digest alert digest alert digest alert evidence after fixing metadata-only and redaction flags."
        }
        "empty" => {
            "Run the tool safety audit to produce release gate digest alert digest alert digest alert digest alerts."
        }
        _ => {
            "Review release gate digest alert digest alert digest alert digest alert evidence and refresh audit history."
        }
    };
    let summary = format!(
        "Release gate digest alert digest alert digest alert digest alert digest {state}: {alert_count} alerts, severity {severity}, blocked {blocked_count}, warning {warning_count}, runbooks {runbook_present_count}/{runbook_count} present."
    );

    DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigest {
        schema_version:
            "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest.v1"
                .to_string(),
        generated_at_ms: now_ms(),
        state: state.to_string(),
        ready,
        release_blocking,
        severity: severity.to_string(),
        alert_count,
        blocked_count,
        warning_count,
        ok_count,
        info_count,
        top_alert_id: top_alert.map(|alert| redact_audit_review_alert_text(&alert.id)),
        top_alert_level: top_alert.map(|alert| redact_audit_review_alert_text(&alert.level)),
        alert_digest_alert_digest_alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_alert_digest_alert_digest_runbook_target,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_target,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present,
        alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target:
            alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target
                .to_string(),
        runbook_count,
        runbook_present_count,
        missing_runbook_count,
        all_runbooks_present,
        safe_to_share,
        metadata_only,
        redacted,
        stores_config_values,
        duplicates_history_rows,
        recovery_hint,
        next_action: redact_audit_review_alert_text(next_action),
        summary: redact_audit_review_alert_text(&summary),
    }
}

fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
    digest: &DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigest,
) -> Vec<DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert>{
    let mut alerts = Vec::new();
    let runbook_present =
        digest.alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present;
    let runbook_target = digest
        .alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target
        .clone();
    let alert_runbook_present = dx_agents_repo_dir()
        .join(
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        )
        .is_file();
    let alert_runbook_target = "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook";
    let (id, level, title, detail, recovery_hint) = match digest.state.as_str() {
        "ready" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_ready",
            "ok",
            "Release gate digest alert digest alert digest alert digest alert digest is ready",
            "The release gate digest alert digest alert digest alert digest alert digest is ready, metadata-only, redacted, and backed by required runbooks.",
            None,
        ),
        "warning_review" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_warning_review",
            "warning",
            "Release gate digest alert digest alert digest alert digest alert digest needs warning review",
            "The release gate digest alert digest alert digest alert digest alert digest has warning-level evidence that needs operator review.",
            Some(
                "Review warning release-gate digest alert digest alert digest alert digest alert digest evidence, confirm expected remediation, then rerun the dry-run audit path.",
            ),
        ),
        "blocked" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_blocked",
            "blocked",
            "Release gate digest alert digest alert digest alert digest alert digest is blocked",
            "The release gate digest alert digest alert digest alert digest alert digest is blocking release or promotion work.",
            Some(
                "Keep promotion work paused, resolve blocked release-gate digest alert digest alert digest alert digest alert digest evidence, then rerun the dry-run safety drill.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest alert digest runbook is missing",
            "The upstream release gate digest alert digest alert digest alert digest runbook is required before alert digest alert digest alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest alert digest alert digest runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
            "blocked",
            "Release gate digest alert digest alert digest alert digest alert runbook is missing",
            "The upstream release gate digest alert digest alert digest alert digest alert runbook is required before alert digest alert digest alert digest alert digest evidence is complete.",
            Some(
                "Restore the release gate digest alert digest alert digest alert digest alert runbook, rerun release-readiness checks, and refresh audit history.",
            ),
        ),
        "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing" => {
            (
                "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
                "blocked",
                "Release gate digest alert digest alert digest alert digest alert digest runbook is missing",
                "The release gate digest alert digest alert digest alert digest alert digest runbook is required before alert digest alert digest alert digest alert digest alerts are release-ready.",
                Some(
                    "Restore the release gate digest alert digest alert digest alert digest alert digest runbook and rerun release-readiness checks.",
                ),
            )
        }
        "redaction_review" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review",
            "blocked",
            "Release gate digest alert digest alert digest alert digest alert digest needs redaction",
            "The release gate digest alert digest alert digest alert digest alert digest cannot be treated as shareable metadata-only evidence.",
            Some(
                "Regenerate release-gate digest alert digest alert digest alert digest alert digest evidence after fixing metadata-only and redaction flags.",
            ),
        ),
        "empty" => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_empty",
            "blocked",
            "Release gate digest alert digest alert digest alert digest alert digest is empty",
            "No release gate digest alert digest alert digest alert digest alert evidence is available for release readiness.",
            Some(
                "Run the tool safety audit, refresh audit history, and rerun release-readiness checks.",
            ),
        ),
        _ => (
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_review_required",
            "warning",
            "Release gate digest alert digest alert digest alert digest alert digest needs review",
            "The release gate digest alert digest alert digest alert digest alert digest state needs operator review.",
            Some(
                "Follow the redacted alert digest alert digest alert digest alert digest action and refresh audit history.",
            ),
        ),
    };
    let fallback_hint = if digest.ready {
        None
    } else {
        Some(digest.next_action.as_str())
    };
    alerts.push(
        DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert {
            id: id.to_string(),
            level: level.to_string(),
            title: title.to_string(),
            detail: redact_audit_review_alert_text(detail),
            recovery_hint: recovery_hint
                .or(fallback_hint)
                .map(redact_audit_review_alert_text),
            digest_state: redact_audit_review_alert_text(&digest.state),
            runbook_present,
            runbook_target: redact_audit_review_alert_text(&runbook_target),
            alert_runbook_present,
            alert_runbook_target: alert_runbook_target.to_string(),
            metadata_only: true,
            redacted: digest.redacted,
            stores_config_values: false,
            duplicates_history_rows: false,
        },
    );

    if !alert_runbook_present && !alerts.iter().any(|alert| {
        alert.id
            == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
    }) {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
                    .to_string(),
                level: "blocked".to_string(),
                title:
                    "Release gate digest alert digest alert digest alert digest alert digest alert runbook is missing"
                        .to_string(),
                detail:
                    "The release gate digest alert digest alert digest alert digest alert digest alert runbook is required before alert digest alert digest alert digest alert digest alerts are release-ready."
                        .to_string(),
                recovery_hint: Some(
                    "Restore the release gate digest alert digest alert digest alert digest alert digest alert runbook and rerun release-readiness checks."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    if (!digest.safe_to_share
        || !digest.redacted
        || !digest.metadata_only
        || digest.stores_config_values
        || digest.duplicates_history_rows)
        && !alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review"
        })
    {
        alerts.push(
            DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigestAlertDigestAlertDigestAlert {
                id: "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review"
                    .to_string(),
                level: "blocked".to_string(),
                title:
                    "Release gate digest alert digest alert digest alert digest alert digest needs redaction"
                        .to_string(),
                detail:
                    "The release gate digest alert digest alert digest alert digest alert digest must stay metadata-only, redacted, and must not duplicate raw rows."
                        .to_string(),
                recovery_hint: Some(
                    "Regenerate alert digest alert digest alert digest alert digest evidence after fixing metadata-only and redaction flags."
                        .to_string(),
                ),
                digest_state: redact_audit_review_alert_text(&digest.state),
                runbook_present,
                runbook_target: redact_audit_review_alert_text(&runbook_target),
                alert_runbook_present,
                alert_runbook_target: alert_runbook_target.to_string(),
                metadata_only: true,
                redacted: digest.redacted,
                stores_config_values: false,
                duplicates_history_rows: false,
            },
        );
    }

    alerts.sort_by_key(|alert| alert_level_rank(&alert.level));
    alerts
}

fn redact_audit_review_alert_text(input: &str) -> String {
    redact_local_path_tokens(&redact_sensitive_text(input))
}

fn redact_local_path_tokens(input: &str) -> String {
    input
        .split_whitespace()
        .map(|token| {
            let trimmed = token.trim_matches(|ch: char| {
                matches!(
                    ch,
                    '"' | '\'' | '`' | ',' | ';' | ':' | ')' | ']' | '}' | '(' | '[' | '{'
                )
            });
            if is_local_path_token(trimmed) {
                token.replace(trimmed, "[redacted]")
            } else {
                token.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_local_path_token(token: &str) -> bool {
    let bytes = token.as_bytes();
    (bytes.len() > 2
        && bytes[1] == b':'
        && matches!(bytes[2], b'\\' | b'/')
        && bytes[0].is_ascii_alphabetic())
        || token.starts_with("\\\\")
}

fn alert_level_rank(level: &str) -> usize {
    match level {
        "blocked" => 0,
        "warning" => 1,
        "info" => 2,
        "ok" => 3,
        _ => 4,
    }
}

fn bool_delta(latest: bool, previous: bool) -> i64 {
    latest as i64 - previous as i64
}

fn count_delta(latest: usize, previous: usize) -> i64 {
    latest as i64 - previous as i64
}

fn tool_safety_audit_history_path() -> PathBuf {
    host_telemetry_dir().join(TOOL_SAFETY_AUDIT_HISTORY_FILE)
}

fn tool_safety_audit_history_export_path(timestamp_ms: u64) -> PathBuf {
    host_telemetry_dir().join(format!("tool-safety-audit-history-{timestamp_ms}.json"))
}

fn export_tool_safety_audit_history() -> Result<DxAgentsToolSafetyAuditHistoryExport, String> {
    let exported_at_ms = now_ms();
    let history = read_tool_safety_audit_history(
        &tool_safety_audit_history_path(),
        TOOL_SAFETY_AUDIT_HISTORY_LIMIT,
    )?;
    let redacted = history.entries.iter().all(|entry| {
        entry.redacted && !entry.stores_config_values && !entry.duplicates_history_rows
    });
    let export = DxAgentsToolSafetyAuditHistoryExport {
        exported_at_ms,
        export_path: tool_safety_audit_history_export_path(exported_at_ms)
            .display()
            .to_string(),
        history,
        redacted,
    };
    let path = PathBuf::from(&export.export_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety audit export directory: {error}")
        })?;
    }
    let body = serde_json::to_string_pretty(&export)
        .map_err(|error| format!("failed to serialize tool safety audit export: {error}"))?;
    fs::write(&path, redact_sensitive_text(&body)).map_err(|error| {
        format!(
            "failed to write tool safety audit export {}: {error}",
            path.display()
        )
    })?;
    Ok(export)
}

fn open_tool_safety_audit_history() -> Result<DxAgentsOpenPathResult, String> {
    let path = tool_safety_audit_history_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety audit history directory: {error}")
        })?;
    }
    if !path.is_file() {
        fs::write(&path, "")
            .map_err(|error| format!("failed to initialize {}: {error}", path.display()))?;
    }
    open_path_with_default_app(&path)?;
    Ok(DxAgentsOpenPathResult {
        target: "tool-safety-audit-history".to_string(),
        path: path.display().to_string(),
        opened: true,
    })
}

fn tool_safety_drill_history_path() -> PathBuf {
    host_telemetry_dir().join(TOOL_SAFETY_DRILL_HISTORY_FILE)
}

fn tool_safety_drill_history_export_path(timestamp_ms: u64) -> PathBuf {
    host_telemetry_dir().join(format!("tool-safety-drill-history-{timestamp_ms}.json"))
}

fn export_tool_safety_drill_history() -> Result<DxAgentsToolSafetyDrillHistoryExport, String> {
    let exported_at_ms = now_ms();
    let export = DxAgentsToolSafetyDrillHistoryExport {
        exported_at_ms,
        export_path: tool_safety_drill_history_export_path(exported_at_ms)
            .display()
            .to_string(),
        history: read_tool_safety_drill_history(
            &tool_safety_drill_history_path(),
            TOOL_SAFETY_DRILL_HISTORY_LIMIT,
        )?,
        redacted: true,
    };
    let path = PathBuf::from(&export.export_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety drill export directory: {error}")
        })?;
    }
    let body = serde_json::to_string_pretty(&export)
        .map_err(|error| format!("failed to serialize tool safety drill export: {error}"))?;
    fs::write(&path, redact_sensitive_text(&body)).map_err(|error| {
        format!(
            "failed to write tool safety drill export {}: {error}",
            path.display()
        )
    })?;
    Ok(export)
}

fn open_tool_safety_drill_history() -> Result<DxAgentsOpenPathResult, String> {
    let path = tool_safety_drill_history_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!("failed to create tool safety drill history directory: {error}")
        })?;
    }
    if !path.is_file() {
        fs::write(&path, "")
            .map_err(|error| format!("failed to initialize {}: {error}", path.display()))?;
    }
    open_path_with_default_app(&path)?;
    Ok(DxAgentsOpenPathResult {
        target: "tool-safety-drill-history".to_string(),
        path: path.display().to_string(),
        opened: true,
    })
}

fn dashboard_compatibility_status(
    contract: &serde_json::Value,
    repo_dir: &Path,
) -> DxAgentsDashboardCompatibilityStatus {
    let compatibility = contract
        .get("dashboard_compatibility")
        .cloned()
        .unwrap_or_else(dashboard_compatibility_contract);
    let contract_schema_version = json_str(&compatibility, "schema_version").map(str::to_string);
    let product_name = json_str(&compatibility, "product_name").map(str::to_string);
    let package_name = json_str(&compatibility, "package_name").map(str::to_string);
    let source = json_str(&compatibility, "source")
        .unwrap_or("bridge_fallback")
        .to_string();
    let env_alias_count = dashboard_array_len(&compatibility, "env_aliases");
    let window_global_count = dashboard_array_len(&compatibility, "window_globals");
    let storage_alias_count = dashboard_array_len(&compatibility, "storage_aliases");
    let event_alias_count = dashboard_array_len(&compatibility, "event_aliases");
    let websocket_protocol_count = dashboard_array_len(&compatibility, "websocket_protocols");
    let alias_counts = [
        env_alias_count,
        window_global_count,
        storage_alias_count,
        event_alias_count,
        websocket_protocol_count,
    ];
    let alias_category_count = alias_counts.iter().filter(|count| **count > 0).count();
    let alias_count = alias_counts.iter().sum();
    let policy = compatibility
        .get("compatibility_policy")
        .unwrap_or(&serde_json::Value::Null);
    let legacy_readable = json_bool(policy, "legacy_readable", false);
    let legacy_writable = json_bool(policy, "legacy_writable", false);
    let exposes_stored_values = json_bool(policy, "exposes_stored_values", true);
    let migration_plan_path = repo_dir.join(DASHBOARD_COMPATIBILITY_MIGRATION_PLAN);
    let cleanup_gate_ready = dashboard_compatibility_migration_plan_ready(&migration_plan_path);
    let usage_telemetry = dashboard_compatibility_usage_status(repo_dir);
    let decommission_ready = cleanup_gate_ready && usage_telemetry.decommission_ready;
    let mut drift_checks = Vec::new();

    drift_checks.push(dashboard_compatibility_drift(
        "schema",
        "Source contract schema",
        if contract_schema_version.as_deref() == Some("dx.dashboard_compatibility.v1") {
            "ok"
        } else {
            "warn"
        },
        format!(
            "expected dx.dashboard_compatibility.v1, got {}",
            contract_schema_version.as_deref().unwrap_or("missing")
        ),
    ));
    drift_checks.push(dashboard_compatibility_drift(
        "policy",
        "Compatibility policy",
        if legacy_readable && legacy_writable && !exposes_stored_values {
            "ok"
        } else {
            "warn"
        },
        format!(
            "legacy readable {legacy_readable}, legacy writable {legacy_writable}, exposes stored values {exposes_stored_values}"
        ),
    ));
    drift_checks.push(dashboard_compatibility_drift(
        "categories",
        "Alias categories",
        if alias_category_count == 5 && alias_count >= 10 {
            "ok"
        } else {
            "warn"
        },
        format!("{alias_category_count}/5 categories, {alias_count} alias rows"),
    ));

    match dashboard_compatibility_source_text(repo_dir) {
        Ok(source_text) => {
            let missing_tokens = dashboard_compatibility_tokens(&compatibility)
                .into_iter()
                .filter(|token| !source_text.contains(token))
                .collect::<Vec<_>>();
            drift_checks.push(dashboard_compatibility_drift(
                "alias_tokens",
                "Dashboard and gateway source tokens",
                if missing_tokens.is_empty() {
                    "ok"
                } else {
                    "warn"
                },
                if missing_tokens.is_empty() {
                    "All primary and legacy alias tokens appear in dashboard or gateway source."
                        .to_string()
                } else {
                    format!("missing {}", missing_tokens.join(", "))
                },
            ));

            let missing_usage_tokens = dashboard_compatibility_usage_tokens()
                .into_iter()
                .filter(|token| !source_text.contains(token))
                .collect::<Vec<_>>();
            drift_checks.push(dashboard_compatibility_drift(
                "usage_telemetry_instrumentation",
                "Dashboard compatibility usage telemetry",
                if missing_usage_tokens.is_empty() {
                    "ok"
                } else {
                    "warn"
                },
                if missing_usage_tokens.is_empty() {
                    "Dashboard source records redacted legacy alias reads, writes, removes, and migrations."
                        .to_string()
                } else {
                    format!("missing {}", missing_usage_tokens.join(", "))
                },
            ));
        }
        Err(error) => drift_checks.push(dashboard_compatibility_drift(
            "alias_tokens",
            "Dashboard and gateway source tokens",
            "warn",
            error,
        )),
    }

    drift_checks.push(dashboard_compatibility_drift(
        "stored_values",
        "Stored value exposure",
        if !exposes_stored_values { "ok" } else { "warn" },
        "Status export includes alias names and presence policy only; browser storage values are not read.",
    ));
    drift_checks.push(dashboard_compatibility_drift(
        "migration_plan",
        "Legacy alias migration gate",
        if cleanup_gate_ready { "ok" } else { "warn" },
        format!(
            "migration plan {}",
            if cleanup_gate_ready {
                "is present and versioned"
            } else {
                "is missing or incomplete"
            }
        ),
    ));

    let ready = drift_checks.iter().all(|check| check.status == "ok");

    DxAgentsDashboardCompatibilityStatus {
        schema_version: "dx.dashboard_compatibility_status.v1".to_string(),
        generated_at_ms: now_ms(),
        ready,
        source,
        contract_schema_version,
        product_name,
        package_name,
        alias_category_count,
        alias_count,
        env_alias_count,
        window_global_count,
        storage_alias_count,
        event_alias_count,
        websocket_protocol_count,
        legacy_readable,
        legacy_writable,
        exposes_stored_values,
        cleanup_gate_ready,
        decommission_ready,
        migration_plan_path: migration_plan_path.display().to_string(),
        usage_telemetry,
        drift_checks,
        next_action: if !ready {
            "repair_dashboard_compatibility_drift".to_string()
        } else if decommission_ready {
            "dashboard_legacy_alias_decommission_ready".to_string()
        } else {
            "collect_dashboard_compatibility_usage_telemetry".to_string()
        },
    }
}

fn unavailable_dashboard_compatibility_status(error: &str) -> DxAgentsDashboardCompatibilityStatus {
    DxAgentsDashboardCompatibilityStatus {
        schema_version: "dx.dashboard_compatibility_status.v1".to_string(),
        generated_at_ms: now_ms(),
        ready: false,
        source: "unavailable".to_string(),
        contract_schema_version: None,
        product_name: None,
        package_name: None,
        alias_category_count: 0,
        alias_count: 0,
        env_alias_count: 0,
        window_global_count: 0,
        storage_alias_count: 0,
        event_alias_count: 0,
        websocket_protocol_count: 0,
        legacy_readable: false,
        legacy_writable: false,
        exposes_stored_values: false,
        cleanup_gate_ready: false,
        decommission_ready: false,
        migration_plan_path: dx_agents_repo_dir()
            .join(DASHBOARD_COMPATIBILITY_MIGRATION_PLAN)
            .display()
            .to_string(),
        usage_telemetry: unavailable_dashboard_compatibility_usage_status(error),
        drift_checks: vec![dashboard_compatibility_drift(
            "contract_load",
            "Dashboard compatibility contract load",
            "warn",
            error.to_string(),
        )],
        next_action: "repair_dashboard_compatibility_contract_load".to_string(),
    }
}

fn dashboard_compatibility_drift(
    id: impl Into<String>,
    label: impl Into<String>,
    status: impl Into<String>,
    detail: impl Into<String>,
) -> DxAgentsDashboardCompatibilityDrift {
    DxAgentsDashboardCompatibilityDrift {
        id: id.into(),
        label: label.into(),
        status: status.into(),
        detail: detail.into(),
    }
}

fn dashboard_array_len(value: &serde_json::Value, key: &str) -> usize {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(Vec::len)
        .unwrap_or_default()
}

fn dashboard_compatibility_tokens(compatibility: &serde_json::Value) -> Vec<String> {
    let mut tokens = HashSet::new();
    for category in [
        "env_aliases",
        "window_globals",
        "storage_aliases",
        "event_aliases",
        "websocket_protocols",
    ] {
        if let Some(items) = compatibility
            .get(category)
            .and_then(serde_json::Value::as_array)
        {
            for item in items {
                if let Some(primary) = json_str(item, "primary") {
                    tokens.insert(primary.to_string());
                }
                for token in json_string_values(item.get("legacy")) {
                    tokens.insert(token);
                }
            }
        }
    }
    let mut tokens = tokens.into_iter().collect::<Vec<_>>();
    tokens.sort();
    tokens
}

fn dashboard_compatibility_usage_tokens() -> Vec<&'static str> {
    vec![
        "DASHBOARD_COMPATIBILITY_USAGE_KEY",
        "dx_agents_dashboard_compatibility_usage_v1",
        "getDashboardCompatibilityUsageTelemetry",
        "legacy_read_count",
        "legacy_write_count",
        "legacy_remove_count",
        "migration_count",
    ]
}

fn dashboard_compatibility_usage_status(
    repo_dir: &Path,
) -> DxAgentsDashboardCompatibilityUsageStatus {
    let telemetry_path = dashboard_compatibility_usage_path(repo_dir);
    let telemetry_path_display = telemetry_path.display().to_string();
    let Ok(source) = fs::read_to_string(&telemetry_path) else {
        return DxAgentsDashboardCompatibilityUsageStatus {
            schema_version: "dx.dashboard_compatibility_usage_status.v1".to_string(),
            supported: true,
            state: "missing_telemetry".to_string(),
            telemetry_path: telemetry_path_display,
            browser_storage_key: DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY.to_string(),
            primary_usage_count: 0,
            legacy_usage_count: 0,
            legacy_read_count: 0,
            legacy_write_count: 0,
            legacy_remove_count: 0,
            migration_count: 0,
            decommission_ready: false,
            detail: "No exported browser compatibility telemetry was found, so legacy alias removal remains blocked.".to_string(),
        };
    };

    let parsed = match serde_json::from_str::<serde_json::Value>(&source) {
        Ok(parsed) => parsed,
        Err(error) => {
            return DxAgentsDashboardCompatibilityUsageStatus {
                schema_version: "dx.dashboard_compatibility_usage_status.v1".to_string(),
                supported: true,
                state: "invalid_telemetry".to_string(),
                telemetry_path: telemetry_path_display,
                browser_storage_key: DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY.to_string(),
                primary_usage_count: 0,
                legacy_usage_count: 0,
                legacy_read_count: 0,
                legacy_write_count: 0,
                legacy_remove_count: 0,
                migration_count: 0,
                decommission_ready: false,
                detail: format!("Telemetry JSON could not be parsed: {error}"),
            };
        }
    };

    if json_str(&parsed, "schema_version") != Some("dx.dashboard_compatibility_usage.v1") {
        return DxAgentsDashboardCompatibilityUsageStatus {
            schema_version: "dx.dashboard_compatibility_usage_status.v1".to_string(),
            supported: true,
            state: "invalid_telemetry".to_string(),
            telemetry_path: telemetry_path_display,
            browser_storage_key: DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY.to_string(),
            primary_usage_count: 0,
            legacy_usage_count: 0,
            legacy_read_count: 0,
            legacy_write_count: 0,
            legacy_remove_count: 0,
            migration_count: 0,
            decommission_ready: false,
            detail: format!(
                "Expected dx.dashboard_compatibility_usage.v1 telemetry, got {}.",
                json_str(&parsed, "schema_version").unwrap_or("missing")
            ),
        };
    }

    let counters = parsed
        .get("counters")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let counter_primary_usage_count = counters
        .iter()
        .map(|counter| {
            json_u64(counter.get("primary_read_count")).unwrap_or_default()
                + json_u64(counter.get("primary_write_count")).unwrap_or_default()
                + json_u64(counter.get("primary_remove_count")).unwrap_or_default()
        })
        .sum::<u64>();
    let counter_legacy_read_count = counters
        .iter()
        .map(|counter| json_u64(counter.get("legacy_read_count")).unwrap_or_default())
        .sum::<u64>();
    let counter_legacy_write_count = counters
        .iter()
        .map(|counter| json_u64(counter.get("legacy_write_count")).unwrap_or_default())
        .sum::<u64>();
    let counter_legacy_remove_count = counters
        .iter()
        .map(|counter| json_u64(counter.get("legacy_remove_count")).unwrap_or_default())
        .sum::<u64>();
    let counter_migration_count = counters
        .iter()
        .map(|counter| json_u64(counter.get("migration_count")).unwrap_or_default())
        .sum::<u64>();

    let primary_usage_count = json_u64(parsed.get("primary_usage_count"))
        .unwrap_or_default()
        .max(counter_primary_usage_count);
    let legacy_read_count = json_u64(parsed.get("legacy_read_count"))
        .unwrap_or_default()
        .max(counter_legacy_read_count);
    let legacy_write_count = json_u64(parsed.get("legacy_write_count"))
        .unwrap_or_default()
        .max(counter_legacy_write_count);
    let legacy_remove_count = json_u64(parsed.get("legacy_remove_count"))
        .unwrap_or_default()
        .max(counter_legacy_remove_count);
    let migration_count = json_u64(parsed.get("migration_count"))
        .unwrap_or_default()
        .max(counter_migration_count);
    let derived_legacy_usage_count = legacy_read_count + legacy_write_count + legacy_remove_count;
    let legacy_usage_count = json_u64(parsed.get("legacy_usage_count"))
        .unwrap_or_default()
        .max(derived_legacy_usage_count);
    let decommission_ready = legacy_usage_count == 0 && migration_count == 0;

    DxAgentsDashboardCompatibilityUsageStatus {
        schema_version: "dx.dashboard_compatibility_usage_status.v1".to_string(),
        supported: true,
        state: if decommission_ready {
            "zero_legacy_usage".to_string()
        } else {
            "legacy_usage_observed".to_string()
        },
        telemetry_path: telemetry_path_display,
        browser_storage_key: DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY.to_string(),
        primary_usage_count,
        legacy_usage_count,
        legacy_read_count,
        legacy_write_count,
        legacy_remove_count,
        migration_count,
        decommission_ready,
        detail: if decommission_ready {
            "Exported telemetry shows zero legacy alias usage; cleanup may proceed after the migration gate.".to_string()
        } else {
            "Exported telemetry still shows legacy alias activity; keep compatibility aliases active.".to_string()
        },
    }
}

fn unavailable_dashboard_compatibility_usage_status(
    error: &str,
) -> DxAgentsDashboardCompatibilityUsageStatus {
    DxAgentsDashboardCompatibilityUsageStatus {
        schema_version: "dx.dashboard_compatibility_usage_status.v1".to_string(),
        supported: false,
        state: "unavailable".to_string(),
        telemetry_path: dashboard_compatibility_usage_path(&dx_agents_repo_dir())
            .display()
            .to_string(),
        browser_storage_key: DASHBOARD_COMPATIBILITY_USAGE_STORAGE_KEY.to_string(),
        primary_usage_count: 0,
        legacy_usage_count: 0,
        legacy_read_count: 0,
        legacy_write_count: 0,
        legacy_remove_count: 0,
        migration_count: 0,
        decommission_ready: false,
        detail: format!("Dashboard compatibility status is unavailable: {error}"),
    }
}

fn dashboard_compatibility_usage_path(repo_dir: &Path) -> PathBuf {
    repo_dir
        .join("target")
        .join("host-telemetry")
        .join(DASHBOARD_COMPATIBILITY_USAGE_FILE)
}

fn dashboard_compatibility_source_text(repo_dir: &Path) -> Result<String, String> {
    let mut source = String::new();
    append_dashboard_source_file(&repo_dir.join("web").join("package.json"), &mut source)?;
    append_dashboard_source_file(&repo_dir.join("web").join("vite.config.ts"), &mut source)?;
    append_dashboard_source_file(&repo_dir.join("web").join("index.html"), &mut source)?;
    append_dashboard_source_files_under(&repo_dir.join("web").join("src"), &mut source)?;
    append_dashboard_source_files_under(
        &repo_dir.join("crates").join("dx-agent-gateway").join("src"),
        &mut source,
    )?;
    Ok(source)
}

fn dashboard_compatibility_migration_plan_ready(path: &Path) -> bool {
    let Ok(source) = fs::read_to_string(path) else {
        return false;
    };
    source.contains("Migration gate")
        && source.contains("Version: 1")
        && source.contains("Legacy aliases remain active")
}

fn append_dashboard_source_file(path: &Path, output: &mut String) -> Result<(), String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    output.push_str(&source);
    output.push('\n');
    Ok(())
}

fn append_dashboard_source_files_under(path: &Path, output: &mut String) -> Result<(), String> {
    let entries = fs::read_dir(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    for entry in entries {
        let entry =
            entry.map_err(|error| format!("failed to read {} entry: {error}", path.display()))?;
        let child = entry.path();
        if child.is_dir() {
            append_dashboard_source_files_under(&child, output)?;
        } else if dashboard_compatibility_source_file(&child) {
            append_dashboard_source_file(&child, output)?;
        }
    }
    Ok(())
}

fn dashboard_compatibility_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("rs" | "ts" | "tsx" | "js" | "jsx" | "json" | "html")
    )
}

fn host_telemetry_dir() -> PathBuf {
    dx_agents_repo_dir().join("target").join("host-telemetry")
}

fn native_promotion_archive_status() -> serde_json::Value {
    match run_json_action("dx.native_promotion_archive".to_string(), None) {
        Ok(value) => value,
        Err(error) => serde_json::json!({
            "schema_version": "dx.embedded_terminal_native_promotion_archive.v1",
            "available": false,
            "diagnostic_only": true,
            "error": redact_sensitive_text(&error),
            "status": null,
        }),
    }
}

fn export_bridge_status() -> Result<DxAgentsBridgeStatusExport, String> {
    let exported_at_ms = now_ms();
    let contract_value_result = load_contract_value_from_path(&contract_path(&dx_cli_root()));
    let tool_safety_drill = run_tool_safety_drill_dashboard_command(
        "Tool configuration safety drill",
        Some("dry-run"),
        "bridge_status_export",
    )?;
    let tool_safety_history = read_tool_safety_drill_history(
        &tool_safety_drill_history_path(),
        TOOL_SAFETY_DRILL_HISTORY_LIMIT,
    )?;
    let tool_safety_audit = tool_safety_audit_summary(
        &tool_safety_history,
        dx_agents_repo_dir()
            .join(TOOL_SAFETY_ALERT_RUNBOOK)
            .is_file(),
    );
    let _ = append_tool_safety_audit_record(
        &tool_safety_audit_history_path(),
        &tool_safety_audit,
        TOOL_SAFETY_AUDIT_HISTORY_LIMIT,
    );
    let tool_safety_audit_history = read_tool_safety_audit_history(
        &tool_safety_audit_history_path(),
        TOOL_SAFETY_AUDIT_HISTORY_LIMIT,
    )?;
    let tool_safety_audit_digest = tool_safety_audit_history.digest.clone();
    let export = DxAgentsBridgeStatusExport {
        exported_at_ms,
        export_path: bridge_status_export_path(exported_at_ms)
            .display()
            .to_string(),
        self_test: dx_cli_bridge_self_test(),
        command_history: load_contract()
            .and_then(|contract| read_command_history(&contract, 20))
            .unwrap_or_else(|error| DxCliCommandHistory {
                history_path: format!("unavailable: {error}"),
                count: 0,
                entries: Vec::new(),
            }),
        provider_health: run_dx_agents_dashboard_command(
            "Provider health",
            &dx_agents_provider_health_args(Some("dry-run"), None)
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        provider_failover_drill: run_dx_agents_dashboard_command(
            "Provider failover drill",
            &dx_agents_provider_failover_drill_args(Some("dry-run"))
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        gateway_pairing_drill: run_dx_agents_dashboard_command(
            "Gateway pairing allowlist drill",
            &dx_agents_gateway_pairing_drill_args(Some("dry-run"))
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        provider_smoke_history: read_provider_smoke_history(&provider_smoke_history_path(), 20)?,
        compact_status: run_dx_agents_dashboard_command(
            "Compact status",
            &dx_agents_compact_status_args()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        cron_preview: run_dx_agents_dashboard_command(
            "Cron delivery preview",
            &dx_agents_cron_preview_args(Some(10))
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        cron_history: run_dx_agents_dashboard_command(
            "Cron run history",
            &dx_agents_cron_history_args(Some(5))
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        cron_delivery_drill: run_dx_agents_dashboard_command(
            "Cron delivery recovery drill",
            &dx_agents_cron_delivery_drill_args(Some("dry-run"))
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        tool_safety_drill,
        tool_safety_history,
        tool_safety_audit,
        tool_safety_audit_digest,
        tool_safety_audit_history,
        continuation_status: run_dx_agents_dashboard_command(
            "Continuation journal",
            &["workloop", "status", "--limit", "5", "--json"],
        )
        .map(redact_dashboard_command)?,
        session_tool_routing: run_dx_agents_dashboard_command(
            "Session tool routing",
            &dx_agents_session_tool_routing_args()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        memory_skill_learning: run_dx_agents_dashboard_command(
            "Memory skill learning loop",
            &dx_agents_memory_skill_learning_args()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map(redact_dashboard_command)?,
        native_promotion_status: native_promotion_archive_status(),
        native_promotion_archive_diff: native_promotion_archive_diff()?,
        dashboard_compatibility: contract_value_result
            .as_ref()
            .map(|contract| dashboard_compatibility_status(contract, &dx_agents_repo_dir()))
            .unwrap_or_else(|error| unavailable_dashboard_compatibility_status(error)),
        media_canary_evidence: contract_value_result
            .as_ref()
            .map(|contract| embedded_terminal_media_canary_evidence(contract))
            .unwrap_or_else(|_| embedded_terminal_media_canary_evidence(&serde_json::json!({}))),
    };
    let path = PathBuf::from(&export.export_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create bridge export directory: {error}"))?;
    }
    let body = serde_json::to_string_pretty(&export)
        .map_err(|error| format!("failed to serialize bridge status export: {error}"))?;
    fs::write(&path, redact_sensitive_text(&body))
        .map_err(|error| format!("failed to write bridge export {}: {error}", path.display()))?;
    Ok(export)
}

fn bridge_status_export_path(timestamp_ms: u64) -> PathBuf {
    host_telemetry_dir().join(format!("bridge-status-export-{timestamp_ms}.json"))
}

fn list_bridge_status_exports(limit: usize) -> Result<DxAgentsBridgeStatusExportList, String> {
    let export_dir = host_telemetry_dir();
    if !export_dir.is_dir() {
        return Ok(DxAgentsBridgeStatusExportList {
            export_dir: export_dir.display().to_string(),
            count: 0,
            entries: Vec::new(),
        });
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&export_dir)
        .map_err(|error| format!("failed to read bridge export directory: {error}"))?
    {
        let entry =
            entry.map_err(|error| format!("failed to read bridge export entry: {error}"))?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if !is_bridge_status_export_file_name(file_name) {
            continue;
        }
        let metadata = entry
            .metadata()
            .map_err(|error| format!("failed to read bridge export metadata: {error}"))?;
        entries.push(DxAgentsBridgeStatusExportEntry {
            file_name: file_name.to_string(),
            path: path.display().to_string(),
            modified_at_ms: metadata
                .modified()
                .ok()
                .map(system_time_ms)
                .unwrap_or_default(),
            size_bytes: metadata.len(),
        });
    }

    entries.sort_by_key(|entry| std::cmp::Reverse(entry.modified_at_ms));
    entries.truncate(limit);

    Ok(DxAgentsBridgeStatusExportList {
        export_dir: export_dir.display().to_string(),
        count: entries.len(),
        entries,
    })
}

fn open_bridge_status_export(file_name: &str) -> Result<DxAgentsOpenPathResult, String> {
    let path = bridge_status_export_file_path(file_name)?;
    if !path.is_file() {
        return Err(format!(
            "Bridge status export does not exist: {}",
            path.display()
        ));
    }
    open_path_with_default_app(&path)?;
    Ok(DxAgentsOpenPathResult {
        target: "bridge-status-export".to_string(),
        path: path.display().to_string(),
        opened: true,
    })
}

fn bridge_status_export_file_path(file_name: &str) -> Result<PathBuf, String> {
    let file_name = file_name.trim();
    if !is_bridge_status_export_file_name(file_name) {
        return Err("Expected a bridge-status-export-*.json file name.".to_string());
    }
    let path = Path::new(file_name);
    if path.components().count() != 1 {
        return Err("Bridge status export must be opened by file name, not path.".to_string());
    }
    Ok(host_telemetry_dir().join(file_name))
}

fn is_bridge_status_export_file_name(file_name: &str) -> bool {
    file_name.starts_with("bridge-status-export-") && file_name.ends_with(".json")
}

fn native_promotion_archive_dir() -> PathBuf {
    dx_cli_root().join("target").join("native-promotion")
}

fn list_native_promotion_archives(limit: usize) -> Result<DxCliNativePromotionArchiveList, String> {
    let archive_dir = native_promotion_archive_dir();
    if !archive_dir.is_dir() {
        return Ok(DxCliNativePromotionArchiveList {
            archive_dir: archive_dir.display().to_string(),
            count: 0,
            entries: Vec::new(),
        });
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&archive_dir)
        .map_err(|error| format!("failed to read {}: {error}", archive_dir.display()))?
    {
        let entry =
            entry.map_err(|error| format!("failed to read archive directory entry: {error}"))?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if !is_native_promotion_archive_file_name(file_name) {
            continue;
        }
        if let Some(summary) = native_promotion_archive_entry(&path)? {
            entries.push(summary);
        }
    }

    entries.sort_by(|left, right| right.file_name.cmp(&left.file_name));
    entries.truncate(limit);
    Ok(DxCliNativePromotionArchiveList {
        archive_dir: archive_dir.display().to_string(),
        count: entries.len(),
        entries,
    })
}

fn native_promotion_archive_diff() -> Result<DxCliNativePromotionArchiveDiffSummary, String> {
    let list = list_native_promotion_archives(50)?;
    Ok(native_promotion_archive_diff_from_entries(
        list.archive_dir,
        &list.entries,
    ))
}

fn native_promotion_archive_diff_from_entries(
    archive_dir: String,
    entries: &[DxCliNativePromotionArchiveEntry],
) -> DxCliNativePromotionArchiveDiffSummary {
    let snapshot_count = entries.len();
    let trend_history = native_promotion_archive_trend_history(entries);
    let runbook = native_promotion_archive_trend_runbook();
    let Some(latest) = entries.first() else {
        return DxCliNativePromotionArchiveDiffSummary {
            archive_dir,
            available: false,
            snapshot_count,
            latest_file_name: None,
            previous_file_name: None,
            blocker_delta: 0,
            retention_delta: 0,
            production_ready_changed: false,
            next_surface_changed: false,
            rollback_changed: false,
            redacted: true,
            diagnostic_only: true,
            alert_level: "clean".to_string(),
            summary: "No retained native promotion snapshots yet.".to_string(),
            recovery_hint: Some(
                "Run Export diagnostics to create a redacted native promotion snapshot."
                    .to_string(),
            ),
            changes: Vec::new(),
            alerts: Vec::new(),
            trend_history,
            runbook,
        };
    };

    let Some(previous) = entries.get(1) else {
        return DxCliNativePromotionArchiveDiffSummary {
            archive_dir,
            available: true,
            snapshot_count,
            latest_file_name: Some(latest.file_name.clone()),
            previous_file_name: None,
            blocker_delta: 0,
            retention_delta: 0,
            production_ready_changed: false,
            next_surface_changed: false,
            rollback_changed: false,
            redacted: latest.redacted,
            diagnostic_only: latest.diagnostic_only,
            alert_level: if latest.redacted && latest.diagnostic_only {
                "clean"
            } else {
                "blocked"
            }
            .to_string(),
            summary:
                "Only one native promotion snapshot is retained; diff needs at least two snapshots."
                    .to_string(),
            recovery_hint: Some(
                "Run Export diagnostics again after the next native promotion status change."
                    .to_string(),
            ),
            changes: Vec::new(),
            alerts: native_promotion_archive_drift_alerts(latest, latest, 0),
            trend_history,
            runbook,
        };
    };

    let blocker_delta = latest.blocker_count as i64 - previous.blocker_count as i64;
    let retention_delta = latest.retained_count as i64 - previous.retained_count as i64;
    let production_ready_changed = latest.production_ready != previous.production_ready;
    let next_surface_changed = latest.next_surface != previous.next_surface;
    let rollback_changed = latest.rollback_summary != previous.rollback_summary;
    let redacted = latest.redacted && previous.redacted;
    let diagnostic_only = latest.diagnostic_only && previous.diagnostic_only;
    let alerts = native_promotion_archive_drift_alerts(latest, previous, blocker_delta);
    let alert_level = native_promotion_archive_alert_level(&alerts).to_string();
    let changes = vec![
        native_promotion_diff_change(
            "blockers",
            "Blockers",
            previous.blocker_count.to_string(),
            latest.blocker_count.to_string(),
            Some(blocker_delta),
        ),
        native_promotion_diff_change(
            "retention",
            "Retention",
            format!("{}/{}", previous.retained_count, previous.retention_limit),
            format!("{}/{}", latest.retained_count, latest.retention_limit),
            Some(retention_delta),
        ),
        native_promotion_diff_change(
            "production_ready",
            "Production ready",
            yes_no(previous.production_ready),
            yes_no(latest.production_ready),
            None,
        ),
        native_promotion_diff_change(
            "next_surface",
            "Next surface",
            optional_text(previous.next_surface.as_deref()),
            optional_text(latest.next_surface.as_deref()),
            None,
        ),
        native_promotion_diff_change(
            "rollback",
            "Rollback",
            previous.rollback_summary.clone(),
            latest.rollback_summary.clone(),
            None,
        ),
    ];
    let changed_labels = changes
        .iter()
        .filter(|change| change.changed)
        .map(|change| change.label.as_str())
        .collect::<Vec<_>>();
    let summary = if changed_labels.is_empty() {
        "Latest native promotion snapshot matches the previous snapshot across blockers, retention, production state, next surface, and rollback.".to_string()
    } else {
        format!(
            "Latest native promotion snapshot changed {}.",
            changed_labels.join(", ")
        )
    };

    DxCliNativePromotionArchiveDiffSummary {
        archive_dir,
        available: true,
        snapshot_count,
        latest_file_name: Some(latest.file_name.clone()),
        previous_file_name: Some(previous.file_name.clone()),
        blocker_delta,
        retention_delta,
        production_ready_changed,
        next_surface_changed,
        rollback_changed,
        redacted,
        diagnostic_only,
        alert_level,
        summary,
        recovery_hint: None,
        changes,
        alerts,
        trend_history,
        runbook,
    }
}

fn native_promotion_archive_trend_history(
    entries: &[DxCliNativePromotionArchiveEntry],
) -> DxCliNativePromotionArchiveTrendHistory {
    let sample_count = entries.len();
    let points = entries
        .iter()
        .map(|entry| DxCliNativePromotionArchiveTrendPoint {
            file_name: entry.file_name.clone(),
            blocker_count: entry.blocker_count,
            alert_level: native_promotion_archive_single_snapshot_alert_level(entry).to_string(),
            production_ready: entry.production_ready,
            redacted: entry.redacted,
            diagnostic_only: entry.diagnostic_only,
            next_surface: entry.next_surface.clone(),
        })
        .collect::<Vec<_>>();
    let Some(latest) = entries.first() else {
        return DxCliNativePromotionArchiveTrendHistory {
            available: false,
            trend: "short".to_string(),
            sample_count,
            latest_file_name: None,
            oldest_file_name: None,
            latest_blocker_count: None,
            oldest_blocker_count: None,
            blocker_delta_total: 0,
            warning_alert_count: 0,
            blocked_alert_count: 0,
            redacted: true,
            diagnostic_only: true,
            summary: "No retained native promotion snapshots are available for trend history."
                .to_string(),
            recovery_hint: Some(
                "Run Export diagnostics multiple times to build a metadata-only trend history."
                    .to_string(),
            ),
            points,
        };
    };
    let oldest = entries.last().unwrap_or(latest);
    let blocker_delta_total = latest.blocker_count as i64 - oldest.blocker_count as i64;
    let mut warning_alert_count = 0;
    let mut blocked_alert_count = 0;
    for pair in entries.windows(2) {
        let pair_latest = &pair[0];
        let pair_previous = &pair[1];
        let pair_delta = pair_latest.blocker_count as i64 - pair_previous.blocker_count as i64;
        for alert in native_promotion_archive_drift_alerts(pair_latest, pair_previous, pair_delta) {
            match alert.severity.as_str() {
                "blocked" => blocked_alert_count += 1,
                "warning" => warning_alert_count += 1,
                _ => {}
            }
        }
    }
    let redacted = entries.iter().all(|entry| entry.redacted);
    let diagnostic_only = entries.iter().all(|entry| entry.diagnostic_only);
    let available = sample_count >= 2;
    let trend = native_promotion_archive_trend_label(
        available,
        blocker_delta_total,
        warning_alert_count,
        blocked_alert_count,
    )
    .to_string();
    let summary = native_promotion_archive_trend_summary(
        &trend,
        blocker_delta_total,
        warning_alert_count,
        blocked_alert_count,
        sample_count,
    );
    let recovery_hint = if available {
        match trend.as_str() {
            "worsening" => Some(
                "Keep production embedded routing disabled and inspect the blocked drift alerts before continuing promotion."
                    .to_string(),
            ),
            "stable" if warning_alert_count > 0 => Some(
                "Review warning drift before treating the archive history as stable.".to_string(),
            ),
            _ => None,
        }
    } else {
        Some("Retain at least two archive snapshots to classify trend history.".to_string())
    };

    DxCliNativePromotionArchiveTrendHistory {
        available,
        trend,
        sample_count,
        latest_file_name: Some(latest.file_name.clone()),
        oldest_file_name: Some(oldest.file_name.clone()),
        latest_blocker_count: Some(latest.blocker_count),
        oldest_blocker_count: Some(oldest.blocker_count),
        blocker_delta_total,
        warning_alert_count,
        blocked_alert_count,
        redacted,
        diagnostic_only,
        summary,
        recovery_hint,
        points,
    }
}

fn native_promotion_archive_single_snapshot_alert_level(
    entry: &DxCliNativePromotionArchiveEntry,
) -> &'static str {
    if !entry.redacted || !entry.diagnostic_only {
        "blocked"
    } else if entry.production_ready {
        "warning"
    } else {
        "clean"
    }
}

fn native_promotion_archive_trend_label(
    available: bool,
    blocker_delta_total: i64,
    warning_alert_count: usize,
    blocked_alert_count: usize,
) -> &'static str {
    if !available {
        "short"
    } else if blocked_alert_count > 0 || blocker_delta_total > 0 {
        "worsening"
    } else if blocker_delta_total < 0 {
        "improving"
    } else if warning_alert_count > 0 {
        "stable"
    } else {
        "stable"
    }
}

fn native_promotion_archive_trend_summary(
    trend: &str,
    blocker_delta_total: i64,
    warning_alert_count: usize,
    blocked_alert_count: usize,
    sample_count: usize,
) -> String {
    match trend {
        "short" => format!("Need at least two snapshots for trend history; found {sample_count}."),
        "improving" => format!(
            "Native promotion blocker trend is improving by {} blockers across {sample_count} retained snapshots.",
            blocker_delta_total.abs()
        ),
        "worsening" => format!(
            "Native promotion archive trend is worsening: blocker delta {blocker_delta_total}, blocked alerts {blocked_alert_count}, warning alerts {warning_alert_count}."
        ),
        _ => format!(
            "Native promotion archive trend is stable across {sample_count} retained snapshots with blocker delta {blocker_delta_total}, blocked alerts {blocked_alert_count}, and warning alerts {warning_alert_count}."
        ),
    }
}

fn native_promotion_archive_trend_runbook() -> DxCliNativePromotionArchiveTrendRunbook {
    DxCliNativePromotionArchiveTrendRunbook {
        path: NATIVE_PROMOTION_ARCHIVE_TREND_RUNBOOK.to_string(),
        title: "Native promotion archive trend runbook".to_string(),
        diagnostic_only: true,
        external_fallbacks: vec![
            "Windows Terminal".to_string(),
            "mpv".to_string(),
            "tplay".to_string(),
            "viu".to_string(),
        ],
        safety_summary: "Use archive trends to decide what to inspect next; keep external fallbacks active until the native terminal promotion gates pass."
            .to_string(),
        guidance: vec![
            native_promotion_archive_trend_runbook_guidance(
                "short",
                "info",
                "Fewer than two retained snapshots are available.",
                "Run Export diagnostics again after the next status change, then compare the new snapshot.",
            ),
            native_promotion_archive_trend_runbook_guidance(
                "stable",
                "info",
                "Blocker count is flat across the retained samples.",
                "Keep reviewing warning alerts and rollback text before treating the archive history as settled.",
            ),
            native_promotion_archive_trend_runbook_guidance(
                "improving",
                "info",
                "Blocker count is lower in the newest retained snapshot.",
                "Continue collecting redacted snapshots and verify the underlying blocker rows before any promotion decision.",
            ),
            native_promotion_archive_trend_runbook_guidance(
                "worsening",
                "blocked",
                "Blocker count or blocked alert severity has increased.",
                "Pause native promotion work, inspect blocked alerts, and keep Windows Terminal, mpv, tplay, and viu as fallbacks.",
            ),
        ],
    }
}

fn native_promotion_archive_trend_runbook_guidance(
    state: &str,
    severity: &str,
    meaning: &str,
    operator_action: &str,
) -> DxCliNativePromotionArchiveTrendRunbookGuidance {
    DxCliNativePromotionArchiveTrendRunbookGuidance {
        state: state.to_string(),
        severity: severity.to_string(),
        meaning: meaning.to_string(),
        operator_action: operator_action.to_string(),
    }
}

#[cfg(test)]
fn native_promotion_archive_trend_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "safe to enable",
        "you can enable",
        "turn on production",
        "remove external fallback",
        "remove external fallbacks",
        "disable external fallback",
        "disable external fallbacks",
        "native routing is production-ready",
        "native routing is production ready",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("diagnostic")
        && lower.contains("windows terminal")
        && lower.contains("mpv")
        && lower.contains("tplay")
        && lower.contains("viu")
}

#[cfg(test)]
fn tool_safety_alert_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "disable approval",
        "bypass approval",
        "ignore denied tools",
        "expose allowlist",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("does not")
        && lower.contains("dry-run")
        && lower.contains("redacted")
}

#[cfg(test)]
fn tool_safety_audit_review_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("audit history")
        && lower.contains("redacted")
        && lower.contains("ready-to-blocked")
}

#[cfg(test)]
fn tool_safety_audit_review_alert_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("audit_empty")
        && lower.contains("audit_ready_to_blocked")
        && lower.contains("blocked")
        && lower.contains("redacted")
}

#[cfg(test)]
fn tool_safety_audit_recovery_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("blocked_recovery")
        && lower.contains("warning_recovery")
        && lower.contains("cleared_recovery")
        && lower.contains("dry-run")
        && lower.contains("redacted")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("recovery_blocked")
        && lower.contains("recovery_warning_review")
        && lower.contains("recovery_pending_evidence")
        && lower.contains("recovery_cleared")
        && lower.contains("recovery_alert_runbook_missing")
        && lower.contains("redacted")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("digest_runbook_missing")
        && lower.contains("runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("ok")
        && lower.contains("redacted")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_runbook_is_safe(source: &str) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_runbook_missing")
        && lower.contains("digest_runbook_missing")
        && lower.contains("runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("ok")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_runbook_missing")
        && lower.contains("runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("ready")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_ready")
        && lower.contains("release_gate_digest_warning_review")
        && lower.contains("release_gate_digest_blocked")
        && lower.contains("release_gate_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_runbook_missing")
        && lower.contains("release_gate_digest_redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("ready")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("release_gate_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_alert_digest_ready")
        && lower.contains("release_gate_digest_alert_digest_warning_review")
        && lower.contains("release_gate_digest_alert_digest_blocked")
        && lower.contains("release_gate_digest_alert_digest_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alerts_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("ready")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("release_gate_digest_alert_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_ready")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_warning_review")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_blocked")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alert_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alerts_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("ready")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_runbook_missing")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alert_runbook_missing")
        && lower
            .contains("release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing")
        && lower.contains("redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alert_digest_ready")
        && lower
            .contains("release_gate_digest_alert_digest_alert_digest_alert_digest_warning_review")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alert_digest_blocked")
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
        )
        && lower
            .contains("release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing")
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing",
        )
        && lower
            .contains("release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("ready")
        && lower.contains("warning_review")
        && lower.contains("blocked")
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
        )
        && lower.contains("redaction_review")
        && lower.contains("redacted")
        && lower.contains("config-free")
}

#[cfg(test)]
fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe(
    source: &str,
) -> bool {
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "paste your token",
        "write the api key",
        "commit the secret",
        "store command values",
        "store path values",
        "store env values",
        "copy raw history rows",
        "disable approval",
        "bypass approval",
        "ignore blocked",
        "expose allowlist",
        "invoke tools",
    ];

    !forbidden.iter().any(|phrase| lower.contains(phrase))
        && lower.contains("metadata-only")
        && lower.contains("release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_ready")
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_warning_review",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_blocked",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing",
        )
        && lower.contains(
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review",
        )
        && lower.contains("redacted")
        && lower.contains("config-free")
}

fn native_promotion_archive_drift_alerts(
    latest: &DxCliNativePromotionArchiveEntry,
    previous: &DxCliNativePromotionArchiveEntry,
    blocker_delta: i64,
) -> Vec<DxCliNativePromotionArchiveDriftAlert> {
    let mut alerts = Vec::new();

    if blocker_delta > 0 {
        alerts.push(native_promotion_archive_drift_alert(
            "blockers_increased",
            "blocked",
            "Blockers increased",
            format!(
                "Native promotion blockers increased from {} to {}.",
                previous.blocker_count, latest.blocker_count
            ),
            "Keep external fallbacks active and inspect the native promotion blocker rows before enabling embedded routing.",
        ));
    }

    if !latest.redacted || !previous.redacted {
        alerts.push(native_promotion_archive_drift_alert(
            if previous.redacted && !latest.redacted {
                "redaction_regressed"
            } else if latest.redacted {
                "previous_redaction_not_confirmed"
            } else {
                "redaction_not_confirmed"
            },
            "blocked",
            if previous.redacted && !latest.redacted {
                "Redaction regressed"
            } else if latest.redacted {
                "Previous redaction not confirmed"
            } else {
                "Redaction not confirmed"
            },
            "One of the compared native promotion snapshots is not confirmed metadata-only."
                .to_string(),
            "Stop using this archive comparison for operator review until the snapshot redaction contract is restored.",
        ));
    }

    if !latest.diagnostic_only {
        alerts.push(native_promotion_archive_drift_alert(
            if previous.diagnostic_only {
                "diagnostic_only_regressed"
            } else {
                "diagnostic_only_missing"
            },
            "blocked",
            if previous.diagnostic_only {
                "Diagnostic boundary regressed"
            } else {
                "Diagnostic boundary missing"
            },
            "The latest snapshot is no longer marked diagnostic-only.".to_string(),
            "Keep production embedded routing disabled and restore the diagnostic-only archive contract.",
        ));
    }

    if previous.production_ready != latest.production_ready {
        if latest.production_ready {
            alerts.push(native_promotion_archive_drift_alert(
                "production_ready_flip",
                "warning",
                "Production readiness changed",
                "The latest snapshot reports production-ready while the previous snapshot did not.".to_string(),
                "Require a manual verification checkpoint before routing TUI, video, or audio into the native surface.",
            ));
        } else {
            alerts.push(native_promotion_archive_drift_alert(
                "production_ready_regressed",
                "blocked",
                "Production readiness regressed",
                "The latest snapshot no longer reports production-ready.".to_string(),
                "Keep Windows Terminal, mpv, tplay, and viu as active fallbacks until readiness is restored.",
            ));
        }
    }

    if latest.rollback_summary != previous.rollback_summary {
        alerts.push(native_promotion_archive_drift_alert(
            "rollback_changed",
            "warning",
            "Rollback changed",
            "The rollback summary changed between the latest retained snapshots.".to_string(),
            "Review the rollback text before treating the archive trend as stable.",
        ));
    }

    alerts
}

fn native_promotion_archive_drift_alert(
    id: &str,
    severity: &str,
    label: &str,
    detail: String,
    recovery_hint: &str,
) -> DxCliNativePromotionArchiveDriftAlert {
    DxCliNativePromotionArchiveDriftAlert {
        id: id.to_string(),
        severity: severity.to_string(),
        label: label.to_string(),
        detail,
        recovery_hint: recovery_hint.to_string(),
    }
}

fn native_promotion_archive_alert_level(
    alerts: &[DxCliNativePromotionArchiveDriftAlert],
) -> &'static str {
    if alerts.iter().any(|alert| alert.severity == "blocked") {
        "blocked"
    } else if alerts.iter().any(|alert| alert.severity == "warning") {
        "warning"
    } else {
        "clean"
    }
}

fn native_promotion_diff_change(
    id: &str,
    label: &str,
    before: String,
    after: String,
    delta: Option<i64>,
) -> DxCliNativePromotionArchiveDiffChange {
    let changed = before != after;
    DxCliNativePromotionArchiveDiffChange {
        id: id.to_string(),
        label: label.to_string(),
        before,
        after,
        delta,
        changed,
    }
}

fn yes_no(value: bool) -> String {
    if value { "yes" } else { "no" }.to_string()
}

fn optional_text(value: Option<&str>) -> String {
    value.unwrap_or("none").to_string()
}

fn native_promotion_archive_entry(
    path: &Path,
) -> Result<Option<DxCliNativePromotionArchiveEntry>, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("failed to stat {}: {error}", path.display()))?;
    if !metadata.is_file() {
        return Ok(None);
    }
    let modified_at_ms = metadata
        .modified()
        .ok()
        .map(system_time_ms)
        .unwrap_or_default();
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| format!("invalid archive file name: {}", path.display()))?
        .to_string();
    let body = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    let status = value.get("status").unwrap_or(&serde_json::Value::Null);
    let redaction = status.get("redaction").unwrap_or(&serde_json::Value::Null);
    let evidence = status
        .get("evidence_lines")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let redacted = redaction
        .get("stores_payloads")
        .and_then(serde_json::Value::as_bool)
        == Some(false)
        && redaction
            .get("stores_input_values")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && redaction
            .get("stores_terminal_frames")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && evidence.iter().all(|line| {
            line.get("redacted").and_then(serde_json::Value::as_bool) == Some(true)
                && line
                    .get("stores_payload")
                    .and_then(serde_json::Value::as_bool)
                    == Some(false)
        });

    Ok(Some(DxCliNativePromotionArchiveEntry {
        file_name,
        path: path.display().to_string(),
        modified_at_ms,
        size_bytes: metadata.len(),
        archived_at_ms: json_u64(value.get("archived_at_ms")).unwrap_or_default(),
        retained_count: json_usize(value.get("retained_count")).unwrap_or_default(),
        retention_limit: json_usize(value.get("retention_limit")).unwrap_or_default(),
        blocker_count: json_usize(status.get("blocker_count")).unwrap_or_default(),
        surface_count: json_usize(status.get("surface_count")).unwrap_or_default(),
        diagnostic_only: status
            .get("diagnostic_only")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        production_ready: status
            .get("production_ready")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        redacted,
        rollback_summary: native_promotion_rollback_summary(status),
        next_surface: status
            .get("next_surface")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string),
    }))
}

fn native_promotion_rollback_summary(status: &serde_json::Value) -> String {
    let messages = status
        .get("rollback_messages")
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .take(2)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if messages.is_empty() {
        "external terminal fallback preserved".to_string()
    } else {
        messages.join(" | ")
    }
}

fn open_native_promotion_archive(file_name: &str) -> Result<DxAgentsOpenPathResult, String> {
    let path = native_promotion_archive_file_path(file_name)?;
    if !path.is_file() {
        return Err(format!(
            "Native promotion archive does not exist: {}",
            path.display()
        ));
    }
    open_path_with_default_app(&path)?;
    Ok(DxAgentsOpenPathResult {
        target: "native-promotion-archive".to_string(),
        path: path.display().to_string(),
        opened: true,
    })
}

fn native_promotion_archive_file_path(file_name: &str) -> Result<PathBuf, String> {
    let file_name = file_name.trim();
    if !is_native_promotion_archive_file_name(file_name) {
        return Err("Expected a native-promotion-status-*.json file name.".to_string());
    }
    let path = Path::new(file_name);
    if path.components().count() != 1 {
        return Err("Native promotion archive must be opened by file name, not path.".to_string());
    }
    Ok(native_promotion_archive_dir().join(file_name))
}

fn is_native_promotion_archive_file_name(file_name: &str) -> bool {
    file_name.starts_with("native-promotion-status-") && file_name.ends_with(".json")
}

fn release_readiness_report() -> DxAgentsReleaseReadiness {
    let repo_dir = dx_agents_repo_dir();
    let tauri_dir = repo_dir.join("apps").join("tauri");
    let tauri_config_path = tauri_dir.join("tauri.conf.json");
    let tauri_config = read_json_file(&tauri_config_path);
    let cli_candidates = dx_agents_cli_candidates(&repo_dir);
    let host_contract_path = contract_path(&dx_cli_root());
    let mut items = Vec::new();

    items.push(readiness_item(
        "repo_dir",
        "Repository",
        repo_dir.is_dir(),
        format!("DX Agents repo directory: {}", repo_dir.display()),
        "Set DX_AGENTS_REPO_DIR to the active checkout.",
    ));
    items.push(readiness_item(
        "debug_binary",
        "Debug binary",
        cli_candidates.first().is_some_and(|path| path.is_file()),
        format!(
            "Expected debug binary: {}",
            cli_candidates
                .first()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        ),
        "Run `cargo check -p dx-agents --bin dx-agents` or build the debug CLI once.",
    ));
    items.push(readiness_item(
        "release_binary",
        "Release binary",
        cli_candidates.get(1).is_some_and(|path| path.is_file()),
        format!(
            "Expected release binary: {}",
            cli_candidates
                .get(1)
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "unknown".to_string())
        ),
        "Defer release build until the current feature batch is stable, then run the release build once.",
    ));

    items.push(match &tauri_config {
        Ok(config) => {
            let product_name = config
                .get("productName")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let identifier = config
                .get("identifier")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("unknown");
            let bundle_active = config
                .pointer("/bundle/active")
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false);
            readiness_item(
                "tauri_bundle_config",
                "Tauri bundle config",
                product_name == "DX Agents"
                    && identifier == "ai.dxagents.desktop"
                    && bundle_active,
                format!(
                    "Tauri config: product `{product_name}`, identifier `{identifier}`, bundle active `{bundle_active}` at {}.",
                    tauri_config_path.display()
                ),
                "Review `apps/tauri/tauri.conf.json` before packaging the desktop app.",
            )
        }
        Err(error) => readiness_item(
            "tauri_bundle_config",
            "Tauri bundle config",
            false,
            format!(
                "Could not read Tauri config at {}: {error}",
                tauri_config_path.display()
            ),
            "Restore `apps/tauri/tauri.conf.json` before release packaging.",
        ),
    });

    let workspace_version =
        toml_string_value(&repo_dir.join("Cargo.toml"), "workspace.package", "version");
    let desktop_version = toml_string_value(&tauri_dir.join("Cargo.toml"), "package", "version");
    let tauri_version = tauri_config
        .as_ref()
        .ok()
        .and_then(|config| config.get("version"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_string);
    items.push(readiness_item(
        "version_metadata",
        "Version metadata",
        workspace_version.is_some()
            && tauri_version.is_some()
            && workspace_version.as_deref() == tauri_version.as_deref()
            && desktop_version.is_some(),
        format!(
            "Workspace version `{}`, Tauri package version `{}`, desktop crate version `{}`.",
            workspace_version.as_deref().unwrap_or("missing"),
            tauri_version.as_deref().unwrap_or("missing"),
            desktop_version.as_deref().unwrap_or("missing")
        ),
        "Align workspace and Tauri package versions before release packaging.",
    ));

    items.push(installer_icon_readiness_item(
        &tauri_dir,
        tauri_config.as_ref().ok(),
    ));
    items.push(bundle_targets_readiness_item(tauri_config.as_ref().ok()));
    items.push(expected_distribution_outputs_item(&repo_dir));

    let contract_status = load_contract_from_path(&host_contract_path);
    items.push(match contract_status {
        Ok(contract) => DxAgentsReleaseReadinessItem {
            id: "host_contract".to_string(),
            label: "Host contract".to_string(),
            status: "ok".to_string(),
            detail: format!(
                "Host contract parsed at {} with {} actions.",
                host_contract_path.display(),
                contract.actions.len()
            ),
            recovery_hint: None,
        },
        Err(error) => DxAgentsReleaseReadinessItem {
            id: "host_contract".to_string(),
            label: "Host contract".to_string(),
            status: "warn".to_string(),
            detail: format!(
                "Host contract is not ready at {}: {error}",
                host_contract_path.display()
            ),
            recovery_hint: Some(
                "Regenerate the DX CLI host contract before release packaging.".to_string(),
            ),
        },
    });

    for (id, label, file) in [
        ("todo", "TODO", "TODO.md"),
        ("changelog", "Changelog", "CHANGELOG.md"),
        (
            "parity_docs",
            "Parity docs",
            "docs/parity/openclaw-hermes-core.md",
        ),
        (
            "desktop_bridge_docs",
            "Desktop bridge docs",
            "docs/book/src/ops/desktop-bridge.md",
        ),
        (
            "operator_qa_docs",
            "Operator QA docs",
            "docs/book/src/ops/operator-qa.md",
        ),
        (
            "dashboard_compatibility_migration",
            "Dashboard compatibility migration",
            DASHBOARD_COMPATIBILITY_MIGRATION_PLAN,
        ),
        (
            "native_promotion_trend_runbook",
            "Native promotion trend runbook",
            NATIVE_PROMOTION_ARCHIVE_TREND_RUNBOOK,
        ),
        (
            "tool_safety_alert_runbook",
            "Tool safety alert runbook",
            TOOL_SAFETY_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_runbook",
            "Tool safety audit review runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_alert_runbook",
            "Tool safety audit review alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_runbook",
            "Tool safety audit review recovery runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_runbook",
            "Tool safety audit review recovery alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_runbook",
            "Tool safety audit review recovery alert digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_runbook",
            "Tool safety audit review recovery alert digest release gate runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_runbook",
            "Tool safety audit review recovery alert digest release gate digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest alert digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest alert digest alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest alert digest alert digest runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        ),
        (
            "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook",
            "Tool safety audit review recovery alert digest release gate digest alert digest alert digest alert digest alert digest alert runbook",
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        ),
    ] {
        let path = repo_dir.join(file);
        items.push(readiness_item(
            id,
            label,
            path.is_file(),
            format!("Expected path: {}", path.display()),
            "Restore or update this project status artifact before cutting a release.",
        ));
    }

    let recovery_alert_digest_release_gate =
        read_tool_safety_audit_history(&tool_safety_audit_history_path(), 8)
            .map(|history| history.recovery_alert_digest_release_gate);
    let (gate_ready, gate_detail) = match recovery_alert_digest_release_gate {
        Ok(gate) => (
            gate.ready,
            format!(
                "State: {}; severity: {}; digest: {}; alerts: {}; release blocking: {}; safe to share: {}",
                gate.state,
                gate.severity,
                gate.digest_state,
                gate.alert_count,
                gate.release_blocking,
                gate.safe_to_share
            ),
        ),
        Err(error) => (
            false,
            format!("Recovery alert digest release gate unavailable: {error}"),
        ),
    };
    items.push(readiness_item(
        "tool_safety_audit_recovery_alert_digest_release_gate",
        "Tool safety audit recovery alert digest release gate",
        gate_ready,
        gate_detail,
        "Refresh metadata-only audit history and resolve recovery alert digest release-gate blockers before release.",
    ));

    let migrate_help = Command::new(dx_agents_cli_program())
        .args(["migrate", "--help"])
        .current_dir(&repo_dir)
        .output();
    items.push(match migrate_help {
        Ok(output) if output.status.success() => DxAgentsReleaseReadinessItem {
            id: "migration_surface".to_string(),
            label: "Migration surface".to_string(),
            status: "ok".to_string(),
            detail: "Migration CLI help is available.".to_string(),
            recovery_hint: None,
        },
        Ok(output) => DxAgentsReleaseReadinessItem {
            id: "migration_surface".to_string(),
            label: "Migration surface".to_string(),
            status: "warn".to_string(),
            detail: summarize_error(&String::from_utf8_lossy(&output.stderr)),
            recovery_hint: Some(
                "Run `dx-agent migrate --help` locally and repair migration command wiring."
                    .to_string(),
            ),
        },
        Err(error) => DxAgentsReleaseReadinessItem {
            id: "migration_surface".to_string(),
            label: "Migration surface".to_string(),
            status: "warn".to_string(),
            detail: format!("Could not run migration help: {error}"),
            recovery_hint: Some(
                "Confirm the DX Agents CLI binary is built or set DX_AGENTS_CLI.".to_string(),
            ),
        },
    });

    let ok_count = items.iter().filter(|item| item.status == "ok").count();
    let score = ((ok_count * 100) / items.len().max(1)) as u8;
    let next_action = items
        .iter()
        .find(|item| item.status != "ok")
        .and_then(|item| item.recovery_hint.clone());

    DxAgentsReleaseReadiness {
        generated_at_ms: now_ms(),
        score,
        ready: score == 100,
        items,
        next_action,
    }
}

fn installer_icon_readiness_item(
    tauri_dir: &Path,
    tauri_config: Option<&serde_json::Value>,
) -> DxAgentsReleaseReadinessItem {
    let icon_paths = tauri_config
        .and_then(|config| config.pointer("/bundle/icon"))
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();
    let mut missing = Vec::new();
    let mut configured = 0usize;

    for icon in icon_paths.iter().filter_map(serde_json::Value::as_str) {
        configured += 1;
        if !tauri_dir.join(icon).is_file() {
            missing.push(icon.to_string());
        }
    }

    readiness_item(
        "installer_icons",
        "Installer icons",
        configured > 0 && missing.is_empty(),
        if missing.is_empty() {
            format!(
                "{configured} bundle icon assets are present under {}.",
                tauri_dir.display()
            )
        } else {
            format!(
                "{configured} bundle icon assets configured; missing: {}.",
                missing.join(", ")
            )
        },
        "Restore missing icon assets under `apps/tauri/icons` before packaging installers.",
    )
}

fn bundle_targets_readiness_item(
    tauri_config: Option<&serde_json::Value>,
) -> DxAgentsReleaseReadinessItem {
    let bundle = tauri_config.and_then(|config| config.get("bundle"));
    let active = bundle
        .and_then(|bundle| bundle.get("active"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let targets = bundle
        .and_then(|bundle| bundle.get("targets"))
        .map(|targets| match targets {
            serde_json::Value::String(value) => value.clone(),
            serde_json::Value::Array(values) => values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .collect::<Vec<_>>()
                .join(", "),
            other => other.to_string(),
        })
        .unwrap_or_else(|| "missing".to_string());

    readiness_item(
        "bundle_targets",
        "Bundle targets",
        active && targets != "missing" && !targets.trim().is_empty(),
        format!("Tauri bundle active `{active}` with targets `{targets}`."),
        "Set active bundle targets in `apps/tauri/tauri.conf.json` before release packaging.",
    )
}

fn expected_distribution_outputs_item(repo_dir: &Path) -> DxAgentsReleaseReadinessItem {
    let bundle_dir = repo_dir.join("target").join("release").join("bundle");
    let expected_dirs = ["msi", "nsis", "dmg", "appimage", "deb", "rpm"];
    let existing = expected_dirs
        .iter()
        .filter(|name| bundle_dir.join(name).is_dir())
        .copied()
        .collect::<Vec<_>>();

    readiness_item(
        "expected_distribution_outputs",
        "Expected distribution outputs",
        bundle_dir.is_dir() && !existing.is_empty(),
        if existing.is_empty() {
            format!(
                "Expected release bundle root: {}. No release package directories are present; this check did not run a release build.",
                bundle_dir.display()
            )
        } else {
            format!(
                "Expected release bundle root: {}; present package directories: {}.",
                bundle_dir.display(),
                existing.join(", ")
            )
        },
        "After operator QA is stable, run one release packaging build and rerun this readiness report.",
    )
}

fn readiness_item(
    id: &str,
    label: &str,
    ok: bool,
    detail: String,
    recovery_hint: &str,
) -> DxAgentsReleaseReadinessItem {
    DxAgentsReleaseReadinessItem {
        id: id.to_string(),
        label: label.to_string(),
        status: if ok { "ok" } else { "warn" }.to_string(),
        detail,
        recovery_hint: (!ok).then(|| recovery_hint.to_string()),
    }
}

fn embedded_terminal_fixtures() -> DxAgentsEmbeddedTerminalFixtures {
    let timestamp_ms = now_ms();
    DxAgentsEmbeddedTerminalFixtures {
        schema_version: "dx.embedded_terminal_fixtures.v1".to_string(),
        generated_at_ms: timestamp_ms,
        redaction_policy: "metadata_only_no_raw_payloads".to_string(),
        input_events: vec![
            input_fixture("keyboard", timestamp_ms, 2, 1),
            input_fixture("paste", timestamp_ms, 0, 128),
            input_fixture("focus", timestamp_ms, 0, 0),
            input_fixture("mouse", timestamp_ms, 1, 0),
            input_fixture("control_sequence", timestamp_ms, 0, 4),
        ],
        resize_events: vec![
            resize_fixture("initial_size", 120, 34, false, true, false),
            resize_fixture("viewport_resize", 132, 38, false, true, true),
            resize_fixture("debounce", 132, 38, true, true, false),
            resize_fixture("renderer_reflow", 132, 38, true, true, false),
            resize_fixture("pty_resize", 132, 38, true, false, true),
        ],
    }
}

fn input_fixture(
    kind: &str,
    timestamp_ms: u64,
    modifier_count: u8,
    payload_bytes: usize,
) -> DxAgentsEmbeddedTerminalInputFixture {
    DxAgentsEmbeddedTerminalInputFixture {
        event_id: format!("input.{kind}"),
        kind: kind.to_string(),
        timestamp_ms,
        modifier_count,
        payload_bytes,
        redacted: true,
        stores_payload: false,
    }
}

fn resize_fixture(
    kind: &str,
    columns: u16,
    rows: u16,
    debounced: bool,
    renderer_reflow_required: bool,
    pty_resize_required: bool,
) -> DxAgentsEmbeddedTerminalResizeFixture {
    DxAgentsEmbeddedTerminalResizeFixture {
        event_id: format!("resize.{kind}"),
        kind: kind.to_string(),
        columns,
        rows,
        debounced,
        renderer_reflow_required,
        pty_resize_required,
        redacted: true,
    }
}

fn embedded_terminal_readiness_export(
    contract_path: &Path,
    contract: &serde_json::Value,
) -> DxAgentsEmbeddedTerminalReadiness {
    let fixtures = embedded_terminal_fixtures();
    let input_contract_present = contract_schema_is(
        contract,
        "/embedded_terminal_input/schema_version",
        "dx.embedded_terminal_input.v1",
    );
    let resize_contract_present = contract_schema_is(
        contract,
        "/embedded_terminal_resize/schema_version",
        "dx.embedded_terminal_resize.v1",
    );
    let media_session_contract_present = contract_schema_is(
        contract,
        "/embedded_terminal_media_session/schema_version",
        "dx.embedded_terminal_media_session.v1",
    );
    let embedded_pty_production_ready = contract
        .pointer("/embedded_pty/production_ready")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let input_enabled = contract
        .pointer("/embedded_terminal_input/enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let resize_enabled = contract
        .pointer("/embedded_terminal_resize/enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let media_session_enabled = contract
        .pointer("/embedded_terminal_media_session/enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let mut evidence = vec![
        readiness_evidence(
            "embedded_pty",
            "Embedded PTY",
            embedded_pty_production_ready,
            if embedded_pty_production_ready {
                "Embedded PTY reports production-ready gates.".to_string()
            } else {
                "Embedded PTY stays gated; external terminal remains the production route."
                    .to_string()
            },
        ),
        readiness_evidence(
            "input_contract",
            "Input contract",
            input_contract_present,
            contract_detail(contract, "/embedded_terminal_input/schema_version"),
        ),
        readiness_evidence(
            "resize_contract",
            "Resize contract",
            resize_contract_present,
            contract_detail(contract, "/embedded_terminal_resize/schema_version"),
        ),
        readiness_evidence(
            "media_session_contract",
            "Media-session contract",
            media_session_contract_present,
            contract_detail(contract, "/embedded_terminal_media_session/schema_version"),
        ),
        readiness_evidence(
            "input_redaction",
            "Input redaction",
            input_redaction_is_safe(contract),
            "Input contract must store metadata only: payload length, kind, modifier count, and redacted flag.".to_string(),
        ),
        readiness_evidence(
            "fixtures_no_payload",
            "No-payload fixtures",
            fixtures
                .input_events
                .iter()
                .all(|event| event.redacted && !event.stores_payload)
                && fixtures.resize_events.iter().all(|event| event.redacted),
            "Fixture evidence contains event metadata and dimensions without raw key, paste, mouse, or control-sequence payloads.".to_string(),
        ),
    ];

    evidence.push(readiness_evidence(
        "external_fallback",
        "External fallback",
        contract
            .pointer("/embedded_pty/fallback_surface")
            .and_then(serde_json::Value::as_str)
            == Some("external_terminal"),
        "Windows Terminal remains the fallback surface until all production gates are ready."
            .to_string(),
    ));

    let ready = embedded_pty_production_ready
        && input_enabled
        && resize_enabled
        && media_session_enabled
        && evidence.iter().all(|item| item.status == "ok");

    DxAgentsEmbeddedTerminalReadiness {
        schema_version: "dx.embedded_terminal_readiness.v1".to_string(),
        generated_at_ms: now_ms(),
        contract_path: contract_path.display().to_string(),
        ready,
        embedded_pty_production_ready,
        input_contract_present,
        resize_contract_present,
        media_session_contract_present,
        evidence,
        fixtures,
    }
}

fn embedded_terminal_session_timeline(
    contract_path: &Path,
    contract: &serde_json::Value,
) -> DxAgentsEmbeddedTerminalSessionTimeline {
    let readiness = embedded_terminal_readiness_export(contract_path, contract);
    let fixtures = readiness.fixtures.clone();
    let external_terminal_fallback = contract
        .pointer("/embedded_pty/fallback_surface")
        .and_then(serde_json::Value::as_str)
        == Some("external_terminal");
    let echo_process_pilot_ready = readiness.input_contract_present
        && readiness.resize_contract_present
        && readiness.media_session_contract_present
        && input_redaction_is_safe(contract)
        && fixtures
            .input_events
            .iter()
            .all(|event| event.redacted && !event.stores_payload)
        && fixtures.resize_events.iter().all(|event| event.redacted)
        && external_terminal_fallback;
    let mut events = vec![
        synthetic_session_event(
            1,
            "session.open",
            "session",
            "synthetic",
            "Open an in-memory terminal session fixture; no process is spawned.",
        ),
        synthetic_session_event(
            2,
            "session.attach_renderer",
            "renderer",
            "synthetic",
            "Attach a synthetic renderer state so resize reflow can be recorded without drawing terminal output.",
        ),
    ];

    for fixture in fixtures
        .resize_events
        .iter()
        .filter(|event| event.kind == "initial_size")
    {
        events.push(synthetic_session_event(
            3,
            fixture.event_id.as_str(),
            "resize",
            "fixture.resize",
            format!(
                "Record initial terminal dimensions {}x{} as metadata only.",
                fixture.columns, fixture.rows
            ),
        ));
    }

    for (offset, fixture) in fixtures.input_events.iter().enumerate() {
        events.push(synthetic_session_event(
            (offset + 4) as u8,
            fixture.event_id.as_str(),
            "input",
            "fixture.input",
            format!(
                "Accept {} input metadata with {} payload bytes reported and no stored payload.",
                fixture.kind, fixture.payload_bytes
            ),
        ));
    }

    for (offset, fixture) in fixtures
        .resize_events
        .iter()
        .filter(|event| event.kind != "initial_size")
        .enumerate()
    {
        events.push(synthetic_session_event(
            (offset + 9) as u8,
            fixture.event_id.as_str(),
            "resize",
            "fixture.resize",
            format!(
                "Record {} resize metadata at {}x{}; renderer reflow {}, PTY resize {}.",
                fixture.kind,
                fixture.columns,
                fixture.rows,
                if fixture.renderer_reflow_required {
                    "required"
                } else {
                    "not required"
                },
                if fixture.pty_resize_required {
                    "required"
                } else {
                    "not required"
                }
            ),
        ));
    }

    events.push(synthetic_session_event(
        13,
        "session.interrupt",
        "lifecycle",
        "synthetic",
        "Record an interrupt transition against the synthetic session; no child process exists.",
    ));
    events.push(synthetic_session_event(
        14,
        "session.close",
        "lifecycle",
        "synthetic",
        "Close the synthetic session and verify cleanup without orphaning a process.",
    ));

    DxAgentsEmbeddedTerminalSessionTimeline {
        schema_version: "dx.embedded_terminal_session_timeline.v1".to_string(),
        generated_at_ms: now_ms(),
        phase: "synthetic_session".to_string(),
        contract_path: contract_path.display().to_string(),
        process_spawned: false,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        external_terminal_fallback,
        readiness_ready: readiness.ready,
        echo_process_pilot_ready,
        readiness_evidence: readiness.evidence,
        events,
        next_phase: if echo_process_pilot_ready {
            "controlled_local_echo_process".to_string()
        } else {
            "repair_redacted_fixture_or_contract_evidence".to_string()
        },
    }
}

fn embedded_terminal_media_canary_evidence(
    contract: &serde_json::Value,
) -> DxAgentsEmbeddedTerminalMediaCanaryEvidence {
    let media_session_contract_present = contract_schema_is(
        contract,
        "/embedded_terminal_media_session/schema_version",
        "dx.embedded_terminal_media_session.v1",
    );
    let media_session_enabled = contract
        .pointer("/embedded_terminal_media_session/enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let external_terminal_fallback = contract
        .pointer("/embedded_terminal_media_session/fallback_surface")
        .or_else(|| contract.pointer("/embedded_pty/fallback_surface"))
        .and_then(serde_json::Value::as_str)
        .unwrap_or("external_terminal")
        == "external_terminal";
    let max_frame_rate = contract
        .pointer("/embedded_terminal_media_session/frame_budget/max_frame_rate")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .unwrap_or(30);
    let max_pending_frames = contract
        .pointer("/embedded_terminal_media_session/frame_budget/max_pending_frames")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .unwrap_or(3);
    let max_audio_buffer_ms = contract
        .pointer("/embedded_terminal_media_session/frame_budget/max_audio_buffer_ms")
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .unwrap_or(250);
    let sample_status = if media_session_enabled {
        "capturable"
    } else {
        "gated"
    };
    let samples = vec![
        media_canary_sample(
            1,
            "media.terminal_video",
            "video",
            "terminal_video",
            sample_status,
            "embedded_terminal_media_session.supported_media.terminal_video",
            "dx.watch",
            Some(max_frame_rate),
            Some(max_pending_frames),
            None,
            Some("120x34 cells"),
            "Terminal-video canary records frame cadence, dimensions, and fallback action without frame contents.",
        ),
        media_canary_sample(
            2,
            "media.audio_stream",
            "audio",
            "audio",
            sample_status,
            "embedded_terminal_media_session.supported_media.audio",
            "dx.watch",
            None,
            None,
            Some(max_audio_buffer_ms),
            None,
            "Audio canary records buffer budget and fallback route without audio samples.",
        ),
        media_canary_sample(
            3,
            "media.image_preview",
            "image",
            "image_preview",
            sample_status,
            "embedded_terminal_media_session.supported_media.image_preview",
            "dx.image",
            None,
            None,
            None,
            Some("bounded preview dimensions"),
            "Image canary records dimensions and fallback action without image pixels.",
        ),
        media_canary_sample(
            4,
            "media.backpressure",
            "video",
            "backpressure",
            sample_status,
            "embedded_terminal_media_session.events.backpressure",
            "dx.watch",
            Some(max_frame_rate),
            Some(max_pending_frames),
            None,
            None,
            "Backpressure evidence keeps only pending-frame limits and drop policy metadata.",
        ),
        media_canary_sample(
            5,
            "media.frame_budget",
            "mixed",
            "frame_budget",
            sample_status,
            "embedded_terminal_media_session.events.frame_budget",
            "dx.watch",
            Some(max_frame_rate),
            Some(max_pending_frames),
            Some(max_audio_buffer_ms),
            None,
            "Frame budget evidence combines video frame and audio buffer caps without payloads.",
        ),
        media_canary_sample(
            6,
            "media.close",
            "mixed",
            "close",
            sample_status,
            "embedded_terminal_media_session.events.close",
            "external_media_routes",
            None,
            None,
            None,
            None,
            "Close evidence verifies media sessions remain dismissible without orphaned playback state.",
        ),
        media_canary_sample(
            7,
            "media.cleanup",
            "mixed",
            "cleanup",
            sample_status,
            "embedded_terminal_media_session.events.cleanup",
            "external_media_routes",
            None,
            None,
            None,
            None,
            "Cleanup evidence verifies no embedded media payload, buffer, or frame state is retained.",
        ),
    ];
    let operator_export_ready = samples
        .iter()
        .all(|sample| sample.redacted && !sample.stores_payload && !sample.process_spawned)
        && external_terminal_fallback;

    DxAgentsEmbeddedTerminalMediaCanaryEvidence {
        schema_version: "dx.embedded_terminal_media_canary_evidence.v1".to_string(),
        generated_at_ms: now_ms(),
        status: if media_session_enabled {
            "capturable".to_string()
        } else {
            "gated".to_string()
        },
        process_spawned: false,
        media_session_contract_present,
        media_session_enabled,
        external_terminal_fallback,
        production_routing_enabled: false,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        sample_count: samples.len(),
        samples,
        fallback_actions: vec![
            "dx.watch".to_string(),
            "dx.image".to_string(),
            "external_media_routes".to_string(),
        ],
        operator_export_ready,
        rollback_state: if operator_export_ready {
            "safe_to_discard_media_canary_evidence".to_string()
        } else {
            "requires_manual_media_evidence_review".to_string()
        },
        next_phase: if operator_export_ready {
            "operator_media_evidence_export".to_string()
        } else {
            "repair_media_canary_redaction_or_fallback".to_string()
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn media_canary_sample(
    step: u8,
    sample_id: &str,
    media_kind: &str,
    phase: &str,
    status: &str,
    source: &str,
    fallback_action: &str,
    max_frame_rate: Option<u16>,
    max_pending_frames: Option<u16>,
    max_audio_buffer_ms: Option<u16>,
    dimensions: Option<&str>,
    detail: &str,
) -> DxAgentsEmbeddedTerminalMediaCanarySample {
    DxAgentsEmbeddedTerminalMediaCanarySample {
        step,
        sample_id: sample_id.to_string(),
        media_kind: media_kind.to_string(),
        phase: phase.to_string(),
        status: status.to_string(),
        source: source.to_string(),
        fallback_action: fallback_action.to_string(),
        max_frame_rate,
        max_pending_frames,
        max_audio_buffer_ms,
        dimensions: dimensions.map(str::to_string),
        redacted: true,
        stores_payload: false,
        process_spawned: false,
        detail: detail.to_string(),
    }
}

fn run_embedded_terminal_echo_pilot(
    contract_path: &Path,
    contract: &serde_json::Value,
) -> Result<DxAgentsEmbeddedTerminalEchoPilot, String> {
    let timeline = embedded_terminal_session_timeline(contract_path, contract);
    let args = dx_agents_embedded_terminal_echo_pilot_args();
    let program = dx_agents_cli_program();
    let cwd = dx_agents_repo_dir();
    let mut argv = vec![program.display().to_string()];
    argv.extend(args.iter().cloned());

    if !timeline.echo_process_pilot_ready {
        return Ok(DxAgentsEmbeddedTerminalEchoPilot {
            schema_version: "dx.embedded_terminal_echo_process_pilot.v1".to_string(),
            generated_at_ms: now_ms(),
            status: "skipped".to_string(),
            success: false,
            exit_code: None,
            duration_ms: 0,
            process_spawned: false,
            allows_arbitrary_shell: false,
            stores_payloads: false,
            stdout_preview: String::new(),
            stderr_preview: String::new(),
            stdout_bytes: 0,
            stderr_bytes: 0,
            json: None,
            argv,
            cwd: cwd.display().to_string(),
            timeline_phase: timeline.phase,
            timeline_next_phase: timeline.next_phase,
            skipped_reason: Some(
                "Synthetic session evidence is not ready for the controlled local echo-process pilot."
                    .to_string(),
            ),
        });
    }

    let started = Instant::now();
    let output = Command::new(&program)
        .args(&args)
        .current_dir(&cwd)
        .output()
        .map_err(|error| format!("failed to run echo pilot {}: {error}", program.display()))?;
    let duration_ms = started.elapsed().as_millis() as u64;
    let stdout = redact_sensitive_text(&String::from_utf8_lossy(&output.stdout));
    let stderr = redact_sensitive_text(&String::from_utf8_lossy(&output.stderr));
    let (json, json_error) = parse_optional_json(&stdout);
    let payload_ok = json
        .as_ref()
        .and_then(|value| value.get("message"))
        .and_then(serde_json::Value::as_str)
        == Some("dx-agent-embedded-terminal-echo-ok");
    let success = output.status.success() && json_error.is_none() && payload_ok;

    Ok(DxAgentsEmbeddedTerminalEchoPilot {
        schema_version: "dx.embedded_terminal_echo_process_pilot.v1".to_string(),
        generated_at_ms: now_ms(),
        status: if success { "success" } else { "failed" }.to_string(),
        success,
        exit_code: output.status.code(),
        duration_ms,
        process_spawned: true,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        stdout_preview: summarize_error(&stdout),
        stderr_preview: if let Some(error) = json_error {
            summarize_error(&format!("{stderr}\nJSON parse error: {error}"))
        } else {
            summarize_error(&stderr)
        },
        stdout_bytes: output.stdout.len(),
        stderr_bytes: output.stderr.len(),
        json,
        argv,
        cwd: cwd.display().to_string(),
        timeline_phase: timeline.phase,
        timeline_next_phase: timeline.next_phase,
        skipped_reason: None,
    })
}

fn run_embedded_terminal_tui_canary_runner(
    contract: &serde_json::Value,
    env_value: Option<&str>,
) -> Result<DxAgentsEmbeddedTerminalTuiCanaryRunner, String> {
    let runner_contract = tui_canary_runner_contract(contract);
    let lifecycle = embedded_terminal_tui_canary_lifecycle(contract, env_value);
    let args = runner_contract.args.clone();
    let program = dx_agents_cli_program();
    let cwd = dx_agents_repo_dir();
    let mut argv = vec![program.display().to_string()];
    argv.extend(args.iter().cloned());

    if !lifecycle.gate_enabled {
        return Ok(DxAgentsEmbeddedTerminalTuiCanaryRunner {
            schema_version: "dx.embedded_terminal_tui_canary_runner.v1".to_string(),
            generated_at_ms: now_ms(),
            status: "skipped".to_string(),
            success: false,
            exit_code: None,
            duration_ms: 0,
            process_spawned: false,
            gate_enabled: false,
            allows_arbitrary_shell: false,
            stores_payloads: false,
            stdout_preview: String::new(),
            stderr_preview: String::new(),
            stdout_bytes: 0,
            stderr_bytes: 0,
            stdout_limit_bytes: runner_contract.stdout_limit_bytes,
            stderr_limit_bytes: runner_contract.stderr_limit_bytes,
            max_duration_ms: runner_contract.max_duration_ms,
            json: None,
            argv,
            cwd: cwd.display().to_string(),
            lifecycle_status: lifecycle.status,
            lifecycle_next_phase: lifecycle.next_phase,
            contract_present: runner_contract.present,
            contract_accepted: runner_contract.accepted,
            contract_source: runner_contract.source,
            contract_diagnostics: runner_contract.diagnostics,
            contract_fixed_command: runner_contract.fixed_command,
            contract_result_states: runner_contract.result_states,
            skip_reasons: runner_contract.skip_reasons,
            skipped_reason: Some(
                "Developer TUI canary gate is disabled; set DX_AGENTS_TUI_CANARY=developer for local evidence only."
                    .to_string(),
            ),
        });
    }

    let started = Instant::now();
    let output = Command::new(&program)
        .args(&args)
        .current_dir(&cwd)
        .output()
        .map_err(|error| {
            format!(
                "failed to run TUI canary runner {}: {error}",
                program.display()
            )
        })?;
    let duration_ms = started.elapsed().as_millis() as u64;
    let stdout = redact_sensitive_text(&String::from_utf8_lossy(&output.stdout));
    let stderr = redact_sensitive_text(&String::from_utf8_lossy(&output.stderr));
    let (json, json_error) = parse_optional_json(&stdout);
    let payload_ok = json.as_ref().is_some_and(|value| {
        value.get("message").and_then(serde_json::Value::as_str) == Some("dx-agent-tui-canary-ok")
            && value.get("runner").and_then(serde_json::Value::as_str)
                == Some("developer_tui_canary")
            && value
                .get("allows_arbitrary_shell")
                .and_then(serde_json::Value::as_bool)
                == Some(false)
            && value
                .get("stores_payloads")
                .and_then(serde_json::Value::as_bool)
                == Some(false)
            && value
                .get("production_routing_enabled")
                .and_then(serde_json::Value::as_bool)
                == Some(false)
    });
    let output_within_limits = output.stdout.len() <= runner_contract.stdout_limit_bytes
        && output.stderr.len() <= runner_contract.stderr_limit_bytes
        && duration_ms <= runner_contract.max_duration_ms;
    let success =
        output.status.success() && json_error.is_none() && payload_ok && output_within_limits;
    let stderr_preview = if !output_within_limits {
        summarize_error(&format!(
            "{stderr}\nOutput exceeded canary limits: stdout {}/{} bytes, stderr {}/{} bytes.",
            output.stdout.len(),
            runner_contract.stdout_limit_bytes,
            output.stderr.len(),
            runner_contract.stderr_limit_bytes
        ))
    } else if let Some(error) = json_error {
        summarize_error(&format!("{stderr}\nJSON parse error: {error}"))
    } else if !payload_ok {
        summarize_error(&format!(
            "{stderr}\nTUI canary payload did not match the fixed developer-runner contract."
        ))
    } else {
        summarize_error(&stderr)
    };

    Ok(DxAgentsEmbeddedTerminalTuiCanaryRunner {
        schema_version: "dx.embedded_terminal_tui_canary_runner.v1".to_string(),
        generated_at_ms: now_ms(),
        status: if success { "success" } else { "failed" }.to_string(),
        success,
        exit_code: output.status.code(),
        duration_ms,
        process_spawned: true,
        gate_enabled: true,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        stdout_preview: summarize_error(&stdout),
        stderr_preview,
        stdout_bytes: output.stdout.len(),
        stderr_bytes: output.stderr.len(),
        stdout_limit_bytes: runner_contract.stdout_limit_bytes,
        stderr_limit_bytes: runner_contract.stderr_limit_bytes,
        max_duration_ms: runner_contract.max_duration_ms,
        json,
        argv,
        cwd: cwd.display().to_string(),
        lifecycle_status: lifecycle.status,
        lifecycle_next_phase: lifecycle.next_phase,
        contract_present: runner_contract.present,
        contract_accepted: runner_contract.accepted,
        contract_source: runner_contract.source,
        contract_diagnostics: runner_contract.diagnostics,
        contract_fixed_command: runner_contract.fixed_command,
        contract_result_states: runner_contract.result_states,
        skip_reasons: runner_contract.skip_reasons,
        skipped_reason: None,
    })
}

fn embedded_terminal_tui_canary_gate(
    contract: &serde_json::Value,
    env_value: Option<&str>,
) -> DxAgentsEmbeddedTerminalTuiCanaryGate {
    let enabled = tui_canary_env_enabled(env_value);
    let normal_terminal_action_count = contract
        .get("actions")
        .and_then(serde_json::Value::as_array)
        .map(|actions| {
            actions
                .iter()
                .filter(|action| {
                    action
                        .get("requires_terminal")
                        .and_then(serde_json::Value::as_bool)
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or_default();
    let preferred_terminal_surface = contract
        .pointer("/settings/defaults/preferred_terminal_surface")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("windows_terminal")
        .to_string();
    let production_terminal_surface = contract
        .pointer("/embedded_pty/fallback_surface")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("external_terminal")
        .to_string();

    DxAgentsEmbeddedTerminalTuiCanaryGate {
        schema_version: "dx.embedded_terminal_tui_canary_gate.v1".to_string(),
        generated_at_ms: now_ms(),
        enabled,
        default_enabled: false,
        developer_only: true,
        env_var: TUI_CANARY_ENV.to_string(),
        env_value_present: env_value.is_some_and(|value| !value.trim().is_empty()),
        mode: if enabled {
            "developer_tui_canary".to_string()
        } else {
            "off".to_string()
        },
        production_terminal_surface,
        preferred_terminal_surface,
        normal_terminal_actions_unchanged: true,
        normal_terminal_action_count,
        registers_host_action: false,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        guardrails: vec![
            "defaults_off".to_string(),
            "developer_env_opt_in_only".to_string(),
            "does_not_register_or_modify_host_actions".to_string(),
            "normal_terminal_actions_stay_on_external_terminal".to_string(),
            "no_arbitrary_shell_input".to_string(),
            "no_raw_payload_storage".to_string(),
        ],
        next_phase: if enabled {
            "bounded_tui_process_lifecycle_evidence".to_string()
        } else {
            "set_dx_agents_tui_canary_for_developer_evidence_only".to_string()
        },
    }
}

fn tui_canary_env_enabled(value: Option<&str>) -> bool {
    matches!(
        value.map(str::trim).map(str::to_ascii_lowercase).as_deref(),
        Some("1" | "true" | "yes" | "on" | "enabled" | "developer")
    )
}

fn embedded_terminal_tui_canary_lifecycle(
    contract: &serde_json::Value,
    env_value: Option<&str>,
) -> DxAgentsEmbeddedTerminalTuiCanaryLifecycle {
    let gate = embedded_terminal_tui_canary_gate(contract, env_value);
    let runner_contract = tui_canary_runner_contract(contract);
    let event_status = if gate.enabled { "armed" } else { "gated" };

    DxAgentsEmbeddedTerminalTuiCanaryLifecycle {
        schema_version: "dx.embedded_terminal_tui_canary_lifecycle.v1".to_string(),
        generated_at_ms: now_ms(),
        status: if gate.enabled {
            "armed".to_string()
        } else {
            "gated".to_string()
        },
        gate_enabled: gate.enabled,
        process_spawned: false,
        process_kind: "developer_tui_canary".to_string(),
        max_duration_ms: runner_contract.max_duration_ms,
        stdout_limit_bytes: runner_contract.stdout_limit_bytes,
        stderr_limit_bytes: runner_contract.stderr_limit_bytes,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        external_terminal_fallback: gate.production_terminal_surface == "external_terminal",
        events: vec![
            tui_canary_lifecycle_event(
                1,
                "tui.open",
                event_status,
                2_000,
                true,
                "Open only after the developer gate is enabled; normal terminal actions remain external.",
            ),
            tui_canary_lifecycle_event(
                2,
                "tui.resize",
                event_status,
                1_000,
                false,
                "Apply bounded columns and rows from the canary viewport without changing the production terminal surface.",
            ),
            tui_canary_lifecycle_event(
                3,
                "tui.interrupt",
                event_status,
                2_000,
                true,
                "Interrupt must be observable and must not leave a child process alive.",
            ),
            tui_canary_lifecycle_event(
                4,
                "tui.close",
                event_status,
                2_000,
                true,
                "Close must complete even after resize or interrupt evidence has been recorded.",
            ),
            tui_canary_lifecycle_event(
                5,
                "tui.cleanup",
                event_status,
                3_000,
                true,
                "Cleanup must verify no orphaned process, stale renderer state, or captured raw payload remains.",
            ),
        ],
        rollback_triggers: vec![
            "process_exceeds_max_duration".to_string(),
            "stdout_or_stderr_exceeds_limit".to_string(),
            "interrupt_or_close_leaves_child_alive".to_string(),
            "renderer_state_persists_after_cleanup".to_string(),
            "normal_terminal_action_routing_changes".to_string(),
        ],
        next_phase: if gate.enabled {
            "developer_tui_canary_runner".to_string()
        } else {
            "developer_canary_gate_opt_in".to_string()
        },
    }
}

fn embedded_terminal_tui_canary_renderer_evidence(
    contract: &serde_json::Value,
    env_value: Option<&str>,
) -> DxAgentsEmbeddedTerminalTuiCanaryRendererEvidence {
    let gate = embedded_terminal_tui_canary_gate(contract, env_value);
    let renderer_contract_present = contract_schema_is(
        contract,
        "/embedded_terminal_renderer/schema_version",
        "dx.embedded_terminal_renderer.v1",
    );
    let renderer_enabled = contract
        .pointer("/embedded_terminal_renderer/enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let snapshot_status = if gate.enabled { "capturable" } else { "gated" };
    let source_contract = contract
        .pointer("/embedded_terminal_tui_canary_renderer_evidence")
        .map(|value| {
            (
                value,
                "host_contract.embedded_terminal_tui_canary_renderer_evidence",
            )
        })
        .or_else(|| {
            contract
                .pointer("/embedded_terminal_tui_canary/renderer_evidence")
                .map(|value| {
                    (
                        value,
                        "host_contract.embedded_terminal_tui_canary.renderer_evidence",
                    )
                })
        });

    let source_contract_present = source_contract.is_some();
    let mut source_contract_source = source_contract
        .as_ref()
        .map(|(_, source)| (*source).to_string())
        .unwrap_or_else(|| "local_fallback".to_string());
    let mut source_contract_diagnostics = Vec::new();
    let mut source_contract_accepted = false;
    let mut drift_checks = vec![
        "required_snapshot_ids_present".to_string(),
        "snapshots_are_redacted".to_string(),
        "no_payload_storage".to_string(),
        "no_process_spawned".to_string(),
        "production_routing_stays_disabled".to_string(),
    ];
    let mut rollback_state = "safe_to_discard_renderer_evidence".to_string();
    let mut next_phase = if gate.enabled {
        "desktop_renderer_evidence_review".to_string()
    } else {
        "developer_canary_gate_opt_in".to_string()
    };
    let mut snapshots = renderer_evidence_fallback_snapshots(snapshot_status);

    if let Some((value, source)) = source_contract {
        let (candidate_snapshots, diagnostics) =
            renderer_evidence_snapshots_from_contract(value, snapshot_status);
        source_contract_diagnostics = diagnostics;
        let source_checks = value
            .get("drift_checks")
            .and_then(serde_json::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .map(str::to_string)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if !source_checks.is_empty() {
            drift_checks = source_checks;
        }
        if source_contract_diagnostics.is_empty() {
            source_contract_accepted = true;
            source_contract_source = source.to_string();
            snapshots = candidate_snapshots;
            rollback_state = value
                .get("rollback_state")
                .and_then(serde_json::Value::as_str)
                .unwrap_or("safe_to_discard_renderer_evidence")
                .to_string();
            next_phase = value
                .get("next_phase")
                .and_then(serde_json::Value::as_str)
                .unwrap_or(next_phase.as_str())
                .to_string();
        }
    }
    let all_snapshots_redacted = snapshots
        .iter()
        .all(|snapshot| snapshot.redacted && !snapshot.stores_payload && !snapshot.process_spawned);

    DxAgentsEmbeddedTerminalTuiCanaryRendererEvidence {
        schema_version: "dx.embedded_terminal_tui_canary_renderer_evidence.v1".to_string(),
        generated_at_ms: now_ms(),
        status: if gate.enabled {
            "capturable".to_string()
        } else {
            "gated".to_string()
        },
        gate_enabled: gate.enabled,
        process_spawned: false,
        source_contract_present,
        source_contract_accepted,
        source_contract_source,
        source_contract_diagnostics,
        renderer_contract_present,
        renderer_enabled,
        external_terminal_fallback: gate.production_terminal_surface == "external_terminal",
        production_routing_enabled: false,
        allows_arbitrary_shell: false,
        stores_payloads: false,
        snapshot_count: snapshots.len(),
        snapshots,
        rollback_state: if all_snapshots_redacted {
            rollback_state
        } else {
            "requires_manual_redaction_review".to_string()
        },
        drift_checks,
        next_phase: if all_snapshots_redacted && gate.enabled {
            next_phase
        } else {
            "developer_canary_gate_opt_in".to_string()
        },
    }
}

fn renderer_evidence_fallback_snapshots(
    snapshot_status: &str,
) -> Vec<DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot> {
    vec![
        tui_canary_renderer_snapshot(
            1,
            "renderer.alternate_screen",
            "alternate_screen",
            snapshot_status,
            "embedded_terminal_renderer.features.alternate_screen",
            "alternate_screen",
            120,
            34,
            1,
            1,
            0,
            true,
            true,
            false,
            "Alternate-screen state is tracked as metadata without copying TUI frame contents.",
        ),
        tui_canary_renderer_snapshot(
            2,
            "renderer.cursor_state",
            "cursor",
            snapshot_status,
            "embedded_terminal_renderer.features.cursor_state",
            "alternate_screen",
            120,
            34,
            12,
            24,
            0,
            true,
            false,
            false,
            "Cursor row, column, visibility, and restoration are recorded without terminal text.",
        ),
        tui_canary_renderer_snapshot(
            3,
            "renderer.scrollback",
            "scrollback",
            snapshot_status,
            "embedded_terminal_renderer.features.scrollback",
            "normal_screen",
            120,
            34,
            34,
            1,
            256,
            false,
            true,
            false,
            "Scrollback evidence stores bounded line counts only and does not retain output payloads.",
        ),
        tui_canary_renderer_snapshot(
            4,
            "renderer.resize_reflow",
            "resize",
            snapshot_status,
            "embedded_terminal_resize.events.renderer_reflow",
            "alternate_screen_resized",
            132,
            38,
            12,
            24,
            0,
            true,
            false,
            true,
            "Resize evidence verifies clamped dimensions and renderer reflow metadata.",
        ),
        tui_canary_renderer_snapshot(
            5,
            "renderer.interrupt",
            "interrupt",
            snapshot_status,
            "embedded_terminal_sessions.operations.interrupt",
            "interrupting",
            132,
            38,
            12,
            24,
            0,
            true,
            false,
            false,
            "Interrupt evidence must leave no child process and no unflushed renderer payload.",
        ),
        tui_canary_renderer_snapshot(
            6,
            "renderer.close",
            "close",
            snapshot_status,
            "embedded_terminal_sessions.operations.close",
            "closing",
            132,
            38,
            1,
            1,
            0,
            false,
            true,
            false,
            "Close evidence restores cursor and alternate-screen state before cleanup.",
        ),
        tui_canary_renderer_snapshot(
            7,
            "renderer.cleanup",
            "cleanup",
            snapshot_status,
            "embedded_terminal_sessions.operations.close",
            "clean",
            120,
            34,
            1,
            1,
            0,
            false,
            true,
            false,
            "Cleanup evidence verifies no orphaned renderer state, no stored payloads, and fallback routing unchanged.",
        ),
    ]
}

fn renderer_evidence_snapshots_from_contract(
    value: &serde_json::Value,
    fallback_status: &str,
) -> (
    Vec<DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot>,
    Vec<String>,
) {
    let mut diagnostics = Vec::new();
    if !value
        .get("supported")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push("renderer_evidence_contract_not_supported".to_string());
    }
    if value
        .get("schema_version")
        .and_then(serde_json::Value::as_str)
        != Some("dx.embedded_terminal_tui_canary_renderer_evidence.v1")
    {
        diagnostics.push("renderer_evidence_schema_drift".to_string());
    }
    if value
        .get("production_routing_enabled")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push("renderer_evidence_production_routing_enabled".to_string());
    }
    if value
        .get("allows_arbitrary_shell")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push("renderer_evidence_arbitrary_shell_enabled".to_string());
    }
    if value
        .get("stores_payloads")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push("renderer_evidence_payload_storage_enabled".to_string());
    }
    if value
        .get("process_spawned")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
    {
        diagnostics.push("renderer_evidence_process_spawned".to_string());
    }
    if value
        .get("rollback_state")
        .and_then(serde_json::Value::as_str)
        .is_some_and(|state| state != "safe_to_discard_renderer_evidence")
    {
        diagnostics.push("renderer_evidence_rollback_state_drift".to_string());
    }

    let snapshots = value
        .get("snapshots")
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .enumerate()
                .map(|(index, snapshot)| {
                    renderer_evidence_snapshot_from_value(snapshot, index + 1, fallback_status)
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    if snapshots.len() < 7 {
        diagnostics.push("renderer_evidence_snapshot_count_incomplete".to_string());
    }
    for required_id in [
        "renderer.alternate_screen",
        "renderer.cursor_state",
        "renderer.scrollback",
        "renderer.resize_reflow",
        "renderer.interrupt",
        "renderer.close",
        "renderer.cleanup",
    ] {
        if !snapshots
            .iter()
            .any(|snapshot| snapshot.snapshot_id == required_id)
        {
            diagnostics.push(format!("renderer_evidence_missing_{required_id}"));
        }
    }
    if snapshots.iter().any(|snapshot| !snapshot.redacted) {
        diagnostics.push("renderer_evidence_snapshot_not_redacted".to_string());
    }
    if snapshots.iter().any(|snapshot| snapshot.stores_payload) {
        diagnostics.push("renderer_evidence_snapshot_stores_payload".to_string());
    }
    if snapshots.iter().any(|snapshot| snapshot.process_spawned) {
        diagnostics.push("renderer_evidence_snapshot_spawned_process".to_string());
    }

    (snapshots, diagnostics)
}

fn renderer_evidence_snapshot_from_value(
    value: &serde_json::Value,
    fallback_step: usize,
    fallback_status: &str,
) -> DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot {
    DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot {
        step: value
            .get("step")
            .and_then(serde_json::Value::as_u64)
            .and_then(|step| u8::try_from(step).ok())
            .unwrap_or(fallback_step as u8),
        snapshot_id: value
            .get("snapshot_id")
            .or_else(|| value.get("event_id"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("renderer.unknown")
            .to_string(),
        phase: value
            .get("phase")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
            .to_string(),
        status: value
            .get("status")
            .and_then(serde_json::Value::as_str)
            .unwrap_or(fallback_status)
            .to_string(),
        source: value
            .get("source")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("embedded_terminal_tui_canary_renderer_evidence.snapshots")
            .to_string(),
        terminal_state: value
            .get("terminal_state")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("metadata_only")
            .to_string(),
        columns: json_u16(value, "columns", 120),
        rows: json_u16(value, "rows", 34),
        cursor_row: json_u16(value, "cursor_row", 1),
        cursor_column: json_u16(value, "cursor_column", 1),
        scrollback_lines: json_u16(value, "scrollback_lines", 0),
        alternate_screen: value
            .get("alternate_screen")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        cursor_visible: value
            .get("cursor_visible")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(true),
        renderer_reflow_required: value
            .get("renderer_reflow_required")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        process_spawned: value
            .get("process_spawned")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        redacted: value
            .get("redacted")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        stores_payload: value
            .get("stores_payload")
            .or_else(|| value.get("payload_stored"))
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        detail: value
            .get("detail")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("Metadata-only renderer evidence; no terminal text, frame contents, or shell input is stored.")
            .to_string(),
    }
}

fn json_u16(value: &serde_json::Value, key: &str, fallback: u16) -> u16 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .and_then(|number| u16::try_from(number).ok())
        .unwrap_or(fallback)
}

#[allow(clippy::too_many_arguments)]
fn tui_canary_renderer_snapshot(
    step: u8,
    snapshot_id: &str,
    phase: &str,
    status: &str,
    source: &str,
    terminal_state: &str,
    columns: u16,
    rows: u16,
    cursor_row: u16,
    cursor_column: u16,
    scrollback_lines: u16,
    alternate_screen: bool,
    cursor_visible: bool,
    renderer_reflow_required: bool,
    detail: &str,
) -> DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot {
    DxAgentsEmbeddedTerminalTuiCanaryRendererSnapshot {
        step,
        snapshot_id: snapshot_id.to_string(),
        phase: phase.to_string(),
        status: status.to_string(),
        source: source.to_string(),
        terminal_state: terminal_state.to_string(),
        columns,
        rows,
        cursor_row,
        cursor_column,
        scrollback_lines,
        alternate_screen,
        cursor_visible,
        renderer_reflow_required,
        process_spawned: false,
        redacted: true,
        stores_payload: false,
        detail: detail.to_string(),
    }
}

fn tui_canary_lifecycle_event(
    step: u8,
    event_id: &str,
    status: &str,
    timeout_ms: u64,
    cleanup_required: bool,
    detail: &str,
) -> DxAgentsEmbeddedTerminalTuiCanaryLifecycleEvent {
    DxAgentsEmbeddedTerminalTuiCanaryLifecycleEvent {
        step,
        event_id: event_id.to_string(),
        phase: event_id
            .strip_prefix("tui.")
            .unwrap_or(event_id)
            .to_string(),
        status: status.to_string(),
        timeout_ms,
        cleanup_required,
        detail: detail.to_string(),
        redacted: true,
    }
}

fn synthetic_session_event(
    step: u8,
    event_id: &str,
    kind: &str,
    source: &str,
    detail: impl Into<String>,
) -> DxAgentsEmbeddedTerminalSessionEvent {
    DxAgentsEmbeddedTerminalSessionEvent {
        event_id: event_id.to_string(),
        step,
        kind: kind.to_string(),
        status: "ok".to_string(),
        source: source.to_string(),
        detail: detail.into(),
        redacted: true,
    }
}

fn contract_schema_is(contract: &serde_json::Value, pointer: &str, expected: &str) -> bool {
    contract
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
        == Some(expected)
}

fn contract_detail(contract: &serde_json::Value, pointer: &str) -> String {
    match contract
        .pointer(pointer)
        .and_then(serde_json::Value::as_str)
    {
        Some(schema) => format!("Contract schema `{schema}` is present."),
        None => format!("Contract schema is missing at `{pointer}`."),
    }
}

fn input_redaction_is_safe(contract: &serde_json::Value) -> bool {
    contract
        .pointer("/embedded_terminal_input/redaction/stores_payloads")
        .and_then(serde_json::Value::as_bool)
        == Some(false)
        && contract
            .pointer("/embedded_terminal_input/redaction/stores_text_values")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
}

fn readiness_evidence(
    id: &str,
    label: &str,
    ok: bool,
    detail: String,
) -> DxAgentsEmbeddedTerminalReadinessEvidence {
    DxAgentsEmbeddedTerminalReadinessEvidence {
        id: id.to_string(),
        label: label.to_string(),
        status: if ok { "ok" } else { "warn" }.to_string(),
        detail,
        redacted: true,
    }
}

fn read_json_file(path: &Path) -> Result<serde_json::Value, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

fn toml_string_value(path: &Path, section: &str, key: &str) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    let mut in_section = false;
    let section_header = format!("[{section}]");
    let key_prefix = format!("{key} =");

    for line in text.lines().map(str::trim) {
        if line.starts_with('[') && line.ends_with(']') {
            in_section = line == section_header;
            continue;
        }
        if !in_section || !line.starts_with(&key_prefix) {
            continue;
        }
        let (_, value) = line.split_once('=')?;
        return Some(value.trim().trim_matches('"').to_string());
    }

    None
}

fn redact_sensitive_text(input: &str) -> String {
    let mut output = input.to_string();
    for prefix in [
        "gsk_",
        "sk-",
        "sk-proj-",
        "sk-ant-",
        "xai-",
        "ghp_",
        "AIza",
        "token=",
        "api_key=",
        "secret=",
        "password=",
    ] {
        output = redact_token_prefix(&output, prefix);
    }
    redact_windows_paths(&output)
}

fn redact_token_prefix(input: &str, prefix: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(index) = rest.find(prefix) {
        let (before, after_before) = rest.split_at(index);
        output.push_str(before);

        let token_len = after_before
            .char_indices()
            .take_while(|(_, ch)| {
                !ch.is_whitespace()
                    && !matches!(*ch, '"' | '\'' | '`' | ',' | ';' | ')' | ']' | '}')
            })
            .map(|(idx, ch)| idx + ch.len_utf8())
            .last()
            .unwrap_or(prefix.len());
        let (_, after_token) = after_before.split_at(token_len);
        output.push_str("[redacted]");
        rest = after_token;
    }

    output.push_str(rest);
    output
}

fn redact_windows_paths(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut rest = input;

    while let Some(index) = windows_path_start(rest) {
        let (before, after_before) = rest.split_at(index);
        output.push_str(before);

        let path_len = after_before
            .char_indices()
            .take_while(|(_, ch)| {
                !ch.is_whitespace()
                    && !matches!(*ch, '"' | '\'' | '`' | ',' | ';' | ')' | ']' | '}')
            })
            .map(|(idx, ch)| idx + ch.len_utf8())
            .last()
            .unwrap_or(after_before.len());
        output.push_str("[redacted-path]");
        rest = &after_before[path_len..];
    }

    output.push_str(rest);
    output
}

fn windows_path_start(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    bytes.windows(3).position(|window| {
        window[0].is_ascii_alphabetic() && window[1] == b':' && matches!(window[2], b'\\' | b'/')
    })
}

fn run_json_action(action_id: String, input: Option<String>) -> Result<serde_json::Value, String> {
    let result = run_captured_action(action_id, input)?;
    if result.output != "json" {
        return Err(format!(
            "DX action does not produce JSON: {}",
            result.action_id
        ));
    }

    if !result.success {
        return Err(format!(
            "{} failed: {}",
            result.action_id,
            result.stderr.trim()
        ));
    }

    serde_json::from_str(&result.stdout)
        .map_err(|error| format!("{} returned invalid JSON: {error}", result.action_id))
}

fn run_captured_action(action_id: String, input: Option<String>) -> Result<DxCliRunResult, String> {
    let contract = load_contract()?;
    let action = find_action(&contract, &action_id)?;
    ensure_capturable_action(action)?;

    let argv = argv_for_action(action, input.as_deref())?;
    let (program, args) = argv
        .split_first()
        .ok_or_else(|| format!("DX action has no argv: {}", action.id))?;

    let started = Instant::now();
    let output = Command::new(program)
        .args(args)
        .current_dir(&action.cwd)
        .output();

    let output = match output {
        Ok(output) => output,
        Err(error) => {
            let record = telemetry_record_for_error(
                action,
                &argv,
                "captured_output",
                "failed",
                started.elapsed().as_millis() as u64,
                input.is_some(),
                format!("failed to run {}: {error}", action.id),
            );
            let _ = append_telemetry_record(&contract, &record);
            return Err(format!("failed to run {}: {error}", action.id));
        }
    };

    let result = DxCliRunResult {
        action_id: action.id.clone(),
        label: action.label.clone(),
        output: action.output.clone(),
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        argv,
        cwd: action.cwd.clone(),
    };
    let record = telemetry_record_for_run(
        action,
        &result,
        started.elapsed().as_millis() as u64,
        input.is_some(),
    );
    let _ = append_telemetry_record(&contract, &record);

    Ok(result)
}

fn launch_terminal_action(
    action_id: String,
    input: Option<String>,
) -> Result<DxCliLaunchResult, String> {
    let contract = load_contract()?;
    let action = find_action(&contract, &action_id)?;
    if !action.requires_terminal {
        return Err(format!(
            "DX action does not require a terminal pane: {}",
            action.id
        ));
    }

    let action_argv = argv_for_action(action, input.as_deref())?;
    let cwd = PathBuf::from(&action.cwd);
    let started = Instant::now();
    let launch = if cfg!(windows) {
        let terminal_argv = windows_terminal_argv(action, &action_argv);
        spawn_process("wt.exe", &terminal_argv, &cwd).map(|child| {
            let mut argv = vec!["wt.exe".to_string()];
            argv.extend(terminal_argv);
            (child, argv, true)
        })
    } else {
        Err("Windows Terminal is not available on this platform".into())
    };

    let (child, argv, launched_in_terminal) = match launch {
        Ok(result) => result,
        Err(terminal_error) => {
            let (program, args) = action_argv
                .split_first()
                .ok_or_else(|| format!("DX action has no argv: {}", action.id))?;
            let child = spawn_process(program, args, &cwd).map_err(|fallback_error| {
                let message = format!(
                    "failed to launch {} with Windows Terminal ({terminal_error}); fallback failed: {fallback_error}",
                    action.id
                );
                let record = telemetry_record_for_error(
                    action,
                    &action_argv,
                    "external_terminal",
                    "failed",
                    started.elapsed().as_millis() as u64,
                    input.is_some(),
                    message.clone(),
                );
                let _ = append_telemetry_record(&contract, &record);
                message
            })?;
            (child, action_argv, false)
        }
    };

    let result = DxCliLaunchResult {
        action_id: action.id.clone(),
        label: action.label.clone(),
        pid: child.id(),
        argv,
        cwd: action.cwd.clone(),
        launched_in_terminal,
    };
    let record = telemetry_record_for_launch(
        action,
        &result,
        started.elapsed().as_millis() as u64,
        input.is_some(),
    );
    let _ = append_telemetry_record(&contract, &record);

    Ok(result)
}

fn spawn_process(
    program: &str,
    args: &[String],
    cwd: &Path,
) -> Result<std::process::Child, String> {
    Command::new(program)
        .args(args)
        .current_dir(cwd)
        .spawn()
        .map_err(|error| format!("failed to spawn {program}: {error}"))
}

fn windows_terminal_argv(action: &DxCliAction, action_argv: &[String]) -> Vec<String> {
    let mut argv = vec![
        "new-tab".to_string(),
        "--title".to_string(),
        terminal_title(&action.label),
        "-d".to_string(),
        action.cwd.clone(),
    ];
    argv.extend(action_argv.iter().cloned());
    argv
}

fn terminal_title(label: &str) -> String {
    if label.starts_with("DX ") {
        label.to_string()
    } else {
        format!("DX {label}")
    }
}

fn run_dx_agents_dashboard_command(
    label: &str,
    args: &[&str],
) -> Result<DxAgentsDashboardCommand, String> {
    let program = dx_agents_cli_program();
    let cwd = dx_agents_repo_dir();
    let output = Command::new(&program)
        .args(args)
        .current_dir(&cwd)
        .output()
        .map_err(|error| format!("failed to run {}: {error}", program.display()))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let (json, json_error) = parse_optional_json(&stdout);
    let recovery_hint = dashboard_recovery_hint(
        label,
        args,
        output.status.success(),
        &stderr,
        json_error.as_deref(),
    );
    let mut argv = vec![program.display().to_string()];
    argv.extend(args.iter().map(|arg| (*arg).to_string()));

    Ok(DxAgentsDashboardCommand {
        label: label.to_string(),
        success: output.status.success(),
        exit_code: output.status.code(),
        stdout,
        stderr,
        json,
        json_error,
        recovery_hint,
        argv,
        cwd: cwd.display().to_string(),
    })
}

fn dashboard_recovery_hint(
    label: &str,
    args: &[&str],
    success: bool,
    stderr: &str,
    json_error: Option<&str>,
) -> Option<String> {
    if success && json_error.is_none() {
        return None;
    }

    let command = args.join(" ");
    let label = label.to_ascii_lowercase();
    let stderr = stderr.to_ascii_lowercase();

    if command.starts_with("models health") {
        return Some(
            "Check provider profile config and API-key environment, then rerun `dx-agent models health --mode dry-run --json` before trying live mode."
                .to_string(),
        );
    }
    if command.starts_with("agent ") || label.contains("provider live smoke") {
        return Some(
            "Confirm the `dx` agent exists and the selected provider/model works with `dx-agent models health --mode live --json`, then retry the live smoke."
                .to_string(),
        );
    }
    if command.starts_with("gateway get-paircode") || label.contains("gateway") {
        return Some(
            "Start the gateway with `dx-agent gateway start`, then request a fresh pairing code with `dx-agent gateway get-paircode --new`."
                .to_string(),
        );
    }
    if command.starts_with("cron history") {
        return Some(
            "Run `dx-agent cron list` to confirm jobs exist, then inspect scheduler output or run history after the next job completes."
                .to_string(),
        );
    }
    if command.starts_with("cron preview") {
        return Some(
            "Run `dx-agent cron list` and verify the cron database/config path before using scheduler preview."
                .to_string(),
        );
    }
    if command.starts_with("status --compact") {
        return Some(
            "Run `dx-agent status --compact --json` in a terminal to see the failing subsystem and confirm config paths are readable."
                .to_string(),
        );
    }
    if command.starts_with("workloop status") {
        return Some(
            "Check `TODO.md`, `CHANGELOG.md`, and the continuation journal path, then rerun `dx-agent workloop status --json`."
                .to_string(),
        );
    }
    if json_error.is_some() {
        return Some(
            "The command ran but returned invalid JSON; rerun it in a terminal and inspect stdout for non-JSON diagnostics."
                .to_string(),
        );
    }
    if stderr.contains("not found") || stderr.contains("not recognized") {
        return Some(
            "Confirm the DX Agents debug binary is built or set `DX_AGENTS_CLI` to the intended executable."
                .to_string(),
        );
    }

    Some("Rerun the command in a terminal with the same arguments and inspect stderr for the failing subsystem.".to_string())
}

fn parse_optional_json(stdout: &str) -> (Option<serde_json::Value>, Option<String>) {
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        return (None, None);
    }

    match serde_json::from_str(trimmed) {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error.to_string())),
    }
}

fn telemetry_record_for_run(
    action: &DxCliAction,
    result: &DxCliRunResult,
    duration_ms: u64,
    input_supplied: bool,
) -> DxCliCommandRunRecord {
    DxCliCommandRunRecord {
        schema_version: "dx.host_telemetry.v1".to_string(),
        recorded_at_ms: now_ms(),
        action_id: action.id.clone(),
        label: action.label.clone(),
        surface: "captured_output".to_string(),
        status: if result.success { "success" } else { "failed" }.to_string(),
        output: action.output.clone(),
        duration_ms,
        exit_code: result.exit_code,
        pid: None,
        launched_in_terminal: None,
        input_supplied,
        argv_program: result.argv.first().cloned(),
        argv_arg_count: result.argv.len().saturating_sub(1),
        cwd: action.cwd.clone(),
        error_summary: (!result.success).then(|| summarize_error(&result.stderr)),
    }
}

fn telemetry_record_for_launch(
    action: &DxCliAction,
    result: &DxCliLaunchResult,
    duration_ms: u64,
    input_supplied: bool,
) -> DxCliCommandRunRecord {
    DxCliCommandRunRecord {
        schema_version: "dx.host_telemetry.v1".to_string(),
        recorded_at_ms: now_ms(),
        action_id: action.id.clone(),
        label: action.label.clone(),
        surface: "external_terminal".to_string(),
        status: if result.launched_in_terminal {
            "launched"
        } else {
            "fallback_launched"
        }
        .to_string(),
        output: action.output.clone(),
        duration_ms,
        exit_code: None,
        pid: Some(result.pid),
        launched_in_terminal: Some(result.launched_in_terminal),
        input_supplied,
        argv_program: result.argv.first().cloned(),
        argv_arg_count: result.argv.len().saturating_sub(1),
        cwd: action.cwd.clone(),
        error_summary: None,
    }
}

fn telemetry_record_for_error(
    action: &DxCliAction,
    argv: &[String],
    surface: &str,
    status: &str,
    duration_ms: u64,
    input_supplied: bool,
    error: String,
) -> DxCliCommandRunRecord {
    DxCliCommandRunRecord {
        schema_version: "dx.host_telemetry.v1".to_string(),
        recorded_at_ms: now_ms(),
        action_id: action.id.clone(),
        label: action.label.clone(),
        surface: surface.to_string(),
        status: status.to_string(),
        output: action.output.clone(),
        duration_ms,
        exit_code: None,
        pid: None,
        launched_in_terminal: None,
        input_supplied,
        argv_program: argv.first().cloned(),
        argv_arg_count: argv.len().saturating_sub(1),
        cwd: action.cwd.clone(),
        error_summary: Some(summarize_error(&error)),
    }
}

fn append_telemetry_record(
    contract: &DxCliHostContract,
    record: &DxCliCommandRunRecord,
) -> Result<(), String> {
    let Some(path) = telemetry_history_path(contract) else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create telemetry directory: {error}"))?;
    }
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|error| {
            format!(
                "failed to open telemetry history {}: {error}",
                path.display()
            )
        })?;
    serde_json::to_writer(&mut file, record)
        .map_err(|error| format!("failed to serialize telemetry record: {error}"))?;
    writeln!(file)
        .map_err(|error| format!("failed to append telemetry history newline: {error}"))?;
    Ok(())
}

fn read_command_history(
    contract: &DxCliHostContract,
    limit: usize,
) -> Result<DxCliCommandHistory, String> {
    let path = telemetry_history_path(contract).unwrap_or_else(|| {
        PathBuf::from(&contract.workspace)
            .join("target")
            .join("host-telemetry")
            .join("dx-command-runs.jsonl")
    });

    if !path.is_file() {
        return Ok(DxCliCommandHistory {
            history_path: path.display().to_string(),
            count: 0,
            entries: Vec::new(),
        });
    }

    let source = fs::read_to_string(&path).map_err(|error| {
        format!(
            "failed to read telemetry history {}: {error}",
            path.display()
        )
    })?;
    let entries = source
        .lines()
        .rev()
        .filter_map(|line| serde_json::from_str::<DxCliCommandRunRecord>(line).ok())
        .take(limit)
        .collect::<Vec<_>>();

    Ok(DxCliCommandHistory {
        history_path: path.display().to_string(),
        count: entries.len(),
        entries,
    })
}

fn telemetry_history_path(contract: &DxCliHostContract) -> Option<PathBuf> {
    contract
        .telemetry
        .as_ref()
        .filter(|telemetry| telemetry.supported && telemetry.storage == "jsonl")
        .map(|telemetry| PathBuf::from(&telemetry.history_path))
}

fn default_bridge_settings_for_contract(contract: &DxCliHostContract) -> DxCliBridgeSettings {
    contract
        .settings
        .as_ref()
        .filter(|settings| settings.supported && settings.persistence == "host_app_store")
        .map(|settings| settings.defaults.clone())
        .unwrap_or_else(default_bridge_settings)
}

fn default_bridge_settings() -> DxCliBridgeSettings {
    DxCliBridgeSettings {
        media_input: PathBuf::from(DEFAULT_DX_CLI_ROOT)
            .join("samples")
            .join("dx-smoke.mp4")
            .display()
            .to_string(),
        preferred_terminal_surface: "external_terminal".to_string(),
        command_history_limit: 6,
        safe_launch_policy: "confirm_external_terminal".to_string(),
        provider_health_mode: "dry-run".to_string(),
    }
}

fn normalize_bridge_settings(
    contract: &DxCliHostContract,
    mut settings: DxCliBridgeSettings,
) -> DxCliBridgeSettings {
    let defaults = default_bridge_settings_for_contract(contract);
    let constraints = contract
        .settings
        .as_ref()
        .map(|settings| &settings.constraints);

    if settings.media_input.trim().is_empty() {
        settings.media_input = defaults.media_input.clone();
    }

    if !constraints
        .map(|constraints| {
            constraints
                .terminal_surfaces
                .iter()
                .any(|surface| surface == &settings.preferred_terminal_surface)
        })
        .unwrap_or(matches!(
            settings.preferred_terminal_surface.as_str(),
            "external_terminal" | "captured_output" | "embedded_pty"
        ))
    {
        settings.preferred_terminal_surface = defaults.preferred_terminal_surface.clone();
    }

    let min = constraints
        .map(|constraints| constraints.command_history_limit_min)
        .unwrap_or(1)
        .max(1);
    let max = constraints
        .map(|constraints| constraints.command_history_limit_max)
        .unwrap_or(50)
        .max(min);
    settings.command_history_limit = settings.command_history_limit.clamp(min, max);

    if !constraints
        .map(|constraints| {
            constraints
                .safe_launch_policies
                .iter()
                .any(|policy| policy == &settings.safe_launch_policy)
        })
        .unwrap_or(matches!(
            settings.safe_launch_policy.as_str(),
            "confirm_external_terminal" | "direct_external_terminal"
        ))
    {
        settings.safe_launch_policy = defaults.safe_launch_policy.clone();
    }

    if !constraints
        .map(|constraints| {
            constraints
                .provider_health_modes
                .iter()
                .any(|mode| mode == &settings.provider_health_mode)
        })
        .unwrap_or(matches!(
            settings.provider_health_mode.as_str(),
            "mock" | "dry-run" | "live"
        ))
    {
        settings.provider_health_mode = defaults.provider_health_mode;
    }

    settings
}

fn summarize_error(error: &str) -> String {
    let trimmed = error.trim();
    if trimmed.chars().count() <= 240 {
        return trimmed.to_string();
    }
    trimmed.chars().take(240).collect()
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

fn system_time_ms(value: SystemTime) -> u64 {
    value
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

fn load_contract() -> Result<DxCliHostContract, String> {
    load_contract_from_path(&contract_path(&dx_cli_root()))
}

fn load_contract_value() -> Result<serde_json::Value, String> {
    load_contract_value_from_path(&contract_path(&dx_cli_root()))
}

fn load_contract_value_from_path(path: &Path) -> Result<serde_json::Value, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let mut value: serde_json::Value = serde_json::from_str(&source)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    serde_json::from_value::<DxCliHostContract>(value.clone())
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    value["dashboard_compatibility"] = dashboard_compatibility_contract();
    Ok(value)
}

fn dashboard_compatibility_contract() -> serde_json::Value {
    serde_json::json!({
        "supported": true,
        "schema_version": "dx.dashboard_compatibility.v1",
        "source": "dx-agent.host_contract",
        "product_name": "DX Agents",
        "package_name": "dx-agent-web",
        "legacy_product_names": ["DX Agent"],
        "compatibility_policy": {
            "legacy_readable": true,
            "legacy_writable": true,
            "exposes_stored_values": false,
            "cleanup_requires_migration_plan": true
        },
        "env_aliases": [
            {
                "label": "Gateway port env",
                "primary": "DX_AGENTS_GATEWAY_PORT",
                "legacy": ["DX_AGENT_GATEWAY_PORT"]
            }
        ],
        "window_globals": [
            {
                "label": "Dashboard base path",
                "primary": "__DX_AGENTS_BASE__",
                "legacy": ["__DX_AGENT_BASE__"]
            },
            {
                "label": "Gateway base URL",
                "primary": "__DX_AGENTS_GATEWAY__",
                "legacy": ["__DX_AGENT_GATEWAY__"]
            }
        ],
        "storage_aliases": [
            {
                "label": "Auth token storage",
                "scope": "localStorage",
                "primary": "dx_agents_token",
                "legacy": ["zeroclaw_token"],
                "sensitive": true
            },
            {
                "label": "Session storage",
                "scope": "localStorage",
                "primary": "dx_agents_session_id",
                "legacy": ["zeroclaw_session_id"],
                "sensitive": false
            },
            {
                "label": "Chat history storage",
                "scope": "localStorage",
                "primary": "dx_agents_chat_history_v1:",
                "legacy": ["zeroclaw_chat_history_v1:"],
                "sensitive": true
            },
            {
                "label": "Chat compact preference",
                "scope": "localStorage",
                "primary": "dx_agents_chat_compact",
                "legacy": ["zeroclaw_chat_compact"],
                "sensitive": false
            },
            {
                "label": "Tool activity preference",
                "scope": "localStorage",
                "primary": "dx_agents_show_tool_activity",
                "legacy": ["zeroclaw_show_tool_activity"],
                "sensitive": false
            },
            {
                "label": "Theme storage",
                "scope": "localStorage",
                "primary": "dx_agents-theme",
                "legacy": ["zeroclaw-theme"],
                "sensitive": false
            },
            {
                "label": "Locale storage",
                "scope": "localStorage",
                "primary": "dx_agents-locale",
                "legacy": ["zeroclaw-locale"],
                "sensitive": false
            },
            {
                "label": "Sidebar collapse state",
                "scope": "localStorage",
                "primary": "dx_agents-sidebar-collapsed",
                "legacy": ["zeroclaw-sidebar-collapsed"],
                "sensitive": false
            },
            {
                "label": "Live log storage",
                "scope": "sessionStorage",
                "primary": "dx_agents_live_logs",
                "legacy": ["zeroclaw_live_logs"],
                "sensitive": false
            }
        ],
        "event_aliases": [
            {
                "label": "Unauthorized event",
                "primary": "dx-agent-unauthorized",
                "legacy": ["zeroclaw-unauthorized"]
            }
        ],
        "websocket_protocols": [
            {
                "label": "Chat WebSocket protocol",
                "route": "/ws/chat",
                "primary": "dx-agent.v1",
                "legacy": ["zeroclaw.v1"]
            },
            {
                "label": "Canvas WebSocket protocol",
                "route": "/ws/canvas/:id",
                "primary": "dx-agent.v1",
                "legacy": ["zeroclaw.v1"]
            }
        ]
    })
}

fn load_contract_from_path(path: &Path) -> Result<DxCliHostContract, String> {
    let source = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    serde_json::from_str(&source)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))
}

fn find_action<'a>(
    contract: &'a DxCliHostContract,
    action_id: &str,
) -> Result<&'a DxCliAction, String> {
    contract
        .actions
        .iter()
        .find(|action| action.id == action_id)
        .filter(|action| action.enabled)
        .ok_or_else(|| format!("DX action is missing or disabled: {action_id}"))
}

fn ensure_capturable_action(action: &DxCliAction) -> Result<(), String> {
    if action.requires_terminal {
        return Err(format!("DX action requires a terminal pane: {}", action.id));
    }

    if !matches!(action.output.as_str(), "json" | "human_text") {
        return Err(format!(
            "DX action is not safe for captured run: {} ({})",
            action.id, action.output
        ));
    }

    Ok(())
}

fn count_actions(
    contract: Option<&DxCliHostContract>,
    predicate: impl Fn(&DxCliAction) -> bool,
) -> usize {
    contract
        .map(|contract| {
            contract
                .actions
                .iter()
                .filter(|action| predicate(action))
                .count()
        })
        .unwrap_or_default()
}

fn host_menu(contract: &DxCliHostContract) -> DxCliHostMenu {
    let groups = menu_groups(contract);
    let enabled_action_count = groups.iter().map(|group| group.enabled_action_count).sum();

    DxCliHostMenu {
        workspace: contract.workspace.clone(),
        group_count: groups.len(),
        action_count: contract.actions.len(),
        enabled_action_count,
        groups,
    }
}

fn menu_groups(contract: &DxCliHostContract) -> Vec<DxCliMenuGroup> {
    let mut seen_groups = HashSet::new();
    let mut groups = contract
        .action_groups
        .iter()
        .map(|group| {
            seen_groups.insert(group.id.as_str());
            menu_group(contract, group)
        })
        .collect::<Vec<_>>();

    let unknown_group_ids = contract
        .actions
        .iter()
        .filter(|action| !seen_groups.contains(action.group.as_str()))
        .map(|action| action.group.as_str())
        .collect::<HashSet<_>>();

    for group_id in unknown_group_ids {
        let fallback = DxCliActionGroup {
            id: group_id.to_string(),
            label: group_id.to_string(),
            description: "Actions discovered without explicit group metadata.".to_string(),
        };
        groups.push(menu_group(contract, &fallback));
    }

    groups
        .into_iter()
        .filter(|group| group.action_count > 0)
        .collect()
}

fn menu_group(contract: &DxCliHostContract, group: &DxCliActionGroup) -> DxCliMenuGroup {
    let actions = contract
        .actions
        .iter()
        .filter(|action| action.group == group.id)
        .map(|action| DxCliMenuAction {
            action_id: action.id.clone(),
            label: action.label.clone(),
            description: action.description.clone(),
            output: action.output.clone(),
            requires_terminal: action.requires_terminal,
            accepts_input: action.accepts_input,
            enabled: action.enabled,
        })
        .collect::<Vec<_>>();

    DxCliMenuGroup {
        id: group.id.clone(),
        label: group.label.clone(),
        description: group.description.clone(),
        action_count: actions.len(),
        enabled_action_count: actions.iter().filter(|action| action.enabled).count(),
        terminal_action_count: actions
            .iter()
            .filter(|action| action.enabled && action.requires_terminal)
            .count(),
        json_action_count: actions
            .iter()
            .filter(|action| action.enabled && action.output == "json")
            .count(),
        input_action_count: actions
            .iter()
            .filter(|action| action.enabled && action.accepts_input)
            .count(),
        actions,
    }
}

fn quick_terminal_actions(contract: &DxCliHostContract) -> Vec<DxCliQuickAction> {
    contract
        .actions
        .iter()
        .filter(|action| action.enabled && action.requires_terminal && !action.accepts_input)
        .map(|action| DxCliQuickAction {
            action_id: action.id.clone(),
            label: action.label.clone(),
            group: action.group.clone(),
            accepts_input: action.accepts_input,
        })
        .collect()
}

fn quick_captured_actions(contract: &DxCliHostContract) -> Vec<DxCliQuickAction> {
    contract
        .actions
        .iter()
        .filter(|action| {
            action.enabled
                && !action.requires_terminal
                && !action.accepts_input
                && matches!(action.output.as_str(), "json" | "human_text")
        })
        .map(|action| DxCliQuickAction {
            action_id: action.id.clone(),
            label: action.label.clone(),
            group: action.group.clone(),
            accepts_input: action.accepts_input,
        })
        .collect()
}

fn argv_for_action(action: &DxCliAction, input: Option<&str>) -> Result<Vec<String>, String> {
    if action.accepts_input && input.is_none() {
        return Err(format!("DX action requires input: {}", action.id));
    }

    Ok(action
        .argv
        .iter()
        .map(|part| {
            if part == INPUT_TOKEN {
                input.unwrap_or_default().to_string()
            } else {
                part.clone()
            }
        })
        .collect())
}

fn dx_cli_root() -> PathBuf {
    env::var_os("DX_CLI_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DX_CLI_ROOT))
}

fn dx_agents_cli_program() -> PathBuf {
    if let Some(path) = env::var_os("DX_AGENTS_CLI").map(PathBuf::from) {
        return path;
    }

    for candidate in dx_agents_cli_candidates(&dx_agents_repo_dir()) {
        if candidate.is_file() {
            return candidate;
        }
    }

    PathBuf::from(dx_agents_cli_file_name())
}

fn dx_agents_cli_candidates(repo_dir: &Path) -> Vec<PathBuf> {
    let mut candidates = vec![
        repo_dir
            .join("target")
            .join("debug")
            .join(dx_agents_cli_file_name()),
        repo_dir
            .join("target")
            .join("release")
            .join(dx_agents_cli_file_name()),
    ];

    if let Ok(exe) = env::current_exe()
        && let Some(dir) = exe.parent()
    {
        candidates.push(dir.join(dx_agents_cli_file_name()));
    }

    candidates
}

fn dx_agents_repo_dir() -> PathBuf {
    if let Some(path) = env::var_os("DX_AGENTS_REPO_DIR").map(PathBuf::from) {
        return path;
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .unwrap_or(manifest_dir)
}

fn dx_agents_cli_file_name() -> &'static str {
    if cfg!(windows) {
        "dx-agent.exe"
    } else {
        "dx-agent"
    }
}

fn contract_path(root: &Path) -> PathBuf {
    CONTRACT_RELATIVE_PATH
        .iter()
        .fold(root.to_path_buf(), |path, segment| path.join(segment))
}

#[cfg(test)]
mod tests {
    use super::{
        DASHBOARD_COMPATIBILITY_MIGRATION_PLAN, DX_CLI_IPC_COMMANDS, DxAgentsDashboardCommand,
        DxCliAction, DxCliActionGroup, DxCliBridgeSettings, DxCliHealth, DxCliHostContract,
        DxCliLauncher, DxCliNativePromotionArchiveEntry, DxCliRunResult, DxCliSettingsConstraints,
        DxCliSettingsContract, DxCliTelemetry, TOOL_SAFETY_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_HISTORY_LIMIT, TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK, TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK,
        TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK, TOOL_SAFETY_DRILL_HISTORY_LIMIT,
        append_provider_smoke_record, append_telemetry_record, append_tool_safety_audit_record,
        append_tool_safety_drill_record, argv_for_action, bool_delta,
        bridge_status_export_file_path, bridge_status_export_path, continuation_target_path,
        contract_path, dashboard_compatibility_contract, dashboard_compatibility_status,
        dashboard_compatibility_tokens, dashboard_compatibility_usage_path,
        dashboard_compatibility_usage_tokens, dashboard_recovery_hint,
        default_bridge_settings_for_contract, duplicate_command_names, dx_agents_cli_candidates,
        dx_agents_cli_file_name, dx_agents_compact_status_args, dx_agents_cron_delivery_drill_args,
        dx_agents_cron_history_args, dx_agents_cron_preview_args,
        dx_agents_embedded_terminal_echo_pilot_args,
        dx_agents_embedded_terminal_tui_canary_runner_args, dx_agents_gateway_paircode_args,
        dx_agents_gateway_pairing_drill_args, dx_agents_memory_skill_learning_args,
        dx_agents_provider_failover_drill_args, dx_agents_provider_health_args,
        dx_agents_provider_smoke_args, dx_agents_repo_dir, dx_agents_session_tool_routing_args,
        dx_agents_tool_safety_drill_args, dx_cli_bridge_self_test, embedded_terminal_fixtures,
        embedded_terminal_media_canary_evidence, embedded_terminal_readiness_export,
        embedded_terminal_session_timeline, embedded_terminal_tui_canary_gate,
        embedded_terminal_tui_canary_lifecycle, embedded_terminal_tui_canary_renderer_evidence,
        ensure_capturable_action, host_menu, is_bridge_status_export_file_name,
        is_native_promotion_archive_file_name, list_native_promotion_archives,
        load_contract_value_from_path, native_promotion_archive_diff_from_entries,
        native_promotion_archive_entry, native_promotion_archive_file_path,
        native_promotion_archive_status, native_promotion_archive_trend_runbook,
        native_promotion_archive_trend_runbook_is_safe, normalize_bridge_settings, now_ms,
        parse_optional_json, provider_smoke_record, quick_captured_actions, quick_terminal_actions,
        read_command_history, read_provider_smoke_history, read_tool_safety_audit_history,
        read_tool_safety_drill_history, readiness_item, redact_sensitive_text,
        release_readiness_report, run_embedded_terminal_echo_pilot,
        run_embedded_terminal_tui_canary_runner, telemetry_record_for_run,
        tool_safety_alert_runbook_is_safe, tool_safety_audit_digest_from_parts,
        tool_safety_audit_escalation_evidence, tool_safety_audit_history_export_path,
        tool_safety_audit_history_path, tool_safety_audit_recovery_alert_digest,
        tool_safety_audit_recovery_alert_digest_release_gate,
        tool_safety_audit_recovery_alert_digest_release_gate_digest,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts,
        tool_safety_audit_recovery_alert_digest_release_gate_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_release_gate_runbook_is_safe,
        tool_safety_audit_recovery_alert_digest_runbook_is_safe,
        tool_safety_audit_recovery_alert_runbook_is_safe, tool_safety_audit_recovery_digest,
        tool_safety_audit_recovery_digest_alerts, tool_safety_audit_recovery_drill,
        tool_safety_audit_recovery_runbook_is_safe, tool_safety_audit_review_alert_runbook_is_safe,
        tool_safety_audit_review_alerts, tool_safety_audit_review_runbook_is_safe,
        tool_safety_audit_summary, tool_safety_drill_alerts, tool_safety_drill_history_export_path,
        tool_safety_drill_record, tool_safety_drill_trend, tui_canary_env_enabled,
        tui_canary_runner_contract, windows_terminal_argv,
    };
    use std::{
        collections::HashSet,
        fs,
        path::{Path, PathBuf},
    };

    #[test]
    fn builds_default_contract_path() {
        assert_eq!(
            contract_path(r"G:\Cli".as_ref()).display().to_string(),
            r"G:\Cli\target\host-contract\dx-host-contract.json"
        );
    }

    #[test]
    fn host_contract_value_preserves_embedded_terminal_protocol_fields() {
        let dir = std::env::temp_dir().join(format!("dx-host-contract-value-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dx-host-contract.json");
        let history = dir.join("history.jsonl");
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_terminal_input"] = serde_json::json!({
            "supported": true,
            "schema_version": "dx.embedded_terminal_input.v1",
            "enabled": false,
            "events": [
                { "id": "keyboard", "enabled": false, "required": true }
            ]
        });
        contract["embedded_terminal_resize"] = serde_json::json!({
            "supported": true,
            "schema_version": "dx.embedded_terminal_resize.v1",
            "enabled": false,
            "events": [
                { "id": "viewport_resize", "ready": false, "required": true }
            ]
        });
        fs::write(&path, serde_json::to_string(&contract).unwrap()).unwrap();

        let value = load_contract_value_from_path(&path).unwrap();

        assert_eq!(
            value
                .pointer("/embedded_terminal_input/schema_version")
                .and_then(serde_json::Value::as_str),
            Some("dx.embedded_terminal_input.v1")
        );
        assert_eq!(
            value
                .pointer("/embedded_terminal_resize/schema_version")
                .and_then(serde_json::Value::as_str),
            Some("dx.embedded_terminal_resize.v1")
        );
    }

    #[test]
    fn host_contract_value_injects_dashboard_compatibility_contract() {
        let dir = std::env::temp_dir().join(format!("dx-dashboard-compat-contract-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("dx-host-contract.json");
        let history = dir.join("history.jsonl");
        let contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        fs::write(&path, serde_json::to_string(&contract).unwrap()).unwrap();

        let value = load_contract_value_from_path(&path).unwrap();

        assert_eq!(
            value
                .pointer("/dashboard_compatibility/schema_version")
                .and_then(serde_json::Value::as_str),
            Some("dx.dashboard_compatibility.v1")
        );
        assert_eq!(
            value
                .pointer("/dashboard_compatibility/product_name")
                .and_then(serde_json::Value::as_str),
            Some("DX Agents")
        );
        assert_eq!(
            value
                .pointer("/dashboard_compatibility/compatibility_policy/exposes_stored_values")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
    }

    #[test]
    fn dashboard_compatibility_contract_preserves_legacy_aliases() {
        let contract = dashboard_compatibility_contract();
        let storage_aliases = contract["storage_aliases"]
            .as_array()
            .expect("storage aliases must be an array");
        assert!(storage_aliases.iter().any(|alias| {
            alias["primary"] == "dx_agents_token"
                && alias["legacy"]
                    .as_array()
                    .is_some_and(|legacy| legacy.iter().any(|value| value == "zeroclaw_token"))
        }));
        assert!(
            contract["websocket_protocols"]
                .as_array()
                .is_some_and(|protocols| protocols.iter().any(|protocol| {
                    protocol["primary"] == "dx-agent.v1"
                        && protocol["legacy"]
                            .as_array()
                            .is_some_and(|legacy| legacy.iter().any(|value| value == "zeroclaw.v1"))
                }))
        );
    }

    #[test]
    fn dashboard_compatibility_contract_covers_dashboard_alias_categories() {
        let contract = dashboard_compatibility_contract();

        assert_eq!(contract["package_name"].as_str(), Some("dx-agent-web"));
        assert_eq!(
            contract
                .pointer("/compatibility_policy/legacy_readable")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_eq!(
            contract
                .pointer("/compatibility_policy/legacy_writable")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );

        let env_aliases = contract["env_aliases"]
            .as_array()
            .expect("env aliases must be an array");
        assert!(env_aliases.iter().any(|alias| {
            alias["primary"] == "DX_AGENTS_GATEWAY_PORT"
                && alias["legacy"].as_array().is_some_and(|legacy| {
                    legacy.iter().any(|value| value == "DX_AGENT_GATEWAY_PORT")
                })
        }));

        let window_globals = contract["window_globals"]
            .as_array()
            .expect("window globals must be an array");
        assert!(window_globals.iter().any(|alias| {
            alias["primary"] == "__DX_AGENTS_BASE__"
                && alias["legacy"]
                    .as_array()
                    .is_some_and(|legacy| legacy.iter().any(|value| value == "__DX_AGENT_BASE__"))
        }));
        assert!(window_globals.iter().any(|alias| {
            alias["primary"] == "__DX_AGENTS_GATEWAY__"
                && alias["legacy"].as_array().is_some_and(|legacy| {
                    legacy.iter().any(|value| value == "__DX_AGENT_GATEWAY__")
                })
        }));

        let event_aliases = contract["event_aliases"]
            .as_array()
            .expect("event aliases must be an array");
        assert!(event_aliases.iter().any(|alias| {
            alias["primary"] == "dx-agent-unauthorized"
                && alias["legacy"].as_array().is_some_and(|legacy| {
                    legacy.iter().any(|value| value == "zeroclaw-unauthorized")
                })
        }));
    }

    #[test]
    fn dashboard_compatibility_status_reports_ready_with_source_tokens() {
        let contract = serde_json::json!({
            "dashboard_compatibility": dashboard_compatibility_contract()
        });

        let status = dashboard_compatibility_status(&contract, &dx_agents_repo_dir());

        assert!(status.ready, "{:#?}", status.drift_checks);
        assert_eq!(
            status.schema_version,
            "dx.dashboard_compatibility_status.v1"
        );
        assert_eq!(
            status.contract_schema_version.as_deref(),
            Some("dx.dashboard_compatibility.v1")
        );
        assert_eq!(status.alias_category_count, 5);
        assert_eq!(status.storage_alias_count, 9);
        assert!(status.legacy_readable);
        assert!(status.legacy_writable);
        assert!(!status.exposes_stored_values);
        assert!(status.cleanup_gate_ready);
        assert!(
            status
                .migration_plan_path
                .ends_with(DASHBOARD_COMPATIBILITY_MIGRATION_PLAN)
        );
        assert!(!status.decommission_ready);
        assert_eq!(status.usage_telemetry.state, "missing_telemetry");
        assert!(status.drift_checks.iter().all(|check| check.status == "ok"));
    }

    #[test]
    fn dashboard_compatibility_status_warns_on_source_drift() {
        let dir = std::env::temp_dir().join(format!("dx-dashboard-compat-drift-{}", now_ms()));
        write_dashboard_compatibility_source_fixture(&dir, "DX Agents");
        let contract = serde_json::json!({
            "dashboard_compatibility": dashboard_compatibility_contract()
        });

        let status = dashboard_compatibility_status(&contract, &dir);

        assert!(!status.ready);
        assert!(status.drift_checks.iter().any(|check| {
            check.id == "alias_tokens"
                && check.status == "warn"
                && check.detail.contains("dx_agents_token")
        }));
    }

    #[test]
    fn dashboard_compatibility_status_blocks_decommission_without_usage_export() {
        let dir = std::env::temp_dir().join(format!("dx-dashboard-compat-no-usage-{}", now_ms()));
        write_dashboard_compatibility_ready_fixture(&dir);

        let contract = serde_json::json!({
            "dashboard_compatibility": dashboard_compatibility_contract()
        });
        let status = dashboard_compatibility_status(&contract, &dir);

        assert!(status.ready, "{:#?}", status.drift_checks);
        assert!(!status.decommission_ready);
        assert_eq!(status.usage_telemetry.state, "missing_telemetry");
        assert_eq!(
            status.next_action,
            "collect_dashboard_compatibility_usage_telemetry"
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn dashboard_compatibility_status_allows_decommission_after_zero_legacy_usage() {
        let dir = std::env::temp_dir().join(format!("dx-dashboard-compat-zero-usage-{}", now_ms()));
        write_dashboard_compatibility_ready_fixture(&dir);
        write_dashboard_compatibility_usage(
            &dir,
            r#"{
                "schema_version":"dx.dashboard_compatibility_usage.v1",
                "primary_usage_count":8,
                "legacy_usage_count":0,
                "legacy_read_count":0,
                "legacy_write_count":0,
                "legacy_remove_count":0,
                "migration_count":0,
                "counters":[]
            }"#,
        );

        let contract = serde_json::json!({
            "dashboard_compatibility": dashboard_compatibility_contract()
        });
        let status = dashboard_compatibility_status(&contract, &dir);

        assert!(status.ready, "{:#?}", status.drift_checks);
        assert!(status.decommission_ready);
        assert_eq!(status.usage_telemetry.state, "zero_legacy_usage");
        assert_eq!(status.usage_telemetry.primary_usage_count, 8);
        assert_eq!(
            status.next_action,
            "dashboard_legacy_alias_decommission_ready"
        );

        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn dashboard_compatibility_status_blocks_decommission_with_active_legacy_usage() {
        let dir =
            std::env::temp_dir().join(format!("dx-dashboard-compat-active-usage-{}", now_ms()));
        write_dashboard_compatibility_ready_fixture(&dir);
        write_dashboard_compatibility_usage(
            &dir,
            r#"{
                "schema_version":"dx.dashboard_compatibility_usage.v1",
                "primary_usage_count":3,
                "legacy_usage_count":2,
                "legacy_read_count":1,
                "legacy_write_count":1,
                "legacy_remove_count":0,
                "migration_count":1,
                "counters":[]
            }"#,
        );

        let contract = serde_json::json!({
            "dashboard_compatibility": dashboard_compatibility_contract()
        });
        let status = dashboard_compatibility_status(&contract, &dir);

        assert!(status.ready, "{:#?}", status.drift_checks);
        assert!(!status.decommission_ready);
        assert_eq!(status.usage_telemetry.state, "legacy_usage_observed");
        assert_eq!(status.usage_telemetry.legacy_usage_count, 2);
        assert_eq!(status.usage_telemetry.legacy_read_count, 1);
        assert_eq!(status.usage_telemetry.legacy_write_count, 1);

        fs::remove_dir_all(dir).unwrap();
    }

    fn write_dashboard_compatibility_source_fixture(dir: &Path, source: &str) {
        let web_src = dir.join("web").join("src");
        let gateway_src = dir.join("crates").join("dx-agent-gateway").join("src");
        fs::create_dir_all(&web_src).unwrap();
        fs::create_dir_all(&gateway_src).unwrap();
        fs::write(
            dir.join("web").join("package.json"),
            r#"{"name":"dx-agent-web"}"#,
        )
        .unwrap();
        fs::write(dir.join("web").join("vite.config.ts"), source).unwrap();
        fs::write(dir.join("web").join("index.html"), source).unwrap();
        fs::write(web_src.join("compat.ts"), source).unwrap();
        fs::write(gateway_src.join("static_files.rs"), source).unwrap();
    }

    fn write_dashboard_compatibility_ready_fixture(dir: &Path) {
        let mut tokens = dashboard_compatibility_tokens(&dashboard_compatibility_contract());
        tokens.extend(
            dashboard_compatibility_usage_tokens()
                .into_iter()
                .map(str::to_string),
        );
        write_dashboard_compatibility_source_fixture(dir, &tokens.join("\n"));
        fs::create_dir_all(dir.join("docs")).unwrap();
        fs::write(
            dir.join(DASHBOARD_COMPATIBILITY_MIGRATION_PLAN),
            "Version: 1\nMigration gate\nLegacy aliases remain active\n",
        )
        .unwrap();
    }

    fn write_dashboard_compatibility_usage(dir: &Path, source: &str) {
        let path = dashboard_compatibility_usage_path(dir);
        fs::create_dir_all(path.parent().expect("usage parent")).unwrap();
        fs::write(path, source).unwrap();
    }

    #[test]
    fn bridge_self_test_inventory_contains_live_ops_commands() {
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_cli_bridge_settings"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"save_dx_cli_bridge_settings"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_provider_health"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_provider_smoke_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_gateway_paircode"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_gateway_pairing_drill"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_cron_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_tool_safety_drill"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_tool_safety_drill_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_tool_safety_audit"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_tool_safety_audit_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"export_dx_agents_tool_safety_audit_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"open_dx_agents_tool_safety_audit_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"export_dx_agents_tool_safety_drill_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"open_dx_agents_tool_safety_drill_history"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_release_readiness"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_dashboard_compatibility_status"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"export_dx_agents_bridge_status"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_bridge_status_exports"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"open_dx_agents_bridge_status_export"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_fixtures"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_readiness"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_session_timeline"));
        assert!(
            DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_media_canary_evidence")
        );
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_tui_canary_gate"));
        assert!(
            DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_embedded_terminal_tui_canary_lifecycle")
        );
        assert!(
            DX_CLI_IPC_COMMANDS
                .contains(&"get_dx_agents_embedded_terminal_tui_canary_renderer_evidence")
        );
        assert!(DX_CLI_IPC_COMMANDS.contains(&"run_dx_agents_embedded_terminal_echo_pilot"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"run_dx_agents_embedded_terminal_tui_canary_runner"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"get_dx_agents_continuation_status"));
        assert!(DX_CLI_IPC_COMMANDS.contains(&"run_dx_cli_bridge_self_test"));
        assert!(duplicate_command_names(DX_CLI_IPC_COMMANDS).is_empty());
    }

    #[test]
    fn embedded_terminal_fixtures_cover_required_events_without_payloads() {
        let fixtures = embedded_terminal_fixtures();
        let input_kinds = fixtures
            .input_events
            .iter()
            .map(|event| event.kind.as_str())
            .collect::<HashSet<_>>();
        let resize_kinds = fixtures
            .resize_events
            .iter()
            .map(|event| event.kind.as_str())
            .collect::<HashSet<_>>();
        let serialized = serde_json::to_string(&fixtures).unwrap();

        for kind in ["keyboard", "paste", "focus", "mouse", "control_sequence"] {
            assert!(input_kinds.contains(kind));
        }
        for kind in [
            "initial_size",
            "viewport_resize",
            "debounce",
            "renderer_reflow",
            "pty_resize",
        ] {
            assert!(resize_kinds.contains(kind));
        }
        assert!(fixtures.input_events.iter().all(|event| event.redacted));
        assert!(
            fixtures
                .input_events
                .iter()
                .all(|event| !event.stores_payload)
        );
        assert!(fixtures.resize_events.iter().all(|event| event.redacted));
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"text_value\""));
        assert!(!serialized.contains("\"paste_text\""));
    }

    #[test]
    fn embedded_terminal_readiness_export_stays_redacted_and_gated() {
        let dir = std::env::temp_dir().join(format!("dx-terminal-readiness-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = dir.join("history.jsonl");
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "production_ready": false,
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_input"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_input.v1",
            "enabled": false,
            "redaction": {
                "stores_payloads": false,
                "stores_text_values": false
            }
        });
        contract["embedded_terminal_resize"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_resize.v1",
            "enabled": false
        });
        contract["embedded_terminal_media_session"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_media_session.v1",
            "enabled": false
        });

        let export =
            embedded_terminal_readiness_export(&dir.join("dx-host-contract.json"), &contract);
        let serialized = serde_json::to_string(&export).unwrap();

        assert!(!export.ready);
        assert!(export.input_contract_present);
        assert!(export.resize_contract_present);
        assert!(export.media_session_contract_present);
        assert!(export.evidence.iter().all(|item| item.redacted));
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"paste_text\""));
        assert!(!serialized.contains("\"key_text\""));
    }

    #[test]
    fn embedded_terminal_session_timeline_is_synthetic_and_redacted() {
        let dir = std::env::temp_dir().join(format!("dx-terminal-timeline-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = dir.join("history.jsonl");
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "production_ready": false,
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_input"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_input.v1",
            "enabled": false,
            "redaction": {
                "stores_payloads": false,
                "stores_text_values": false
            }
        });
        contract["embedded_terminal_resize"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_resize.v1",
            "enabled": false
        });
        contract["embedded_terminal_media_session"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_media_session.v1",
            "enabled": false
        });

        let timeline =
            embedded_terminal_session_timeline(&dir.join("dx-host-contract.json"), &contract);
        let event_ids = timeline
            .events
            .iter()
            .map(|event| event.event_id.as_str())
            .collect::<HashSet<_>>();
        let serialized = serde_json::to_string(&timeline).unwrap();

        assert_eq!(
            timeline.schema_version,
            "dx.embedded_terminal_session_timeline.v1"
        );
        assert!(!timeline.process_spawned);
        assert!(!timeline.allows_arbitrary_shell);
        assert!(!timeline.stores_payloads);
        assert!(timeline.external_terminal_fallback);
        assert!(!timeline.readiness_ready);
        assert!(timeline.echo_process_pilot_ready);
        assert!(timeline.events.iter().all(|event| event.redacted));
        for event_id in [
            "session.open",
            "input.keyboard",
            "input.paste",
            "resize.viewport_resize",
            "session.interrupt",
            "session.close",
        ] {
            assert!(event_ids.contains(event_id));
        }
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"paste_text\""));
        assert!(!serialized.contains("\"key_text\""));
        assert!(!serialized.contains("\"shell_command\""));
    }

    #[test]
    fn embedded_terminal_echo_pilot_args_are_fixed_and_shell_free() {
        let args = dx_agents_embedded_terminal_echo_pilot_args();

        assert_eq!(args, vec!["echo-pilot", "--json"]);
        assert!(!args.iter().any(|arg| arg.contains("{{input}}")));
        assert!(!args.iter().any(|arg| {
            [";", "&", "|", "<", ">", "`"]
                .iter()
                .any(|token| arg.contains(token))
        }));
    }

    #[test]
    fn embedded_terminal_tui_canary_runner_args_are_fixed_and_shell_free() {
        let args = dx_agents_embedded_terminal_tui_canary_runner_args();

        assert_eq!(args, vec!["tui-canary", "--json"]);
        assert!(!args.iter().any(|arg| arg.contains("{{input}}")));
        assert!(!args.iter().any(|arg| {
            [";", "&", "|", "<", ">", "`"]
                .iter()
                .any(|token| arg.contains(token))
        }));
    }

    #[test]
    fn embedded_terminal_echo_pilot_skips_without_safe_synthetic_gate() {
        let dir = std::env::temp_dir().join(format!("dx-terminal-echo-pilot-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = dir.join("history.jsonl");
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "production_ready": false,
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_input"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_input.v1",
            "enabled": false,
            "redaction": {
                "stores_payloads": true,
                "stores_text_values": false
            }
        });
        contract["embedded_terminal_resize"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_resize.v1",
            "enabled": false
        });
        contract["embedded_terminal_media_session"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_media_session.v1",
            "enabled": false
        });

        let pilot = run_embedded_terminal_echo_pilot(&dir.join("dx-host-contract.json"), &contract)
            .unwrap();

        assert_eq!(pilot.status, "skipped");
        assert!(!pilot.success);
        assert!(!pilot.process_spawned);
        assert!(!pilot.allows_arbitrary_shell);
        assert!(!pilot.stores_payloads);
        assert!(pilot.skipped_reason.is_some());
        assert!(
            pilot
                .argv
                .ends_with(&["echo-pilot".to_string(), "--json".to_string()])
        );
    }

    #[test]
    fn embedded_terminal_tui_canary_runner_skips_without_developer_gate() {
        let history = std::env::temp_dir().join(format!("dx-tui-runner-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });

        let runner = run_embedded_terminal_tui_canary_runner(&contract, None).unwrap();

        assert_eq!(
            runner.schema_version,
            "dx.embedded_terminal_tui_canary_runner.v1"
        );
        assert_eq!(runner.status, "skipped");
        assert!(!runner.success);
        assert!(!runner.process_spawned);
        assert!(!runner.gate_enabled);
        assert!(!runner.allows_arbitrary_shell);
        assert!(!runner.stores_payloads);
        assert_eq!(runner.stdout_limit_bytes, 16 * 1024);
        assert_eq!(runner.stderr_limit_bytes, 16 * 1024);
        assert_eq!(runner.max_duration_ms, 15_000);
        assert!(!runner.contract_present);
        assert!(!runner.contract_accepted);
        assert_eq!(runner.contract_source, "local_fallback");
        assert_eq!(
            runner.contract_fixed_command,
            vec!["dx-agent", "tui-canary", "--json"]
        );
        assert_eq!(
            runner.contract_result_states,
            vec!["skipped", "success", "failed"]
        );
        assert!(runner.skipped_reason.is_some());
        assert!(
            runner
                .argv
                .ends_with(&["tui-canary".to_string(), "--json".to_string()])
        );
    }

    #[test]
    fn embedded_terminal_tui_canary_runner_prefers_safe_source_owned_contract() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-runner-contract-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_tui_canary_runner"] = serde_json::json!({
            "supported": true,
            "schema_version": "dx.embedded_terminal_tui_canary_runner.v1",
            "fixed_command": ["dx-agent", "tui-canary", "--json"],
            "result_schema_version": "dx.embedded_terminal_tui_canary_runner.v1",
            "expected_message": "dx-agent-tui-canary-ok",
            "expected_runner": "developer_tui_canary",
            "default_enabled": false,
            "developer_only": true,
            "shell_free": true,
            "accepts_user_input": false,
            "allows_arbitrary_shell": false,
            "registers_host_action": false,
            "captures_stdout": true,
            "captures_stderr": true,
            "redacts_output": true,
            "stores_payloads": false,
            "production_routing_enabled": false,
            "external_terminal_fallback": true,
            "stdout_limit_bytes": 4096,
            "stderr_limit_bytes": 2048,
            "max_duration_ms": 9000,
            "result_states": ["skipped", "success", "failed"],
            "skip_reasons": ["developer_gate_required"]
        });

        let runner_contract = tui_canary_runner_contract(&contract);
        let lifecycle = embedded_terminal_tui_canary_lifecycle(&contract, None);
        let runner = run_embedded_terminal_tui_canary_runner(&contract, None).unwrap();

        assert!(runner_contract.present);
        assert!(runner_contract.accepted);
        assert_eq!(
            runner_contract.source,
            "host_contract.embedded_terminal_tui_canary_runner"
        );
        assert_eq!(runner_contract.args, vec!["tui-canary", "--json"]);
        assert_eq!(
            runner_contract.fixed_command,
            vec!["dx-agent", "tui-canary", "--json"]
        );
        assert_eq!(runner_contract.stdout_limit_bytes, 4096);
        assert_eq!(runner_contract.stderr_limit_bytes, 2048);
        assert_eq!(runner_contract.max_duration_ms, 9000);
        assert_eq!(
            runner_contract.result_states,
            vec!["skipped", "success", "failed"]
        );
        assert_eq!(lifecycle.stdout_limit_bytes, 4096);
        assert_eq!(lifecycle.stderr_limit_bytes, 2048);
        assert_eq!(lifecycle.max_duration_ms, 9000);
        assert!(runner.contract_present);
        assert!(runner.contract_accepted);
        assert_eq!(
            runner.contract_source,
            "host_contract.embedded_terminal_tui_canary_runner"
        );
        assert_eq!(
            runner.contract_fixed_command,
            vec!["dx-agent", "tui-canary", "--json"]
        );
        assert_eq!(
            runner.contract_result_states,
            vec!["skipped", "success", "failed"]
        );
        assert_eq!(runner.skip_reasons, vec!["developer_gate_required"]);
        assert!(runner.contract_diagnostics.is_empty());
    }

    #[test]
    fn embedded_terminal_tui_canary_runner_rejects_contract_drift() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-runner-drift-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_terminal_tui_canary_runner"] = serde_json::json!({
            "supported": true,
            "schema_version": "dx.embedded_terminal_tui_canary_runner.v1",
            "fixed_command": ["dx-agent", "tui-canary", "--json", "{{input}}"],
            "result_schema_version": "bad.schema",
            "expected_message": "wrong",
            "expected_runner": "wrong",
            "shell_free": false,
            "accepts_user_input": true,
            "allows_arbitrary_shell": true,
            "registers_host_action": true,
            "redacts_output": false,
            "stores_payloads": true,
            "production_routing_enabled": true,
            "external_terminal_fallback": false,
            "stdout_limit_bytes": 999999,
            "stderr_limit_bytes": 999999,
            "max_duration_ms": 999999,
            "result_states": ["success"]
        });

        let runner_contract = tui_canary_runner_contract(&contract);
        let runner = run_embedded_terminal_tui_canary_runner(&contract, None).unwrap();

        assert!(runner_contract.present);
        assert!(!runner_contract.accepted);
        assert_eq!(
            runner_contract.source,
            "local_fallback_due_to_contract_drift"
        );
        assert_eq!(runner_contract.args, vec!["tui-canary", "--json"]);
        assert_eq!(runner_contract.stdout_limit_bytes, 16 * 1024);
        assert_eq!(runner_contract.stderr_limit_bytes, 16 * 1024);
        assert_eq!(runner_contract.max_duration_ms, 15_000);
        for diagnostic in [
            "fixed_command_drift",
            "shell_free_not_declared",
            "accepts_user_input_not_allowed",
            "arbitrary_shell_not_allowed",
            "host_action_registration_not_allowed",
            "output_redaction_not_declared",
            "result_schema_drift",
            "expected_message_drift",
            "expected_runner_drift",
            "payload_storage_not_allowed",
            "production_routing_not_allowed",
            "external_terminal_fallback_missing",
            "result_state_missing_skipped",
            "result_state_missing_failed",
        ] {
            assert!(
                runner_contract
                    .diagnostics
                    .contains(&diagnostic.to_string())
            );
            assert!(
                runner
                    .contract_diagnostics
                    .contains(&diagnostic.to_string())
            );
        }
        assert!(runner.contract_present);
        assert!(!runner.contract_accepted);
        assert_eq!(
            runner.contract_source,
            "local_fallback_due_to_contract_drift"
        );
    }

    #[test]
    fn embedded_terminal_tui_canary_env_parser_is_explicit() {
        for value in [None, Some(""), Some("0"), Some("false"), Some("production")] {
            assert!(!tui_canary_env_enabled(value));
        }
        for value in [
            Some("1"),
            Some("true"),
            Some("yes"),
            Some("on"),
            Some("enabled"),
            Some("developer"),
        ] {
            assert!(tui_canary_env_enabled(value));
        }
    }

    #[test]
    fn embedded_terminal_tui_canary_gate_defaults_off_without_rerouting_actions() {
        let history = std::env::temp_dir().join(format!("dx-tui-gate-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["settings"] = serde_json::json!({
            "defaults": {
                "preferred_terminal_surface": "windows_terminal"
            }
        });
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["actions"] = serde_json::json!([
            { "id": "dx.shell", "requires_terminal": true },
            { "id": "dx.status", "requires_terminal": false }
        ]);

        let gate = embedded_terminal_tui_canary_gate(&contract, None);

        assert_eq!(
            gate.schema_version,
            "dx.embedded_terminal_tui_canary_gate.v1"
        );
        assert!(!gate.enabled);
        assert!(!gate.default_enabled);
        assert!(gate.developer_only);
        assert_eq!(gate.env_var, "DX_AGENTS_TUI_CANARY");
        assert!(!gate.env_value_present);
        assert_eq!(gate.mode, "off");
        assert_eq!(gate.production_terminal_surface, "external_terminal");
        assert_eq!(gate.preferred_terminal_surface, "windows_terminal");
        assert!(gate.normal_terminal_actions_unchanged);
        assert_eq!(gate.normal_terminal_action_count, 1);
        assert!(!gate.registers_host_action);
        assert!(!gate.allows_arbitrary_shell);
        assert!(!gate.stores_payloads);
        assert!(gate.guardrails.contains(&"defaults_off".to_string()));
    }

    #[test]
    fn embedded_terminal_tui_canary_gate_requires_developer_opt_in() {
        let history = std::env::temp_dir().join(format!("dx-tui-gate-on-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["actions"] = serde_json::json!([
            { "id": "dx.shell", "requires_terminal": true }
        ]);

        let gate = embedded_terminal_tui_canary_gate(&contract, Some("developer"));

        assert!(gate.enabled);
        assert_eq!(gate.mode, "developer_tui_canary");
        assert!(gate.env_value_present);
        assert_eq!(gate.production_terminal_surface, "external_terminal");
        assert!(gate.normal_terminal_actions_unchanged);
        assert_eq!(gate.normal_terminal_action_count, 1);
        assert!(!gate.registers_host_action);
        assert_eq!(gate.next_phase, "bounded_tui_process_lifecycle_evidence");
    }

    #[test]
    fn embedded_terminal_tui_canary_lifecycle_is_bounded_and_gated() {
        let history = std::env::temp_dir().join(format!("dx-tui-lifecycle-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });

        let lifecycle = embedded_terminal_tui_canary_lifecycle(&contract, None);
        let event_ids = lifecycle
            .events
            .iter()
            .map(|event| event.event_id.as_str())
            .collect::<HashSet<_>>();
        let serialized = serde_json::to_string(&lifecycle).unwrap();

        assert_eq!(
            lifecycle.schema_version,
            "dx.embedded_terminal_tui_canary_lifecycle.v1"
        );
        assert_eq!(lifecycle.status, "gated");
        assert!(!lifecycle.gate_enabled);
        assert!(!lifecycle.process_spawned);
        assert!(!lifecycle.allows_arbitrary_shell);
        assert!(!lifecycle.stores_payloads);
        assert!(lifecycle.external_terminal_fallback);
        assert_eq!(lifecycle.max_duration_ms, 15_000);
        assert_eq!(lifecycle.stdout_limit_bytes, 16 * 1024);
        assert_eq!(lifecycle.stderr_limit_bytes, 16 * 1024);
        for event_id in [
            "tui.open",
            "tui.resize",
            "tui.interrupt",
            "tui.close",
            "tui.cleanup",
        ] {
            assert!(event_ids.contains(event_id));
        }
        assert!(lifecycle.events.iter().all(|event| event.redacted));
        assert!(lifecycle.events.iter().all(|event| event.timeout_ms > 0));
        assert!(
            lifecycle
                .rollback_triggers
                .contains(&"normal_terminal_action_routing_changes".to_string())
        );
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"paste_text\""));
        assert!(!serialized.contains("\"shell_command\""));
    }

    #[test]
    fn embedded_terminal_tui_canary_lifecycle_arms_only_after_gate() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-lifecycle-on-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });

        let lifecycle = embedded_terminal_tui_canary_lifecycle(&contract, Some("developer"));

        assert_eq!(lifecycle.status, "armed");
        assert!(lifecycle.gate_enabled);
        assert!(!lifecycle.process_spawned);
        assert!(lifecycle.events.iter().all(|event| event.status == "armed"));
        assert_eq!(lifecycle.next_phase, "developer_tui_canary_runner");
    }

    #[test]
    fn embedded_terminal_tui_canary_renderer_evidence_is_redacted_and_complete() {
        let history = std::env::temp_dir().join(format!("dx-tui-renderer-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_renderer"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_renderer.v1",
            "enabled": false
        });

        let evidence = embedded_terminal_tui_canary_renderer_evidence(&contract, Some("developer"));
        let snapshot_ids = evidence
            .snapshots
            .iter()
            .map(|snapshot| snapshot.snapshot_id.as_str())
            .collect::<HashSet<_>>();
        let serialized = serde_json::to_string(&evidence).unwrap();

        assert_eq!(
            evidence.schema_version,
            "dx.embedded_terminal_tui_canary_renderer_evidence.v1"
        );
        assert_eq!(evidence.status, "capturable");
        assert!(evidence.gate_enabled);
        assert!(!evidence.process_spawned);
        assert!(!evidence.source_contract_present);
        assert!(!evidence.source_contract_accepted);
        assert_eq!(evidence.source_contract_source, "local_fallback");
        assert!(evidence.renderer_contract_present);
        assert!(!evidence.renderer_enabled);
        assert!(evidence.external_terminal_fallback);
        assert!(!evidence.production_routing_enabled);
        assert!(!evidence.allows_arbitrary_shell);
        assert!(!evidence.stores_payloads);
        assert_eq!(evidence.snapshot_count, 7);
        for snapshot_id in [
            "renderer.alternate_screen",
            "renderer.cursor_state",
            "renderer.scrollback",
            "renderer.resize_reflow",
            "renderer.interrupt",
            "renderer.close",
            "renderer.cleanup",
        ] {
            assert!(snapshot_ids.contains(snapshot_id));
        }
        assert!(evidence.snapshots.iter().all(|snapshot| snapshot.redacted));
        assert!(
            evidence
                .snapshots
                .iter()
                .all(|snapshot| !snapshot.stores_payload && !snapshot.process_spawned)
        );
        assert!(
            evidence
                .snapshots
                .iter()
                .any(|snapshot| snapshot.alternate_screen)
        );
        assert!(
            evidence
                .snapshots
                .iter()
                .any(|snapshot| snapshot.renderer_reflow_required)
        );
        assert!(
            evidence
                .snapshots
                .iter()
                .any(|snapshot| snapshot.scrollback_lines > 0)
        );
        assert_eq!(evidence.rollback_state, "safe_to_discard_renderer_evidence");
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"terminal_text\""));
        assert!(!serialized.contains("\"frame_contents\""));
        assert!(!serialized.contains("\"shell_command\""));
    }

    #[test]
    fn embedded_terminal_tui_canary_renderer_evidence_prefers_safe_source_contract() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-renderer-source-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_tui_canary_renderer_evidence"] = serde_json::json!({
            "supported": true,
            "schema_version": "dx.embedded_terminal_tui_canary_renderer_evidence.v1",
            "status": "gated",
            "process_spawned": false,
            "production_routing_enabled": false,
            "allows_arbitrary_shell": false,
            "stores_payloads": false,
            "rollback_state": "safe_to_discard_renderer_evidence",
            "drift_checks": ["snapshot_ids_match_dx_agents_renderer", "snapshots_are_redacted"],
            "next_phase": "host_renderer_evidence_contract_import",
            "snapshots": [
                {"step": 1, "snapshot_id": "renderer.alternate_screen", "phase": "alternate_screen", "status": "source", "source": "host.renderer.alt", "terminal_state": "alternate_screen", "columns": 120, "rows": 34, "cursor_row": 1, "cursor_column": 1, "scrollback_lines": 0, "alternate_screen": true, "cursor_visible": true, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned alternate screen metadata"},
                {"step": 2, "snapshot_id": "renderer.cursor_state", "phase": "cursor", "status": "source", "source": "host.renderer.cursor", "terminal_state": "alternate_screen", "columns": 120, "rows": 34, "cursor_row": 12, "cursor_column": 24, "scrollback_lines": 0, "alternate_screen": true, "cursor_visible": false, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned cursor metadata"},
                {"step": 3, "snapshot_id": "renderer.scrollback", "phase": "scrollback", "status": "source", "source": "host.renderer.scrollback", "terminal_state": "normal_screen", "columns": 120, "rows": 34, "cursor_row": 34, "cursor_column": 1, "scrollback_lines": 256, "alternate_screen": false, "cursor_visible": true, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned scrollback metadata"},
                {"step": 4, "snapshot_id": "renderer.resize_reflow", "phase": "resize", "status": "source", "source": "host.renderer.resize", "terminal_state": "alternate_screen_resized", "columns": 132, "rows": 38, "cursor_row": 12, "cursor_column": 24, "scrollback_lines": 0, "alternate_screen": true, "cursor_visible": false, "renderer_reflow_required": true, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned resize metadata"},
                {"step": 5, "snapshot_id": "renderer.interrupt", "phase": "interrupt", "status": "source", "source": "host.renderer.interrupt", "terminal_state": "interrupting", "columns": 132, "rows": 38, "cursor_row": 12, "cursor_column": 24, "scrollback_lines": 0, "alternate_screen": true, "cursor_visible": false, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned interrupt metadata"},
                {"step": 6, "snapshot_id": "renderer.close", "phase": "close", "status": "source", "source": "host.renderer.close", "terminal_state": "closing", "columns": 132, "rows": 38, "cursor_row": 1, "cursor_column": 1, "scrollback_lines": 0, "alternate_screen": false, "cursor_visible": true, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned close metadata"},
                {"step": 7, "snapshot_id": "renderer.cleanup", "phase": "cleanup", "status": "source", "source": "host.renderer.cleanup", "terminal_state": "clean", "columns": 120, "rows": 34, "cursor_row": 1, "cursor_column": 1, "scrollback_lines": 0, "alternate_screen": false, "cursor_visible": true, "renderer_reflow_required": false, "process_spawned": false, "redacted": true, "stores_payload": false, "detail": "source-owned cleanup metadata"}
            ]
        });

        let evidence = embedded_terminal_tui_canary_renderer_evidence(&contract, Some("developer"));

        assert!(evidence.source_contract_present);
        assert!(evidence.source_contract_accepted);
        assert_eq!(
            evidence.source_contract_source,
            "host_contract.embedded_terminal_tui_canary_renderer_evidence"
        );
        assert!(evidence.source_contract_diagnostics.is_empty());
        assert_eq!(evidence.snapshot_count, 7);
        assert_eq!(evidence.snapshots[0].status, "source");
        assert_eq!(evidence.snapshots[0].source, "host.renderer.alt");
        assert!(
            evidence
                .drift_checks
                .contains(&"snapshot_ids_match_dx_agents_renderer".to_string())
        );
        assert_eq!(
            evidence.next_phase,
            "host_renderer_evidence_contract_import"
        );
    }

    #[test]
    fn embedded_terminal_tui_canary_renderer_evidence_rejects_contract_drift() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-renderer-drift-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_tui_canary_renderer_evidence"] = serde_json::json!({
            "supported": false,
            "schema_version": "bad.schema",
            "process_spawned": true,
            "production_routing_enabled": true,
            "allows_arbitrary_shell": true,
            "stores_payloads": true,
            "rollback_state": "manual_review_required",
            "snapshots": [
                {"step": 1, "snapshot_id": "renderer.alternate_screen", "redacted": false, "stores_payload": true, "process_spawned": true}
            ]
        });

        let evidence = embedded_terminal_tui_canary_renderer_evidence(&contract, Some("developer"));

        assert!(evidence.source_contract_present);
        assert!(!evidence.source_contract_accepted);
        for diagnostic in [
            "renderer_evidence_contract_not_supported",
            "renderer_evidence_schema_drift",
            "renderer_evidence_production_routing_enabled",
            "renderer_evidence_payload_storage_enabled",
            "renderer_evidence_snapshot_count_incomplete",
            "renderer_evidence_snapshot_not_redacted",
            "renderer_evidence_snapshot_stores_payload",
            "renderer_evidence_snapshot_spawned_process",
        ] {
            assert!(
                evidence
                    .source_contract_diagnostics
                    .contains(&diagnostic.to_string()),
                "missing diagnostic {diagnostic}"
            );
        }
        assert_eq!(evidence.snapshot_count, 7);
        assert!(evidence.snapshots.iter().all(|snapshot| snapshot.redacted));
        assert_eq!(
            evidence.snapshots[0].source,
            "embedded_terminal_renderer.features.alternate_screen"
        );
    }

    #[test]
    fn embedded_terminal_tui_canary_renderer_evidence_stays_gated_without_opt_in() {
        let history =
            std::env::temp_dir().join(format!("dx-tui-renderer-gated-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });

        let evidence = embedded_terminal_tui_canary_renderer_evidence(&contract, None);

        assert_eq!(evidence.status, "gated");
        assert!(!evidence.gate_enabled);
        assert!(!evidence.process_spawned);
        assert!(!evidence.source_contract_present);
        assert!(!evidence.source_contract_accepted);
        assert_eq!(evidence.snapshot_count, 7);
        assert!(evidence.snapshots.iter().all(|snapshot| {
            snapshot.status == "gated" && snapshot.redacted && !snapshot.stores_payload
        }));
        assert_eq!(evidence.next_phase, "developer_canary_gate_opt_in");
    }

    #[test]
    fn embedded_terminal_media_canary_evidence_is_redacted_and_external_route_safe() {
        let history = std::env::temp_dir().join(format!("dx-media-canary-history-{}", now_ms()));
        let mut contract =
            serde_json::to_value(sample_contract_with_telemetry(vec![], &history)).unwrap();
        contract["embedded_pty"] = serde_json::json!({
            "fallback_surface": "external_terminal"
        });
        contract["embedded_terminal_media_session"] = serde_json::json!({
            "schema_version": "dx.embedded_terminal_media_session.v1",
            "enabled": false,
            "fallback_surface": "external_terminal",
            "frame_budget": {
                "max_frame_rate": 24,
                "max_pending_frames": 2,
                "max_audio_buffer_ms": 180
            }
        });

        let evidence = embedded_terminal_media_canary_evidence(&contract);
        let sample_ids = evidence
            .samples
            .iter()
            .map(|sample| sample.sample_id.as_str())
            .collect::<HashSet<_>>();
        let serialized = serde_json::to_string(&evidence).unwrap();

        assert_eq!(
            evidence.schema_version,
            "dx.embedded_terminal_media_canary_evidence.v1"
        );
        assert_eq!(evidence.status, "gated");
        assert!(!evidence.process_spawned);
        assert!(evidence.media_session_contract_present);
        assert!(!evidence.media_session_enabled);
        assert!(evidence.external_terminal_fallback);
        assert!(!evidence.production_routing_enabled);
        assert!(!evidence.allows_arbitrary_shell);
        assert!(!evidence.stores_payloads);
        assert_eq!(evidence.sample_count, 7);
        for sample_id in [
            "media.terminal_video",
            "media.audio_stream",
            "media.image_preview",
            "media.backpressure",
            "media.frame_budget",
            "media.close",
            "media.cleanup",
        ] {
            assert!(sample_ids.contains(sample_id));
        }
        assert!(
            evidence
                .samples
                .iter()
                .all(|sample| sample.redacted && !sample.stores_payload && !sample.process_spawned)
        );
        assert!(evidence.operator_export_ready);
        assert_eq!(
            evidence.rollback_state,
            "safe_to_discard_media_canary_evidence"
        );
        assert!(evidence.fallback_actions.contains(&"dx.watch".to_string()));
        assert!(evidence.fallback_actions.contains(&"dx.image".to_string()));
        assert!(serialized.contains("\"max_frame_rate\":24"));
        assert!(!serialized.contains("\"raw_payload\""));
        assert!(!serialized.contains("\"frame_contents\""));
        assert!(!serialized.contains("\"audio_samples\""));
        assert!(!serialized.contains("\"image_pixels\""));
        assert!(!serialized.contains("\"shell_command\""));
    }

    #[test]
    fn bridge_self_test_reports_declared_ipc_commands() {
        let report = dx_cli_bridge_self_test();

        assert_eq!(report.command_count, DX_CLI_IPC_COMMANDS.len());
        assert_eq!(report.commands.len(), DX_CLI_IPC_COMMANDS.len());
        assert!(report.commands.iter().all(|command| command.available));
        assert!(report.commands.iter().all(|command| command.kind == "ipc"));
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.id == "ipc_inventory" && diagnostic.status == "ok")
        );
    }

    #[test]
    fn provider_health_args_default_to_dry_run_json() {
        assert_eq!(
            dx_agents_provider_health_args(None, None),
            vec!["models", "health", "--mode", "dry-run", "--json"]
        );
    }

    #[test]
    fn provider_health_args_include_profile_and_mode() {
        assert_eq!(
            dx_agents_provider_health_args(Some("LIVE"), Some(" groq ")),
            vec![
                "models",
                "health",
                "--provider",
                "groq",
                "--mode",
                "live",
                "--json"
            ]
        );
    }

    #[test]
    fn provider_failover_drill_args_request_redacted_dry_run_json() {
        assert_eq!(
            dx_agents_provider_failover_drill_args(Some("LIVE")),
            vec!["models", "failover-drill", "--mode", "dry-run", "--json"]
        );
        assert_eq!(
            dx_agents_provider_failover_drill_args(Some(" mock ")),
            vec!["models", "failover-drill", "--mode", "mock", "--json"]
        );
    }

    #[test]
    fn compact_status_args_request_json_dashboard() {
        assert_eq!(
            dx_agents_compact_status_args(),
            vec!["status", "--compact", "--json"]
        );
    }

    #[test]
    fn cron_preview_args_request_json_with_bounded_limit() {
        assert_eq!(
            dx_agents_cron_preview_args(Some(100)),
            vec!["cron", "preview", "--limit", "25", "--json"]
        );
    }

    #[test]
    fn cron_history_args_request_json_with_bounded_limit() {
        assert_eq!(
            dx_agents_cron_history_args(Some(100)),
            vec!["cron", "history", "--limit", "25", "--json"]
        );
    }

    #[test]
    fn cron_delivery_drill_args_request_redacted_dry_run_json() {
        assert_eq!(
            dx_agents_cron_delivery_drill_args(Some("LIVE")),
            vec!["cron", "delivery-drill", "--mode", "dry-run", "--json"]
        );
        assert_eq!(
            dx_agents_cron_delivery_drill_args(Some(" mock ")),
            vec!["cron", "delivery-drill", "--mode", "mock", "--json"]
        );
    }

    #[test]
    fn tool_safety_drill_args_request_redacted_dry_run_json() {
        assert_eq!(
            dx_agents_tool_safety_drill_args(Some("LIVE")),
            vec!["tools", "safety-drill", "--mode", "dry-run", "--json"]
        );
        assert_eq!(
            dx_agents_tool_safety_drill_args(Some(" mock ")),
            vec!["tools", "safety-drill", "--mode", "mock", "--json"]
        );
    }

    #[test]
    fn session_tool_routing_args_request_json_report() {
        assert_eq!(
            dx_agents_session_tool_routing_args(),
            vec!["sessions", "tool-routing", "--json"]
        );
    }

    #[test]
    fn memory_skill_learning_args_request_json_report() {
        assert_eq!(
            dx_agents_memory_skill_learning_args(),
            vec!["memory", "learning-loop", "--json"]
        );
    }

    #[test]
    fn gateway_paircode_args_request_new_pairing_code() {
        assert_eq!(
            dx_agents_gateway_paircode_args(),
            vec!["gateway", "get-paircode", "--new"]
        );
    }

    #[test]
    fn gateway_pairing_drill_args_request_redacted_dry_run_json() {
        assert_eq!(
            dx_agents_gateway_pairing_drill_args(Some("LIVE")),
            vec!["gateway", "pairing-drill", "--mode", "dry-run", "--json"]
        );
        assert_eq!(
            dx_agents_gateway_pairing_drill_args(Some(" mock ")),
            vec!["gateway", "pairing-drill", "--mode", "mock", "--json"]
        );
    }

    #[test]
    fn provider_smoke_args_use_selected_profile_model_and_safe_message() {
        assert_eq!(
            dx_agents_provider_smoke_args(Some(" groq "), Some(" llama-test "), Some(2)),
            vec![
                "agent",
                "-a",
                "dx",
                "-p",
                "groq",
                "--model",
                "llama-test",
                "--message",
                "Reply with exactly: dx-agents-provider-ok",
            ]
        );
    }

    #[test]
    fn provider_smoke_redaction_hides_common_secret_tokens() {
        let redacted = redact_sensitive_text(
            "failed with gsk_secret123 and Authorization: Bearer sk-proj-abc123",
        );

        assert!(!redacted.contains("gsk_secret123"));
        assert!(!redacted.contains("sk-proj-abc123"));
        assert!(redacted.contains("[redacted]"));
    }

    #[test]
    fn dashboard_recovery_hints_cover_operator_surfaces() {
        let provider = dashboard_recovery_hint(
            "Provider health",
            &["models", "health", "--mode", "live", "--json"],
            false,
            "401",
            None,
        )
        .unwrap();
        let cron = dashboard_recovery_hint(
            "Cron run history",
            &["cron", "history", "--json"],
            false,
            "database unavailable",
            None,
        )
        .unwrap();
        let gateway = dashboard_recovery_hint(
            "Gateway pairing code",
            &["gateway", "get-paircode", "--new"],
            false,
            "connection refused",
            None,
        )
        .unwrap();

        assert!(provider.contains("provider profile"));
        assert!(cron.contains("cron list"));
        assert!(gateway.contains("gateway start"));
    }

    #[test]
    fn release_readiness_report_scores_local_release_inputs() {
        let report = release_readiness_report();

        assert!(!report.items.is_empty());
        assert!(report.items.iter().any(|item| item.id == "host_contract"));
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tauri_bundle_config")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "version_metadata")
        );
        assert!(report.items.iter().any(|item| item.id == "installer_icons"));
        assert!(report.items.iter().any(|item| item.id == "bundle_targets"));
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "desktop_bridge_docs")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "operator_qa_docs")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tool_safety_alert_runbook")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tool_safety_audit_review_runbook")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tool_safety_audit_review_alert_runbook")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tool_safety_audit_review_recovery_runbook")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "tool_safety_audit_review_recovery_alert_runbook")
        );
        assert!(
            report.items.iter().any(|item| {
                item.id == "tool_safety_audit_review_recovery_alert_digest_runbook"
            })
        );
        assert!(report.items.iter().any(|item| {
            item.id == "tool_safety_audit_review_recovery_alert_digest_release_gate_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook"
        }));
        assert!(report.items.iter().any(|item| {
            item.id
                == "tool_safety_audit_review_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook"
        }));
        assert!(
            report
                .items
                .iter()
                .any(|item| { item.id == "tool_safety_audit_recovery_alert_digest_release_gate" })
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "dashboard_compatibility_migration")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "expected_distribution_outputs")
        );
        assert!(
            report
                .items
                .iter()
                .any(|item| item.id == "migration_surface")
        );
        assert!(report.score <= 100);
    }

    #[test]
    fn readiness_item_includes_recovery_only_for_warnings() {
        let ok = readiness_item("ok", "OK", true, "ready".to_string(), "fix it");
        let warn = readiness_item("warn", "Warn", false, "missing".to_string(), "fix it");

        assert_eq!(ok.status, "ok");
        assert!(ok.recovery_hint.is_none());
        assert_eq!(warn.recovery_hint.as_deref(), Some("fix it"));
    }

    fn tool_safety_command(
        ready: bool,
        allowed_count: usize,
        approval_required_count: usize,
        denied_count: usize,
        missing_count: usize,
        critical_blocker_count: usize,
    ) -> DxAgentsDashboardCommand {
        DxAgentsDashboardCommand {
            label: "Tool configuration safety drill".to_string(),
            success: true,
            exit_code: Some(0),
            stdout: "token gsk_secret".to_string(),
            stderr: String::new(),
            json: Some(serde_json::json!({
                "mode": "dry-run",
                "ready": ready,
                "summary": {
                    "allowed_count": allowed_count,
                    "approval_required_count": approval_required_count,
                    "denied_count": denied_count,
                    "missing_count": missing_count,
                    "critical_blocker_count": critical_blocker_count
                },
                "approval": {
                    "autonomy_level": "approval_required",
                    "auto_approve_count": 2,
                    "always_ask_count": 1,
                    "tool_filter_group_count": 3
                },
                "allowlists": {
                    "mcp_server_count": 4
                },
                "redaction": {
                    "exports_secret_values": false,
                    "exports_allowlist_values": false,
                    "exports_command_values": false,
                    "exports_path_values": false
                },
                "recovery_hint": "rotate token gsk_secret before live mode"
            })),
            json_error: None,
            recovery_hint: None,
            argv: vec![],
            cwd: r"G:\Dx\agent".to_string(),
        }
    }

    #[test]
    fn provider_smoke_history_reads_newest_first_and_stays_redacted() {
        let dir = std::env::temp_dir().join(format!("dx-agent-provider-smoke-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("smoke.jsonl");
        let mut command = DxAgentsDashboardCommand {
            label: "Provider live smoke".to_string(),
            success: false,
            exit_code: Some(1),
            stdout: "token gsk_secret123".to_string(),
            stderr: "Authorization: sk-proj-secret456".to_string(),
            json: None,
            json_error: None,
            recovery_hint: None,
            argv: vec![],
            cwd: r"G:\Dx\agent".to_string(),
        };
        command = super::redact_dashboard_command(command);
        let first = provider_smoke_record("groq", Some("llama-test".to_string()), &command, 42);
        append_provider_smoke_record(&path, &first).unwrap();
        let second = provider_smoke_record("openai", None, &command, 84);
        append_provider_smoke_record(&path, &second).unwrap();

        let history = read_provider_smoke_history(&path, 2).unwrap();

        assert_eq!(history.count, 2);
        assert_eq!(history.entries[0].provider, "openai");
        assert_eq!(history.entries[1].provider, "groq");
        assert!(!history.entries[0].stdout_summary.contains("gsk_secret123"));
        assert!(
            !history.entries[0]
                .stderr_summary
                .contains("sk-proj-secret456")
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_history_retains_newest_redacted_trend_records() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-safety-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");

        for index in 0..5 {
            let mut command =
                tool_safety_command(index == 4, index, 0, 0, 0, usize::from(index == 4));
            let mut record = tool_safety_drill_record("unit_test", &command);
            record.recorded_at_ms = index as u64;
            record.recovery_hint = Some(format!("token sk-proj-secret{index}"));
            record.recovery_hint = record.recovery_hint.as_deref().map(redact_sensitive_text);
            append_tool_safety_drill_record(&path, &record, 3).unwrap();
            command.stdout.clear();
        }

        let history = read_tool_safety_drill_history(&path, 10).unwrap();

        assert_eq!(history.count, 3);
        assert_eq!(history.retention_limit, TOOL_SAFETY_DRILL_HISTORY_LIMIT);
        assert_eq!(history.entries[0].recorded_at_ms, 4);
        assert_eq!(history.entries[1].recorded_at_ms, 3);
        assert_eq!(history.entries[2].recorded_at_ms, 2);
        assert_eq!(history.trend.state, "worsening");
        assert_eq!(history.trend.critical_blocker_delta, 1);
        assert!(
            !history.entries[0]
                .recovery_hint
                .as_deref()
                .unwrap_or_default()
                .contains("sk-proj-secret")
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_record_is_metadata_only() {
        let command = tool_safety_command(false, 2, 1, 1, 1, 1);
        let record = tool_safety_drill_record("bridge_command", &command);

        assert_eq!(record.mode, "dry-run");
        assert_eq!(record.allowed_count, 2);
        assert_eq!(record.approval_required_count, 1);
        assert_eq!(record.denied_count, 1);
        assert_eq!(record.missing_count, 1);
        assert_eq!(record.critical_blocker_count, 1);
        assert_eq!(record.autonomy_level.as_deref(), Some("approval_required"));
        assert!(record.redaction_ok);
        assert!(
            !record
                .recovery_hint
                .as_deref()
                .unwrap_or_default()
                .contains("gsk_secret")
        );
    }

    #[test]
    fn tool_safety_drill_history_alerts_empty_history() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-safety-empty-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = read_tool_safety_drill_history(&dir.join("missing.jsonl"), 8).unwrap();

        assert_eq!(history.alerts.len(), 1);
        assert_eq!(history.alerts[0].id, "empty_history");
        assert_eq!(history.alerts[0].level, "info");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_history_alerts_worsening_blockers() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-safety-alerts-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");

        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &previous, 10).unwrap();
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 1, 1, 1, 1));
        latest.recorded_at_ms = 2;
        append_tool_safety_drill_record(&path, &latest, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let ids = history
            .alerts
            .iter()
            .map(|alert| alert.id.as_str())
            .collect::<HashSet<_>>();

        assert_eq!(history.trend.state, "worsening");
        assert!(ids.contains("critical_blockers"));
        assert!(ids.contains("denied_tools"));
        assert!(ids.contains("missing_tools"));
        assert!(ids.contains("approval_required_tools"));
        assert!(history.alerts.iter().any(|alert| alert.level == "blocked"));
        assert!(
            !serde_json::to_string(&history.alerts)
                .unwrap()
                .contains("gsk_secret")
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_history_alerts_improving_state() {
        let dir =
            std::env::temp_dir().join(format!("dx-agent-tool-safety-improving-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");

        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 0, 0, 2));
        previous.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &previous, 10).unwrap();
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        latest.recorded_at_ms = 2;
        append_tool_safety_drill_record(&path, &latest, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();

        assert_eq!(history.trend.state, "improving");
        assert_eq!(history.alerts.len(), 1);
        assert_eq!(history.alerts[0].id, "improving_history");
        assert_eq!(history.alerts[0].level, "ok");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_history_alerts_stable_state() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-safety-stable-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");

        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &previous, 10).unwrap();
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        latest.recorded_at_ms = 2;
        append_tool_safety_drill_record(&path, &latest, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();

        assert_eq!(history.trend.state, "stable");
        assert_eq!(history.alerts.len(), 1);
        assert_eq!(history.alerts[0].id, "stable_history");
        assert_eq!(history.alerts[0].level, "ok");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_drill_alerts_metadata_stays_redacted() {
        let command = tool_safety_command(false, 3, 1, 0, 0, 0);
        let record = tool_safety_drill_record("unit_test", &command);
        let trend = super::tool_safety_drill_trend(std::slice::from_ref(&record));
        let alerts = tool_safety_drill_alerts(&trend, &[record]);
        let body = serde_json::to_string(&alerts).unwrap();

        assert!(!body.contains("gsk_secret"));
        assert!(alerts.iter().all(|alert| {
            matches!(alert.level.as_str(), "ok" | "info" | "warning" | "blocked")
        }));
    }

    #[test]
    fn tool_safety_audit_reports_missing_history() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-missing-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = read_tool_safety_drill_history(&dir.join("missing.jsonl"), 8).unwrap();
        let audit = tool_safety_audit_summary(&history, true);

        assert_eq!(audit.state, "missing_history");
        assert!(!audit.ready);
        assert_eq!(audit.history_snapshot_count, 0);
        assert_eq!(audit.info_alert_count, 1);
        assert!(!audit.duplicates_history_rows);
        assert!(!audit.stores_config_values);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_reports_blocked_alerts() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-blocked-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");
        let mut record =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 1, 1, 1));
        record.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &record, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let audit = tool_safety_audit_summary(&history, true);

        assert_eq!(audit.state, "blocked");
        assert!(!audit.ready);
        assert!(audit.blocked_alert_count >= 1);
        assert!(audit.alert_ids.contains(&"critical_blockers".to_string()));
        assert!(audit.latest_recovery_hint_available);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_reports_missing_runbook() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-runbook-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");
        let mut record =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        record.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &record, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let audit = tool_safety_audit_summary(&history, false);

        assert_eq!(audit.state, "runbook_missing");
        assert!(!audit.ready);
        assert!(!audit.runbook_present);
        assert!(audit.next_remediation_action.contains("runbook"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_reports_ready_state() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-ready-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");
        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        append_tool_safety_drill_record(&path, &previous, 10).unwrap();
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        latest.recorded_at_ms = 2;
        append_tool_safety_drill_record(&path, &latest, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let audit = tool_safety_audit_summary(&history, true);

        assert_eq!(audit.state, "ready");
        assert!(audit.ready);
        assert_eq!(audit.blocked_alert_count, 0);
        assert_eq!(audit.warning_alert_count, 0);
        assert_eq!(audit.alert_ids, vec!["stable_history".to_string()]);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_reports_redaction_review() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-redaction-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");
        let mut record =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        record.recorded_at_ms = 1;
        record.redaction_ok = false;
        append_tool_safety_drill_record(&path, &record, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let audit = tool_safety_audit_summary(&history, true);

        assert_eq!(audit.state, "redaction_review");
        assert!(!audit.ready);
        assert!(!audit.redacted);
        assert!(!audit.stores_config_values);
        assert!(!audit.duplicates_history_rows);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_stays_metadata_only_and_redacted() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-redacted-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("tool-safety.jsonl");
        let mut record =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 1, 1, 0, 0, 0));
        record.recorded_at_ms = 1;
        record.recovery_hint = Some(redact_sensitive_text(
            r"token gsk_secret and local path G:\Secret\tool.json",
        ));
        append_tool_safety_drill_record(&path, &record, 10).unwrap();

        let history = read_tool_safety_drill_history(&path, 8).unwrap();
        let audit = tool_safety_audit_summary(&history, true);
        let body = serde_json::to_string(&audit).unwrap();

        assert!(audit.redacted);
        assert!(!audit.stores_config_values);
        assert!(!audit.duplicates_history_rows);
        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("tool.json"));
        assert!(!body.contains(&history.history_path));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_history_retains_newest_metadata_only_summaries() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-history-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("audit.jsonl");

        for index in 0..5 {
            let mut history =
                read_tool_safety_drill_history(&dir.join("missing.jsonl"), 8).unwrap();
            history.count = 1;
            let mut audit = tool_safety_audit_summary(&history, true);
            audit.generated_at_ms = index;
            audit.next_remediation_action = redact_sensitive_text(&format!(
                r"inspect token gsk_secret{index} and path G:\Secret\audit.json"
            ));
            append_tool_safety_audit_record(&path, &audit, 3).unwrap();
        }

        let history = read_tool_safety_audit_history(&path, 10).unwrap();
        let body = serde_json::to_string(&history).unwrap();

        assert_eq!(history.count, 3);
        assert_eq!(history.retention_limit, TOOL_SAFETY_AUDIT_HISTORY_LIMIT);
        assert_eq!(history.entries[0].generated_at_ms, 4);
        assert_eq!(history.entries[1].generated_at_ms, 3);
        assert_eq!(history.entries[2].generated_at_ms, 2);
        assert!(history.runbook_present);
        assert_eq!(history.runbook_target, "tool-safety-audit-runbook");
        assert_eq!(history.digest.audit_count, 3);
        assert!(history.digest.metadata_only);
        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains("G:\\Secret"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_digest_reports_empty_history_without_rows() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-empty-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let history = read_tool_safety_audit_history(&dir.join("missing.jsonl"), 8).unwrap();
        let body = serde_json::to_string(&history.digest).unwrap();

        assert_eq!(history.digest.state, "empty");
        assert!(!history.digest.ready);
        assert!(history.digest.review_required);
        assert_eq!(history.digest.audit_count, 0);
        assert!(history.digest.metadata_only);
        assert!(!body.contains("history_path"));
        assert!(!body.contains("entries"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_digest_reports_stable_ready_history() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-stable-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("audit.jsonl");
        let drill_path = dir.join("tool-safety.jsonl");

        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        append_tool_safety_drill_record(&drill_path, &previous, 10).unwrap();
        let mut stable =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        stable.recorded_at_ms = 2;
        append_tool_safety_drill_record(&drill_path, &stable, 10).unwrap();

        let ready_history = read_tool_safety_drill_history(&drill_path, 8).unwrap();
        let mut first = tool_safety_audit_summary(&ready_history, true);
        first.generated_at_ms = 1;
        append_tool_safety_audit_record(&path, &first, 10).unwrap();
        let mut second = tool_safety_audit_summary(&ready_history, true);
        second.generated_at_ms = 2;
        append_tool_safety_audit_record(&path, &second, 10).unwrap();

        let history = read_tool_safety_audit_history(&path, 8).unwrap();

        assert_eq!(history.digest.state, "stable");
        assert!(history.digest.ready);
        assert!(!history.digest.review_required);
        assert_eq!(history.digest.latest_audit_state.as_deref(), Some("ready"));
        assert_eq!(history.digest.trend_state, "stable");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_history_trend_reports_ready_to_blocked() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-trend-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("audit.jsonl");
        let drill_path = dir.join("tool-safety.jsonl");

        let mut previous_record =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous_record.recorded_at_ms = 1;
        append_tool_safety_drill_record(&drill_path, &previous_record, 10).unwrap();
        let mut stable_record =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        stable_record.recorded_at_ms = 2;
        append_tool_safety_drill_record(&drill_path, &stable_record, 10).unwrap();

        let ready_history = read_tool_safety_drill_history(&drill_path, 8).unwrap();
        let mut ready_audit = tool_safety_audit_summary(&ready_history, true);
        ready_audit.generated_at_ms = 2;
        append_tool_safety_audit_record(&path, &ready_audit, 10).unwrap();

        let mut blocked_record =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 1, 1, 1));
        blocked_record.recorded_at_ms = 3;
        append_tool_safety_drill_record(&drill_path, &blocked_record, 10).unwrap();

        let blocked_history = read_tool_safety_drill_history(&drill_path, 8).unwrap();
        let mut blocked_audit = tool_safety_audit_summary(&blocked_history, true);
        blocked_audit.generated_at_ms = 3;
        append_tool_safety_audit_record(&path, &blocked_audit, 10).unwrap();

        let history = read_tool_safety_audit_history(&path, 8).unwrap();

        assert_eq!(history.trend.state, "worsening");
        assert_eq!(history.trend.ready_delta, -1);
        assert_eq!(history.trend.alert_delta, 2);
        assert!(history.trend.blocked_alert_delta > 0);
        assert_eq!(history.digest.state, "ready_to_blocked");
        assert!(history.digest.review_required);
        assert!(history.latest_remediation_action.is_some());
        assert!(
            history
                .entries
                .iter()
                .all(|entry| !entry.duplicates_history_rows)
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_digest_reports_empty_stable_blocked_and_runbook_states() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-digest-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let audit_path = dir.join("audit.jsonl");
        let drill_path = dir.join("tool-safety.jsonl");

        let empty = read_tool_safety_audit_history(&dir.join("missing-audit.jsonl"), 8).unwrap();
        assert_eq!(empty.digest.state, "empty");
        assert!(empty.digest.review_required);
        assert!(empty.digest.metadata_only);

        for recorded_at_ms in [1, 2, 3] {
            let mut record =
                tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
            record.recorded_at_ms = recorded_at_ms;
            append_tool_safety_drill_record(&drill_path, &record, 10).unwrap();
        }
        let ready_history = read_tool_safety_drill_history(&drill_path, 8).unwrap();
        for generated_at_ms in [1, 2] {
            let mut ready_audit = tool_safety_audit_summary(&ready_history, true);
            ready_audit.generated_at_ms = generated_at_ms;
            append_tool_safety_audit_record(&audit_path, &ready_audit, 10).unwrap();
        }

        let stable = read_tool_safety_audit_history(&audit_path, 8).unwrap();
        assert_eq!(stable.digest.state, "stable");
        assert!(stable.digest.ready);
        assert!(!stable.digest.stores_config_values);
        assert!(!stable.digest.duplicates_history_rows);

        let missing_runbook = tool_safety_audit_digest_from_parts(
            stable.count,
            false,
            "tool-safety-audit-runbook",
            &stable.trend,
            stable.latest_remediation_action.as_deref(),
            &stable.entries,
        );
        assert_eq!(missing_runbook.state, "runbook_missing");
        assert!(!missing_runbook.ready);

        let mut blocked_record =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 1, 1, 1));
        blocked_record.recorded_at_ms = 3;
        append_tool_safety_drill_record(&drill_path, &blocked_record, 10).unwrap();
        let blocked_history = read_tool_safety_drill_history(&drill_path, 8).unwrap();
        let mut blocked_audit = tool_safety_audit_summary(&blocked_history, true);
        blocked_audit.generated_at_ms = 3;
        append_tool_safety_audit_record(&audit_path, &blocked_audit, 10).unwrap();

        let blocked = read_tool_safety_audit_history(&audit_path, 8).unwrap();
        let body = serde_json::to_string(&blocked.digest).unwrap();
        assert_eq!(blocked.digest.state, "ready_to_blocked");
        assert!(blocked.digest.review_required);
        assert!(blocked.digest.latest_remediation_action.is_some());
        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains(&blocked.history_path));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_review_alerts_cover_digest_states_and_severity() {
        let dir = std::env::temp_dir().join(format!("dx-agent-tool-audit-alerts-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let empty = read_tool_safety_audit_history(&dir.join("missing-audit.jsonl"), 8).unwrap();

        let empty_alerts = tool_safety_audit_review_alerts(&empty.digest);
        assert_eq!(empty_alerts[0].id, "audit_empty");
        assert_eq!(empty_alerts[0].level, "info");

        let mut stable = empty.digest.clone();
        stable.state = "stable".to_string();
        stable.ready = true;
        stable.review_required = false;
        stable.runbook_present = true;
        stable.audit_count = 2;
        stable.redacted = true;
        stable.metadata_only = true;
        stable.stores_config_values = false;
        stable.duplicates_history_rows = false;
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        assert_eq!(stable_alerts[0].id, "audit_stable");
        assert_eq!(stable_alerts[0].level, "ok");

        let mut changed = stable.clone();
        changed.state = "changed".to_string();
        changed.ready = false;
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        assert_eq!(changed_alerts[0].id, "audit_changed");
        assert_eq!(changed_alerts[0].level, "warning");

        let mut ready_to_blocked = stable.clone();
        ready_to_blocked.state = "ready_to_blocked".to_string();
        ready_to_blocked.ready = false;
        ready_to_blocked.ready_delta = -1;
        let blocked_alerts = tool_safety_audit_review_alerts(&ready_to_blocked);
        assert_eq!(blocked_alerts[0].id, "audit_ready_to_blocked");
        assert_eq!(blocked_alerts[0].level, "blocked");
        assert!(blocked_alerts[0].recovery_hint.is_some());

        let mut missing_runbook = stable.clone();
        missing_runbook.state = "runbook_missing".to_string();
        missing_runbook.runbook_present = false;
        let runbook_alerts = tool_safety_audit_review_alerts(&missing_runbook);
        assert_eq!(runbook_alerts[0].id, "audit_runbook_missing");
        assert_eq!(runbook_alerts[0].level, "blocked");

        let mut redaction = stable.clone();
        redaction.state = "redaction_review".to_string();
        redaction.redacted = false;
        redaction.metadata_only = false;
        redaction.stores_config_values = true;
        redaction.duplicates_history_rows = true;
        let redaction_alerts = tool_safety_audit_review_alerts(&redaction);
        assert_eq!(redaction_alerts[0].level, "blocked");
        assert!(
            redaction_alerts
                .iter()
                .any(|alert| alert.id == "audit_redaction_review")
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_review_alerts_stay_redacted() {
        let dir =
            std::env::temp_dir().join(format!("dx-agent-tool-audit-alert-redaction-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let empty = read_tool_safety_audit_history(&dir.join("missing-audit.jsonl"), 8).unwrap();
        let mut digest = empty.digest.clone();
        digest.state = "stable".to_string();
        digest.ready = true;
        digest.runbook_present = true;
        digest.redacted = true;
        digest.metadata_only = true;
        digest.stores_config_values = false;
        digest.duplicates_history_rows = false;
        digest.latest_remediation_action = Some(redact_sensitive_text(
            r"token gsk_secret and local path G:\Secret\audit.json",
        ));

        let alerts = tool_safety_audit_review_alerts(&digest);
        let body = serde_json::to_string(&alerts).unwrap();

        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn tool_safety_audit_history_export_path_uses_host_telemetry_directory() {
        let path = tool_safety_audit_history_export_path(1234);

        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("tool-safety-audit-history-1234.json")
        );
        assert!(path.display().to_string().contains("host-telemetry"));
        assert_eq!(
            tool_safety_audit_history_path()
                .file_name()
                .and_then(|name| name.to_str()),
            Some("dx-tool-safety-audit.jsonl")
        );
        assert_eq!(bool_delta(false, true), -1);
    }

    #[test]
    fn tool_safety_audit_digest_reports_runbook_missing_without_values() {
        let trend = super::DxAgentsToolSafetyAuditTrend {
            state: "stable".to_string(),
            snapshot_count: 2,
            latest_generated_at_ms: Some(2),
            previous_generated_at_ms: Some(1),
            ready_delta: 0,
            alert_delta: 0,
            blocked_alert_delta: 0,
            warning_alert_delta: 0,
            recovery_hint: None,
            summary: "stable".to_string(),
        };

        let digest = tool_safety_audit_digest_from_parts(
            2,
            false,
            "tool-safety-audit-runbook",
            &trend,
            Some(r"inspect token gsk_secret and path G:\Secret\audit.json"),
            &[],
        );
        let body = serde_json::to_string(&digest).unwrap();

        assert_eq!(digest.state, "runbook_missing");
        assert!(!digest.ready);
        assert!(digest.review_required);
        assert!(digest.metadata_only);
        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("entries"));
    }

    fn tool_safety_audit_digest_fixture(
        state: &str,
        ready: bool,
        runbook_present: bool,
    ) -> super::DxAgentsToolSafetyAuditDigest {
        super::DxAgentsToolSafetyAuditDigest {
            schema_version: "dx.tool_safety_audit_digest.v1".to_string(),
            generated_at_ms: 42,
            state: state.to_string(),
            ready,
            review_required: !ready,
            audit_count: usize::from(state != "empty"),
            latest_audit_state: Some(state.to_string()),
            latest_ready: Some(ready),
            trend_state: state.to_string(),
            ready_delta: if state == "ready_to_blocked" { -1 } else { 0 },
            alert_delta: usize::from(state == "changed" || state == "worsening") as i64,
            blocked_alert_delta: usize::from(state == "ready_to_blocked" || state == "worsening")
                as i64,
            warning_alert_delta: usize::from(state == "changed") as i64,
            runbook_present,
            runbook_target: "tool-safety-audit-runbook".to_string(),
            alert_runbook_present: true,
            alert_runbook_target: "tool-safety-audit-alert-runbook".to_string(),
            latest_remediation_action: None,
            next_action: "Keep reviewing metadata-only audit evidence.".to_string(),
            redacted: true,
            metadata_only: true,
            stores_config_values: false,
            duplicates_history_rows: false,
            summary: "metadata-only digest fixture".to_string(),
        }
    }

    #[test]
    fn tool_safety_audit_review_alerts_cover_digest_severity_levels() {
        let cases = [
            ("stable", true, true, "audit_stable", "ok"),
            ("improving", false, true, "audit_improving", "ok"),
            ("empty", false, true, "audit_empty", "info"),
            (
                "single_snapshot",
                false,
                true,
                "audit_single_snapshot",
                "info",
            ),
            ("changed", false, true, "audit_changed", "warning"),
            (
                "ready_to_blocked",
                false,
                true,
                "audit_ready_to_blocked",
                "blocked",
            ),
            (
                "runbook_missing",
                false,
                false,
                "audit_runbook_missing",
                "blocked",
            ),
            (
                "redaction_review",
                false,
                true,
                "audit_redaction_review",
                "blocked",
            ),
        ];

        for (state, ready, runbook_present, expected_id, expected_level) in cases {
            let digest = tool_safety_audit_digest_fixture(state, ready, runbook_present);
            let alerts = tool_safety_audit_review_alerts(&digest);

            assert_eq!(alerts[0].id, expected_id, "state {state}");
            assert_eq!(alerts[0].level, expected_level, "state {state}");
            assert!(alerts.iter().all(|alert| matches!(
                alert.level.as_str(),
                "blocked" | "warning" | "info" | "ok"
            )));
        }
    }

    #[test]
    fn tool_safety_audit_review_alerts_sort_and_stay_redacted() {
        let mut digest = tool_safety_audit_digest_fixture("stable", true, false);
        digest.redacted = false;
        digest.metadata_only = false;
        digest.stores_config_values = true;
        digest.duplicates_history_rows = true;
        digest.latest_remediation_action = Some(redact_sensitive_text(
            r"inspect token gsk_secret and path G:\Secret\audit.json",
        ));

        let alerts = tool_safety_audit_review_alerts(&digest);
        let body = serde_json::to_string(&alerts).unwrap();

        assert_eq!(alerts[0].level, "blocked");
        assert!(
            alerts
                .iter()
                .any(|alert| alert.id == "audit_runbook_missing")
        );
        assert!(
            alerts
                .iter()
                .any(|alert| alert.id == "audit_redaction_review")
        );
        assert!(alerts.iter().any(|alert| alert.id == "audit_stable"));
        assert!(!body.contains("gsk_secret"));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
    }

    #[test]
    fn tool_safety_audit_escalation_evidence_covers_severity_paths() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_evidence = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        assert_eq!(stable_evidence.state, "no_escalation");
        assert_eq!(stable_evidence.severity, "ok");
        assert_eq!(stable_evidence.blocked_count, 0);
        assert!(!stable_evidence.review_required);

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_evidence = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        assert_eq!(changed_evidence.state, "warning_escalation");
        assert_eq!(changed_evidence.severity, "warning");
        assert_eq!(changed_evidence.warning_alert_ids, vec!["audit_changed"]);
        assert!(changed_evidence.review_required);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_evidence = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        assert_eq!(blocked_evidence.state, "blocked_escalation");
        assert_eq!(blocked_evidence.severity, "blocked");
        assert_eq!(
            blocked_evidence.blocked_alert_ids,
            vec!["audit_ready_to_blocked"]
        );
        assert_eq!(
            blocked_evidence.alert_runbook_target,
            "tool-safety-audit-alert-runbook"
        );
        assert!(blocked_evidence.metadata_only);
        assert!(!blocked_evidence.duplicates_history_rows);
    }

    #[test]
    fn tool_safety_audit_escalation_evidence_stays_redacted() {
        let digest = tool_safety_audit_digest_fixture("changed", false, true);
        let alerts = vec![super::DxAgentsToolSafetyAuditReviewAlert {
            id: "audit_changed".to_string(),
            level: "warning".to_string(),
            title: "Changed".to_string(),
            detail: "Changed warning".to_string(),
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
        }];

        let evidence = tool_safety_audit_escalation_evidence(&digest, &alerts);
        let body = serde_json::to_string(&evidence).unwrap();

        assert_eq!(evidence.severity, "warning");
        assert!(evidence.recovery_hint.is_some());
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_drill_covers_blocked_warning_and_cleared_states() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        assert_eq!(stable_drill.state, "cleared_recovery");
        assert_eq!(stable_drill.outcome, "cleared");
        assert!(stable_drill.cleared);
        assert!(!stable_drill.invokes_tools);
        assert!(stable_drill.dry_run_only);
        assert!(stable_drill.runbook_present);
        assert_eq!(
            stable_drill.runbook_target,
            "tool-safety-audit-recovery-runbook"
        );

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        assert_eq!(changed_drill.state, "warning_recovery");
        assert_eq!(changed_drill.outcome, "review");
        assert_eq!(changed_drill.warning_alert_ids, vec!["audit_changed"]);
        assert!(changed_drill.review_required);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        assert_eq!(blocked_drill.state, "blocked_recovery");
        assert_eq!(blocked_drill.outcome, "blocked");
        assert_eq!(
            blocked_drill.blocked_alert_ids,
            vec!["audit_ready_to_blocked"]
        );
        assert!(blocked_drill.review_required);
        assert!(blocked_drill.metadata_only);
        assert!(!blocked_drill.duplicates_history_rows);
        assert!(blocked_drill.runbook_present);
    }

    #[test]
    fn tool_safety_audit_recovery_drill_stays_redacted_and_metadata_only() {
        let escalation = super::DxAgentsToolSafetyAuditEscalationEvidence {
            schema_version: "dx.tool_safety_audit_escalation.v1".to_string(),
            generated_at_ms: 42,
            state: "blocked_escalation".to_string(),
            severity: "blocked".to_string(),
            alert_count: 1,
            blocked_count: 1,
            warning_count: 0,
            info_count: 0,
            ok_count: 0,
            blocked_alert_ids: vec!["audit_ready_to_blocked".to_string()],
            warning_alert_ids: vec![],
            top_alert_id: Some("audit_ready_to_blocked".to_string()),
            top_alert_title: Some("Path G:\\Secret\\audit.json".to_string()),
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
            next_action: r"open G:\Secret\audit.json".to_string(),
            runbook_target: "tool-safety-audit-runbook".to_string(),
            alert_runbook_target: "tool-safety-audit-alert-runbook".to_string(),
            review_required: true,
            metadata_only: true,
            redacted: true,
            stores_config_values: false,
            duplicates_history_rows: false,
            summary: "blocked escalation".to_string(),
        };

        let drill = tool_safety_audit_recovery_drill(&escalation);
        let body = serde_json::to_string(&drill).unwrap();

        assert_eq!(drill.state, "blocked_recovery");
        assert!(drill.metadata_only);
        assert!(!drill.invokes_tools);
        assert!(!drill.stores_config_values);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_digest_covers_blocked_warning_pending_and_cleared_states() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        assert_eq!(stable_digest.state, "cleared");
        assert!(stable_digest.ready);
        assert!(!stable_digest.review_required);

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        assert_eq!(changed_digest.state, "warning_review");
        assert_eq!(changed_digest.warning_count, 1);
        assert!(!changed_digest.ready);
        assert!(changed_digest.review_required);

        let pending = tool_safety_audit_digest_fixture("improving", false, true);
        let pending_alerts = tool_safety_audit_review_alerts(&pending);
        let pending_escalation = tool_safety_audit_escalation_evidence(&pending, &pending_alerts);
        let pending_drill = tool_safety_audit_recovery_drill(&pending_escalation);
        let pending_digest = tool_safety_audit_recovery_digest(&pending_drill);
        assert_eq!(pending_digest.state, "pending_evidence");
        assert_eq!(pending_digest.outcome, "pending");
        assert!(!pending_digest.ready);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        assert_eq!(blocked_digest.state, "blocked");
        assert_eq!(blocked_digest.blocked_count, 1);
        assert!(!blocked_digest.ready);
    }

    #[test]
    fn tool_safety_audit_recovery_digest_stays_metadata_only_and_redacted() {
        let escalation = super::DxAgentsToolSafetyAuditEscalationEvidence {
            schema_version: "dx.tool_safety_audit_escalation.v1".to_string(),
            generated_at_ms: 42,
            state: "blocked_escalation".to_string(),
            severity: "blocked".to_string(),
            alert_count: 1,
            blocked_count: 1,
            warning_count: 0,
            info_count: 0,
            ok_count: 0,
            blocked_alert_ids: vec![r"token=secret_token".to_string()],
            warning_alert_ids: vec![],
            top_alert_id: Some("audit_ready_to_blocked".to_string()),
            top_alert_title: Some("Path G:\\Secret\\audit.json".to_string()),
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
            next_action: r"open G:\Secret\audit.json".to_string(),
            runbook_target: "tool-safety-audit-runbook".to_string(),
            alert_runbook_target: "tool-safety-audit-alert-runbook".to_string(),
            review_required: true,
            metadata_only: true,
            redacted: true,
            stores_config_values: false,
            duplicates_history_rows: false,
            summary: "blocked escalation".to_string(),
        };
        let drill = tool_safety_audit_recovery_drill(&escalation);
        let digest = tool_safety_audit_recovery_digest(&drill);
        let body = serde_json::to_string(&digest).unwrap();

        assert_eq!(digest.state, "blocked");
        assert!(digest.metadata_only);
        assert!(!digest.invokes_tools);
        assert!(!digest.stores_config_values);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_digest_alerts_cover_states_and_ordering() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        assert_eq!(stable_recovery_alerts[0].id, "recovery_cleared");
        assert_eq!(stable_recovery_alerts[0].level, "ok");
        assert!(stable_recovery_alerts[0].runbook_present);
        assert_eq!(
            stable_recovery_alerts[0].runbook_target,
            "tool-safety-audit-recovery-alert-runbook"
        );

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        assert_eq!(changed_recovery_alerts[0].id, "recovery_warning_review");
        assert_eq!(changed_recovery_alerts[0].level, "warning");

        let pending = tool_safety_audit_digest_fixture("improving", false, true);
        let pending_alerts = tool_safety_audit_review_alerts(&pending);
        let pending_escalation = tool_safety_audit_escalation_evidence(&pending, &pending_alerts);
        let pending_drill = tool_safety_audit_recovery_drill(&pending_escalation);
        let pending_digest = tool_safety_audit_recovery_digest(&pending_drill);
        let pending_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&pending_digest);
        assert_eq!(pending_recovery_alerts[0].id, "recovery_pending_evidence");
        assert_eq!(pending_recovery_alerts[0].level, "warning");

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        assert_eq!(blocked_recovery_alerts[0].id, "recovery_blocked");
        assert_eq!(blocked_recovery_alerts[0].level, "blocked");

        let mut runbook_missing_digest = stable_digest.clone();
        runbook_missing_digest.state = "runbook_missing".to_string();
        runbook_missing_digest.runbook_present = false;
        runbook_missing_digest.ready = false;
        let runbook_missing_alerts =
            tool_safety_audit_recovery_digest_alerts(&runbook_missing_digest);
        assert_eq!(runbook_missing_alerts[0].id, "recovery_runbook_missing");
        assert_eq!(runbook_missing_alerts[0].level, "blocked");

        let mut unsafe_digest = stable_digest.clone();
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.invokes_tools = true;
        let unsafe_alerts = tool_safety_audit_recovery_digest_alerts(&unsafe_digest);
        assert_eq!(unsafe_alerts[0].id, "recovery_redaction_review");
        assert_eq!(unsafe_alerts[0].level, "blocked");
        assert!(unsafe_alerts[0].metadata_only);
        assert!(!unsafe_alerts[0].duplicates_history_rows);
    }

    #[test]
    fn tool_safety_audit_recovery_digest_alerts_stay_redacted() {
        let mut digest = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        digest.redacted = true;
        let alerts = vec![super::DxAgentsToolSafetyAuditReviewAlert {
            id: r"token=secret_token".to_string(),
            level: "blocked".to_string(),
            title: "Blocked".to_string(),
            detail: "Blocked detail".to_string(),
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
        }];
        let escalation = tool_safety_audit_escalation_evidence(&digest, &alerts);
        let drill = tool_safety_audit_recovery_drill(&escalation);
        let recovery_digest = tool_safety_audit_recovery_digest(&drill);
        let recovery_alerts = tool_safety_audit_recovery_digest_alerts(&recovery_digest);
        let body = serde_json::to_string(&recovery_alerts).unwrap();

        assert!(!recovery_alerts.is_empty());
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_covers_severity_and_runbook_paths() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        assert_eq!(stable_alert_digest.state, "ok");
        assert_eq!(stable_alert_digest.severity, "ok");
        assert!(stable_alert_digest.ready);
        assert!(!stable_alert_digest.review_required);
        assert!(stable_alert_digest.digest_runbook_present);
        assert_eq!(
            stable_alert_digest.digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-runbook"
        );
        assert!(
            stable_alert_digest
                .summary
                .contains("digest runbook present")
        );

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        let changed_alert_digest =
            tool_safety_audit_recovery_alert_digest(&changed_recovery_alerts);
        assert_eq!(changed_alert_digest.state, "warning_review");
        assert_eq!(changed_alert_digest.severity, "warning");
        assert_eq!(changed_alert_digest.warning_count, 1);
        assert!(!changed_alert_digest.ready);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest(&blocked_recovery_alerts);
        assert_eq!(blocked_alert_digest.state, "blocked");
        assert_eq!(blocked_alert_digest.severity, "blocked");
        assert_eq!(blocked_alert_digest.blocked_count, 1);

        let mut missing_runbook_alerts = stable_recovery_alerts.clone();
        missing_runbook_alerts[0].runbook_present = false;
        let missing_runbook_digest =
            tool_safety_audit_recovery_alert_digest(&missing_runbook_alerts);
        assert_eq!(missing_runbook_digest.state, "runbook_missing");
        assert_eq!(missing_runbook_digest.severity, "blocked");
        assert!(!missing_runbook_digest.runbook_present);

        let mut unsafe_alerts = stable_recovery_alerts.clone();
        unsafe_alerts[0].metadata_only = false;
        unsafe_alerts[0].redacted = false;
        unsafe_alerts[0].stores_config_values = true;
        let unsafe_digest = tool_safety_audit_recovery_alert_digest(&unsafe_alerts);
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert_eq!(unsafe_digest.severity, "blocked");
        assert!(!unsafe_digest.metadata_only);
        assert!(!unsafe_digest.redacted);
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_stays_metadata_only_and_redacted() {
        let alerts = vec![super::DxAgentsToolSafetyAuditRecoveryAlert {
            id: r"token=secret_token".to_string(),
            level: "blocked".to_string(),
            title: r"Path G:\Secret\audit.json".to_string(),
            detail: "Blocked detail".to_string(),
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
            runbook_present: true,
            runbook_target: "tool-safety-audit-recovery-alert-runbook".to_string(),
            metadata_only: true,
            redacted: true,
            stores_config_values: false,
            duplicates_history_rows: false,
        }];
        let digest = tool_safety_audit_recovery_alert_digest(&alerts);
        let body = serde_json::to_string(&digest).unwrap();

        assert_eq!(digest.state, "blocked");
        assert!(digest.metadata_only);
        assert!(digest.redacted);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_covers_core_states() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        assert_eq!(stable_gate.state, "ok");
        assert!(stable_gate.ready);
        assert!(!stable_gate.release_blocking);
        assert!(stable_gate.safe_to_share);
        assert_eq!(stable_gate.digest_state, "ok");
        assert!(stable_gate.release_gate_runbook_present);
        assert_eq!(
            stable_gate.release_gate_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-runbook"
        );
        assert!(stable_gate.summary.contains("runbook present"));

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        let changed_alert_digest =
            tool_safety_audit_recovery_alert_digest(&changed_recovery_alerts);
        let changed_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&changed_alert_digest);
        assert_eq!(changed_gate.state, "warning_review");
        assert_eq!(changed_gate.severity, "warning");
        assert!(changed_gate.release_blocking);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest(&blocked_recovery_alerts);
        let blocked_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&blocked_alert_digest);
        assert_eq!(blocked_gate.state, "blocked");
        assert_eq!(blocked_gate.severity, "blocked");
        assert!(blocked_gate.release_blocking);

        let mut missing_runbook_digest = stable_alert_digest.clone();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.runbook_present = false;
        let missing_runbook_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&missing_runbook_digest);
        assert_eq!(missing_runbook_gate.state, "runbook_missing");
        assert!(missing_runbook_gate.release_blocking);

        let mut missing_digest_runbook = stable_alert_digest.clone();
        missing_digest_runbook.ready = false;
        missing_digest_runbook.digest_runbook_present = false;
        let missing_digest_runbook_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&missing_digest_runbook);
        assert_eq!(missing_digest_runbook_gate.state, "digest_runbook_missing");
        assert!(missing_digest_runbook_gate.release_blocking);

        let mut unsafe_digest = stable_alert_digest.clone();
        unsafe_digest.ready = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let unsafe_gate = tool_safety_audit_recovery_alert_digest_release_gate(&unsafe_digest);
        let body = serde_json::to_string(&unsafe_gate).unwrap();
        assert_eq!(unsafe_gate.state, "redaction_review");
        assert!(!unsafe_gate.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_covers_core_states() {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        assert_eq!(stable_gate_digest.state, "ready");
        assert_eq!(stable_gate_digest.severity, "ok");
        assert!(stable_gate_digest.ready);
        assert!(!stable_gate_digest.release_blocking);
        assert_eq!(stable_gate_digest.runbook_present_count, 4);
        assert_eq!(stable_gate_digest.missing_runbook_count, 0);
        assert!(stable_gate_digest.all_runbooks_present);
        assert!(stable_gate_digest.release_gate_digest_runbook_present);
        assert_eq!(
            stable_gate_digest.release_gate_digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook"
        );

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        let changed_alert_digest =
            tool_safety_audit_recovery_alert_digest(&changed_recovery_alerts);
        let changed_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&changed_alert_digest);
        let changed_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&changed_gate);
        assert_eq!(changed_gate_digest.state, "warning_review");
        assert_eq!(changed_gate_digest.severity, "warning");
        assert!(changed_gate_digest.release_blocking);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest(&blocked_recovery_alerts);
        let blocked_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&blocked_alert_digest);
        let blocked_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&blocked_gate);
        assert_eq!(blocked_gate_digest.state, "blocked");
        assert_eq!(blocked_gate_digest.severity, "blocked");
        assert!(blocked_gate_digest.release_blocking);

        let mut missing_runbook_gate = stable_gate.clone();
        missing_runbook_gate.ready = false;
        missing_runbook_gate.release_gate_runbook_present = false;
        let missing_runbook_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&missing_runbook_gate);
        assert_eq!(missing_runbook_digest.state, "runbook_missing");
        assert_eq!(missing_runbook_digest.missing_runbook_count, 1);
        assert!(!missing_runbook_digest.all_runbooks_present);

        let mut unsafe_gate = stable_gate.clone();
        unsafe_gate.ready = false;
        unsafe_gate.safe_to_share = false;
        unsafe_gate.metadata_only = false;
        unsafe_gate.redacted = false;
        unsafe_gate.stores_config_values = true;
        unsafe_gate.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let unsafe_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&unsafe_gate);
        let body = serde_json::to_string(&unsafe_digest).unwrap();
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert!(!unsafe_digest.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts_cover_states_and_ordering()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        assert_eq!(stable_gate_digest_alerts[0].id, "release_gate_digest_ready");
        assert_eq!(stable_gate_digest_alerts[0].level, "ok");
        assert!(stable_gate_digest_alerts[0].runbook_present);
        assert_eq!(
            stable_gate_digest_alerts[0].runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook"
        );
        assert!(stable_gate_digest_alerts[0].alert_runbook_present);
        assert_eq!(
            stable_gate_digest_alerts[0].alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook"
        );

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        let changed_alert_digest =
            tool_safety_audit_recovery_alert_digest(&changed_recovery_alerts);
        let changed_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&changed_alert_digest);
        let changed_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&changed_gate);
        let changed_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
                &changed_gate_digest,
            );
        assert_eq!(
            changed_gate_digest_alerts[0].id,
            "release_gate_digest_warning_review"
        );
        assert_eq!(changed_gate_digest_alerts[0].level, "warning");

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest(&blocked_recovery_alerts);
        let blocked_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&blocked_alert_digest);
        let blocked_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&blocked_gate);
        let blocked_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
                &blocked_gate_digest,
            );
        assert_eq!(
            blocked_gate_digest_alerts[0].id,
            "release_gate_digest_blocked"
        );
        assert_eq!(blocked_gate_digest_alerts[0].level, "blocked");

        let mut missing_runbook_digest = stable_gate_digest.clone();
        missing_runbook_digest.state = "release_gate_digest_runbook_missing".to_string();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.release_gate_digest_runbook_present = false;
        let missing_runbook_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
                &missing_runbook_digest,
            );
        assert_eq!(
            missing_runbook_alerts[0].id,
            "release_gate_digest_runbook_missing"
        );
        assert_eq!(missing_runbook_alerts[0].level, "blocked");
        assert!(!missing_runbook_alerts[0].runbook_present);
        assert!(missing_runbook_alerts[0].alert_runbook_present);

        let mut unsafe_digest = stable_gate_digest.clone();
        unsafe_digest.state = "redaction_review".to_string();
        unsafe_digest.ready = false;
        unsafe_digest.safe_to_share = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        let unsafe_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&unsafe_digest);
        assert_eq!(unsafe_alerts[0].id, "release_gate_digest_redaction_review");
        assert_eq!(unsafe_alerts[0].level, "blocked");
        assert!(unsafe_alerts[0].metadata_only);
        assert!(!unsafe_alerts[0].duplicates_history_rows);
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_covers_core_states()
    {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        assert_eq!(stable_gate_alert_digest.state, "ready");
        assert_eq!(stable_gate_alert_digest.severity, "ok");
        assert!(stable_gate_alert_digest.ready);
        assert!(stable_gate_alert_digest.all_runbooks_present);
        assert!(stable_gate_alert_digest.alert_digest_runbook_present);
        assert_eq!(
            stable_gate_alert_digest.alert_digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook"
        );
        assert_eq!(stable_gate_alert_digest.runbook_present_count, 3);

        let changed = tool_safety_audit_digest_fixture("changed", false, true);
        let changed_alerts = tool_safety_audit_review_alerts(&changed);
        let changed_escalation = tool_safety_audit_escalation_evidence(&changed, &changed_alerts);
        let changed_drill = tool_safety_audit_recovery_drill(&changed_escalation);
        let changed_digest = tool_safety_audit_recovery_digest(&changed_drill);
        let changed_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&changed_digest);
        let changed_alert_digest =
            tool_safety_audit_recovery_alert_digest(&changed_recovery_alerts);
        let changed_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&changed_alert_digest);
        let changed_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&changed_gate);
        let changed_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
                &changed_gate_digest,
            );
        let changed_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &changed_gate_digest_alerts,
            );
        assert_eq!(changed_gate_alert_digest.state, "warning_review");
        assert_eq!(changed_gate_alert_digest.severity, "warning");
        assert!(changed_gate_alert_digest.release_blocking);

        let blocked = tool_safety_audit_digest_fixture("ready_to_blocked", false, true);
        let blocked_alerts = tool_safety_audit_review_alerts(&blocked);
        let blocked_escalation = tool_safety_audit_escalation_evidence(&blocked, &blocked_alerts);
        let blocked_drill = tool_safety_audit_recovery_drill(&blocked_escalation);
        let blocked_digest = tool_safety_audit_recovery_digest(&blocked_drill);
        let blocked_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&blocked_digest);
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest(&blocked_recovery_alerts);
        let blocked_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&blocked_alert_digest);
        let blocked_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&blocked_gate);
        let blocked_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(
                &blocked_gate_digest,
            );
        let blocked_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &blocked_gate_digest_alerts,
            );
        assert_eq!(blocked_gate_alert_digest.state, "blocked");
        assert_eq!(blocked_gate_alert_digest.severity, "blocked");

        let mut missing_digest_runbook_alerts = stable_gate_digest_alerts.clone();
        missing_digest_runbook_alerts[0].runbook_present = false;
        let missing_digest_runbook =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &missing_digest_runbook_alerts,
            );
        assert_eq!(
            missing_digest_runbook.state,
            "release_gate_digest_runbook_missing"
        );
        assert_eq!(missing_digest_runbook.missing_runbook_count, 1);

        let mut missing_alert_runbook_alerts = stable_gate_digest_alerts.clone();
        missing_alert_runbook_alerts[0].alert_runbook_present = false;
        let missing_alert_runbook =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &missing_alert_runbook_alerts,
            );
        assert_eq!(
            missing_alert_runbook.state,
            "release_gate_digest_alert_runbook_missing"
        );
        assert_eq!(missing_alert_runbook.missing_runbook_count, 1);

        let mut unsafe_alerts = stable_gate_digest_alerts.clone();
        unsafe_alerts[0].metadata_only = false;
        unsafe_alerts[0].redacted = false;
        unsafe_alerts[0].stores_config_values = true;
        unsafe_alerts[0].gate_state = r"token=secret_token".to_string();
        unsafe_alerts[0].runbook_target = r"G:\Secret\audit.json".to_string();
        unsafe_alerts[0].alert_runbook_target = r"G:\Secret\alert.json".to_string();
        unsafe_alerts[0].recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let unsafe_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &unsafe_alerts,
            );
        let body = serde_json::to_string(&unsafe_digest).unwrap();
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert!(!unsafe_digest.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("alert.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts_cover_states_and_ordering()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let ready_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        assert_eq!(ready_alerts[0].id, "release_gate_digest_alert_digest_ready");
        assert_eq!(ready_alerts[0].level, "ok");
        assert!(ready_alerts[0].runbook_present);
        assert!(ready_alerts[0].alert_runbook_present);
        assert_eq!(
            ready_alerts[0].alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook"
        );
        assert!(ready_alerts[0].metadata_only);
        assert!(ready_alerts[0].redacted);
        assert!(!ready_alerts[0].stores_config_values);
        assert!(!ready_alerts[0].duplicates_history_rows);

        let mut warning_digest = stable_gate_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &warning_digest,
            );
        assert_eq!(
            warning_alerts[0].id,
            "release_gate_digest_alert_digest_warning_review"
        );
        assert_eq!(warning_alerts[0].level, "warning");

        let mut blocked_digest = stable_gate_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &blocked_digest,
            );
        assert_eq!(
            blocked_alerts[0].id,
            "release_gate_digest_alert_digest_blocked"
        );
        assert_eq!(blocked_alerts[0].level, "blocked");

        let mut missing_runbook_digest = stable_gate_alert_digest.clone();
        missing_runbook_digest.state =
            "release_gate_digest_alert_digest_runbook_missing".to_string();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.release_blocking = true;
        missing_runbook_digest.alert_digest_runbook_present = false;
        let missing_runbook_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &missing_runbook_digest,
            );
        assert_eq!(
            missing_runbook_alerts[0].id,
            "release_gate_digest_alert_digest_runbook_missing"
        );
        assert_eq!(missing_runbook_alerts[0].level, "blocked");
        assert!(!missing_runbook_alerts[0].runbook_present);
        assert!(missing_runbook_alerts[0].alert_runbook_present);

        let mut unsafe_digest = stable_gate_alert_digest.clone();
        unsafe_digest.state = "redaction_review".to_string();
        unsafe_digest.ready = false;
        unsafe_digest.release_blocking = true;
        unsafe_digest.safe_to_share = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.duplicates_history_rows = true;
        let unsafe_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &unsafe_digest,
            );
        assert_eq!(
            unsafe_alerts[0].id,
            "release_gate_digest_alert_digest_redaction_review"
        );
        assert_eq!(unsafe_alerts[0].level, "blocked");
        assert!(
            unsafe_alerts
                .iter()
                .all(|alert| alert.metadata_only && !alert.stores_config_values)
        );
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_covers_core_states()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        assert_eq!(stable_gate_alert_digest_alert_digest.state, "ready");
        assert_eq!(stable_gate_alert_digest_alert_digest.severity, "ok");
        assert!(stable_gate_alert_digest_alert_digest.ready);
        assert!(stable_gate_alert_digest_alert_digest.all_runbooks_present);
        assert_eq!(
            stable_gate_alert_digest_alert_digest.runbook_present_count,
            3
        );
        assert!(stable_gate_alert_digest_alert_digest.alert_digest_alert_digest_runbook_present);
        assert_eq!(
            stable_gate_alert_digest_alert_digest.alert_digest_alert_digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook"
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest.alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook"
        );

        let mut warning_digest = stable_gate_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &warning_digest,
            );
        let warning_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &warning_alerts,
            );
        assert_eq!(warning_alert_digest.state, "warning_review");
        assert_eq!(warning_alert_digest.severity, "warning");
        assert!(warning_alert_digest.release_blocking);

        let mut blocked_digest = stable_gate_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &blocked_digest,
            );
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &blocked_alerts,
            );
        assert_eq!(blocked_alert_digest.state, "blocked");
        assert_eq!(blocked_alert_digest.severity, "blocked");

        let mut missing_alert_digest_runbook_alerts = stable_gate_alert_digest_alerts.clone();
        missing_alert_digest_runbook_alerts[0].runbook_present = false;
        let missing_alert_digest_runbook =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &missing_alert_digest_runbook_alerts,
            );
        assert_eq!(
            missing_alert_digest_runbook.state,
            "release_gate_digest_alert_digest_runbook_missing"
        );
        assert_eq!(missing_alert_digest_runbook.missing_runbook_count, 1);

        let mut missing_alert_runbook_alerts = stable_gate_alert_digest_alerts.clone();
        missing_alert_runbook_alerts[0].alert_runbook_present = false;
        let missing_alert_runbook =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &missing_alert_runbook_alerts,
            );
        assert_eq!(
            missing_alert_runbook.state,
            "release_gate_digest_alert_digest_alert_runbook_missing"
        );
        assert_eq!(missing_alert_runbook.missing_runbook_count, 1);

        let mut unsafe_alerts = stable_gate_alert_digest_alerts.clone();
        unsafe_alerts[0].metadata_only = false;
        unsafe_alerts[0].redacted = false;
        unsafe_alerts[0].stores_config_values = true;
        unsafe_alerts[0].duplicates_history_rows = true;
        unsafe_alerts[0].digest_state = r"token=secret_token".to_string();
        unsafe_alerts[0].runbook_target = r"G:\Secret\alert-digest.json".to_string();
        unsafe_alerts[0].alert_runbook_target = r"G:\Secret\alert-runbook.json".to_string();
        unsafe_alerts[0].recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\alert.json".to_string());
        let unsafe_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &unsafe_alerts,
            );
        let body = serde_json::to_string(&unsafe_digest).unwrap();
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert!(!unsafe_digest.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest.json"));
        assert!(!body.contains("alert-runbook.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts_cover_states_and_ordering()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        let ready_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest,
            );
        let ready_alert = ready_alerts
            .iter()
            .find(|alert| alert.id == "release_gate_digest_alert_digest_alert_digest_ready")
            .unwrap();
        assert_eq!(ready_alert.level, "ok");
        assert!(ready_alert.runbook_present);
        assert!(ready_alert.alert_runbook_present);
        assert_eq!(
            ready_alert.alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook"
        );
        assert!(ready_alert.metadata_only);
        assert!(ready_alert.redacted);
        assert!(!ready_alert.stores_config_values);
        assert!(!ready_alert.duplicates_history_rows);
        assert!(!ready_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_alerts_runbook_missing"
        }));

        let mut warning_digest = stable_gate_alert_digest_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &warning_digest,
            );
        assert!(warning_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_warning_review"
                && alert.level == "warning"
        }));

        let mut blocked_digest = stable_gate_alert_digest_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &blocked_digest,
            );
        assert!(blocked_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_blocked"
                && alert.level == "blocked"
        }));

        let mut missing_runbook_digest = stable_gate_alert_digest_alert_digest.clone();
        missing_runbook_digest.state =
            "release_gate_digest_alert_digest_alert_digest_runbook_missing".to_string();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.release_blocking = true;
        missing_runbook_digest.alert_digest_alert_digest_runbook_present = false;
        let missing_runbook_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &missing_runbook_digest,
            );
        assert!(missing_runbook_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_runbook_missing"
                && alert.level == "blocked"
                && !alert.runbook_present
        }));

        let mut unsafe_digest = stable_gate_alert_digest_alert_digest.clone();
        unsafe_digest.state = "redaction_review".to_string();
        unsafe_digest.ready = false;
        unsafe_digest.release_blocking = true;
        unsafe_digest.safe_to_share = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.duplicates_history_rows = true;
        let unsafe_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &unsafe_digest,
            );
        assert!(unsafe_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_redaction_review"
                && alert.level == "blocked"
        }));
        assert!(
            unsafe_alerts
                .iter()
                .all(|alert| alert.metadata_only && !alert.stores_config_values)
        );
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts_stay_redacted()
     {
        let mut digest =
            super::DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigestAlertDigest {
                schema_version:
                    "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest.v1"
                        .to_string(),
                generated_at_ms: now_ms(),
                state: r"unknown token=secret_token".to_string(),
                ready: false,
                release_blocking: true,
                severity: "blocked".to_string(),
                alert_count: 1,
                blocked_count: 1,
                warning_count: 0,
                ok_count: 0,
                info_count: 0,
                top_alert_id: Some(r"token=secret_token".to_string()),
                top_alert_level: Some("blocked".to_string()),
                alert_digest_runbook_present: true,
                alert_digest_runbook_target: r"G:\Secret\alert-digest.json".to_string(),
                alert_runbook_present: true,
                alert_runbook_target: r"G:\Secret\alert.json".to_string(),
                alert_digest_alert_digest_runbook_present: true,
                alert_digest_alert_digest_runbook_target: r"G:\Secret\alert-digest-alert-digest.json"
                    .to_string(),
                runbook_count: 3,
                runbook_present_count: 3,
                missing_runbook_count: 0,
                all_runbooks_present: true,
                safe_to_share: false,
                metadata_only: false,
                redacted: false,
                stores_config_values: true,
                duplicates_history_rows: true,
                recovery_hint: Some(
                    r"inspect token=secret_token at G:\Secret\audit.json".to_string(),
                ),
                next_action: r"inspect token=secret_token at G:\Secret\alert-digest-alert-digest.json"
                    .to_string(),
                summary: "Blocked".to_string(),
            };
        digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &digest,
            );
        let body = serde_json::to_string(&alerts).unwrap();

        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_redaction_review"
        }));
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest-alert-digest.json"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_covers_core_states()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alerts,
            );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest.state,
            "ready"
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest.severity,
            "ok"
        );
        assert!(stable_gate_alert_digest_alert_digest_alert_digest.ready);
        assert!(stable_gate_alert_digest_alert_digest_alert_digest.all_runbooks_present);
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest.runbook_present_count,
            3
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_runbook_present
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_runbook_present
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_runbook_present
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook"
        );

        let mut warning_digest = stable_gate_alert_digest_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &warning_digest,
            );
        let warning_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &warning_alerts,
            );
        assert_eq!(warning_alert_digest.state, "warning_review");
        assert_eq!(warning_alert_digest.severity, "warning");
        assert!(warning_alert_digest.release_blocking);

        let mut blocked_digest = stable_gate_alert_digest_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &blocked_digest,
            );
        let blocked_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &blocked_alerts,
            );
        assert_eq!(blocked_alert_digest.state, "blocked");
        assert_eq!(blocked_alert_digest.severity, "blocked");

        let mut missing_runbook_alerts = stable_gate_alert_digest_alert_digest_alerts.clone();
        missing_runbook_alerts[0].runbook_present = false;
        let missing_runbook_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &missing_runbook_alerts,
            );
        assert_eq!(
            missing_runbook_digest.state,
            "release_gate_digest_alert_digest_alert_digest_runbook_missing"
        );
        assert_eq!(missing_runbook_digest.missing_runbook_count, 1);

        let mut missing_alert_runbook_alerts = stable_gate_alert_digest_alert_digest_alerts.clone();
        missing_alert_runbook_alerts[0].alert_runbook_present = false;
        let missing_alert_runbook_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &missing_alert_runbook_alerts,
            );
        assert_eq!(
            missing_alert_runbook_digest.state,
            "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing"
        );
        assert_eq!(missing_alert_runbook_digest.missing_runbook_count, 1);

        let mut unsafe_alerts = stable_gate_alert_digest_alert_digest_alerts.clone();
        unsafe_alerts[0].metadata_only = false;
        unsafe_alerts[0].redacted = false;
        unsafe_alerts[0].stores_config_values = true;
        unsafe_alerts[0].duplicates_history_rows = true;
        unsafe_alerts[0].digest_state = r"token=secret_token".to_string();
        unsafe_alerts[0].runbook_target = r"G:\Secret\alert-digest-alert-digest.json".to_string();
        unsafe_alerts[0].alert_runbook_target = r"G:\Secret\alert-runbook.json".to_string();
        unsafe_alerts[0].recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\alert.json".to_string());
        let unsafe_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &unsafe_alerts,
            );
        let body = serde_json::to_string(&unsafe_digest).unwrap();
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert!(!unsafe_digest.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest-alert-digest.json"));
        assert!(!body.contains("alert-runbook.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_cover_states_and_ordering()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alerts,
            );
        let ready_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest_alert_digest,
            );
        let ready_alert = ready_alerts
            .iter()
            .find(|alert| {
                alert.id == "release_gate_digest_alert_digest_alert_digest_alert_digest_ready"
            })
            .unwrap();
        assert_eq!(ready_alert.level, "ok");
        assert!(ready_alert.runbook_present);
        assert!(ready_alert.alert_runbook_present);
        assert_eq!(
            ready_alert.alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        );
        assert!(ready_alert.metadata_only);
        assert!(ready_alert.redacted);
        assert!(!ready_alert.stores_config_values);
        assert!(!ready_alert.duplicates_history_rows);
        assert!(!ready_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
        }));

        let mut warning_digest = stable_gate_alert_digest_alert_digest_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &warning_digest,
            );
        assert!(warning_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_alert_digest_warning_review"
                && alert.level == "warning"
        }));

        let mut blocked_digest = stable_gate_alert_digest_alert_digest_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &blocked_digest,
            );
        assert!(blocked_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_alert_digest_blocked"
                && alert.level == "blocked"
        }));

        let mut missing_runbook_digest = stable_gate_alert_digest_alert_digest_alert_digest.clone();
        missing_runbook_digest.state =
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
                .to_string();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.release_blocking = true;
        missing_runbook_digest.alert_digest_alert_digest_alert_digest_runbook_present = false;
        let missing_runbook_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &missing_runbook_digest,
            );
        assert!(missing_runbook_alerts.iter().any(|alert| {
            alert.id == "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
                && alert.level == "blocked"
                && !alert.runbook_present
        }));

        let mut unsafe_digest = stable_gate_alert_digest_alert_digest_alert_digest.clone();
        unsafe_digest.state = "redaction_review".to_string();
        unsafe_digest.ready = false;
        unsafe_digest.release_blocking = true;
        unsafe_digest.safe_to_share = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.duplicates_history_rows = true;
        unsafe_digest.top_alert_id = Some(r"token=secret_token".to_string());
        unsafe_digest.alert_digest_alert_digest_alert_digest_runbook_target =
            r"G:\Secret\alert-digest-alert-digest-alert-digest.json".to_string();
        unsafe_digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let unsafe_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &unsafe_digest,
            );
        let body = serde_json::to_string(&unsafe_alerts).unwrap();
        assert!(unsafe_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review"
                && alert.level == "blocked"
        }));
        assert!(
            unsafe_alerts
                .iter()
                .all(|alert| alert.metadata_only && !alert.stores_config_values)
        );
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest-alert-digest-alert-digest.json"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_covers_core_states()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alert_digest_alerts,
            );

        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.state,
            "ready"
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.severity,
            "ok"
        );
        assert!(stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.ready);
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.all_runbooks_present
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.runbook_count,
            3
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.runbook_present_count,
            3
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_alert_digest_runbook_present
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_present
        );
        assert!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        );
        assert_eq!(
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest
                .alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook"
        );

        let mut warning_alerts = stable_gate_alert_digest_alert_digest_alert_digest_alerts.clone();
        warning_alerts[0].level = "warning".to_string();
        warning_alerts[0].id =
            "release_gate_digest_alert_digest_alert_digest_alert_digest_warning_review".to_string();
        let warning_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &warning_alerts,
            );
        assert_eq!(warning_digest.state, "warning_review");
        assert_eq!(warning_digest.severity, "warning");
        assert!(warning_digest.release_blocking);

        let mut blocked_alerts = stable_gate_alert_digest_alert_digest_alert_digest_alerts.clone();
        blocked_alerts[0].level = "blocked".to_string();
        blocked_alerts[0].id =
            "release_gate_digest_alert_digest_alert_digest_alert_digest_blocked".to_string();
        let blocked_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &blocked_alerts,
            );
        assert_eq!(blocked_digest.state, "blocked");
        assert_eq!(blocked_digest.severity, "blocked");

        let mut missing_runbook_alerts =
            stable_gate_alert_digest_alert_digest_alert_digest_alerts.clone();
        missing_runbook_alerts[0].runbook_present = false;
        let missing_runbook_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &missing_runbook_alerts,
            );
        assert_eq!(
            missing_runbook_digest.state,
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
        );
        assert_eq!(missing_runbook_digest.missing_runbook_count, 1);

        let mut missing_alert_runbook_alerts =
            stable_gate_alert_digest_alert_digest_alert_digest_alerts.clone();
        missing_alert_runbook_alerts[0].alert_runbook_present = false;
        let missing_alert_runbook_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &missing_alert_runbook_alerts,
            );
        assert_eq!(
            missing_alert_runbook_digest.state,
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing"
        );
        assert_eq!(missing_alert_runbook_digest.missing_runbook_count, 1);

        let mut unsafe_alerts = stable_gate_alert_digest_alert_digest_alert_digest_alerts.clone();
        unsafe_alerts[0].metadata_only = false;
        unsafe_alerts[0].redacted = false;
        unsafe_alerts[0].stores_config_values = true;
        unsafe_alerts[0].duplicates_history_rows = true;
        unsafe_alerts[0].digest_state = r"token=secret_token".to_string();
        unsafe_alerts[0].runbook_target =
            r"G:\Secret\alert-digest-alert-digest-alert-digest-alert-digest.json".to_string();
        unsafe_alerts[0].alert_runbook_target =
            r"G:\Secret\alert-digest-alert-digest-alert-digest-alert-digest-alert.json".to_string();
        unsafe_alerts[0].recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\alert.json".to_string());
        let unsafe_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &unsafe_alerts,
            );
        let body = serde_json::to_string(&unsafe_digest).unwrap();
        assert_eq!(unsafe_digest.state, "redaction_review");
        assert!(!unsafe_digest.safe_to_share);
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest-alert-digest-alert-digest-alert-digest.json"));
        assert!(!body.contains("alert-digest-alert-digest-alert-digest-alert-digest-alert.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_cover_states_and_ordering()
     {
        let stable = tool_safety_audit_digest_fixture("stable", true, true);
        let stable_alerts = tool_safety_audit_review_alerts(&stable);
        let stable_escalation = tool_safety_audit_escalation_evidence(&stable, &stable_alerts);
        let stable_drill = tool_safety_audit_recovery_drill(&stable_escalation);
        let stable_digest = tool_safety_audit_recovery_digest(&stable_drill);
        let stable_recovery_alerts = tool_safety_audit_recovery_digest_alerts(&stable_digest);
        let stable_alert_digest = tool_safety_audit_recovery_alert_digest(&stable_recovery_alerts);
        let stable_gate =
            tool_safety_audit_recovery_alert_digest_release_gate(&stable_alert_digest);
        let stable_gate_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest(&stable_gate);
        let stable_gate_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&stable_gate_digest);
        let stable_gate_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest(
                &stable_gate_digest_alerts,
            );
        let stable_gate_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &stable_gate_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alerts,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest_alert_digest,
            );
        let stable_gate_alert_digest_alert_digest_alert_digest_alert_digest =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest(
                &stable_gate_alert_digest_alert_digest_alert_digest_alerts,
            );
        let ready_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
                &stable_gate_alert_digest_alert_digest_alert_digest_alert_digest,
            );
        let ready_alert = ready_alerts
            .iter()
            .find(|alert| {
                alert.id
                    == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_ready"
            })
            .unwrap();
        assert_eq!(ready_alert.level, "ok");
        assert!(ready_alert.runbook_present);
        assert!(ready_alert.alert_runbook_present);
        assert_eq!(
            ready_alert.alert_runbook_target,
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        );
        assert!(ready_alert.metadata_only);
        assert!(ready_alert.redacted);
        assert!(!ready_alert.stores_config_values);
        assert!(!ready_alert.duplicates_history_rows);
        assert!(!ready_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing"
        }));

        let mut warning_digest =
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.clone();
        warning_digest.state = "warning_review".to_string();
        warning_digest.ready = false;
        warning_digest.release_blocking = true;
        warning_digest.severity = "warning".to_string();
        let warning_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
                &warning_digest,
            );
        assert!(warning_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_warning_review"
                && alert.level == "warning"
        }));

        let mut blocked_digest =
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.clone();
        blocked_digest.state = "blocked".to_string();
        blocked_digest.ready = false;
        blocked_digest.release_blocking = true;
        blocked_digest.severity = "blocked".to_string();
        let blocked_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
                &blocked_digest,
            );
        assert!(blocked_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_blocked"
                && alert.level == "blocked"
        }));

        let mut missing_runbook_digest =
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.clone();
        missing_runbook_digest.state =
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
                .to_string();
        missing_runbook_digest.ready = false;
        missing_runbook_digest.release_blocking = true;
        missing_runbook_digest
            .alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_present =
            false;
        let missing_runbook_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
                &missing_runbook_digest,
            );
        assert!(missing_runbook_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing"
                && alert.level == "blocked"
                && !alert.runbook_present
        }));

        let mut unsafe_digest =
            stable_gate_alert_digest_alert_digest_alert_digest_alert_digest.clone();
        unsafe_digest.state = "redaction_review".to_string();
        unsafe_digest.ready = false;
        unsafe_digest.release_blocking = true;
        unsafe_digest.safe_to_share = false;
        unsafe_digest.metadata_only = false;
        unsafe_digest.redacted = false;
        unsafe_digest.stores_config_values = true;
        unsafe_digest.duplicates_history_rows = true;
        unsafe_digest.top_alert_id = Some(r"token=secret_token".to_string());
        unsafe_digest
            .alert_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_target =
            r"G:\Secret\alert-digest-alert-digest-alert-digest-alert-digest-alert-digest.json"
                .to_string();
        unsafe_digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let unsafe_alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts(
                &unsafe_digest,
            );
        let body = serde_json::to_string(&unsafe_alerts).unwrap();
        assert!(unsafe_alerts.iter().any(|alert| {
            alert.id
                == "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review"
                && alert.level == "blocked"
        }));
        assert!(
            unsafe_alerts
                .iter()
                .all(|alert| alert.metadata_only && !alert.stores_config_values)
        );
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(
            !body.contains("alert-digest-alert-digest-alert-digest-alert-digest-alert-digest.json")
        );
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts_stay_redacted()
     {
        let mut digest =
            super::DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigestAlertDigest {
                schema_version:
                    "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest.v1"
                        .to_string(),
                generated_at_ms: now_ms(),
                state: r"unknown token=secret_token".to_string(),
                ready: false,
                release_blocking: true,
                severity: "blocked".to_string(),
                alert_count: 1,
                blocked_count: 1,
                warning_count: 0,
                ok_count: 0,
                info_count: 0,
                top_alert_id: Some(r"token=secret_token".to_string()),
                top_alert_level: Some("blocked".to_string()),
                digest_runbook_present: true,
                digest_runbook_target: r"G:\Secret\digest.json".to_string(),
                alert_runbook_present: true,
                alert_runbook_target: r"G:\Secret\alert.json".to_string(),
                alert_digest_runbook_present: true,
                alert_digest_runbook_target: r"G:\Secret\alert-digest.json".to_string(),
                runbook_count: 3,
                runbook_present_count: 3,
                missing_runbook_count: 0,
                all_runbooks_present: true,
                safe_to_share: false,
                metadata_only: false,
                redacted: false,
                stores_config_values: true,
                duplicates_history_rows: true,
                recovery_hint: Some(
                    r"inspect token=secret_token at G:\Secret\audit.json".to_string(),
                ),
                next_action: r"inspect token=secret_token at G:\Secret\alert-digest.json"
                    .to_string(),
                summary: "Blocked".to_string(),
            };
        digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let alerts =
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alerts(
                &digest,
            );
        let body = serde_json::to_string(&alerts).unwrap();

        assert!(!alerts.is_empty());
        assert!(
            alerts
                .iter()
                .any(|alert| alert.id == "release_gate_digest_alert_digest_redaction_review")
        );
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("alert-digest.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts_stay_redacted() {
        let mut digest = super::DxAgentsToolSafetyAuditRecoveryAlertDigestReleaseGateDigest {
            schema_version: "dx.tool_safety_audit_recovery_alert_digest_release_gate_digest.v1"
                .to_string(),
            generated_at_ms: now_ms(),
            state: "blocked".to_string(),
            ready: false,
            release_blocking: true,
            severity: "blocked".to_string(),
            gate_state: r"token=secret_token".to_string(),
            gate_ready: false,
            alert_count: 1,
            blocked_count: 1,
            warning_count: 0,
            runbook_count: 4,
            runbook_present_count: 4,
            missing_runbook_count: 0,
            runbook_present: true,
            digest_runbook_present: true,
            release_gate_runbook_present: true,
            release_gate_digest_runbook_present: true,
            release_gate_digest_runbook_target: r"G:\Secret\audit.json".to_string(),
            all_runbooks_present: true,
            review_required: true,
            safe_to_share: true,
            metadata_only: true,
            redacted: true,
            stores_config_values: false,
            duplicates_history_rows: false,
            recovery_hint: Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string()),
            next_action: "Keep release work paused.".to_string(),
            summary: "Blocked".to_string(),
        };
        digest.recovery_hint =
            Some(r"inspect token=secret_token at G:\Secret\audit.json".to_string());
        let alerts = tool_safety_audit_recovery_alert_digest_release_gate_digest_alerts(&digest);
        let body = serde_json::to_string(&alerts).unwrap();

        assert!(!alerts.is_empty());
        assert!(!body.contains("secret_token"));
        assert!(!body.contains("token="));
        assert!(!body.contains("G:\\Secret"));
        assert!(!body.contains("audit.json"));
        assert!(!body.contains("entries"));
    }

    #[test]
    fn tool_safety_drill_history_export_path_uses_host_telemetry_directory() {
        let path = tool_safety_drill_history_export_path(1234);

        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("tool-safety-drill-history-1234.json")
        );
        assert!(path.display().to_string().contains("host-telemetry"));
    }

    #[test]
    fn tool_safety_drill_alerts_report_empty_history() {
        let trend = tool_safety_drill_trend(&[]);
        let alerts = tool_safety_drill_alerts(&trend, &[]);

        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].id, "empty_history");
        assert_eq!(alerts[0].level, "info");
    }

    #[test]
    fn tool_safety_drill_alerts_report_warning_state() {
        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 2, 0, 0, 0));
        latest.recorded_at_ms = 2;
        let records = vec![latest, previous];
        let trend = tool_safety_drill_trend(&records);
        let alerts = tool_safety_drill_alerts(&trend, &records);

        assert_eq!(trend.state, "changed");
        assert!(
            alerts
                .iter()
                .any(|alert| alert.id == "approval_required_tools" && alert.level == "warning")
        );
    }

    #[test]
    fn tool_safety_drill_alerts_report_blocked_state() {
        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        previous.recorded_at_ms = 1;
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 1, 1, 1));
        latest.recorded_at_ms = 2;
        let records = vec![latest, previous];
        let trend = tool_safety_drill_trend(&records);
        let alerts = tool_safety_drill_alerts(&trend, &records);

        assert_eq!(trend.state, "worsening");
        assert!(alerts.iter().any(|alert| alert.level == "blocked"));
        assert!(alerts.iter().any(|alert| alert.id == "critical_blockers"));
        assert!(alerts.iter().any(|alert| alert.id == "denied_tools"));
        assert!(alerts.iter().any(|alert| alert.id == "missing_tools"));
    }

    #[test]
    fn tool_safety_drill_alerts_report_improving_state() {
        let mut previous =
            tool_safety_drill_record("unit_test", &tool_safety_command(false, 4, 0, 0, 0, 1));
        previous.recorded_at_ms = 1;
        let mut latest =
            tool_safety_drill_record("unit_test", &tool_safety_command(true, 4, 0, 0, 0, 0));
        latest.recorded_at_ms = 2;
        let records = vec![latest, previous];
        let trend = tool_safety_drill_trend(&records);
        let alerts = tool_safety_drill_alerts(&trend, &records);

        assert_eq!(trend.state, "improving");
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].id, "improving_history");
        assert_eq!(alerts[0].level, "ok");
    }

    #[test]
    fn bridge_status_export_path_uses_host_telemetry_directory() {
        let path = bridge_status_export_path(1234);

        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("bridge-status-export-1234.json")
        );
        assert!(path.display().to_string().contains("host-telemetry"));
    }

    #[test]
    fn native_promotion_archive_status_is_schema_safe() {
        let value = native_promotion_archive_status();

        assert_eq!(
            value
                .get("schema_version")
                .and_then(serde_json::Value::as_str),
            Some("dx.embedded_terminal_native_promotion_archive.v1")
        );
        assert_ne!(
            value
                .pointer("/status/redaction/stores_payloads")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_ne!(
            value
                .pointer("/status/redaction/stores_input_values")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_ne!(
            value
                .pointer("/status/redaction/stores_terminal_frames")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
        assert_ne!(
            value
                .get("diagnostic_only")
                .and_then(serde_json::Value::as_bool),
            Some(false)
        );
    }

    #[test]
    fn bridge_status_export_file_name_validation_blocks_paths() {
        assert!(is_bridge_status_export_file_name(
            "bridge-status-export-1234.json"
        ));
        assert!(!is_bridge_status_export_file_name("notes.json"));
        assert!(bridge_status_export_file_path("bridge-status-export-1234.json").is_ok());
        assert!(bridge_status_export_file_path("..\\bridge-status-export-1234.json").is_err());
        assert!(bridge_status_export_file_path("bridge-status-export-1234.txt").is_err());
    }

    #[test]
    fn native_promotion_archive_file_name_validation_blocks_paths() {
        assert!(is_native_promotion_archive_file_name(
            "native-promotion-status-1234.json"
        ));
        assert!(!is_native_promotion_archive_file_name(
            "native-promotion.json"
        ));
        assert!(native_promotion_archive_file_path("native-promotion-status-1234.json").is_ok());
        assert!(
            native_promotion_archive_file_path("..\\native-promotion-status-1234.json").is_err()
        );
        assert!(native_promotion_archive_file_path("native-promotion-status-1234.txt").is_err());
    }

    #[test]
    fn native_promotion_archive_list_respects_limit() {
        let list = list_native_promotion_archives(1).unwrap();

        assert!(list.count <= 1);
        assert!(list.archive_dir.contains("native-promotion"));
    }

    #[test]
    fn native_promotion_archive_entry_reads_redacted_summary() {
        let dir = dx_agents_repo_dir()
            .join("target")
            .join(format!("native-promotion-entry-test-{}", now_ms()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("native-promotion-status-1234.json");
        fs::write(
            &path,
            r#"{
  "schema_version": "dx.embedded_terminal_native_promotion_archive.v1",
  "archived_at_ms": 1234,
  "retained_count": 2,
  "retention_limit": 20,
  "diagnostic_only": true,
  "status": {
    "schema_version": "dx.embedded_terminal_native_promotion_status.v1",
    "surface_count": 4,
    "blocker_count": 19,
    "diagnostic_only": true,
    "production_ready": false,
    "next_surface": "interactive_tui",
    "redaction": {
      "stores_payloads": false,
      "stores_input_values": false,
      "stores_terminal_frames": false
    },
    "evidence_lines": [
      {"redacted": true, "stores_payload": false}
    ],
    "rollback_messages": [
      "Keep mpv, tplay, viu, and Windows Terminal as production fallbacks until media-session gates pass."
    ]
  }
}"#,
        )
        .unwrap();

        let entry = native_promotion_archive_entry(&path).unwrap().unwrap();

        assert_eq!(entry.file_name, "native-promotion-status-1234.json");
        assert_eq!(entry.archived_at_ms, 1234);
        assert_eq!(entry.retained_count, 2);
        assert_eq!(entry.retention_limit, 20);
        assert_eq!(entry.blocker_count, 19);
        assert_eq!(entry.surface_count, 4);
        assert!(entry.diagnostic_only);
        assert!(!entry.production_ready);
        assert!(entry.redacted);
        assert_eq!(entry.next_surface.as_deref(), Some("interactive_tui"));
        assert!(entry.rollback_summary.contains("mpv, tplay, viu"));
        assert!(!entry.rollback_summary.contains("payload="));

        let _ = fs::remove_dir_all(dir);
    }

    fn native_promotion_archive_entry_fixture(
        file_name: &str,
        blocker_count: usize,
        retained_count: usize,
        production_ready: bool,
        next_surface: Option<&str>,
        rollback_summary: &str,
        redacted: bool,
    ) -> DxCliNativePromotionArchiveEntry {
        DxCliNativePromotionArchiveEntry {
            file_name: file_name.to_string(),
            path: format!(r"G:\Cli\target\native-promotion\{file_name}"),
            modified_at_ms: 1234,
            size_bytes: 256,
            archived_at_ms: 1234,
            retained_count,
            retention_limit: 20,
            blocker_count,
            surface_count: 4,
            diagnostic_only: true,
            production_ready,
            redacted,
            rollback_summary: rollback_summary.to_string(),
            next_surface: next_surface.map(str::to_string),
        }
    }

    #[test]
    fn native_promotion_archive_diff_reports_empty_history() {
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[],
        );

        assert!(!diff.available);
        assert_eq!(diff.snapshot_count, 0);
        assert!(diff.changes.is_empty());
        assert!(diff.summary.contains("No retained"));
        assert_eq!(diff.trend_history.trend, "short");
        assert_eq!(diff.trend_history.sample_count, 0);
        assert!(!diff.trend_history.available);
        assert!(diff.runbook.diagnostic_only);
        assert!(diff.runbook.safety_summary.contains("external fallbacks"));
        assert!(diff.redacted);
        assert!(diff.diagnostic_only);
        assert_eq!(diff.alert_level, "clean");
        assert!(diff.alerts.is_empty());
        assert!(diff.recovery_hint.is_some());
    }

    #[test]
    fn native_promotion_archive_diff_reports_single_snapshot() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            19,
            1,
            false,
            Some("interactive_tui"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest],
        );

        assert!(diff.available);
        assert_eq!(diff.snapshot_count, 1);
        assert_eq!(
            diff.latest_file_name.as_deref(),
            Some("native-promotion-status-2000.json")
        );
        assert!(diff.previous_file_name.is_none());
        assert!(diff.summary.contains("Only one"));
        assert_eq!(diff.trend_history.trend, "short");
        assert_eq!(diff.trend_history.sample_count, 1);
        assert!(!diff.trend_history.available);
        assert!(diff.redacted);
        assert!(diff.diagnostic_only);
        assert_eq!(diff.alert_level, "clean");
        assert!(diff.alerts.is_empty());
    }

    #[test]
    fn native_promotion_archive_diff_summarizes_latest_two_snapshots() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "keep external media fallback",
            true,
        );
        let previous = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            19,
            2,
            false,
            Some("interactive_tui"),
            "fallback preserved",
            true,
        );
        let older = native_promotion_archive_entry_fixture(
            "native-promotion-status-1000.json",
            21,
            1,
            false,
            Some("interactive_tui"),
            "older fallback",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, previous, older],
        );

        assert!(diff.available);
        assert_eq!(diff.snapshot_count, 3);
        assert_eq!(diff.blocker_delta, -2);
        assert_eq!(diff.trend_history.trend, "improving");
        assert_eq!(diff.trend_history.blocker_delta_total, -4);
        assert_eq!(diff.trend_history.points.len(), 3);
        assert_eq!(diff.retention_delta, 1);
        assert!(!diff.production_ready_changed);
        assert!(diff.next_surface_changed);
        assert!(diff.rollback_changed);
        assert!(diff.redacted);
        assert!(diff.diagnostic_only);
        assert!(diff.summary.contains("Blockers"));
        assert!(diff.summary.contains("Next surface"));
        assert_eq!(diff.alert_level, "warning");
        assert!(
            diff.alerts
                .iter()
                .any(|alert| alert.id == "rollback_changed")
        );
        assert_eq!(diff.changes.len(), 5);
        assert_eq!(
            diff.changes
                .iter()
                .find(|change| change.id == "blockers")
                .and_then(|change| change.delta),
            Some(-2)
        );
        assert!(!diff.summary.contains("payload="));
    }

    #[test]
    fn native_promotion_archive_diff_requires_redacted_snapshots() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let previous = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            2,
            false,
            Some("terminal_video"),
            "fallback preserved",
            false,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, previous],
        );

        assert!(!diff.redacted);
        assert_eq!(diff.trend_history.trend, "worsening");
        assert_eq!(diff.trend_history.blocked_alert_count, 1);
        assert!(diff.diagnostic_only);
        assert_eq!(diff.retention_delta, 1);
        assert_eq!(diff.alert_level, "blocked");
        assert!(
            diff.alerts
                .iter()
                .any(|alert| alert.id == "previous_redaction_not_confirmed")
        );
    }

    #[test]
    fn native_promotion_archive_drift_alerts_report_clean_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let previous = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, previous],
        );

        assert_eq!(diff.alert_level, "clean");
        assert!(diff.alerts.is_empty());
    }

    #[test]
    fn native_promotion_archive_drift_alerts_report_warning_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            17,
            3,
            true,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let previous = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, previous],
        );

        assert_eq!(diff.alert_level, "warning");
        assert!(
            diff.alerts
                .iter()
                .any(|alert| alert.id == "production_ready_flip")
        );
        assert!(
            diff.alerts
                .iter()
                .all(|alert| !alert.recovery_hint.is_empty())
        );
    }

    #[test]
    fn native_promotion_archive_drift_alerts_report_blocked_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            21,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            false,
        );
        let previous = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, previous],
        );

        assert_eq!(diff.alert_level, "blocked");
        assert!(
            diff.alerts
                .iter()
                .any(|alert| alert.id == "blockers_increased")
        );
        assert!(
            diff.alerts
                .iter()
                .any(|alert| alert.id == "redaction_regressed")
        );
        assert!(
            diff.alerts
                .iter()
                .all(|alert| !alert.recovery_hint.is_empty())
        );
    }

    #[test]
    fn native_promotion_archive_trend_history_reports_stable_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            17,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let middle = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            2,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let oldest = native_promotion_archive_entry_fixture(
            "native-promotion-status-1000.json",
            17,
            1,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, middle, oldest],
        );

        assert_eq!(diff.trend_history.trend, "stable");
        assert_eq!(diff.trend_history.blocker_delta_total, 0);
        assert_eq!(diff.trend_history.warning_alert_count, 0);
        assert_eq!(diff.trend_history.blocked_alert_count, 0);
        assert!(diff.trend_history.recovery_hint.is_none());
    }

    #[test]
    fn native_promotion_archive_trend_history_reports_improving_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            15,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let middle = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            17,
            2,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let oldest = native_promotion_archive_entry_fixture(
            "native-promotion-status-1000.json",
            19,
            1,
            false,
            Some("interactive_tui"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, middle, oldest],
        );

        assert_eq!(diff.trend_history.trend, "improving");
        assert_eq!(diff.trend_history.blocker_delta_total, -4);
        assert_eq!(diff.trend_history.latest_blocker_count, Some(15));
        assert_eq!(diff.trend_history.oldest_blocker_count, Some(19));
        assert!(diff.trend_history.summary.contains("improving"));
    }

    #[test]
    fn native_promotion_archive_trend_history_reports_worsening_state() {
        let latest = native_promotion_archive_entry_fixture(
            "native-promotion-status-3000.json",
            23,
            3,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let middle = native_promotion_archive_entry_fixture(
            "native-promotion-status-2000.json",
            21,
            2,
            false,
            Some("terminal_video"),
            "fallback preserved",
            true,
        );
        let oldest = native_promotion_archive_entry_fixture(
            "native-promotion-status-1000.json",
            19,
            1,
            false,
            Some("interactive_tui"),
            "fallback preserved",
            true,
        );
        let diff = native_promotion_archive_diff_from_entries(
            r"G:\Cli\target\native-promotion".to_string(),
            &[latest, middle, oldest],
        );

        assert_eq!(diff.trend_history.trend, "worsening");
        assert_eq!(diff.trend_history.blocker_delta_total, 4);
        assert!(diff.trend_history.blocked_alert_count >= 2);
        assert!(diff.trend_history.recovery_hint.is_some());
    }

    #[test]
    fn native_promotion_archive_trend_runbook_covers_states_and_fallbacks() {
        let runbook = native_promotion_archive_trend_runbook();
        let states = runbook
            .guidance
            .iter()
            .map(|item| item.state.as_str())
            .collect::<HashSet<_>>();

        assert!(runbook.diagnostic_only);
        assert!(states.contains("short"));
        assert!(states.contains("stable"));
        assert!(states.contains("improving"));
        assert!(states.contains("worsening"));
        assert!(
            runbook
                .external_fallbacks
                .contains(&"Windows Terminal".to_string())
        );
        assert!(runbook.external_fallbacks.contains(&"mpv".to_string()));
        assert!(runbook.external_fallbacks.contains(&"tplay".to_string()));
        assert!(runbook.external_fallbacks.contains(&"viu".to_string()));
    }

    #[test]
    fn native_promotion_archive_trend_runbook_text_stays_diagnostic_only() {
        let source = include_str!("../../../../docs/native-promotion-archive-trend-runbook.md");

        assert!(native_promotion_archive_trend_runbook_is_safe(source));
    }

    #[test]
    fn tool_safety_alert_runbook_text_stays_metadata_only() {
        let source = include_str!("../../../../docs/tool-safety-alert-runbook.md");

        assert!(tool_safety_alert_runbook_is_safe(source));
        for state in ["blocked", "warning", "improving", "stable", "empty"] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
    }

    #[test]
    fn tool_safety_audit_review_runbook_text_stays_metadata_only() {
        let source = include_str!("../../../../docs/tool-safety-audit-review-runbook.md");

        assert!(tool_safety_audit_review_runbook_is_safe(source));
        for state in [
            "empty",
            "single snapshot",
            "stable",
            "changed",
            "improving",
            "worsening",
            "ready-to-blocked",
            "redaction review",
            "runbook missing",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
    }

    #[test]
    fn tool_safety_audit_review_alert_runbook_text_stays_metadata_only() {
        let source = include_str!("../../../../docs/tool-safety-audit-review-alert-runbook.md");

        assert!(tool_safety_audit_review_alert_runbook_is_safe(source));
        for alert_id in [
            "audit_empty",
            "audit_single_snapshot",
            "audit_stable",
            "audit_improving",
            "audit_changed",
            "audit_ready_to_blocked",
            "audit_runbook_missing",
            "audit_redaction_review",
            "audit_worsening",
            "audit_review_required",
        ] {
            assert!(source.contains(alert_id));
        }
        for level in ["ok", "info", "warning", "blocked"] {
            assert!(source.to_ascii_lowercase().contains(level));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_runbook_text_stays_metadata_only() {
        let source = include_str!("../../../../docs/tool-safety-audit-review-recovery-runbook.md");

        assert!(tool_safety_audit_recovery_runbook_is_safe(source));
        for state in [
            "blocked_recovery",
            "warning_recovery",
            "evidence_pending",
            "cleared_recovery",
            "runbook_missing_recovery",
        ] {
            assert!(source.contains(state));
        }
        for outcome in ["blocked", "review", "pending", "cleared"] {
            assert!(source.to_ascii_lowercase().contains(outcome));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_runbook_text_stays_metadata_only() {
        let source =
            include_str!("../../../../docs/tool-safety-audit-review-recovery-alert-runbook.md");

        assert!(tool_safety_audit_recovery_alert_runbook_is_safe(source));
        for alert_id in [
            "recovery_blocked",
            "recovery_warning_review",
            "recovery_pending_evidence",
            "recovery_cleared",
            "recovery_runbook_missing",
            "recovery_alert_runbook_missing",
            "recovery_redaction_review",
        ] {
            assert!(source.contains(alert_id));
        }
        for level in ["blocked", "warning", "ok"] {
            assert!(source.to_ascii_lowercase().contains(level));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_runbook_text_stays_metadata_only() {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-runbook.md"
        );

        assert!(tool_safety_audit_recovery_alert_digest_runbook_is_safe(
            source
        ));
        for state in [
            "ok",
            "warning_review",
            "blocked",
            "runbook_missing",
            "digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_runbook_text_stays_metadata_only() {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-runbook.md"
        );

        assert!(tool_safety_audit_recovery_alert_digest_release_gate_runbook_is_safe(source));
        for state in [
            "ok",
            "warning_review",
            "blocked",
            "runbook_missing",
            "digest_runbook_missing",
            "release_gate_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_runbook_is_safe(source)
        );
        for state in [
            "ready",
            "warning_review",
            "blocked",
            "runbook_missing",
            "release_gate_digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_runbook_is_safe(
                source
            )
        );
        for state in [
            "release_gate_digest_ready",
            "release_gate_digest_warning_review",
            "release_gate_digest_blocked",
            "release_gate_digest_runbook_missing",
            "release_gate_digest_alert_runbook_missing",
            "release_gate_digest_redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook_is_safe(source)
        );
        for state in [
            "ready",
            "warning_review",
            "blocked",
            "release_gate_digest_runbook_missing",
            "release_gate_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook_is_safe(source)
        );
        for state in [
            "release_gate_digest_alert_digest_ready",
            "release_gate_digest_alert_digest_warning_review",
            "release_gate_digest_alert_digest_blocked",
            "release_gate_digest_alert_digest_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alerts_runbook_missing",
            "release_gate_digest_alert_digest_redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook_is_safe(source)
        );
        for state in [
            "ready",
            "warning_review",
            "blocked",
            "release_gate_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook_is_safe(source)
        );
        for state in [
            "release_gate_digest_alert_digest_alert_digest_ready",
            "release_gate_digest_alert_digest_alert_digest_warning_review",
            "release_gate_digest_alert_digest_alert_digest_blocked",
            "release_gate_digest_alert_digest_alert_digest_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alerts_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe(source)
        );
        for state in [
            "ready",
            "warning_review",
            "blocked",
            "release_gate_digest_alert_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe(source)
        );
        for state in [
            "release_gate_digest_alert_digest_alert_digest_alert_digest_ready",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_warning_review",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_blocked",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_is_safe(source)
        );
        for state in [
            "ready",
            "warning_review",
            "blocked",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_text_stays_metadata_only()
     {
        let source = include_str!(
            "../../../../docs/tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md"
        );

        assert!(
            tool_safety_audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_is_safe(source)
        );
        for state in [
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_ready",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_warning_review",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_blocked",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alerts_runbook_missing",
            "release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_redaction_review",
        ] {
            assert!(source.to_ascii_lowercase().contains(state));
        }
        for rule in ["metadata-only", "redacted", "config-free"] {
            assert!(source.to_ascii_lowercase().contains(rule));
        }
    }

    #[test]
    fn continuation_doc_targets_resolve_inside_repo() {
        let todo = continuation_target_path("todo").unwrap();
        let changelog = continuation_target_path(" CHANGELOG ").unwrap();
        let runbook = continuation_target_path("native-promotion-runbook").unwrap();
        let tool_runbook = continuation_target_path("tool-safety-runbook").unwrap();
        let audit_runbook = continuation_target_path("tool-safety-audit-runbook").unwrap();
        let audit_alert_runbook =
            continuation_target_path("tool-safety-audit-alert-runbook").unwrap();
        let audit_recovery_runbook =
            continuation_target_path("tool-safety-audit-recovery-runbook").unwrap();
        let audit_recovery_alert_runbook =
            continuation_target_path("tool-safety-audit-recovery-alert-runbook").unwrap();
        let audit_recovery_alert_digest_runbook =
            continuation_target_path("tool-safety-audit-recovery-alert-digest-runbook").unwrap();
        let audit_recovery_alert_digest_release_gate_runbook = continuation_target_path(
            "tool-safety-audit-recovery-alert-digest-release-gate-runbook",
        )
        .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_runbook = continuation_target_path(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook",
        )
        .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook",
            )
            .unwrap();
        let audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook =
            continuation_target_path(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook",
            )
            .unwrap();

        assert_eq!(
            todo.file_name().and_then(|name| name.to_str()),
            Some("TODO.md")
        );
        assert_eq!(
            changelog.file_name().and_then(|name| name.to_str()),
            Some("CHANGELOG.md")
        );
        assert_eq!(todo.parent(), changelog.parent());
        assert_eq!(
            runbook.file_name().and_then(|name| name.to_str()),
            Some("native-promotion-archive-trend-runbook.md")
        );
        assert_eq!(
            tool_runbook.file_name().and_then(|name| name.to_str()),
            Some("tool-safety-alert-runbook.md")
        );
        assert_eq!(
            tool_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_runbook.file_name().and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-runbook.md")
        );
        assert_eq!(
            audit_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RUNBOOK
        );
        assert_eq!(
            audit_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-alert-runbook.md")
        );
        assert_eq!(
            audit_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-recovery-runbook.md")
        );
        assert_eq!(
            audit_recovery_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-recovery-alert-runbook.md")
        );
        assert_eq!(
            audit_recovery_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-recovery-alert-digest-runbook.md")
        );
        assert_eq!(
            audit_recovery_alert_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-recovery-alert-digest-release-gate-runbook.md")
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some("tool-safety-audit-review-recovery-alert-digest-release-gate-digest-runbook.md")
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_RUNBOOK
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .file_name()
                .and_then(|name| name.to_str()),
            Some(
                "tool-safety-audit-review-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook.md"
            )
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .strip_prefix(dx_agents_repo_dir())
                .unwrap()
                .display()
                .to_string(),
            TOOL_SAFETY_AUDIT_REVIEW_RECOVERY_ALERT_DIGEST_RELEASE_GATE_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_DIGEST_ALERT_RUNBOOK
        );
        assert_eq!(runbook.parent(), tool_runbook.parent());
        assert_eq!(tool_runbook.parent(), audit_runbook.parent());
        assert_eq!(audit_runbook.parent(), audit_alert_runbook.parent());
        assert_eq!(
            audit_alert_runbook.parent(),
            audit_recovery_runbook.parent()
        );
        assert_eq!(
            audit_recovery_runbook.parent(),
            audit_recovery_alert_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_runbook.parent(),
            audit_recovery_alert_digest_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_runbook.parent(),
            audit_recovery_alert_digest_release_gate_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_runbook.parent(),
            audit_recovery_alert_digest_release_gate_digest_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_runbook.parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_runbook.parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_runbook.parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook.parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_runbook.parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook
                .parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_runbook
                .parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook
                .parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_runbook
                .parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook
                .parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_runbook
                .parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook
                .parent()
        );
        assert_eq!(
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_runbook
                .parent(),
            audit_recovery_alert_digest_release_gate_digest_alert_digest_alert_digest_alert_digest_alert_digest_alert_runbook
                .parent()
        );
    }

    #[test]
    fn continuation_doc_targets_reject_unknown_names() {
        let error = continuation_target_path("secrets").unwrap_err();
        assert!(error.contains("tool-safety-audit-recovery-runbook"));
        assert!(error.contains("tool-safety-audit-recovery-alert-runbook"));
        assert!(error.contains("tool-safety-audit-recovery-alert-digest-runbook"));
        assert!(error.contains("tool-safety-audit-recovery-alert-digest-release-gate-runbook"));
        assert!(
            error.contains("tool-safety-audit-recovery-alert-digest-release-gate-digest-runbook")
        );
        assert!(
            error.contains(
                "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-runbook"
            )
        );
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-runbook"
        ));
        assert!(error.contains(
            "tool-safety-audit-recovery-alert-digest-release-gate-digest-alert-digest-alert-digest-alert-digest-alert-digest-alert-runbook"
        ));
    }

    #[test]
    fn substitutes_input_token_without_shell_joining() {
        let action = DxCliAction {
            id: "dx.inspect".into(),
            group: "media".into(),
            label: "Inspect".into(),
            description: String::new(),
            argv: vec!["powershell".into(), "-File".into(), "{{input}}".into()],
            cwd: r"G:\Cli".into(),
            output: "json".into(),
            requires_terminal: false,
            accepts_input: true,
            input_placeholder: Some("{{input}}".into()),
            enabled: true,
            required_tools: vec![],
            required_capabilities: vec![],
        };

        assert_eq!(
            argv_for_action(&action, Some(r"G:\Cli\samples\dx-smoke.mp4")).unwrap(),
            vec![
                "powershell".to_string(),
                "-File".to_string(),
                r"G:\Cli\samples\dx-smoke.mp4".to_string()
            ]
        );
    }

    #[test]
    fn wraps_terminal_action_for_windows_terminal() {
        let action = DxCliAction {
            id: "dx.files".into(),
            group: "terminal".into(),
            label: "Files".into(),
            description: String::new(),
            argv: vec![
                "powershell".into(),
                "-NoProfile".into(),
                "-File".into(),
                r"G:\Cli\dx.ps1".into(),
                "files".into(),
            ],
            cwd: r"G:\Cli".into(),
            output: "terminal".into(),
            requires_terminal: true,
            accepts_input: false,
            input_placeholder: None,
            enabled: true,
            required_tools: vec![],
            required_capabilities: vec![],
        };

        assert_eq!(
            windows_terminal_argv(&action, &action.argv),
            vec![
                "new-tab".to_string(),
                "--title".to_string(),
                "DX Files".to_string(),
                "-d".to_string(),
                r"G:\Cli".to_string(),
                "powershell".to_string(),
                "-NoProfile".to_string(),
                "-File".to_string(),
                r"G:\Cli\dx.ps1".to_string(),
                "files".to_string()
            ]
        );
    }

    #[test]
    fn terminal_title_does_not_duplicate_dx_prefix() {
        let action = DxCliAction {
            id: "dx.tui".into(),
            group: "shell".into(),
            label: "DX Mission Control".into(),
            description: String::new(),
            argv: vec![
                "powershell".into(),
                "-NoProfile".into(),
                "-File".into(),
                r"G:\Cli\dx.ps1".into(),
                "tui".into(),
            ],
            cwd: r"G:\Cli".into(),
            output: "tui".into(),
            requires_terminal: true,
            accepts_input: false,
            input_placeholder: None,
            enabled: true,
            required_tools: vec![],
            required_capabilities: vec![],
        };
        let argv = windows_terminal_argv(&action, &action.argv);

        assert!(argv.contains(&"DX Mission Control".to_string()));
        assert!(!argv.contains(&"DX DX Mission Control".to_string()));
    }

    #[test]
    fn captured_actions_reject_terminal_and_external_media_actions() {
        let terminal = sample_action("dx.files", "Files", true, false);
        let mut external = sample_action("dx.play", "Play", false, false);
        external.output = "external_media".into();
        let captured = sample_action("dx.status", "Status", false, false);

        assert!(ensure_capturable_action(&terminal).is_err());
        assert!(ensure_capturable_action(&external).is_err());
        assert!(ensure_capturable_action(&captured).is_ok());
    }

    #[test]
    fn dx_agents_cli_candidates_prefer_workspace_debug_binary() {
        let candidates = dx_agents_cli_candidates(r"G:\Dx\agent".as_ref());

        assert_eq!(
            candidates[0],
            PathBuf::from(r"G:\Dx\agent")
                .join("target")
                .join("debug")
                .join(dx_agents_cli_file_name())
        );
    }

    #[test]
    fn parses_dashboard_command_json_payload() {
        let (json, error) = parse_optional_json(r#"{"status":"ok"}"#);

        assert_eq!(json.unwrap()["status"], "ok");
        assert!(error.is_none());
    }

    #[test]
    fn preserves_dashboard_json_parse_errors() {
        let (json, error) = parse_optional_json("not json");

        assert!(json.is_none());
        assert!(error.unwrap().contains("expected ident"));
    }

    #[test]
    fn bridge_settings_use_contract_defaults() {
        let history = PathBuf::from(r"G:\Cli\target\history.jsonl");
        let mut contract = sample_contract_with_telemetry(vec![], &history);
        contract.settings = Some(sample_settings_contract());

        let defaults = default_bridge_settings_for_contract(&contract);

        assert_eq!(defaults.media_input, r"G:\Cli\samples\dx-smoke.mp4");
        assert_eq!(defaults.preferred_terminal_surface, "external_terminal");
        assert_eq!(defaults.command_history_limit, 6);
        assert_eq!(defaults.safe_launch_policy, "confirm_external_terminal");
        assert_eq!(defaults.provider_health_mode, "dry-run");
    }

    #[test]
    fn bridge_settings_normalize_to_contract_constraints() {
        let history = PathBuf::from(r"G:\Cli\target\history.jsonl");
        let mut contract = sample_contract_with_telemetry(vec![], &history);
        contract.settings = Some(sample_settings_contract());
        let settings = DxCliBridgeSettings {
            media_input: " ".into(),
            preferred_terminal_surface: "unknown_surface".into(),
            command_history_limit: 999,
            safe_launch_policy: "unsafe".into(),
            provider_health_mode: "network-now".into(),
        };

        let normalized = normalize_bridge_settings(&contract, settings);

        assert_eq!(normalized.media_input, r"G:\Cli\samples\dx-smoke.mp4");
        assert_eq!(normalized.preferred_terminal_surface, "external_terminal");
        assert_eq!(normalized.command_history_limit, 12);
        assert_eq!(normalized.safe_launch_policy, "confirm_external_terminal");
        assert_eq!(normalized.provider_health_mode, "dry-run");
    }

    #[test]
    fn quick_terminal_actions_skip_input_media_actions() {
        let contract = DxCliHostContract {
            schema_version: "dx.host_contract.v1".into(),
            host_name: "DX CLI".into(),
            workspace: r"G:\Cli".into(),
            launcher: DxCliLauncher {
                cwd: r"G:\Cli".into(),
                powershell_script: r"G:\Cli\dx.ps1".into(),
                release_binary: r"G:\Cli\target\release\dx.exe".into(),
                release_binary_exists: true,
                preferred_invocation: vec![],
            },
            health: DxCliHealth {
                score: 100,
                summary: "ready".into(),
                prepared_engines_ready: 3,
                prepared_engines_total: 3,
                sample_artifacts_ready: 2,
                sample_artifacts_total: 2,
            },
            embedding: serde_json::json!({}),
            action_groups: sample_groups(),
            actions: vec![
                sample_action("dx.tui", "DX Mission Control", true, false),
                sample_action("dx.watch", "Watch In Terminal", true, true),
                sample_action("dx.bridge", "Bridge Snapshot", false, false),
            ],
            media_routes: serde_json::json!([]),
            media_viewer: None,
            telemetry: None,
            settings: None,
            terminal_readiness: serde_json::json!({}),
            notes: vec![],
        };

        let quick = quick_terminal_actions(&contract);

        assert_eq!(quick.len(), 1);
        assert_eq!(quick[0].action_id, "dx.tui");
        assert_eq!(quick[0].label, "DX Mission Control");
    }

    #[test]
    fn quick_captured_actions_only_include_no_input_capturable_actions() {
        let mut external = sample_action("dx.play", "Play", false, false);
        external.output = "external_media".into();
        let contract = DxCliHostContract {
            schema_version: "dx.host_contract.v1".into(),
            host_name: "DX CLI".into(),
            workspace: r"G:\Cli".into(),
            launcher: DxCliLauncher {
                cwd: r"G:\Cli".into(),
                powershell_script: r"G:\Cli\dx.ps1".into(),
                release_binary: r"G:\Cli\target\release\dx.exe".into(),
                release_binary_exists: true,
                preferred_invocation: vec![],
            },
            health: DxCliHealth {
                score: 100,
                summary: "ready".into(),
                prepared_engines_ready: 3,
                prepared_engines_total: 3,
                sample_artifacts_ready: 2,
                sample_artifacts_total: 2,
            },
            embedding: serde_json::json!({}),
            action_groups: sample_groups(),
            actions: vec![
                sample_action("dx.status", "Status", false, false),
                sample_action("dx.host_smoke", "Host Smoke", false, false),
                sample_action("dx.inspect", "Inspect Media", false, true),
                sample_action("dx.files", "Files", true, false),
                external,
            ],
            media_routes: serde_json::json!([]),
            media_viewer: None,
            telemetry: None,
            settings: None,
            terminal_readiness: serde_json::json!({}),
            notes: vec![],
        };

        let quick = quick_captured_actions(&contract);

        assert_eq!(quick.len(), 2);
        assert_eq!(quick[0].action_id, "dx.status");
        assert_eq!(quick[1].action_id, "dx.host_smoke");
    }

    #[test]
    fn host_menu_groups_actions_from_contract_metadata() {
        let contract = DxCliHostContract {
            schema_version: "dx.host_contract.v1".into(),
            host_name: "DX CLI".into(),
            workspace: r"G:\Cli".into(),
            launcher: DxCliLauncher {
                cwd: r"G:\Cli".into(),
                powershell_script: r"G:\Cli\dx.ps1".into(),
                release_binary: r"G:\Cli\target\release\dx.exe".into(),
                release_binary_exists: true,
                preferred_invocation: vec![],
            },
            health: DxCliHealth {
                score: 100,
                summary: "ready".into(),
                prepared_engines_ready: 3,
                prepared_engines_total: 3,
                sample_artifacts_ready: 2,
                sample_artifacts_total: 2,
            },
            embedding: serde_json::json!({}),
            action_groups: sample_groups(),
            actions: vec![
                sample_action_in_group("dx.tui", "DX Mission Control", "shell", true, false),
                sample_action_in_group("dx.host_smoke", "Host Smoke", "bridge", false, false),
                sample_action_in_group("dx.inspect", "Inspect Media", "media", false, true),
            ],
            media_routes: serde_json::json!([]),
            media_viewer: None,
            telemetry: None,
            settings: None,
            terminal_readiness: serde_json::json!({}),
            notes: vec![],
        };

        let menu = host_menu(&contract);
        let bridge = menu
            .groups
            .iter()
            .find(|group| group.id == "bridge")
            .expect("bridge group");

        assert_eq!(menu.group_count, 3);
        assert_eq!(menu.action_count, 3);
        assert_eq!(bridge.json_action_count, 1);
        assert_eq!(bridge.actions[0].action_id, "dx.host_smoke");
    }

    #[test]
    fn host_menu_adds_fallback_group_for_unlisted_actions() {
        let mut custom = sample_action("dx.custom", "Custom", false, false);
        custom.group = "custom".into();
        custom.enabled = false;

        let contract = DxCliHostContract {
            schema_version: "dx.host_contract.v1".into(),
            host_name: "DX CLI".into(),
            workspace: r"G:\Cli".into(),
            launcher: DxCliLauncher {
                cwd: r"G:\Cli".into(),
                powershell_script: r"G:\Cli\dx.ps1".into(),
                release_binary: r"G:\Cli\target\release\dx.exe".into(),
                release_binary_exists: true,
                preferred_invocation: vec![],
            },
            health: DxCliHealth {
                score: 100,
                summary: "ready".into(),
                prepared_engines_ready: 3,
                prepared_engines_total: 3,
                sample_artifacts_ready: 2,
                sample_artifacts_total: 2,
            },
            embedding: serde_json::json!({}),
            action_groups: sample_groups(),
            actions: vec![custom],
            media_routes: serde_json::json!([]),
            media_viewer: None,
            telemetry: None,
            settings: None,
            terminal_readiness: serde_json::json!({}),
            notes: vec![],
        };

        let menu = host_menu(&contract);
        let fallback = menu
            .groups
            .iter()
            .find(|group| group.id == "custom")
            .expect("fallback group");

        assert_eq!(fallback.label, "custom");
        assert_eq!(fallback.action_count, 1);
        assert_eq!(fallback.enabled_action_count, 0);
        assert_eq!(
            fallback.description,
            "Actions discovered without explicit group metadata."
        );
    }

    #[test]
    fn telemetry_history_reads_newest_first_and_omits_input_path() {
        let dir =
            std::env::temp_dir().join(format!("dx-bridge-telemetry-test-{}", std::process::id()));
        if dir.exists() {
            fs::remove_dir_all(&dir).unwrap();
        }
        fs::create_dir_all(&dir).unwrap();
        let history_path = dir.join("dx-command-runs.jsonl");
        let mut action = sample_action("dx.inspect", "Inspect Media", false, true);
        action.argv.push("{{input}}".into());
        let input_path = r"G:\Secret Project\private-video.mp4";
        let argv = argv_for_action(&action, Some(input_path)).unwrap();
        let result = DxCliRunResult {
            action_id: action.id.clone(),
            label: action.label.clone(),
            output: action.output.clone(),
            success: true,
            exit_code: Some(0),
            stdout: "{}".into(),
            stderr: String::new(),
            argv,
            cwd: action.cwd.clone(),
        };
        let first = telemetry_record_for_run(&action, &result, 12, true);
        let mut second = first.clone();
        second.action_id = "dx.status".into();
        second.label = "Status".into();
        second.input_supplied = false;

        let serialized = serde_json::to_string(&first).unwrap();
        assert!(first.input_supplied);
        assert_eq!(first.argv_program.as_deref(), Some("powershell"));
        assert!(!serialized.contains("private-video.mp4"));
        assert!(!serialized.contains("Secret Project"));

        let contract = sample_contract_with_telemetry(vec![action], &history_path);
        append_telemetry_record(&contract, &first).unwrap();
        append_telemetry_record(&contract, &second).unwrap();

        let history = read_command_history(&contract, 2).unwrap();

        assert_eq!(history.count, 2);
        assert_eq!(history.entries[0].action_id, "dx.status");
        assert_eq!(history.entries[1].action_id, "dx.inspect");

        fs::remove_dir_all(dir).unwrap();
    }

    fn sample_action(
        id: &str,
        label: &str,
        requires_terminal: bool,
        accepts_input: bool,
    ) -> DxCliAction {
        DxCliAction {
            id: id.into(),
            group: "shell".into(),
            label: label.into(),
            description: String::new(),
            argv: vec!["powershell".into(), "-File".into(), r"G:\Cli\dx.ps1".into()],
            cwd: r"G:\Cli".into(),
            output: if requires_terminal { "tui" } else { "json" }.into(),
            requires_terminal,
            accepts_input,
            input_placeholder: accepts_input.then(|| "{{input}}".into()),
            enabled: true,
            required_tools: vec![],
            required_capabilities: vec![],
        }
    }

    fn sample_action_in_group(
        id: &str,
        label: &str,
        group: &str,
        requires_terminal: bool,
        accepts_input: bool,
    ) -> DxCliAction {
        let mut action = sample_action(id, label, requires_terminal, accepts_input);
        action.group = group.into();
        action
    }

    fn sample_groups() -> Vec<DxCliActionGroup> {
        vec![
            DxCliActionGroup {
                id: "shell".into(),
                label: "Shell".into(),
                description: "Interactive entry points.".into(),
            },
            DxCliActionGroup {
                id: "bridge".into(),
                label: "Bridge".into(),
                description: "Host integration commands.".into(),
            },
            DxCliActionGroup {
                id: "media".into(),
                label: "Media".into(),
                description: "Media routes.".into(),
            },
        ]
    }

    fn sample_settings_contract() -> DxCliSettingsContract {
        DxCliSettingsContract {
            supported: true,
            schema_version: "dx.host_settings.v1".into(),
            persistence: "host_app_store".into(),
            namespace: "dx_cli_bridge".into(),
            defaults: DxCliBridgeSettings {
                media_input: r"G:\Cli\samples\dx-smoke.mp4".into(),
                preferred_terminal_surface: "external_terminal".into(),
                command_history_limit: 6,
                safe_launch_policy: "confirm_external_terminal".into(),
                provider_health_mode: "dry-run".into(),
            },
            constraints: DxCliSettingsConstraints {
                terminal_surfaces: vec!["external_terminal".into(), "captured_output".into()],
                command_history_limit_min: 2,
                command_history_limit_max: 12,
                provider_health_modes: vec!["mock".into(), "dry-run".into()],
                safe_launch_policies: vec![
                    "confirm_external_terminal".into(),
                    "direct_external_terminal".into(),
                ],
            },
        }
    }

    fn sample_contract_with_telemetry(
        actions: Vec<DxCliAction>,
        history_path: &std::path::Path,
    ) -> DxCliHostContract {
        DxCliHostContract {
            schema_version: "dx.host_contract.v1".into(),
            host_name: "DX CLI".into(),
            workspace: r"G:\Cli".into(),
            launcher: DxCliLauncher {
                cwd: r"G:\Cli".into(),
                powershell_script: r"G:\Cli\dx.ps1".into(),
                release_binary: r"G:\Cli\target\release\dx.exe".into(),
                release_binary_exists: true,
                preferred_invocation: vec![],
            },
            health: DxCliHealth {
                score: 100,
                summary: "ready".into(),
                prepared_engines_ready: 3,
                prepared_engines_total: 3,
                sample_artifacts_ready: 2,
                sample_artifacts_total: 2,
            },
            embedding: serde_json::json!({}),
            action_groups: sample_groups(),
            actions,
            media_routes: serde_json::json!([]),
            media_viewer: None,
            telemetry: Some(DxCliTelemetry {
                supported: true,
                schema_version: "dx.host_telemetry.v1".into(),
                storage: "jsonl".into(),
                history_path: history_path.display().to_string(),
                max_entries: 500,
                redaction: serde_json::json!({
                    "input_values": "omitted",
                    "argv_values": "program_and_arg_count_only",
                    "stdout": "omitted",
                    "stderr": "summary_only"
                }),
                event_fields: vec!["input_supplied".into()],
            }),
            settings: None,
            terminal_readiness: serde_json::json!({}),
            notes: vec![],
        }
    }
}
