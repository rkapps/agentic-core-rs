use std::sync::Arc;

use crate::{
    capabilities::model::{CompletionRequest, CompletionResponse},
    llm::client::{ChatStream, LlmClient},
};
use anyhow::Result;
use tracing::debug;

#[derive(Debug)]
pub struct AgentConfig {
    pub client: Arc<dyn LlmClient>,
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
        debug!("Completion Request: {:#?}", request);
        self.config.client.complete(request).await
    }

    pub async fn complete_with_stream(&self, request: CompletionRequest) -> Result<ChatStream> {
        self.config.client.complete_with_stream(request).await
    }
}
