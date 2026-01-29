use agentic_core::{
    agent::service::AgentService,
    capabilities::completion::{message::Message, request::CompletionRequest, response::CompletionResponseContent},
};
use anyhow::Result;
use std::env;
use tracing::Level;
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let filter =
        filter::Targets::new().with_target("agentic_core::providers::gemini", Level::DEBUG);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty()) // Compact format
        .with(filter)
        .init();

    let api_key =
        env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY environment variable not set");

    let agent_service = AgentService::new();
    let agent = agent_service.builder().with_anthropic(&api_key)?.build()?;

    let mut messages = vec![];
    let content = "You are an elementary quiz coordinator. Design a multiple choise quiz after asking them about the grade, subject and difficult level. Provide 20 questions and rate them at the end.".to_string();
    let mut message = Message::User {
        content: content.clone(),
        response_id: None,
    };
    messages.push(message);

    // Create agent
    let response = agent.complete(&messages).await?;
    let response_id = response.response_id;// let aresponse = response.clone();
    let content = response.contents.get(0).unwrap();
    if let CompletionResponseContent::Text(val) = content {
        message = Message::Assistant { content: val.to_string(), response_id: Some(response_id.clone())};
        messages.push(message);
    }
    //create turn message using the response id
    message = Message::User {
        content: "1st Grade".to_string(),
        response_id: Some(response_id),
    };
    messages.push(message);

    let _ = agent.complete(&messages).await?;

    Ok(())
}
