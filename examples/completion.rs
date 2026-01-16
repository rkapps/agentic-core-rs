use agentic_core::{
    agent::service::AgentService,
    capabilities::{
        client::tool::ToolRegistry,
        completion::{message::Message, request::CompletionRequest},
    }, providers::{anthropic, gemini},
};
use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    tracing_subscriber::fmt()
        .compact()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    let tool_registry = ToolRegistry::new();
    let mut agent_service = AgentService::new(tool_registry);

    // let api_key =
    //     env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");
    // let llm = gemini::completion::LLM;
    // let model = gemini::completion::MODEL_GEMINI_3_FLASH_PREVIEW;

    let api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");
    let llm = anthropic::completion::LLM;
    let model = anthropic::completion::MODEL_CLAUDE_SONNET_4_5;


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
    let agent = agent_service.get_chat_agent(llm)?;
    let mut response = agent.complete(request).await?;
    println!("Response: {:#?}", response);

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

    response = agent.complete(request).await?;
    println!("Response: {:#?}", response);

    Ok(())
}
