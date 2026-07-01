use crate::protocol::SidecarProcess;
use anyhow::Result;
use serde_json::Value as JsonValue;

/// Deer-Flow sidecar: lead agent factory, middleware chain, subagent executor, sandbox, run manager.
pub struct DeerFlowSidecar {
    inner: SidecarProcess,
}

impl DeerFlowSidecar {
    pub fn new() -> Self {
        Self {
            inner: SidecarProcess::new("deerflow"),
        }
    }

    /// Spawn `python -m deerflow_server.server` in the deerflow-fork directory.
    pub fn spawn(&mut self, _path: &str) -> Result<()> {
        let program = "python";
        let args = ["-m", "deerflow_server.server"];
        self.inner.spawn(program, &args)
    }

    /// List all middleware names in execution order.
    pub async fn list_middlewares(&self) -> Result<JsonValue> {
        self.inner.request("list_middlewares", serde_json::json!([])).await
    }

    /// Get middleware info by name.
    pub async fn get_middleware_code(&self, name: &str) -> Result<JsonValue> {
        self.inner.request("get_middleware_code", serde_json::json!({"name": name})).await
    }

    /// List available subagent built-ins.
    pub async fn list_subagent_builtins(&self) -> Result<JsonValue> {
        self.inner
            .request("list_subagent_builtins", serde_json::json!([]))
            .await
    }

    /// List sandbox types.
    pub async fn list_sandbox_types(&self) -> Result<JsonValue> {
        self.inner
            .request("list_sandbox_types", serde_json::json!([]))
            .await
    }

    /// Create a new run.
    pub async fn create_run(&self, config: JsonValue) -> Result<JsonValue> {
        self.inner.request("create_run", config).await
    }

    /// Get run status.
    pub async fn get_run_status(&self, run_id: &str) -> Result<JsonValue> {
        self.inner
            .request("get_run_status", serde_json::json!({"run_id": run_id}))
            .await
    }

    /// Shutdown the sidecar process.
    pub async fn shutdown(&mut self) -> Result<()> {
        self.inner.shutdown().await
    }
}
