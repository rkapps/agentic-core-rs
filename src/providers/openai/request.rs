use serde::Serialize;

use crate::capabilities::completion::request::CompletionRequest;

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequest {
    model: String,
    instructions: String,
    input: String,
    store: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    max_output_tokens: i32,
    reasoning: OpenAICompletionRequestReasoning,
}

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequestReasoning {
    effort: String
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
            store: true,
            stream: request.stream,
            previous_response_id: response_id,
            max_output_tokens: request.max_tokens,
            reasoning: OpenAICompletionRequestReasoning { effort: String::from("low") }
        }
    }
}
