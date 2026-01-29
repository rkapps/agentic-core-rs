use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{debug, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use agentic_core::{
    agent::service::AgentService,
    capabilities::{
        client::tool::Tool,
        completion::message::Message,
    },
};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let filter = filter::Targets::new()
        .with_target("agentic_core::providers::openai", Level::DEBUG)
        .with_target("agentic_core::providers::gemini", Level::DEBUG)
        .with_target("agentic_core::agent", Level::DEBUG);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty()) // Compact format
        .with(filter)
        .init();

    // let api_key =
    //     env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let tool = TestTool {};
    let agent_service = AgentService::new();
    let agent = agent_service
        .builder()
        .with_tool(tool)
        .with_openai(&api_key)?
        .build()?;

    let content = "what is the weather in paris and San Fransicso".to_string();
    let message = Message::User {
        content: content.clone(),
        response_id: None,
    };

    let response = agent.complete_with_tools(&vec![message]).await?;
    debug!("Response: {:#?}", response);

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct TestTool {}
#[derive(Deserialize)]
struct Weather {
    #[serde(rename = "location")]
    _location: String,
}

#[async_trait]
impl Tool for TestTool {
    fn name(&self) -> String {
        "get_weather".to_string()
    }

    fn description(&self) -> String {
        "Get current temperatur for a given location".to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        let parameters = json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City and country e.g. Bogotá, Colombia"
                }
            },
            "required": ["location"],
            "additionalProperties": false
        });
        parameters
    }

    async fn execute(&self, value: serde_json::Value) -> Result<Value> {
        let _: Weather = match serde_json::from_value(value.clone()) {
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Error dezerializing arguments: {:#?}",
                    value
                ));
            }
        };
        let result = "20";
        let newvalue = serde_json::from_str(result)?;
        println!("Argements: {:#?}", newvalue);
        Ok(newvalue)
    }
}
