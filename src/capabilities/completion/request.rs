use crate::capabilities::completion::message::Message;

#[derive(Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
}
