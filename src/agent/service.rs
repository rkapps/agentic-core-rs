use anyhow::{Context, Result};
use serde::Serialize;
use std::sync::Arc;

use crate::{
    agent::{builder::AgentBuilder, completion::Agent},
    capabilities::{client::completion::LlmClient, tools::{mcp::MCPRegistry, tool::ToolRegistry}},
    providers::{
        anthropic::{self, completion::AnthropicClient},
        gemini::{self, completion::GeminiClient},
        openai::{self, completion::OpenAIClient},
    },
};

#[derive(Serialize)]
pub struct LlmProvider {
    pub id: String,
    pub llm: String,
    pub models: Vec<String>,
}

pub struct AgentService {
    openai_client: Option<Arc<dyn LlmClient>>,
    gemini_client: Option<Arc<dyn LlmClient>>,
    anthropic_client: Option<Arc<dyn LlmClient>>,

    //Arc<RwLock allows interior mutability
    tool_registry: Option<Arc<ToolRegistry>>,
    mcp_registry: Option<Arc<MCPRegistry>>
}

impl AgentService {
    pub fn new() -> AgentService {
        Self {
            openai_client: None,
            gemini_client: None,
            anthropic_client: None,
            tool_registry: None,
            mcp_registry: None
        }
    }

    pub fn register_client(&mut self, llm: &str, api_key: &str) -> Result<()> {
        match llm {
            "Anthropic" => {
                let client = AnthropicClient::new(api_key.to_string())
                    .with_context(|| anyhow::anyhow!("Error creating Anthropic client"))?;
                self.anthropic_client = Some(Arc::new(client));
            }
            "Gemini" => {
                let client = GeminiClient::new(api_key.to_string())
                    .with_context(|| anyhow::anyhow!("Error creating Gemini client"))?;
                self.gemini_client = Some(Arc::new(client));
            }
            "OpenAI" => {
                let client = OpenAIClient::new(api_key.to_string())
                    .with_context(|| anyhow::anyhow!("Error creating OpenAI client"))?;
                self.openai_client = Some(Arc::new(client));
            }

            _ => return Err(anyhow::anyhow!("Llm client for '{}' is not supported", llm)),
        }

        Ok(())
    }

    pub fn register_tool(&mut self, tool_registry: ToolRegistry) {
        self.tool_registry = Some(Arc::new(tool_registry));
    }

    pub fn register_mcp(&mut self, mcp_registry: MCPRegistry) {
        self.mcp_registry = Some(Arc::new(mcp_registry));
    }

    pub fn get_completion_agent(&self, llm: &str) -> Result<Arc<Agent>> {
        //get client
        let client = match llm {
            openai::completion::LLM => self.openai_client.clone(),
            gemini::completion::LLM => self.gemini_client.clone(),
            anthropic::completion::LLM => self.anthropic_client.clone(),
            _ => None,
        };

        // Build the Agent
        let agent = AgentBuilder::new()
            .with_client(client)
            .with_temperature(0.5)
            .with_max_tokens(10000)
            .with_tool_registry(self.tool_registry.clone())
            .with_mcp_registry(self.mcp_registry.clone())
            .build()?;

        Ok(Arc::new(agent))
    }

    pub fn get_llm_providers(&self) -> Vec<LlmProvider> {
        let mut providers: Vec<LlmProvider> = Vec::new();

        let gemini = LlmProvider {
            id: String::from(gemini::completion::LLM.to_lowercase()),
            llm: gemini::completion::LLM.to_string(),
            models: vec![gemini::completion::MODEL_GEMINI_3_FLASH_PREVIEW.to_string()],
        };
        let openai = LlmProvider {
            id: String::from(openai::completion::LLM.to_lowercase()),
            llm: openai::completion::LLM.to_string(),
            models: vec![openai::completion::MODEL_GPT_5_NANO.to_string()],
        };
        let anthropic = LlmProvider {
            id: String::from(anthropic::completion::LLM.to_lowercase()),
            llm: anthropic::completion::LLM.to_string(),
            models: vec![anthropic::completion::MODEL_CLAUDE_SONNET_4_5.to_string()],
        };

        providers.push(gemini);
        providers.push(openai);
        providers.push(anthropic);

        providers
    }
}
