//! Mobile entry point for DX Agents Desktop (iOS/Android).

#[tauri::mobile_entry_point]
fn main() {
    dx_agents_desktop::run();
}
