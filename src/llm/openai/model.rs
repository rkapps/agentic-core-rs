use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::capabilities::model::CompletionRequest;

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequest {
    model: String,
    instructions: String,
    input: String,
    store: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    // top_p: f32,
    max_output_tokens: i32,
    reasoning: OpenAICompletionRequestReasoning,
}

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequestReasoning {
    effort: String
}


#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub output: Vec<OpenAICompletionResponseOutput>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponseOutput {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub content: Vec<OpenAICompletionResponseContent>,
    #[serde(default)]
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponseContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct OpentAIChunkResponse {
    pub event: String,
    pub data : Option<OpenAIChunkResponseData>
}


#[derive(Debug, Deserialize)]
pub struct OpenAIChunkResponseData {
    pub r#type: String,
    pub delta: Option<String>,
}


impl OpenAICompletionRequest {
    pub fn new(request: CompletionRequest) -> Self {
        let mut input: String = String::new();
        let mut response_id: Option<String> = None;

        for message in request.messages {
            input = message.content;
            if message.role.as_str() == "assistant" {
                response_id = message.response_id;
            }
        }
        // debug!("Input: {:#?} Response Id {:#?}", input, response_id.clone());

        Self {
            model: request.model,
            instructions: request.system.unwrap_or(String::new()),
            input: input,
            store: false,
            stream: request.stream,
            previous_response_id: response_id,
            max_output_tokens: request.max_tokens,
            reasoning: OpenAICompletionRequestReasoning { effort: String::from("low") }
        }
    }
}
