use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use tracing::debug;

use crate::{
    capabilities::completion::{ChatResponseChunk, CompletionRequest, CompletionResponse},
    http::HttpClient,
    llm::{
        anthropic::completion::{
            AnthropicChunkResponse, AnthropicCompletionRequest, AnthropicCompletionResponse,
        },
        client::{ChatStream, LlmClient},
    },
};

#[derive(Debug)]
pub struct AnthropicClient {
    api_key: String,
    anthropic_version: String,
    base_url: String,
    http_client: HttpClient,
}

pub const LLM: &str = "Anthropic";
pub const MODEL_CLAUDE_SONNET_4_5: &str = "claude-sonnet-4-5";
const ANTHROPIC_BASE_URL: &str = "https://api.anthropic.com";

impl AnthropicClient {
    pub fn new(api_key: String, anthropic_version: String) -> Result<Self> {
        Ok(Self {
            api_key: api_key,
            anthropic_version: anthropic_version,
            base_url: ANTHROPIC_BASE_URL.to_string(),
            http_client: HttpClient::new()?,
        })
    }
}

#[async_trait]
impl LlmClient for AnthropicClient {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!("{}/v1/messages", self.base_url);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse()?);
        headers.insert("anthropic-version", self.anthropic_version.parse()?);

        let arequest = AnthropicCompletionRequest::new(request);
        let body = serde_json::json!(arequest);
        let aresponse = self
            .http_client
            .post_request::<AnthropicCompletionResponse>(url, Some(headers), body)
            .await?;
        let cresponse = CompletionResponse {
            id: String::new(),
            content: aresponse.content[0].text.to_string(),
        };

        Ok(cresponse)
    }

    async fn complete_with_stream(&self, request: CompletionRequest) -> Result<ChatStream> {
        let url = format!("{}/v1/messages", self.base_url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse()?);
        headers.insert("anthropic-version", self.anthropic_version.parse()?);
        let arequest = AnthropicCompletionRequest::new(request);
        let body = serde_json::json!(arequest);

        let response = self
            .http_client
            .post_stream_request(url, Some(headers), body)
            .await?;

        let stream = response
            .bytes_stream()
            .eventsource() // ← Parses SSE format
            .map(|event_result| -> anyhow::Result<ChatResponseChunk> {
                let event = event_result?;

                debug!("event: {:#?}", &event);
                // event.data contains the JSON string
                let chunk: AnthropicChunkResponse =
                    serde_json::from_str(&event.data).map_err(|e| {
                        anyhow!(format!(
                            "AnthropicChunkResponse error: {:?} for data {:?}",
                            e, &event.data
                        ))
                    })?;

                // Transform to ChatResponseChunk
                match chunk.r#type.as_str() {
                    "content_block_delta" => {
                        let text = chunk.clone().delta.and_then(|d| d.text).unwrap_or_default();
                        let thinking = &chunk.delta.and_then(|d| d.thinking).unwrap_or_default();

                        Ok(ChatResponseChunk::content(text.to_string(),thinking.to_string()))
                    }
                    "message_stop" => Ok(ChatResponseChunk::stop()),
                    _ => Ok(ChatResponseChunk::default()),
                }
            });

        Ok(Box::pin(stream))
    }
}
