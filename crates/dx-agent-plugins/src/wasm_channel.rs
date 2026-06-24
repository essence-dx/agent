//! Bridge between WASM plugins and the Channel trait.
//!
//! **Status:** Placeholder — `send` and `listen` are not yet wired to the
//! Extism runtime.  Channel plugin support is a Phase 3 deliverable
//! per the [Intentional Architecture RFC](https://github.com/zeroclaw-labs/zeroclaw/wiki/14.1-Intentional-Architecture).
//! See `wasm_tool.rs` and `runtime.rs` for the working tool plugin bridge.

use async_trait::async_trait;
use dx_agent_api::channel::{Channel, ChannelMessage, SendMessage};

/// A channel backed by a WASM plugin.
pub struct WasmChannel {
    name: String,
    plugin_name: String,
}

impl WasmChannel {
    pub fn new(name: String, plugin_name: String) -> Self {
        Self { name, plugin_name }
    }
}

impl ::dx_agent_api::attribution::Attributable for WasmChannel {
    fn role(&self) -> ::dx_agent_api::attribution::Role {
        ::dx_agent_api::attribution::Role::Channel(
            ::dx_agent_api::attribution::ChannelKind::Webhook,
        )
    }
    fn alias(&self) -> &str {
        &self.name
    }
}

#[async_trait]
impl Channel for WasmChannel {
    fn name(&self) -> &str {
        &self.name
    }

    async fn send(&self, message: &SendMessage) -> anyhow::Result<()> {
        // TODO: Wire to WASM plugin send function
        ::dx_agent_log::record!(
            WARN,
            ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                .with_outcome(::dx_agent_log::EventOutcome::Unknown),
            &format!(
                "WasmChannel '{}' (plugin: {}) send not yet connected: {}",
                self.name, self.plugin_name, message.content
            )
        );
        Ok(())
    }

    async fn listen(&self, _tx: tokio::sync::mpsc::Sender<ChannelMessage>) -> anyhow::Result<()> {
        // TODO: Wire to WASM plugin receive/listen function
        ::dx_agent_log::record!(
            WARN,
            ::dx_agent_log::Event::new(module_path!(), ::dx_agent_log::Action::Note)
                .with_outcome(::dx_agent_log::EventOutcome::Unknown),
            &format!(
                "WasmChannel '{}' (plugin: {}) listen not yet connected",
                self.name, self.plugin_name
            )
        );
        Ok(())
    }
}
