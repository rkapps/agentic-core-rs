use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct AnthropicCompletionResponse {
    pub model: String,
    pub role: String,
    pub content: Vec<AnthropicCompletionResponseContent>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicCompletionResponseContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnthropicChunkResponse {
    pub r#type: String,
    pub index: Option<i32>,
    pub delta: Option<AnthropicChunkResponseDelta>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnthropicChunkResponseContentBlock {
    pub r#type: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnthropicChunkResponseDelta {
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub thinking: Option<String>,
}

