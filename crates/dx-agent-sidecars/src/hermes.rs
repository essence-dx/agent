use crate::protocol::SidecarProcess;
use anyhow::Result;
use serde_json::Value as JsonValue;

/// Hermes sidecar: 11-phase auto-update, ACP adapter.
pub struct HermesSidecar {
    inner: SidecarProcess,
}

impl HermesSidecar {
    pub fn new() -> Self {
        Self {
            inner: SidecarProcess::new("hermes"),
        }
    }

    /// Spawn `python -m hermes_update.server` in the hermes-fork directory.
    pub fn spawn(&mut self, path: &str) -> Result<()> {
        let program = "python";
        let args = ["-m", "hermes_update.server"];
        let _full_path = format!("{}/hermes_update", path);
        self.inner.spawn(program, &args)
    }

    /// Check if an update is available.
    pub async fn check_update(&self) -> Result<JsonValue> {
        self.inner.request("check_update", serde_json::json!([])).await
    }

    /// Perform a full update.
    pub async fn perform_update(&self) -> Result<JsonValue> {
        self.inner.request("perform_update", serde_json::json!([])).await
    }

    /// List available update phases.
    pub async fn list_update_phases(&self) -> Result<JsonValue> {
        self.inner.request("list_update_phases", serde_json::json!([])).await
    }

    /// List available ACP methods.
    pub async fn list_acp_methods(&self) -> Result<JsonValue> {
        self.inner.request("list_acp_methods", serde_json::json!([])).await
    }

    /// Shutdown the sidecar process.
    pub async fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown().await
    }
}
