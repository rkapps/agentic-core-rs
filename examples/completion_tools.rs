use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::debug;

use agentic_core::{
    agent::service::AgentService,
    capabilities::{
        client::tool::{Tool, ToolRegistry},
        completion::{
            message::Message,
            request::CompletionRequest,
            tool::ToolDefinition,
        },
    },
    providers::{anthropic, gemini, openai},
};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Clone)]
pub struct TestTool {}
#[derive(Deserialize)]
struct Weather {
    location: String
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

        // todo!()
        let weather: Weather = match serde_json::from_value(value.clone()) {
            Ok(c) => c,
            Err(e) => {
                return Err(anyhow::anyhow!("Error dezerializing arguements: {:#?}", value));
            }
        };
        let result = "20";
        let newvalue = serde_json::from_str(result)?;
        println!("Argements: {:#?}", newvalue);
        Ok(newvalue)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let mut tool_registry = ToolRegistry::new();
    let tool = TestTool {};
    tool_registry.register_tool(tool.clone()).await;
    let mut agent_service = AgentService::new(tool_registry);

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
    let message = Message::User{ content: content.clone(), response_id: None };

    // let message = Message::create_user_message(&content, None);

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
