use std::sync::Arc;

use anyhow::Result;

use crate::{
    agent::completion::Agent,
    capabilities::{client::completion::LlmClient, tools::{mcp::MCPRegistry, tool::ToolRegistry}},
};

const MODEL_TEMPERATURE: f32 = 0.5;
const MODEL_MAX_TOKENS: i32 = 5000;

pub struct AgentBuilder {
    client: Option<Arc<dyn LlmClient>>,
    api_key: String,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
    tool_registry: Option<Arc<ToolRegistry>>,
    mcp_registry: Option<Arc<MCPRegistry>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            api_key: String::new(),
            temperature: None,
            max_tokens: None,
            tool_registry: None,
            mcp_registry: None
        }
    }

    pub fn with_client(mut self, client: Option<Arc<dyn LlmClient>>) -> Self {
        self.client = client;
        self
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
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

    pub fn with_tool_registry(mut self, tool_registry: Option<Arc<ToolRegistry>>) -> Self {
        self.tool_registry = tool_registry;
        self
    }

    pub fn with_mcp_registry(mut self, mcp_registry: Option<Arc<MCPRegistry>>) -> Self {
        self.mcp_registry = mcp_registry;
        self
    }

    //build the agent and take ownership
    pub fn build(self) -> Result<Agent> {
        // find the client
        let client = self
            .client
            .ok_or_else(|| anyhow::anyhow!("Client is required"))?;

        let temperature: f32 = self.temperature.unwrap_or(MODEL_TEMPERATURE);
        let max_tokens = self.max_tokens.unwrap_or(MODEL_MAX_TOKENS);
        let tool_registry = self.tool_registry.unwrap_or(Arc::new(ToolRegistry::new()));
        let mcp_registry = self.mcp_registry.unwrap_or(Arc::new(MCPRegistry::new()));

        Ok(Agent {
            client,
            temperature,
            max_tokens,
            tool_registry,
            mcp_registry
        })
    }
}
