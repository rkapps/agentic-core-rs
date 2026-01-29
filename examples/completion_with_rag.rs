use std::{env, fs::File, io::BufReader, path::PathBuf};

use agentic_core::{
    agent::service::AgentService,
    capabilities::{
        client::{embeddings::EmbeddingClient, tool::Tool},
        completion::message::Message,
    },
    providers::openai::embeddings::OpenAIEmbeddingClient,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use storage_core::vector::search::vector_search;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::models::ticker_embedding::TickerEmbedding;
pub mod models;

#[derive(Debug)]
struct TickerAnalyseTool {
    prompt: String,
    embeddings: Vec<TickerEmbedding>,
}

#[derive(Debug, Deserialize)]
pub struct Ticker {
    symbols: Vec<String>,
}

#[async_trait]
impl Tool for TickerAnalyseTool {
    fn name(&self) -> String {
        "ticker_analyse".to_string()
    }

    fn description(&self) -> String {
        "Provides analysis on Stock tickers".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "symbols": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Array of ticker symbols"
                }
            },
            "required": ["symbols"]
        })
    }

    async fn execute(&self, value: serde_json::Value) -> Result<Value> {
        let ticker: Ticker = match serde_json::from_value(value.clone()) {
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Error dezerializing arguments: {:#?}",
                    value
                ));
            }
        };
        let Some(symbol) = ticker.symbols.iter().find(|&e| e == "AAPL") else {
            return Err(anyhow::anyhow!("Error find Apple stock ticker"));
        };

        println!("Ticker: {:?}", symbol);
        let openai_api_key: String =
            env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
        let embedding_client = OpenAIEmbeddingClient::new(&openai_api_key)?;

        // get the input embeddings for the prompt
        let input_embeddings = embedding_client
            .embed_text(&self.prompt)
            .await
            .map_err(|e| anyhow::anyhow!("Error embedding input prompt {}: {}", self.prompt, e))?;

        //build the candidates from the embeddings for apple stock
        let candidates: Vec<(String, Vec<f32>)> = self
            .embeddings
            .iter()
            .map(|entry| (entry.id.clone(), entry.vector.clone()))
            .collect();

        // top 20 similarit results from vector_search
        let results = vector_search(&input_embeddings.into_vec(), &candidates, 5);
        // iterator through result and return vector of (TickerEmbedding, f32)
        let final_results: Vec<(TickerEmbedding, f32)> = results
            .iter()
            .filter_map(|(id, score)| {
                self.embeddings
                    .iter()
                    .find(|item| item.id == *id)
                    .cloned()
                    .map(|item| (item, *score))
            })
            .collect();

        // Build the text with the summary of the sentiments for the selected vectors            
        let texts = final_results
            .iter()
            .map(|entry| entry.0.embedding_text.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        // Format the the summarize contents to the llm            
        let content: String = format!("{{ {} }}", texts);
        Ok(json!({
            "symbol": symbol,
            "sentiment_count": texts.len(),
            "sentiments": content
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("agentic_core::examples", Level::DEBUG)
        // .with_target("agentic_core::agent", Level::DEBUG)
        ;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty()) // Compact format
        .with(filter)
        .init();

    let path = PathBuf::from("examples/data/ticker_embeddings.json");
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let contents: Vec<TickerEmbedding> = serde_json::from_reader(reader).unwrap();

    println!("Contents: {:?}", contents.len());

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");

    let mut messages = vec![];
    let content = "Is apple a buy".to_string();
    let message = Message::User {
        content: content.clone(),
        response_id: None,
    };
    messages.push(message);

    let system_prompt = "You are financial expert and advisor in analysing stocks and market trends. You will help guide my decision making in the stock market";

    let tool = TickerAnalyseTool {
        prompt: content,
        embeddings: contents
    };

    let agent_service = AgentService::new();
    let agent = agent_service
        .builder()
        .with_system(system_prompt.to_string())
        .with_openai(&api_key)?
        .with_tool(tool)
        .build()?;

    let response = agent.complete_with_tools(&messages).await?;
    println!("Response: {:?}", response);

    Ok(())
}
