use crate::traits::{ChatResponse, ModelProvider, ProviderCapabilities};
use async_trait::async_trait;
use dx_agent_api::attribution::{Attributable, ModelProviderKind, ProviderKind, Role};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Default GGUF model path for MiniCPM5 1B
const DEFAULT_MODEL_PATH: &str = "C:\\Dx\\models\\llm\\minicpm5-1b.gguf";
/// Fallback model key for detection from flow's catalog
const FALLBACK_MODEL_KEY: &str = "minicpm5-1b";

pub struct FlowModelProvider {
    alias: String,
    model_path: String,
    n_gpu_layers: u32,
    num_ctx: Option<u32>,
    llm: Arc<Mutex<Option<flow::models::LocalLlm>>>,
}

impl FlowModelProvider {
    pub fn new(
        alias: &str,
        model_path: Option<String>,
        n_gpu_layers: Option<u32>,
        num_ctx: Option<u32>,
    ) -> Self {
        let resolved_path = model_path.unwrap_or_else(|| {
            flow::models::LocalLlm::model_path_for_key(FALLBACK_MODEL_KEY)
                .unwrap_or_else(|| DEFAULT_MODEL_PATH.to_string())
        });
        Self {
            alias: alias.to_string(),
            model_path: resolved_path,
            n_gpu_layers: n_gpu_layers.unwrap_or(0),
            num_ctx,
            llm: Arc::new(Mutex::new(None)),
        }
    }

    async fn ensure_initialized(&self) -> anyhow::Result<()> {
        let mut guard = self.llm.lock().await;
        if guard.is_some() {
            return Ok(());
        }
        let llm = flow::models::LocalLlm::with_model_path(self.model_path.clone());
        llm.initialize().await?;
        *guard = Some(llm);
        Ok(())
    }
}

#[async_trait]
impl ModelProvider for FlowModelProvider {
    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            native_tool_calling: false,
            vision: false,
            prompt_caching: false,
            extended_thinking: false,
        }
    }

    async fn chat_with_system(
        &self,
        _system_prompt: Option<&str>,
        message: &str,
        _model: &str,
        _temperature: Option<f64>,
    ) -> anyhow::Result<String> {
        self.ensure_initialized().await?;
        let guard = self.llm.lock().await;
        let llm = guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Flow model not initialized"))?;
        let (response, _metrics) = llm.generate_with_metrics(message).await?;
        Ok(response)
    }

    async fn chat(&self, request: crate::traits::ChatRequest<'_>, model: &str, temperature: Option<f64>) -> anyhow::Result<ChatResponse> {
        let system = request
            .messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.as_str());
        let user_msg = request
            .messages
            .iter()
            .rfind(|m| m.role == "user")
            .map(|m| m.content.as_str())
            .unwrap_or("");

        let prompt = if let Some(sys) = system {
            if !sys.is_empty() {
                format!("{}\n\n{}", sys, user_msg)
            } else {
                user_msg.to_string()
            }
        } else {
            user_msg.to_string()
        };

        let text = self.chat_with_system(system, &prompt, model, temperature).await?;
        Ok(ChatResponse {
            text: Some(text),
            tool_calls: vec![],
            usage: None,
            reasoning_content: None,
        })
    }
}

impl Attributable for FlowModelProvider {
    fn role(&self) -> Role {
        Role::Provider(ProviderKind::Model(ModelProviderKind::Flow))
    }
    fn alias(&self) -> &str {
        &self.alias
    }
}
