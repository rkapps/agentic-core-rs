use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use std::env;
use tracing::{Level, debug};

use agentic_core::{
    agent::service::AgentService,
    capabilities::{
        client::tool::Tool,
        completion::{
            message::Message,
            request::CompletionRequest,
        }, tools::tool::{ToolDefinition, ToolRegistry},
    },
    providers::anthropic,
};
use serde_json::{json, Value};


#[tokio::main]
async fn main() -> Result<()> {

    let filter = filter::Targets::new()
        .with_target("agentic_core::providers::anthropic", Level::DEBUG);

     tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty())  // Compact format
        .with(filter)
        .init();    

    let mut tool_registry = ToolRegistry::new();
    let tool = TestTool {};
    tool_registry.register_tool(tool.clone()).await;
    let mut agent_service = AgentService::new();
    agent_service.register_tool(tool_registry);

    // let api_key =
    //     env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    // let llm = openai::completion::LLM;
    // let model = openai::completion::MODEL_GPT_5_NANO;

    // let api_key =
    //     env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    // let llm = gemini::completion::LLM;
    // let model = gemini::completion::MODEL_GEMINI_3_FLASH_PREVIEW;

    let api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");
    let llm = anthropic::completion::LLM;
    let model = anthropic::completion::MODEL_CLAUDE_SONNET_4_5;

    let _ = agent_service.register_client(llm, &api_key).unwrap();

    let content = "what is the weather in paris and San Fransicso".to_string();
    let message = Message::User {
        content: content.clone(),
        response_id: None,
    };


    let mut tools = Vec::new();
    let tool_definition = ToolDefinition::new(tool);
    tools.push(tool_definition);

    let request = CompletionRequest {
        model: model.to_string(),
        system: Some(content),
        messages: vec![message],
        temperature: 0.5,
        max_tokens: 5000,
        stream: false,
        definitions: tools,
    };

    let agent = agent_service.get_chat_agent(llm)?;

    let response = agent.complete_with_tools(request).await?;
    debug!("Response: {:#?}", response);

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct TestTool {}
#[derive(Deserialize)]
struct Weather {
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
