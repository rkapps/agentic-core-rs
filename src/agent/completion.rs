use std::sync::Arc;

use crate::capabilities::{
    client::{completion::{CompletionStreamResponse, LlmClient}, mcp},
    completion::{
        message::Message,
        request::CompletionRequest,
        response::{CompletionResponse, CompletionResponseContent},
    },
    tools::{
        mcp::MCPRegistry,
        tool::{ToolDefinition, ToolRegistry},
    },
};
use anyhow::Result;
use tracing::debug;

#[derive(Debug)]
pub struct Agent {
    pub model: String,
    pub client: Arc<dyn LlmClient>,
    pub system: Option<String>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub tool_registry: Arc<ToolRegistry>,
    pub mcp_registry: Arc<MCPRegistry>,
}

impl Agent {
    pub fn max_tokens(&self) -> i32 {
        self.max_tokens
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    //complete defines a multi turn chat
    pub async fn complete(&self, messages: &Vec<Message>) -> Result<CompletionResponse> {
        // debug!("Completion Request: {:#?}", request);

        let request = CompletionRequest {
            model: self.model.clone(),
            system: self.system.clone(),
            messages: messages.clone(),
            temperature: 0.5,
            max_tokens: 5000,
            stream: false,
            definitions: Vec::new(),
        };

        self.client.complete(request).await
    }

    pub async fn complete_with_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionStreamResponse> {
        self.client.complete_with_stream(request).await
    }

    pub async fn complete_with_tools(&self, messages: &Vec<Message>) -> Result<CompletionResponse> {

        let mut definitions: Vec<ToolDefinition> = self
            .tool_registry
            .get_tools()
            // .cloned()
            .iter()
            .map(|e| ToolDefinition::from_tool(e.as_ref()))
            .collect();
        debug!("Tool_definitions: {:#?}", definitions);

        let mcp_definitions= self.mcp_registry.definitions.clone();
        debug!("Mcp_definitions: {:#?}", mcp_definitions);
        let _ = mcp_definitions.iter().for_each(|e| definitions.push(e.1.clone()));
        debug!("All definitions: {:#?}", definitions);

        let request = CompletionRequest {
            model: self.model.clone(),
            system: self.system.clone(),
            messages: messages.clone(),
            temperature: 0.5,
            max_tokens: 5000,
            stream: false,
            definitions: definitions
        };

        const MAX_ITERATIONS: usize = 5;
        let mut iteration = 0;

        let mut nrequest = request;
        loop {
            iteration += 1;
            if iteration > MAX_ITERATIONS {
                return Err(anyhow::anyhow!("Max tool iterations exceeded"));
            }

            debug!("CompletionRequest: {:#?}", nrequest);

            let mut nmessages = Vec::new();
            let response = self.client.complete(nrequest.clone()).await?;
            debug!("CompletionResponse: {:#?}", response);

            // Check if response has tool calls
            let has_tool_calls = response
                .contents
                .iter()
                .any(|c| matches!(c, CompletionResponseContent::ToolCall { .. }));

            if !has_tool_calls {
                return Ok(response); // Done - return final answer
            }

            for content in response.contents {
                match content {
                    CompletionResponseContent::Thought(text) => {
                        debug!("Text: {}", text);
                        let message = Message::Thought { content: text };
                        nmessages.push(message);
                    }

                    CompletionResponseContent::Text(text) => {
                        debug!("Text: {}", text);
                    }
                    CompletionResponseContent::ToolCall(tool_call_request) => {
                        let tool_option = self.tool_registry.get_tool(&tool_call_request.name);
                        debug!("Tool Option: {:#?}", tool_option);
                        let mut tool_found = false;
                        match tool_option {
                            Some(tool) => {
                                let nmessage = Message::ToolCall {
                                    call_id: tool_call_request.id.clone(),
                                    arguments: tool_call_request.arguments.to_string(),
                                    name: tool_call_request.name.clone(),
                                };
                                nmessages.push(nmessage);

                                let response =
                                    tool.execute(tool_call_request.arguments.clone()).await?;
                                let nmessage = Message::ToolOutput {
                                    call_id: tool_call_request.id.clone(),
                                    output: response,
                                    name: tool_call_request.name.clone(),
                                };
                                nmessages.push(nmessage);
                                tool_found = true
                            }
                            None => {}
                        }

                        if !tool_found {
                            // Call the mcp tool
                            debug!("Mcp tool_call: {:#?}", &tool_call_request.name);
                            let response = self
                                .mcp_registry
                                .call_tool(
                                    &tool_call_request.name,
                                    tool_call_request.arguments.clone(),
                                )
                                .await?;
                            debug!("Mcp tool_call Value: {:#?}", response);

                            let nmessage = Message::ToolCall {
                                call_id: tool_call_request.id.clone(),
                                arguments: tool_call_request.arguments.to_string(),
                                name: tool_call_request.name.clone(),
                            };
                            nmessages.push(nmessage);

                            let nmessage = Message::ToolOutput {
                                call_id: tool_call_request.id.clone(),
                                output: response,
                                name: tool_call_request.name.clone(),
                            };
                            nmessages.push(nmessage);
                        }
                    }
                }
            }

            // If there are toolcall and result messages add them to the next call
            if nmessages.len() > 0 {
                nrequest.messages.extend(nmessages);
            }
        }
    }
}
