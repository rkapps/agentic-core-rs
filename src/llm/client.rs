use anyhow::Result;
use async_trait::async_trait;

use crate::capabilities::completion::{CompletionRequest, CompletionResponse};

// Llm config defines the unified set of parameters
pub struct LlmConfig {
    pub model: String,
}

#[async_trait]
//LlmClient defines the trait for the llm
pub trait LlmClient: Send + Sync + std::fmt::Debug {
    // returns the model
    fn model(&self) -> &str;
    // complete implements completion api
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
}
