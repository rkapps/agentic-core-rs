use serde::Serialize;
use tracing::debug;

use crate::capabilities::completion::{message::MessageRole, request::CompletionRequest};

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequest {
    model: String,
    max_tokens: i32,
    temperature: f32,
    messages: Vec<AnthropicCompletionRequestMessage>,
    system: Vec<AnthropicCompletionRequestSystem>,
    stream: bool,
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


impl AnthropicCompletionRequest {
    pub fn new(request: CompletionRequest) -> AnthropicCompletionRequest {
        let mut system: Vec<AnthropicCompletionRequestSystem> = Vec::new();
        let mut messages: Vec<AnthropicCompletionRequestMessage> = Vec::new();

        if request.system.is_some() {
            system.push(AnthropicCompletionRequestSystem {
                r#type: "text".to_string(),
                text: request.system.unwrap(),
            })
        }
        for message in request.messages {
            let role = MessageRole::as_str(&message.role);
            let amessage = AnthropicCompletionRequestMessage {
                role: role.to_string(),
                content: message.content.to_string(),
            };
            messages.push(amessage);
        }

        let arequest = AnthropicCompletionRequest {
            max_tokens: request.max_tokens,
            messages: messages,
            model: request.model,
            system: system,
            temperature: request.temperature,
            stream: request.stream,
        };

        debug!("AnthropicCompletionRequest {:#?}", arequest);
        arequest
    }
}
