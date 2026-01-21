use crate::{
    capabilities::{client::embeddings::EmbeddingClient, embeddings::Embedding},
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

        let embedding = Embedding{
            text: text.to_string(),
            vector: response.data[0].embedding.clone(),
            dimension: response.data[0].embedding.len(),
            model: request.model,
        };

        Ok(embedding)

    }

    async fn embed_text_batch(&self, texts: Vec<&str>) -> Result<Vec<Embedding>> {

        let url = format!("{}/v1/embeddings", self.base_url,);
        let request = OpenAIEmbeddingsRequest::new(&texts);
        let model = request.model.clone();
        let mut headers = reqwest::header::HeaderMap::new();
        let bearer = format!("Bearer {}", self.api_key);
        headers.insert("Authorization", bearer.parse()?);

        let body = serde_json::json!(request);
        debug!("Request Body: {:#?}", request);

        let response = self
            .http_client
            .post_request::<OpenAIEmbeddingsResponse>(url, Some(headers), body)
            .await?;

        let mut embeddings: Vec<Embedding> = Vec::new();
        for (index, data) in response.data.iter().enumerate() {

            let embedding = Embedding{
                text: texts[index].to_string(),
                vector: data.embedding.clone(),
                dimension: data.embedding.len(),
                model: model.clone(),
            };
            embeddings.push(embedding);
        }

        Ok(embeddings)

    }
}
