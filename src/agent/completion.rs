use std::sync::Arc;

use crate::capabilities::{
    client::completion::{CompletionStreamResponse, LlmClient},
    completion::{request::CompletionRequest, response::CompletionResponse},
};
use anyhow::Result;
use tracing::debug;

// #[derive(Debug)]
// pub struct AgentConfig {
//     pub client: Arc<dyn LlmClient>,
//     pub temperature: Option<f32>,
//     pub max_tokens: Option<i32>,
// }

#[derive(Debug)]
pub struct Agent {
    pub client: Arc<dyn LlmClient>,
    pub temperature: f32,
    pub max_tokens: i32,
}

impl Agent {
    pub fn max_tokens(&self) -> i32 {
        self.max_tokens
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    //complete defines a multi turn chat
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        debug!("Completion Request: {:#?}", request);
        self.client.complete(request).await
    }

    pub async fn complete_with_stream(&self, request: CompletionRequest) -> Result<CompletionStreamResponse> {
        self.client.complete_with_stream(request).await
    }
}
