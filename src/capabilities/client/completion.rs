use std::pin::Pin;

use anyhow::Result;
use async_trait::async_trait;
use futures_util::Stream;

use crate::capabilities::completion::{
    request::CompletionRequest,
    response::{CompletionChunkResponse, CompletionResponse},
};

// Llm config defines the unified set of parameters
pub struct LlmConfig {
    pub model: String,
}

pub type CompletionStreamResponse = Pin<Box<dyn Stream<Item = Result<CompletionChunkResponse>> + Send>>;

#[async_trait]
//LlmClient defines the trait for the llm
pub trait LlmClient: Send + Sync + std::fmt::Debug {
    // complete implements completion api
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse>;
    // async fn complete_with_streaming(&self, request: CompletionRequest) -> Result<()>;
    async fn complete_with_stream(&self, request: CompletionRequest) -> Result<CompletionStreamResponse>;
}
