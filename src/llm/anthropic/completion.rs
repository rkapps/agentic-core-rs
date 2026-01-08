use serde::{Serialize, Deserialize};
use tracing::debug;

use crate::capabilities::{completion::CompletionRequest, messages::MessageRole};


#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequest {
    model: String,
    max_tokens: i32,
    temperature: f32,
    messages: Vec<AnthropicCompletionRequestMessage>,
    system: Vec<AnthropicCompletionRequestSystem>
}

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequestSystem {
    r#type: String,
    text: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequestMessage {
    role: String,
    content: String,
}


#[derive(Debug, Deserialize)]
pub struct AnthropicCompletionResponse {
    pub model: String,
    pub role: String,
    pub content: Vec<AnthropicCompletionResponseContent>
}

#[derive(Debug, Deserialize)]
pub struct AnthropicCompletionResponseContent {
    pub r#type: String,
    pub text: String
}

impl AnthropicCompletionRequest {

    pub fn new(request: CompletionRequest) -> AnthropicCompletionRequest{

        let mut system: Vec<AnthropicCompletionRequestSystem> = Vec::new();
        let mut messages: Vec<AnthropicCompletionRequestMessage> = Vec::new();

        if request.system.is_some() {
            system.push(AnthropicCompletionRequestSystem { r#type: "text".to_string(), text: request.system.unwrap() })
        }
        for message in request.messages {

            let role = MessageRole::as_str(&message.role);
            // let nrole = role.replace("assistant", "model");

            let amessage = AnthropicCompletionRequestMessage{role: role.to_string(), content: message.content.to_string()};
            messages.push(amessage);
        }

        let arequest = AnthropicCompletionRequest{
            max_tokens: request.max_tokens,
            messages: messages,
            model: request.model,
            system : system,
            temperature: request.temperature,
        };

        debug!("AnthropicCompletionRequest {:#?}", arequest);
        arequest
    }
}