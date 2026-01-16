use serde::Serialize;
use serde_json::Value;
use anyhow::{Context, Result};
use tracing::debug;


use crate::capabilities::completion::{message::Message, request::CompletionRequest};

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequest {
    model: String,
    max_tokens: i32,
    temperature: f32,
    messages: Vec<AnthropicCompletionRequestMessage>,
    system: Option<String>,
    stream: bool,
    pub tools: Vec<AnthropicToolDefinition>,
}

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequestSystem {
    r#type: String,
    text: String,
}

#[derive(Serialize, Debug)]
#[serde(untagged)] // Removes the variant wrapper
pub enum AnthropicCompletionRequestMessage {
    Content {
        role: String,
        content: String,
    },
    ToolUse {
        role: String,
        content: Vec<AnthropicCompletionRequestToolUse>,
    },
    ToolResult {
        role: String,
        content: Vec<AnthropicCompletionRequestToolResult>,
    },
}

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequestToolUse {
    r#type: String,
    input: Value,
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicCompletionRequestToolResult {
    r#type: String,
    tool_use_id: String,
    content: String,
}

#[derive(Debug, Serialize)]
pub struct AnthropicToolDefinition {
    name: String,
    description: String,
    input_schema: serde_json::Value,
}

impl AnthropicCompletionRequest {
    pub fn new(request: CompletionRequest) -> Result<AnthropicCompletionRequest> {
        let mut messages: Vec<AnthropicCompletionRequestMessage> = Vec::new();

        let mut tool_result_contents = Vec::new();
        let mut tool_use_contents = Vec::new();

        for message in request.messages {
            match message {
                Message::Thought { content:_ } => {}
                Message::User {
                    content,
                    response_id: _,
                } => {
                    messages.push(AnthropicCompletionRequestMessage::Content {
                        role: "user".to_string(),
                        content: content,
                    });
                    // messages.push(amessage);
                }
                Message::Assistant {
                    content,
                    response_id: _,
                } => {
                    messages.push(AnthropicCompletionRequestMessage::Content {
                        role: "assistant".to_string(),
                        content: content,
                    });
                }
                Message::ToolCall {
                    arguments,
                    call_id,
                    name,
                } => {

                    let value = serde_json::from_str(&arguments)
                        .context("Failed to serialize arguments for OpenAI")?;

                    let content = AnthropicCompletionRequestToolUse {
                        r#type: "tool_use".to_string(),
                        input: value,
                        id: call_id,
                        name: name,
                    };
                    tool_use_contents.push(content);
                }
                Message::ToolOutput {
                    call_id,
                    output,
                    name: _,
                } => {

                    let arg_string = serde_json::to_string(&output)
                        .context("Failed to serialize arguments for Anthropic")?;

                    let content = AnthropicCompletionRequestToolResult {
                        r#type: "tool_result".to_string(),
                        content: arg_string,
                        tool_use_id: call_id,
                    };
                    tool_result_contents.push(content);
                }
            }
        }

        if tool_use_contents.len() > 0 {
            messages.push(AnthropicCompletionRequestMessage::ToolUse {
                role: "assistant".to_string(),
                content: tool_use_contents,
            });
        }

        if tool_result_contents.len() > 0 {
            messages.push(AnthropicCompletionRequestMessage::ToolResult {
                role: "user".to_string(),
                content: tool_result_contents,
            });
        }

        let mut atools = Vec::new();
        for tool in request.definitions {
            let atool = AnthropicToolDefinition {
                name: tool.name,
                description: tool.description,
                input_schema: tool.parameters,
            };
            atools.push(atool);
        }
        let arequest = AnthropicCompletionRequest {
            max_tokens: request.max_tokens,
            messages: messages,
            model: request.model,
            system: request.system,
            temperature: request.temperature,
            stream: request.stream,
            tools: atools,
        };

        debug!("AnthropicCompletionRequest {:#?}", arequest);
        Ok(arequest)
    }
}
