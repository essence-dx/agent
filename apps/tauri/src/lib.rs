//! DX Agents Desktop Tauri application library.

pub mod capabilities;
pub mod commands;
pub mod gateway_client;
pub mod health;
pub mod linux;
pub mod macos;
pub mod state;
pub mod tray;
pub mod windows;

use commands::onboarding::read_onboarding_complete;
use gateway_client::GatewayClient;
use state::shared_state;
use tauri::{Manager, RunEvent};

/// Attempt to auto-pair with the gateway so the WebView has a valid token
/// before the React frontend mounts. Runs on localhost so the admin endpoints
/// are accessible without auth.
async fn auto_pair(state: &state::SharedState) -> Option<String> {
    let url = {
        let s = state.read().await;
        s.gateway_url.clone()
    };

    let client = GatewayClient::new(&url, None);

    // Check if gateway is reachable and requires pairing.
    if !client.requires_pairing().await.unwrap_or(false) {
        return None; // Pairing disabled — no token needed.
    }

    // Check if we already have a valid token in state.
    {
        let s = state.read().await;
        if let Some(ref token) = s.token {
            let authed = GatewayClient::new(&url, Some(token));
            if authed.validate_token().await.unwrap_or(false) {
                return Some(token.clone()); // Existing token is valid.
            }
        }
    }

    // No valid token — auto-pair by requesting a new code and exchanging it.
    let client = GatewayClient::new(&url, None);
    match client.auto_pair().await {
        Ok(token) => {
            let mut s = state.write().await;
            s.token = Some(token.clone());
            Some(token)
        }
        Err(_) => None, // Gateway may not be ready yet; health poller will retry.
    }
}

/// Inject a bearer token into the WebView's localStorage so the React app
/// skips the pairing dialog. Uses Tauri's WebviewWindow scripting API.
fn inject_token_into_webview<R: tauri::Runtime>(window: &tauri::WebviewWindow<R>, token: &str) {
    let escaped = token.replace('\\', "\\\\").replace('\'', "\\'");
    let script = format!(
        "localStorage.setItem('dx_agents_token', '{escaped}');localStorage.setItem('zeroclaw_token', '{escaped}')"
    );
    // WebviewWindow scripting is the standard Tauri API for running JS in the WebView.
    let _ = window.eval(&script);
}

/// Set the macOS dock icon programmatically so it shows even in dev builds
/// (which don't have a proper .app bundle).
#[cfg(target_os = "macos")]
fn set_dock_icon() {
    use objc2::{AnyThread, MainThreadMarker};
    use objc2_app_kit::NSApplication;
    use objc2_app_kit::NSImage;
    use objc2_foundation::NSData;

    let icon_bytes = include_bytes!("../icons/128x128.png");
    // Safety: setup() runs on the main thread in Tauri.
    let mtm = unsafe { MainThreadMarker::new_unchecked() };
    let data = NSData::with_bytes(icon_bytes);
    if let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) {
        let app = NSApplication::sharedApplication(mtm);
        unsafe { app.setApplicationIconImage(Some(&image)) };
    }
}

