use crate::{
    capabilities::{client::embeddings::EmbeddingClient, embeddings::Embedding},
    http::HttpClient,
    providers::{
        gemini::{
            request::GeminiEmbeddingsRequest, response::GeminiEmbeddingsResponse, GEMINI_BASE_URL,
        },
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

        let embedding = Embedding{
            text: text.to_string(),
            vector: response.embedding.clone().values,
            dimension: response.embedding.values.len(),
            model: request.model,
        };

        Ok(embedding)
    }

    async fn embed_text_batch(&self, texts: Vec<&str>) -> Result<Vec<Embedding>> {

        let mut embeddings: Vec<Embedding> = Vec::new();
        for text in texts {
            let embedding = self.embed_text(text).await?;
            embeddings.push(embedding);
        }

        // let url = format!("{}/v1beta/models/gemini-embedding-001:embedContent", self.base_url,);
        // let request: GeminiEmbeddingsRequest = GeminiEmbeddingsRequest::new(texts);

        // let mut headers = reqwest::header::HeaderMap::new();
        // // let bearer = format!("Bearer {}", self.api_key);
        // // headers.insert("Authorization", bearer.parse()?);
        // headers.insert("x-goog-api-key", self.api_key.parse()?);

        // debug!("Request: {:#?}", request);

        // let body = serde_json::json!(request);
        // let response = self
        //     .http_client
        //     .post_request::<GeminiEmbeddingsResponse>(url, Some(headers), body)
        //     .await?;
        // debug!("Response: {:#?}", response.embedding.values.len());
        // Ok(vec![Embedding::empty()])
        Ok(embeddings)
    }
}
