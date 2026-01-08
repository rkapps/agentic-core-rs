use crate::capabilities::messages::Message;

#[derive(Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub system: Option<String>,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    // pub previous_response_id: Option<String>,
}

#[derive(Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub content: String,
}
