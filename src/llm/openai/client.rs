use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;

use crate::{
    capabilities::completion::{CompletionRequest, CompletionResponse},
    http,
    llm::{
        client::LlmClient,
        openai::completion::{OpenAICompletionRequest, OpenAICompletionResponse},
    },
};

#[derive(Debug, Serialize)]
pub struct OpenAIClient {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

pub const LLM: &str = "OpenAI";
pub const MODEL_GPT_5_NANO: &str = "gpt-5-nano";
const OPENAI_BASE_URL: &str = "https://api.openai.com";

impl OpenAIClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key: api_key,
            model: model,
            base_url: OPENAI_BASE_URL.to_string(),
        }
    }
}

#[async_trait]
impl LlmClient for OpenAIClient {
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!("{}/v1/responses", self.base_url,);

        let http = http::HttpClient::new(&url);
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);
        let model = &self.model;

        let orequest = OpenAICompletionRequest::new(model.clone(), request);
        let body = serde_json::json!(orequest);
        let oresponse = http
            .post_request::<OpenAICompletionResponse>(Some(headers), body)
            .await?;

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
            id: id,
            content: message,
        };

        Ok(cresponse)
    }
}
