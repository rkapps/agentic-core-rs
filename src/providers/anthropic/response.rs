use serde::Deserialize;
use serde_json::Value;



#[derive(Debug, Deserialize)]
pub struct AnthropicCompletionResponse {
    pub model: String,
    pub role: String,
    pub content: Vec<AnthropicCompletionResponseContent>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum AnthropicCompletionResponseContent {

    #[serde(rename = "text")]
    Text {
        text: String
    },

    #[serde(rename = "tool_use")]
    ToolUse {
        id: String,
        input: Value,
        name: String
    },
    
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

