use std::fmt::Debug;
use async_trait::async_trait;
use anyhow::Result;
use crate::capabilities::embeddings::{BatchResult, Embedding};


#[async_trait]
pub trait EmbeddingClient: Send + Sync +Debug {

    async fn embed_text(&self, text: &str) -> Result<Embedding>;

    async fn embed_text_batch(&self, texts: &[&str]) -> Result<BatchResult>;

}
