use serde::Serialize;
use anyhow::{Context, Result};

use crate::capabilities::completion::{
    message::Message, request::CompletionRequest, tool::ToolDefinition,
};

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequest {
    model: String,
    instructions: String,
    // input: String,
    input: Vec<OpenAICompletionRequestMessage>,
    store: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    max_output_tokens: i32,
    reasoning: OpenAICompletionRequestReasoning,
    pub tools: Vec<ToolDefinition>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)] // Removes the variant wrapper
pub enum OpenAICompletionRequestMessage {
    Content {
        role: String,
        content: String,
    },
    FunctionCall {
        r#type: String,
        arguments: String,
        call_id: String,
        name: String,
    },
    FunctionCallOutput {
        r#type: String,
        call_id: String,
        output: String,
    },
}

#[derive(Serialize, Debug)]
pub struct OpenAICompletionRequestReasoning {
    effort: String,
}

impl OpenAICompletionRequest {
    pub fn new(request: CompletionRequest) -> Result<Self> {
        let mut id: Option<String> = None;

        let mut inputs = Vec::new();
        for message in request.messages {
            match message {
                Message::Thought { content: _ } => {}
                Message::User {
                    content,
                    response_id: _,
                } => {
                    inputs.push(OpenAICompletionRequestMessage::Content {
                        role: "user".to_string(),
                        content,
                    });
                }
                Message::Assistant {
                    content,
                    response_id,
                } => {
                    id = response_id;
                    inputs.push(OpenAICompletionRequestMessage::Content {
                        role: "assistant".to_string(),
                        content,
                    });
                }

                Message::ToolCall {
                    arguments,
                    call_id,
                    name,
                } => {
                    inputs.push(OpenAICompletionRequestMessage::FunctionCall {
                        r#type: "function_call".to_string(),
                        arguments,
                        call_id,
                        name: name,
                    });
                }
                Message::ToolOutput {
                    call_id,
                    output,
                    name: _,
                } => {

                    let arg_string = serde_json::to_string(&output)
                        .context("Failed to serialize arguments for OpenAI")?;

                    inputs.push(OpenAICompletionRequestMessage::FunctionCallOutput {
                        r#type: "function_call_output".to_string(),
                        call_id,
                        output: arg_string,
                    });
                }
            }
        }

        Ok(Self {
            model: request.model,
            instructions: request.system.unwrap_or(String::new()),
            input: inputs,
            store: true,
            stream: request.stream,
            previous_response_id: id,
            max_output_tokens: request.max_tokens,
            reasoning: OpenAICompletionRequestReasoning {
                effort: String::from("low"),
            },
            tools: request.definitions,
        })
    }
}
