//! DX Agents Desktop main entry point.
//!
//! Prevents an additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    dx_agents_desktop::run();
}
