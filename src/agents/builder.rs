use std::sync::Arc;

use crate::{
    agents::agent::{Agent, AgentConfig}, llm::client::LlmClient
};
use anyhow::Result;

pub struct AgentBuilder {
    client: Option<Arc<dyn LlmClient>>,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            temperature: None,
            max_tokens: None,
        }
    }

    //set the client
    pub fn client(mut self, client: Arc<dyn LlmClient>) -> Self {
        self.client = Some(client);
        self
    }

    //set the temperature
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    //set the max number of tokens
    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    //build the agent
    pub fn build(self) -> Result<Agent> {
        //set the client, error if client is not avaialble
        let client = self
            .client
            .ok_or_else(|| anyhow::anyhow!("Client is required"))?;

        Ok(Agent {
            config: AgentConfig {
                client: client,
                temperature: self.temperature,
                max_tokens: self.max_tokens,
            },
        })
    }
}
