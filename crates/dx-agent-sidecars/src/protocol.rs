use std::process::Stdio;
use anyhow::Result;
use serde_json::Value as JsonValue;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;
use tracing::{debug, info};

/// A JSON-RPC 2.0 sidecar process communicating over stdin/stdout.
pub struct SidecarProcess {
    name: &'static str,
    stdin: Option<Mutex<ChildStdin>>,
    stdout: Option<Mutex<BufReader<ChildStdout>>>,
    child: Option<Child>,
    next_id: Mutex<u64>,
}

impl SidecarProcess {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            stdin: None,
            stdout: None,
            child: None,
            next_id: Mutex::new(1),
        }
    }

    /// Spawn the sidecar process with given command and args.
    pub fn spawn(&mut self, program: &str, args: &[&str]) -> Result<()> {
        info!("Spawning sidecar {}: {} {}", self.name, program, args.join(" "));
        let mut cmd = Command::new(program);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn()?;
        let stdin = child.stdin.take().expect("failed to capture stdin");
        let stdout = child.stdout.take().expect("failed to capture stdout");
        self.stdin = Some(Mutex::new(stdin));
        self.stdout = Some(Mutex::new(BufReader::new(stdout)));
        self.child = Some(child);
        Ok(())
    }

    /// Send a JSON-RPC 2.0 request and wait for response.
    pub async fn request(&self, method: &str, params: JsonValue) -> Result<JsonValue> {
        let id = {
            let mut next = self.next_id.lock().await;
            let id = *next;
            *next += 1;
            id
        };

        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        let payload_str = serde_json::to_string(&payload)?;
        debug!("sidecar {} → {}", self.name, payload_str);

        // Write request
        {
            let stdin_lock = self.stdin.as_ref().expect("sidecar not spawned");
            let mut stdin = stdin_lock.lock().await;
            stdin.write_all(payload_str.as_bytes()).await?;
            stdin.write_all(b"\n").await?;
            stdin.flush().await?;
        }

        // Read response line
        let stdout_lock = self.stdout.as_ref().expect("sidecar not spawned");
        let mut stdout = stdout_lock.lock().await;
        let mut line = String::new();
        stdout.read_line(&mut line).await?;
        debug!("sidecar {} ← {}", self.name, line.trim());
        let resp: JsonValue = serde_json::from_str(&line.trim())?;
        Ok(resp)
    }

    /// Convenience: send a notification (no id / no response).
    pub async fn notify(&self, method: &str, params: JsonValue) -> Result<()> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        let payload_str = serde_json::to_string(&payload)?;
        let stdin_lock = self.stdin.as_ref().expect("sidecar not spawned");
        let mut stdin = stdin_lock.lock().await;
        stdin.write_all(payload_str.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;
        Ok(())
    }

    /// Shutdown the child process.
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("Shutting down sidecar {}", self.name);
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
            child.wait().await?;
        }
        Ok(())
    }
}
