use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use tracing::debug;

use crate::{
    capabilities::completion::{ChatResponseChunk, CompletionRequest, CompletionResponse},
    http::HttpClient,
    llm::{
        client::{ChatStream, LlmClient},
        gemini::interactions::{GeminiInteractionsChunkResponse, GeminiInteractionsRequest, GeminiInteractionsResponse},
    },
};

#[derive(Debug)]
pub struct GeminiClient {
    pub api_key: String,
    pub base_url: String,
    http_client: HttpClient,
}

pub const LLM: &str = "Gemini";
pub const MODEL_GEMINI_3_FLASH_PREVIEW: &str = "gemini-3-flash-preview";
const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com";

impl GeminiClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key: api_key,
            base_url: GEMINI_BASE_URL.to_string(),
            http_client: HttpClient::new()?,
        })
    }

    // async fn complete_generate_content(
    //     &self,
    //     request: CompletionRequest,
    // ) -> Result<CompletionResponse> {

    //     let url = format!(
    //         "{}/v1beta/models/{}:generateContent",
    //         self.base_url, self.model,
    //     );

    //     let mut headers = reqwest::header::HeaderMap::new();
    //     headers.insert("x-goog-api-key", self.api_key.parse()?);

    //     let grequest = GeminiCompletionRequest::new(request);
    //     let body = serde_json::json!(grequest);
    //     let gresponse = self.http_client
    //         .post_request::<GeminiResponse>(url,Some(headers), body)
    //         .await?;
    //     let cresponse = CompletionResponse {
    //         id: String::new(),
    //         content: gresponse.candidates[0].content.parts[0].text.to_string(),
    //     };

    //     Ok(cresponse)
    // }

    async fn complete_interactions(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let url = format!("{}/v1beta/interactions", self.base_url,);

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-goog-api-key", self.api_key.parse()?);

        let grequest = GeminiInteractionsRequest::new(request);
        let body = serde_json::json!(grequest);
        debug!("Body: {:#?}", body);
        let gresponse = self
            .http_client
            .post_request::<GeminiInteractionsResponse>(url, Some(headers), body)
            .await?;

        let id = gresponse.id;
        let mut message = String::new();
        for output in gresponse.outputs {
            if output.r#type == "text" {
                if let Some(value) = output.text {
                    message = value;
                }
            }
        }

        let cresponse = CompletionResponse {
            id: id,
            content: message,
        };

        Ok(cresponse)
    }
}

#[async_trait]
impl LlmClient for GeminiClient {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_interactions(request).await
    }

    async fn complete_with_stream(&self, request: CompletionRequest) -> Result<ChatStream> {
        let url = format!("{}/v1beta/interactions", self.base_url,);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-goog-api-key", self.api_key.parse()?);

        let grequest = GeminiInteractionsRequest::new(request);
        let body = serde_json::json!(grequest);
        debug!("Body: {:#?}", body);
        let response = self
            .http_client
            .post_stream_request(url, Some(headers), body)
            .await?;

        // debug!("Gemini Request: {:#?}", grequest);
        let stream = response
            .bytes_stream()
            .eventsource() // ← Parses SSE format
            .map(|event_result| -> anyhow::Result<ChatResponseChunk> {

                let event = event_result?;
                debug!("event: {:#?}", event.data);

                if event.data.contains("[DONE]") {
                    return Ok(ChatResponseChunk::default())
                }

                let chunk: GeminiInteractionsChunkResponse =
                serde_json::from_str(&event.data).map_err(|e| {
                    anyhow!(format!(
                        "GeminiChunkResponse error: {:?} for data {:?}",
                        e, &event.data
                    ))
                })?;

                // debug!("chunk: {:#?}", chunk);
                match chunk.event_type.as_str() {
                    "content.start" => Ok(ChatResponseChunk::default()),
                    "content.delta" => {

                        if let Some(delta) = chunk.delta {
                            if let Some(text) = delta.text {
                                Ok( ChatResponseChunk::content(text, String::new()))
                            } else {
                                Ok(ChatResponseChunk::default())
                            }
                            
                        } else {
                            Ok(ChatResponseChunk::default())
                        }
                    }
                    "content.stop" => Ok(ChatResponseChunk::default()),
                    "interaction.complete" => Ok(ChatResponseChunk::stop()),
                    _ => Ok(ChatResponseChunk::default())
                }


            });

        Ok(Box::pin(stream))
    }
}