/// Configure and run the Tauri application.
pub fn run() {
    let shared = shared_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // When a second instance launches, focus whichever surface is current.
            let target = app
                .get_webview_window("onboarding")
                .or_else(|| app.get_webview_window("main"));
            if let Some(window) = target {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .manage(shared.clone())
        .invoke_handler(tauri::generate_handler![
            commands::gateway::get_status,
            commands::gateway::get_health,
            commands::channels::list_channels,
            commands::pairing::initiate_pairing,
            commands::pairing::get_devices,
            commands::agent::send_message,
            commands::dx_cli::get_dx_cli_host_contract,
            commands::dx_cli::get_dx_cli_bridge_settings,
            commands::dx_cli::get_dx_cli_command_history,
            commands::dx_cli::get_dx_cli_host_menu,
            commands::dx_cli::get_dx_cli_host_status,
            commands::dx_cli::get_dx_agents_compact_status,
            commands::dx_cli::get_dx_agents_continuation_status,
            commands::dx_cli::get_dx_agents_cron_delivery_drill,
            commands::dx_cli::get_dx_agents_cron_history,
            commands::dx_cli::get_dx_agents_cron_preview,
            commands::dx_cli::get_dx_agents_tool_safety_drill,
            commands::dx_cli::get_dx_agents_tool_safety_drill_history,
            commands::dx_cli::get_dx_agents_tool_safety_audit,
            commands::dx_cli::get_dx_agents_tool_safety_audit_history,
            commands::dx_cli::export_dx_agents_tool_safety_audit_history,
            commands::dx_cli::open_dx_agents_tool_safety_audit_history,
            commands::dx_cli::export_dx_agents_tool_safety_drill_history,
            commands::dx_cli::open_dx_agents_tool_safety_drill_history,
            commands::dx_cli::get_dx_agents_embedded_terminal_fixtures,
            commands::dx_cli::get_dx_agents_embedded_terminal_readiness,
            commands::dx_cli::get_dx_agents_embedded_terminal_session_timeline,
            commands::dx_cli::get_dx_agents_embedded_terminal_media_canary_evidence,
            commands::dx_cli::get_dx_agents_embedded_terminal_tui_canary_gate,
            commands::dx_cli::get_dx_agents_embedded_terminal_tui_canary_lifecycle,
            commands::dx_cli::get_dx_agents_embedded_terminal_tui_canary_renderer_evidence,
            commands::dx_cli::run_dx_agents_embedded_terminal_echo_pilot,
            commands::dx_cli::run_dx_agents_embedded_terminal_tui_canary_runner,
            commands::dx_cli::get_dx_agents_bridge_status_exports,
            commands::dx_cli::get_dx_cli_native_promotion_archives,
            commands::dx_cli::get_dx_cli_native_promotion_archive_diff,
            commands::dx_cli::get_dx_agents_gateway_paircode,
            commands::dx_cli::get_dx_agents_gateway_pairing_drill,
            commands::dx_cli::get_dx_agents_provider_health,
            commands::dx_cli::get_dx_agents_provider_smoke_history,
            commands::dx_cli::get_dx_agents_release_readiness,
            commands::dx_cli::get_dx_agents_dashboard_compatibility_status,
            commands::dx_cli::open_dx_agents_continuation_target,
            commands::dx_cli::open_dx_agents_bridge_status_export,
            commands::dx_cli::open_dx_cli_native_promotion_archive,
            commands::dx_cli::plan_dx_cli_action,
            commands::dx_cli::export_dx_agents_bridge_status,
            commands::dx_cli::run_dx_agents_provider_smoke,
            commands::dx_cli::run_dx_cli_bridge_self_test,
            commands::dx_cli::run_dx_cli_json_action,
            commands::dx_cli::run_dx_cli_captured_action,
            commands::dx_cli::save_dx_cli_bridge_settings,
            commands::dx_cli::launch_dx_cli_terminal_action,
            commands::permissions::get_permissions_status,
            commands::permissions::get_runtime_platform,
            commands::permissions::request_permission,
            commands::permissions::open_privacy_settings,
            commands::onboarding::get_onboarding_state,
            commands::onboarding::complete_onboarding,
            commands::onboarding::reset_onboarding,
            capabilities::screenshot::take_screenshot,
            capabilities::applescript::run_applescript,
        ])
        .setup(move |app| {
            // Set macOS dock icon (needed for dev builds without .app bundle).
            #[cfg(target_os = "macos")]
            set_dock_icon();

            // Set up the system tray.
            let _ = tray::setup_tray(app);

            // First-run: show onboarding window if the user hasn't completed it.
            // Otherwise stay tray-only — main window opens on demand from the tray.
            if !read_onboarding_complete(app.handle())
                && let Some(window) = app.get_webview_window("onboarding")
            {
                let _ = window.show();
                let _ = window.set_focus();
            }

            // Auto-pair with gateway and inject token into the WebView.
            let app_handle = app.handle().clone();
            let pair_state = shared.clone();
            tauri::async_runtime::spawn(async move {
                if let Some(token) = auto_pair(&pair_state).await
                    && let Some(window) = app_handle.get_webview_window("main")
                {
                    inject_token_into_webview(&window, &token);
                }
            });

            // Start background health polling.
            health::spawn_health_poller(app.handle().clone(), shared.clone());

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            // Keep the app running in the background when all windows are closed.
            // This is the standard pattern for menu bar / tray apps.
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
