use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use tracing::debug;

use crate::{
    capabilities::{
        client::completion::CompletionStreamResponse,
        completion::{
            request::CompletionRequest,
            response::{CompletionChunkResponse, CompletionResponse},
        },
    },
    http::HttpClient,
    providers::openai::{
        request::OpenAICompletionRequest,
        response::{OpenAIChunkResponseData, OpenAICompletionResponse},
    },
};

#[derive(Debug)]
pub struct OpenAIClient {
    pub api_key: String,
    pub base_url: String,
    http_client: HttpClient,
}

pub const LLM: &str = "OpenAI";
pub const MODEL_GPT_5_NANO: &str = "gpt-5-nano";
const OPENAI_BASE_URL: &str = "https://api.openai.com";

impl OpenAIClient {
    pub fn new(api_key: String) -> Result<Self> {
        Ok(Self {
            api_key: api_key,
            base_url: OPENAI_BASE_URL.to_string(),
            http_client: HttpClient::new()?,
        })
    }
}

#[async_trait]
impl crate::capabilities::client::completion::LlmClient for OpenAIClient {
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!("{}/v1/responses", self.base_url,);

        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);

        let orequest = OpenAICompletionRequest::new(request);
        let body = serde_json::json!(orequest);
        let oresponse = self
            .http_client
            .post_request::<OpenAICompletionResponse>(url, Some(headers), body)
            .await?;

        debug!("OpenAICompletionResponse: {:#?}", oresponse);
        let id = oresponse.id;
        let mut message = String::new();
        for output in oresponse.output {
            if output.r#type == "message" {
                for content in output.content {
                    if content.r#type == "output_text" {
                        message = content.text;
                    }
                }
            }
        }

        let cresponse = CompletionResponse {
            response_id: id,
            content: message,
        };

        Ok(cresponse)
    }

    async fn complete_with_stream(&self, request: CompletionRequest) -> Result<CompletionStreamResponse> {
        let url = format!("{}/v1/responses", self.base_url,);

        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);
        headers.insert("Accept", "text/event-stream".parse()?);
        headers.insert("Accept-Encoding", "identity".parse()?);

        let request = OpenAICompletionRequest::new(request);
        let body = serde_json::json!(request);
        let response = self
            .http_client
            .post_stream_request(url, Some(headers), body)
            .await?;

        debug!("OpenAI Request: {:#?}", request);
        let stream = response
            .bytes_stream()
            .eventsource() // ← Parses SSE format
            .map(|event_result| -> anyhow::Result<CompletionChunkResponse> {
                let event = event_result?;
                debug!("event: {:#?}", &event);

                let chunk: OpenAIChunkResponseData =
                    serde_json::from_str(&event.data).map_err(|e| {
                        anyhow!(format!(
                            "OpenAIChunkResponse error: {:?} for data {:?}",
                            e, &event.data
                        ))
                    })?;

                match event.event.as_str() {
                    "response.output_text.delta" => {
                        if let Some(delta) = chunk.delta {
                            Ok(CompletionChunkResponse::content(delta, String::new()))
                        } else {
                            Ok(CompletionChunkResponse::default())
                        }
                    }
                    "response.completed" => {
                        if let Some(response) = chunk.response {
                            Ok(CompletionChunkResponse::stop(response.id))
                        } else {
                            Ok(CompletionChunkResponse::default())   
                        }
                    }
                    _ => Ok(CompletionChunkResponse::default()),
                }
            });

        debug!("done streaming");

        Ok(Box::pin(stream))
    }
}
