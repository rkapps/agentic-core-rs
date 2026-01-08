use anyhow::Result;
use async_trait::async_trait;

use crate::{
    capabilities::completion::{CompletionRequest, CompletionResponse},
    http,
    llm::{
        anthropic::completion::{AnthropicCompletionRequest, AnthropicCompletionResponse}, client::LlmClient
    },
};

#[derive(Debug)]
pub struct AnthropicClient {
    pub api_key: String,
    pub anthropic_version: String,
    pub model: String,
    pub base_url: String,
}

pub const LLM: &str = "Anthropic";
pub const MODEL_CLAUDE_SONNET_4_5: &str = "claude-sonnet-4-5";
const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com";

impl AnthropicClient {
    pub fn new(api_key: String, anthropic_version: String, model: String) -> Self {
        Self {
            api_key: api_key,
            anthropic_version: anthropic_version,
            model: model,
            base_url: ANTHROPIC_BASE_URL.to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!(
            "{}/v1/messages",
            self.base_url
        );

        let http = http::HttpClient::new(&url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse()?);
        headers.insert("anthropic-version", self.anthropic_version.parse()?);

        let arequest = AnthropicCompletionRequest::new(request);

        let body = serde_json::json!(arequest);
        let aresponse = http.post_request::<AnthropicCompletionResponse>(Some(headers), body).await?;
        let cresponse = CompletionResponse {
            id: String::new(),
            content: aresponse.content[0].text.to_string(),
        };

        Ok(cresponse)
    }
}
