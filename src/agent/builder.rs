use anyhow::{Context, Result};
use std::sync::Arc;

use crate::{
    agent::{completion::Agent, service::AgentService},
    capabilities::{client::{completion::LlmClient, mcp::MCPServerAdapter, tool::Tool}, tools::mcp::MCPServerConfig},
    providers::{
        anthropic::{self, completion::AnthropicClient},
        gemini::{self, completion::GeminiClient},
        openai::{self, completion::OpenAIClient},
    },
};

const MODEL_TEMPERATURE: f32 = 0.5;
const MODEL_MAX_TOKENS: i32 = 5000;

pub struct AgentBuilder<'a> {
    service: &'a AgentService,
    llm: Option<String>,
    model: Option<String>,
    client: Option<Arc<dyn LlmClient>>,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
}


// AgentBuilder borrows immutably from AgentService using lifetime parameters, 
// enabling multiple concurrent builders while mutating shared state through interior mutability (RwLock).
impl<'a> AgentBuilder<'a> {
    pub fn new(service: &'a AgentService) -> Self {
        Self {
            llm: None,
            model: None,
            service: service,
            client: None,
            temperature: None,
            max_tokens: None,
        }
    }

    pub fn with_anthropic(mut self, api_key: &str) -> Result<Self> {
        let mut clients = self.service.clients.write().unwrap();
        self.llm = Some(anthropic::LLM.to_string());
        self.model = Some(anthropic::MODEL_CLAUDE_SONNET_4_5.to_string());
        let client_key = format! {"{}:{}", anthropic::LLM, anthropic::MODEL_CLAUDE_SONNET_4_5};
        let client = clients
            .entry(client_key)
            .or_insert(self.anthropic_client(api_key)?);
        self.client = Some(client.clone());
        Ok(self)
    }

    fn anthropic_client(&self, api_key: &str) -> Result<Arc<dyn LlmClient>> {
        let client = AnthropicClient::new(api_key.to_string())
            .with_context(|| anyhow::anyhow!("Error creating Anthropic client"))?;
        Ok(Arc::new(client))
    }

    pub fn with_openai(mut self, api_key: &str) -> Result<Self> {
        let mut clients = self.service.clients.write().unwrap();
        self.llm = Some(openai::LLM.to_string());
        self.model = Some(openai::MODEL_GPT_5_NANO.to_string());
        let client_key = format! {"{}:{}", openai::LLM, openai::MODEL_GPT_5_NANO};
        let client = clients
            .entry(client_key)
            .or_insert(self.openai_client(api_key)?);
        self.client = Some(client.clone());
        Ok(self)
    }

    fn openai_client(&self, api_key: &str) -> Result<Arc<dyn LlmClient>> {
        let client = OpenAIClient::new(api_key.to_string())
            .with_context(|| anyhow::anyhow!("Error creating Anthropic client"))?;
        Ok(Arc::new(client))
    }

    pub fn with_gemini(mut self, api_key: &str) -> Result<Self> {
        let mut clients = self.service.clients.write().unwrap();
        self.llm = Some(gemini::LLM.to_string());
        self.model = Some(gemini::MODEL_GEMINI_3_FLASH_PREVIEW.to_string());
        let client_key = format! {"{}:{}", gemini::LLM, gemini::MODEL_GEMINI_3_FLASH_PREVIEW};
        let client = clients
            .entry(client_key)
            .or_insert(self.gemini_client(api_key)?);
        self.client = Some(client.clone());
        Ok(self)
    }

    fn gemini_client(&self, api_key: &str) -> Result<Arc<dyn LlmClient>> {
        let client = GeminiClient::new(api_key.to_string())
            .with_context(|| anyhow::anyhow!("Error creating Anthropic client"))?;
        Ok(Arc::new(client))
    }

    //set the temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    //set the max number of tokens
    pub fn with_max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_tool<T: Tool + 'static>(self, tool: T) -> Self {
        let mut registry = self.service.tool_registry.write().unwrap();
        registry.register_tool(tool);
        self
    }

    pub async fn with_mcp_registry<T: MCPServerAdapter + 'static>(self, config: MCPServerConfig, adapter: T) -> Result<Self> {
        let mut registry = self.service.mcp_registry.write().unwrap();
        let _ = registry.register_server_with_adapter(config, Box::new(adapter)).await?;
        Ok(self)
    }

    pub async fn with_mcp_tool(self, server_name: &str, tool_name: &str,) -> Result<Self>{

        let mut registry = self.service.mcp_registry.write().unwrap();
        let _ = registry.register_tool(server_name, tool_name).await?;
        Ok(self)
    }

    //build the agent and take ownership
    pub fn build(self) -> Result<Agent> {
        let client = self
            .client
            .ok_or_else(|| anyhow::anyhow!("Client is required"))?;
        let llm = self.llm.ok_or_else(||anyhow::anyhow!("LLM is required"))?;
        let model = self
            .model
            .ok_or_else(|| anyhow::anyhow!("Model is required"))?;
        let temperature: f32 = self.temperature.unwrap_or(MODEL_TEMPERATURE);
        let max_tokens = self.max_tokens.unwrap_or(MODEL_MAX_TOKENS);
        let tool_guard = self.service.tool_registry.read().unwrap();
        let tool_registry = tool_guard.clone().into();

        let mcp_tool_guard = self.service.mcp_registry.read().unwrap();
        let mcp_registry = Arc::new(mcp_tool_guard.clone());

        Ok(Agent {
            llm,
            model,
            client,
            temperature,
            max_tokens,
            tool_registry,
            mcp_registry,
        })

    }
}
