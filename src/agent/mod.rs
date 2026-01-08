pub mod builder;

use crate::{
    capabilities::completion::{CompletionRequest, CompletionResponse},
    llm::client::LlmClient,
};
use anyhow::Result;
use tracing::debug;

#[derive(Debug)]
pub struct AgentConfig {
    pub client: Box<dyn LlmClient>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
}

#[derive(Debug)]
pub struct Agent {
    pub config: AgentConfig,
}

impl Agent {
    pub fn max_tokens(&self) -> i32 {
        self.config.max_tokens.expect("this should not happen")
    }

    pub fn temperature(&self) -> f32 {
        self.config.temperature.expect("this should not happen")
    }

    //complete defines a multi turn chat
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {

        // info!("Completion with model: {:?}", self.model);
        debug!("Request {:#?}", request);
        match self.config.client.complete(request).await {
            Ok(response) => Ok(response),
            Err(e) => {
                return Err(e);
            }
        }
    }
}
