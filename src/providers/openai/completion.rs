use anyhow::{anyhow, Result};
use async_trait::async_trait;
use eventsource_stream::Eventsource;
use futures::StreamExt;
use serde_json::Value;
use tracing::{debug, error};

use crate::{
    capabilities::{
        client::completion::CompletionStreamResponse,
        completion::{
            request::CompletionRequest,
            response::{CompletionChunkResponse, CompletionResponse, CompletionResponseContent}
        }, tools::request::ToolCallRequest,
    },
    http::HttpClient,
    providers::openai::{
        request::OpenAICompletionRequest,
        response::{
            OpenAIChunkResponseData, OpenAICompletionResponse,
            OpenAICompletionResponseOutput::{FunctionCall, Message, Reasoning},
        },
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

        let orequest = OpenAICompletionRequest::new(request)?;
        debug!("OpenAICompletionRequest: {:#?}", orequest);
        let body = serde_json::json!(orequest);
        // debug!("Body: {:#?}", body);
        let oresponse = self
            .http_client
            .post_request::<OpenAICompletionResponse>(url, Some(headers), body)
            .await?;

        debug!("OpenAICompletionResponse: {:#?}", oresponse);

        let mut rcontents: Vec<CompletionResponseContent> = Vec::new();
        let id = oresponse.id;

        for output in oresponse.output {
            match output {
                Message {
                    id: _,
                    status,
                    content,
                } => {
                    if status == "completed" {
                        for content in content {
                            if content.r#type == "output_text" {
                                let rcontent= CompletionResponseContent::Text(content.text);
                                rcontents.push(rcontent);
                                break;
                            }
                        }
                    }
                }
                FunctionCall {
                    status,
                    arguments,
                    call_id,
                    name,
                } => {
                    if status == "completed" {
                        let arguments: Value = match serde_json::from_str(arguments.as_str()) {
                            Ok(c) => c,
                            Err(e) => {
                                return Err(anyhow!("Error parsing function arguments: {:#?}", e))
                            }
                        };

                        let rcontent = CompletionResponseContent::ToolCall(ToolCallRequest {
                            id : call_id,
                            name,
                           arguments : arguments,
                        });                        
                        rcontents.push(rcontent);
                    }
                }
                Reasoning{
                    id:_,
                    summary:_
                } => {
                    
                }
                
            }
        }

        let cresponse = CompletionResponse {
            response_id: id,
            contents: rcontents,
        };

        Ok(cresponse)
    }

    async fn complete_with_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionStreamResponse> {
        let url = format!("{}/v1/responses", self.base_url,);
        debug!("OpenAI Request: {:#?}", request);

        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);
        headers.insert("Accept", "text/event-stream".parse()?);
        headers.insert("Accept-Encoding", "identity".parse()?);

        let request = OpenAICompletionRequest::new(request)?;
        let body = serde_json::json!(request);
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
