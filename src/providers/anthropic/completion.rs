use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use tracing::{debug, error};

use crate::{
    capabilities::{
        client::completion::{CompletionStreamResponse, LlmClient},
        completion::{
            request::CompletionRequest,
            response::{CompletionChunkResponse, CompletionResponse, CompletionResponseContent},
        }, tools::request::ToolCallRequest,
    },
    http::HttpClient,
    providers::anthropic::{
        request::AnthropicCompletionRequest,
        response::{
            AnthropicChunkResponse, AnthropicCompletionResponse,
            AnthropicCompletionResponseContent::{Text, ToolUse},
        },
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
const ANTHROPIC_VERSION: &str = "2023-06-01";

impl AnthropicClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key: api_key,
            anthropic_version: ANTHROPIC_VERSION.to_string(),
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

        let arequest = AnthropicCompletionRequest::new(request)?;
        debug!("AnthropicCompletionRequest {:#?}", arequest);

        let body = serde_json::json!(arequest);
        let aresponse = self
            .http_client
            .post_request::<AnthropicCompletionResponse>(url, Some(headers), body)
            .await?;

        debug!("Response: {:#?}", aresponse);

        let mut rcontents: Vec<CompletionResponseContent> = Vec::new();
        for content in aresponse.content {
            match content {
                Text { text } => {
                    let rcontent = CompletionResponseContent::Text(text);
                    rcontents.push(rcontent);
                }
                ToolUse { id, name, input } => {
                    let rcontent = CompletionResponseContent::ToolCall(ToolCallRequest {
                        id,
                        name,
                        arguments: input,
                    });
                    rcontents.push(rcontent);
                }
            }
        }

        let cresponse = CompletionResponse {
            response_id: String::new(),
            contents: rcontents,
        };

        Ok(cresponse)
    }

    async fn complete_with_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionStreamResponse> {
        let url = format!("{}/v1/messages", self.base_url);
        debug!("Gemini Request: {:#?}", request);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-api-key", self.api_key.parse()?);
        headers.insert("anthropic-version", self.anthropic_version.parse()?);
        headers.insert("Accept", "text/event-stream".parse()?);

        let arequest = AnthropicCompletionRequest::new(request)?;
        let body = serde_json::json!(arequest);

        let response = self
            .http_client
            .post_stream_request(url, Some(headers), body)
            .await?;

        // debug!("✅ Got response: {:?}", response.error_for_status());
        if response.status() == 400 {
            let error_body = response.text().await?;
            error!("❌ API ERROR BODY: {}", error_body);
            return Err(anyhow!("Bad request: {}", error_body));
        }

        let stream = response
            .bytes_stream()
            .eventsource() // ← Parses SSE format
            .map(|event_result| -> anyhow::Result<CompletionChunkResponse> {
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

                // Transform to CompletionChunkResponse
                match chunk.r#type.as_str() {
                    "content_block_delta" => {
                        let text = chunk.clone().delta.and_then(|d| d.text).unwrap_or_default();
                        let thinking = &chunk.delta.and_then(|d| d.thinking).unwrap_or_default();

                        Ok(CompletionChunkResponse::content(
                            text.to_string(),
                            thinking.to_string(),
                        ))
                    }
                    "message_stop" => Ok(CompletionChunkResponse::stop(String::new())),
                    _ => Ok(CompletionChunkResponse::default()),
                }
            });

        Ok(Box::pin(stream))
    }
}
