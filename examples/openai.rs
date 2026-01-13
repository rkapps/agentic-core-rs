use std::env;
use anyhow::Result;
use agentic_core::{
    agent::service::AgentService,
    capabilities::completion::{message::Message, request::CompletionRequest},
    providers::openai::{self, completion::MODEL_GPT_5_NANO},
};

#[tokio::main]
async fn main() -> Result<()>{

    let mut agent_service = AgentService::new();
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let _ = agent_service.set_client(openai::completion::LLM, &openai_api_key).unwrap();


    let mut content = "You are an elementary quiz coordinator. Design a multiple choise quiz after asking them about the grade, subject and difficult level. Provide 20 questions and rate them at the end.".to_string();
    let mut message = Message::create_user_message(&content, None);

    let mut request = CompletionRequest {
        model: MODEL_GPT_5_NANO.to_string(),
        system: Some(content),
        messages: vec![message],
        temperature: 0.5,
        max_tokens: 5000,
        stream: false,
    };

    // Create agent
    let agent = agent_service.get_chat_agent(openai::completion::LLM)?;
    let mut response = agent.complete(request).await?;
    println!("Response: {:#?}", response);


    //create turn message using the response id
    content = "1st Grade".to_string();
    message = Message::create_user_message(&content, Some(response.response_id.to_string()));

    //Create turn request
    request = CompletionRequest {
        model: MODEL_GPT_5_NANO.to_string(),
        system: Some(content),
        messages: vec![message],
        temperature: 0.5,
        max_tokens: 5000,
        stream: false,
    };

    response = agent.complete(request).await?;
    println!("Response: {:#?}", response);

    Ok(())

}
