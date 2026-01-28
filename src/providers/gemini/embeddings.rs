use crate::{
    capabilities::{client::embeddings::EmbeddingClient, embeddings::{BatchResult, Embedding}},
    http::HttpClient,
    providers::gemini::{
            GEMINI_BASE_URL, request::GeminiEmbeddingsRequest, response::GeminiEmbeddingsResponse
        },
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::debug;

#[derive(Debug)]
pub struct GeminiEmbeddingClient {
    pub api_key: String,
    pub base_url: String,
    http_client: HttpClient,
}

impl GeminiEmbeddingClient {
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(GeminiEmbeddingClient {
            api_key: api_key.to_string(),
            base_url: GEMINI_BASE_URL.to_string(),
            http_client: HttpClient::new()?,
        })
    }
}

#[async_trait]
impl EmbeddingClient for GeminiEmbeddingClient {
    async fn embed_text(&self, text: &str) -> Result<Embedding> {
        let url = format!("{}/v1beta/models/gemini-embedding-001:embedContent", self.base_url,);
        let request: GeminiEmbeddingsRequest = GeminiEmbeddingsRequest::new(vec![text]);

        let mut headers = reqwest::header::HeaderMap::new();
        // let bearer = format!("Bearer {}", self.api_key);
        // headers.insert("Authorization", bearer.parse()?);
        headers.insert("x-goog-api-key", self.api_key.parse()?);

        debug!("Request: {:#?}", request);

        let body = serde_json::json!(request);
        let response = self
            .http_client
            .post_request::<GeminiEmbeddingsResponse>(url, Some(headers), body)
            .await?;
        debug!("Response: {:#?}", response.embedding.values.len());

        let embedding = Embedding::new(response.embedding.clone().values);

        Ok(embedding)
    }

    async fn embed_text_batch(&self, texts: &[&str]) -> Result<BatchResult> {

        let mut successful = Vec::new();
        let mut failed = Vec::new();

        for (index, text) in texts.iter().enumerate() {
            match self.embed_text(text).await {
                Ok(c) => {
                    successful.push((index, c));
                },
                Err(e) => {
                    failed.push((index, e));
                }

            };
        }

        Ok(BatchResult { successful, failed })
    }
}
