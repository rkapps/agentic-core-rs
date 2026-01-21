use std::{env, sync::Arc};

use agentic_core::{agent::{rag::RagAgent, service::AgentService}, providers::{gemini::{self, embeddings::GeminiEmbeddingClient}, openai::{self, embeddings::OpenAIEmbeddingClient}}};
use anyhow::Result;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {

    let filter = filter::Targets::new()
        // .with_target("agentic_core::http", Level::DEBUG)
        .with_target("agentic_core::agent", Level::DEBUG)
        .with_target("agentic_core::providers::openai", Level::DEBUG)
        .with_target("agentic_core::providers::gemini", Level::DEBUG)
        ;

     tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty())  // Compact format
        .with(filter)
        .init();    


    // let api_key =
    //     env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    // let llm = openai::LLM;

    let api_key =
        env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let llm = gemini::LLM;

    let mut agent_service = AgentService::new();
    let _ = agent_service.register_client(llm, &api_key).unwrap();
    let agent = agent_service.get_completion_agent(llm)?;

    let embedding_client = GeminiEmbeddingClient::new(&api_key)?;
    // let embedding_client = OpenAIEmbeddingClient::new(&api_key)?;

    let rag_agent = RagAgent::new(agent, Arc::new(embedding_client));
    rag_agent.complete_with_rag("The world is small").await?;
    rag_agent.complete_with_rag_batch(vec!["The world is small", "The world is large"]).await?;
    
    Ok(())
}