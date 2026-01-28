use crate::{
    capabilities::{
        client::embeddings::EmbeddingClient,
        embeddings::{BatchResult, Embedding},
    },
    http::HttpClient,
    providers::openai::{
        request::OpenAIEmbeddingsRequest, response::OpenAIEmbeddingsResponse, OPENAI_BASE_URL,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use tracing::debug;

#[derive(Debug)]
pub struct OpenAIEmbeddingClient {
    pub api_key: String,
    pub base_url: String,
    http_client: HttpClient,
}

impl OpenAIEmbeddingClient {
    pub fn new(api_key: &str) -> Result<Self> {
        Ok(OpenAIEmbeddingClient {
            api_key: api_key.to_string(),
            base_url: OPENAI_BASE_URL.to_string(),
            http_client: HttpClient::new()?,
        })
    }
}

#[async_trait]
impl EmbeddingClient for OpenAIEmbeddingClient {
    async fn embed_text(&self, text: &str) -> Result<Embedding> {
        let url = format!("{}/v1/embeddings", self.base_url,);
        let request = OpenAIEmbeddingsRequest::new(&vec![text]);

        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);

        let body = serde_json::json!(request);
        debug!("Request Body: {:#?}", request);

        let response = self
            .http_client
            .post_request::<OpenAIEmbeddingsResponse>(url, Some(headers), body)
            .await?;
        debug!("Response: {:#?}", response.data.len());
        let embedding = Embedding::new(response.data[0].embedding.clone());

        Ok(embedding)
    }

    async fn embed_text_batch(&self, texts: &[&str]) -> Result<BatchResult> {
        let url = format!("{}/v1/embeddings", self.base_url,);
        let request = OpenAIEmbeddingsRequest::new(texts);
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);

        let body = serde_json::json!(request);
        debug!("Request Body: {:#?}", request);

        let response = self
            .http_client
            .post_request::<OpenAIEmbeddingsResponse>(url, Some(headers), body)
            .await?;


        // consume the response, assuming that OpenAI returns the vectors in the same order. 
        // NEED TO VERIFY!!!
        let successful = response
            .data
            .into_iter()
            .map(|data| (data.index, Embedding::new(data.embedding)))
            .collect();

        Ok(BatchResult { successful, failed: Vec::new() })
    }
}
