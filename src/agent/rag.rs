use std::sync::Arc;
use tracing::debug;
use anyhow::Result;

use crate::{agent::completion::Agent, capabilities::{client::embeddings::EmbeddingClient, embeddings::Embedding}};

#[derive(Debug)]
pub struct RagAgent {
    agent: Arc<Agent>,
    embedding_client: Arc<dyn EmbeddingClient>
}

impl RagAgent {
        
    pub fn new(agent: Arc<Agent>, embedding_client: Arc<dyn EmbeddingClient>) -> RagAgent {
        Self{
            agent,
            embedding_client
        }
    }

    pub async fn complete_with_rag(&self, text: &str) -> Result<Embedding>{
        let embedding = self.embedding_client.embed_text(text).await;
        debug!("Embedding: {:#?}", embedding);
        embedding
    }

    pub async fn complete_with_rag_batch(&self, texts: Vec<&str>) -> Result<Vec<Embedding>>{
        let embeddings = self.embedding_client.embed_text_batch(texts).await;
        debug!("Embeddings: {:#?}", embeddings);
        embeddings
    }

}
