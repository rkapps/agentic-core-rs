use crate::capabilities::{completion::message::Message, tools::tool::ToolDefinition};

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub model: String,
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub definitions: Vec<ToolDefinition>,
}
