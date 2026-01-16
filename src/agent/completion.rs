use std::sync::Arc;

use crate::capabilities::{
    client::{
        completion::{CompletionStreamResponse, LlmClient},
        tool::ToolRegistry,
    },
    completion::{
        message::Message,
        request::CompletionRequest,
        response::{CompletionResponse, CompletionResponseContent},
    },
};
use anyhow::Result;
use tracing::debug;

#[derive(Debug)]
pub struct Agent {
    pub client: Arc<dyn LlmClient>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub tool_registry: ToolRegistry,
}

impl Agent {
    pub fn max_tokens(&self) -> i32 {
        self.max_tokens
    }

    pub fn temperature(&self) -> f32 {
        self.temperature
    }

    //complete defines a multi turn chat
    pub async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        debug!("Completion Request: {:#?}", request);
        // if request.
        self.client.complete(request).await
    }

    pub async fn complete_with_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionStreamResponse> {
        self.client.complete_with_stream(request).await
    }

    pub async fn complete_with_tools(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        const MAX_ITERATIONS: usize = 10;
        let mut iteration = 0;

        let mut nrequest = request;
        loop {
            iteration += 1;
            if iteration > MAX_ITERATIONS {
                return Err(anyhow::anyhow!("Max tool iterations exceeded"));
            }

            let mut nmessages = Vec::new();
            let response = self.client.complete(nrequest.clone()).await?;

            // Check if response has tool calls
            let has_tool_calls = response
                .contents
                .iter()
                .any(|c| matches!(c, CompletionResponseContent::ToolCall { .. }));

            if !has_tool_calls {
                return Ok(response); // Done - return final answer
            }

            for content in response.contents {
                // debug!("Response Content: {:#?}", content);
                match content {
                    CompletionResponseContent::Thought(text) => {
                        debug!("Text: {}", text);
                        // let message = Message::User { content: text, response_id: None };
                        let message = Message::Thought { content: text };
                        nmessages.push(message);
                    }

                    CompletionResponseContent::Text(text) => {
                        debug!("Text: {}", text);
                        // response
                    }
                    CompletionResponseContent::ToolCall(tool_call_request) => {
                        let tool_option =
                            self.tool_registry.get_tool(&tool_call_request.name).await;
                        match tool_option {
                            Some(tool) => {
                                let nmessage = Message::ToolCall {
                                    call_id: tool_call_request.id.clone(),
                                    arguments: tool_call_request.arguements.to_string(),
                                    name: tool_call_request.name.clone(),
                                };
                                nmessages.push(nmessage);

                                let response =
                                    tool.execute(tool_call_request.arguements.clone()).await?;

                                let nmessage = Message::ToolOutput {
                                    call_id: tool_call_request.id.clone(),
                                    output: response,
                                    name: tool_call_request.name.clone(),
                                };
                                nmessages.push(nmessage);
                            }
                            None => {
                                debug!("Tool not available");
                            }
                        }
                    }
                }
            }

            // If there are toolcall and result messages add them to the next call
            if nmessages.len() > 0 {
                // nrequest = request.clone();
                nrequest.messages.extend(nmessages);
                debug!("New request: {:#?}", nrequest);
            }
        }

        // Ok(())
    }
}
