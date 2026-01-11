use serde::{Deserialize, Serialize};

use crate::capabilities::messages::Message;

#[derive(Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
}

#[derive(Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct ChatResponseChunk {
    pub content: String,
    pub thinking: String,
    pub is_final: bool,
}

impl ChatResponseChunk {
    
    pub fn default() -> ChatResponseChunk {
        ChatResponseChunk {
            content: String::new(),
            thinking: String::new(),
            is_final: false,
        }
    }

    pub fn stop() -> ChatResponseChunk {
        ChatResponseChunk {
            content: String::new(),
            thinking: String::new(),
            is_final: true,
        }
    }

    pub fn content(content: String, thinking: String) -> ChatResponseChunk {
        ChatResponseChunk {
            content: content,
            thinking: thinking,
            is_final: false,
        }
    }

}
