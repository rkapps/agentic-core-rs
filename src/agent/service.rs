use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{
    agent::builder::AgentBuilder,
    capabilities::{
        client::completion::LlmClient,
        tools::{mcp::MCPRegistry, tool::ToolRegistry},
    },
    providers::{anthropic, gemini, openai},
};

#[derive(Serialize, Debug, Clone)]
pub struct LlmProvider {
    pub id: String,
    pub llm: String,
    pub models: Vec<String>,
}

pub struct AgentService {
    pub clients: Arc<RwLock<HashMap<String, Arc<dyn LlmClient>>>>,
    pub tool_registry: Arc<RwLock<ToolRegistry>>,
    pub mcp_registry: Arc<RwLock<MCPRegistry>>,
}

impl AgentService {
    pub fn new() -> AgentService {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            tool_registry: Arc::new(RwLock::new(ToolRegistry::new())),
            mcp_registry: Arc::new(RwLock::new(MCPRegistry::new())),
        }
    }

    pub fn builder(&self) -> AgentBuilder<'_> {
        AgentBuilder::new(self)
    }


    pub fn get_llm_providers(&self) -> Vec<LlmProvider> {
        let mut providers: Vec<LlmProvider> = Vec::new();

        let gemini = LlmProvider {
            id: String::from(gemini::LLM.to_lowercase()),
            llm: gemini::LLM.to_string(),
            models: vec![gemini::MODEL_GEMINI_3_FLASH_PREVIEW.to_string()],
        };
        let openai = LlmProvider {
            id: String::from(openai::LLM.to_lowercase()),
            llm: openai::LLM.to_string(),
            models: vec![openai::MODEL_GPT_5_NANO.to_string()],
        };
        let anthropic = LlmProvider {
            id: String::from(anthropic::LLM.to_lowercase()),
            llm: anthropic::LLM.to_string(),
            models: vec![anthropic::MODEL_CLAUDE_SONNET_4_5.to_string()],
        };

        providers.push(gemini);
        providers.push(openai);
        providers.push(anthropic);

        providers
    }
}
