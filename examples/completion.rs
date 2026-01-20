use agentic_core::{
    agent::service::AgentService,
    capabilities::{completion::{message::Message, request::CompletionRequest}, tools::tool::ToolRegistry}, providers::{gemini, openai},
};
use anyhow::Result;
use tracing::{Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    let filter = filter::Targets::new()
        .with_target("agentic_core::providers::gemini", Level::DEBUG);

     tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty())  // Compact format
        .with(filter)
        .init();    
    
    let tool_registry = ToolRegistry::new();
    let mut agent_service = AgentService::new();
    agent_service.register_tool(tool_registry);

    let api_key =
        env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    let llm = gemini::completion::LLM;
    let model = gemini::completion::MODEL_GEMINI_3_FLASH_PREVIEW;

    // let api_key =
    //     env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    // let llm = openai::completion::LLM;
    // let model = openai::completion::MODEL_GPT_5_NANO;

    // let api_key =
    //     env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");
    // let llm = anthropic::completion::LLM;
    // let model = anthropic::completion::MODEL_CLAUDE_SONNET_4_5;


    let _ = agent_service
        .register_client(llm, &api_key)
        .unwrap();

    let mut content = "You are an elementary quiz coordinator. Design a multiple choise quiz after asking them about the grade, subject and difficult level. Provide 20 questions and rate them at the end.".to_string();
    let mut message = Message::User{ content: content.clone(), response_id: None };

    let mut request = CompletionRequest {
        model: model.to_string(),
        system: Some(content),
        messages: vec![message],
        temperature: 0.5,
        max_tokens: 5000,
        stream: false,
        definitions: Vec::new(),
    };

    // Create agent
    let agent = agent_service.get_completion_agent(llm)?;
    let mut response = agent.complete(request).await?;

    //create turn message using the response id
    content = "1st Grade".to_string();
    message = Message::User{ content: content.clone(), response_id: Some(response.response_id) };

    //Create turn request
    request = CompletionRequest {
        model: model.to_string(),
        system: Some(content),
        messages: vec![message],
        temperature: 0.5,
        max_tokens: 5000,
        stream: false,
        definitions: Vec::new(),
    };

    let _ = agent.complete(request).await?;

    Ok(())
}
