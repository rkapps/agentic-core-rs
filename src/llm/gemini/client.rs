use anyhow::Result;
use async_trait::async_trait;

use crate::{
    capabilities::completion::{CompletionRequest, CompletionResponse},
    http,
    llm::{
        client::LlmClient,
        gemini::{completion::{GeminiCompletionRequest, GeminiResponse}, interactions::{GeminiInteractionsRequest, GeminiInteractionsResponse}},
    },
};

#[derive(Debug)]
pub struct GeminiClient {
    pub api_key: String,
    pub model: String,
    pub base_url: String,
}

pub const LLM: &str = "Gemini";
pub const MODEL_GEMINI_3_FLASH_PREVIEW: &str = "gemini-3-flash-preview";
const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com";

impl GeminiClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key: api_key,
            model: model,
            base_url: GEMINI_BASE_URL.to_string(),
        }
    }


    async fn complete_generate_content(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!(
            "{}/v1beta/models/{}:generateContent",
            self.base_url, self.model,
        );

        let http = http::HttpClient::new(&url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-goog-api-key", self.api_key.parse()?);

        let grequest = GeminiCompletionRequest::new(request);
        let body = serde_json::json!(grequest);
        let gresponse = http.post_request::<GeminiResponse>(Some(headers), body).await?;
        let cresponse = CompletionResponse {
            id: String::new(),
            content: gresponse.candidates[0].content.parts[0].text.to_string(),
        };

        Ok(cresponse)
    }
    

    async fn complete_interactions(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let url = format!(
            "{}/v1beta/interactions",
            self.base_url,
        );

        let http = http::HttpClient::new(&url);
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("x-goog-api-key", self.api_key.parse()?);

        let grequest = GeminiInteractionsRequest::new(request);
        let body = serde_json::json!(grequest);
        let gresponse = http.post_request::<GeminiInteractionsResponse>(Some(headers), body).await?;

        let id = gresponse.id;
        let mut message = String::new();
        for output in gresponse.outputs {
            if output.r#type == "text" {
                message = output.text;
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
    fn model(&self) -> &str {
        &self.model
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_interactions(request).await
    }

}
