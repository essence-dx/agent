use crate::protocol::SidecarProcess;
use anyhow::Result;
use serde_json::Value as JsonValue;

/// OpenClaw sidecar: channel adapters, pairing, security, secrets, protocol schemas.
pub struct OpenClawSidecar {
    inner: SidecarProcess,
}

impl OpenClawSidecar {
    pub fn new() -> Self {
        Self {
            inner: SidecarProcess::new("openclaw"),
        }
    }

    /// Spawn `node index.mjs --stdio` in the openclaw-fork directory.
    pub fn spawn(&mut self, path: &str) -> Result<()> {
        let program = "node";
        let args = [&format!("{}/index.mjs", path), "--stdio"];
        self.inner.spawn(program, &args)
    }

    /// List available channel adapter types.
    pub async fn list_types(&self) -> Result<JsonValue> {
        self.inner.request("list_types", serde_json::json!([])).await
    }

    /// List channel names (telegram, discord, slack, whatsapp).
    pub async fn list_channels(&self) -> Result<JsonValue> {
        self.inner.request("list_channels", serde_json::json!([])).await
    }

    /// List available protocol schema names.
    pub async fn list_protocol_schemas(&self) -> Result<JsonValue> {
        self.inner.request("list_protocol_schemas", serde_json::json!([])).await
    }

    /// List available security check names.
    pub async fn list_security_checks(&self) -> Result<JsonValue> {
        self.inner.request("list_security_checks", serde_json::json!([])).await
    }

    /// Shutdown the sidecar process.
    pub async fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown().await
    }
}
