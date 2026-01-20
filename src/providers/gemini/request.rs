use serde::Serialize;
use serde_json::Value;
use anyhow::{Context, Result};

use crate::{
    capabilities::{completion::{
        message::Message, request::CompletionRequest,
    }, tools::tool::ToolDefinition},
    providers::gemini::completion::MODEL_GEMINI_3_FLASH_PREVIEW,
};

#[derive(Debug, Serialize)]
pub struct GeminiInteractionsRequest {
    model: String,
    input: Vec<GeminiCompletionRequestInput>,

    #[serde(skip_serializing_if = "Option::is_none")]
    previous_interaction_id: Option<String>,
    system_instruction: String,
    stream: bool,
    pub tools: Vec<ToolDefinition>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)] // Removes the variant wrapper
pub enum GeminiCompletionRequestInput {
    Content {
        role: String,
        content: String,
    },
    Thought {
        role:String,
        r#type: String,
        // thought_signature: GeminiCompletionRequestThoughtSignature,
        thought_signature: String,
        content: String
    },

    FunctionCall {
        role: String,
        content: Vec<GeminiCompletionRequestFunctionCall>
    },
    FunctionCallResult {
        role: String,
        content: Vec<GeminiCompletionRequestFunctionResult>
    },
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestThoughtSignature {
    signature: String,
}


#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestFunctionCall {
    r#type: String,
    arguments: Value,
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestFunctionResult {
    r#type: String,
    call_id: String,
    result: String,
    name: String
}



impl GeminiInteractionsRequest {
    pub fn new(request: CompletionRequest) -> Result<Self> {
        let mut id: Option<String> = None;

        let mut inputs = Vec::new();
        let mut function_result_contents = Vec::new();
        let mut function_call_contents = Vec::new();

        for message in request.messages {
            match message {

                Message::Thought {content:_} => {
                    // let input = GeminiCompletionRequestInput::Thought { 
                    //     role: "model".to_string(),
                    //     r#type: "thought_signature".to_string(),
                    //     // thought_signature: GeminiCompletionRequestThoughtSignature { signature: content.clone() },
                    //     thought_signature: content,
                    //     content: "what is the weather in paris and San Fransicso".to_string()
                    // };
                    // inputs.push(input);
                }

                Message::User {
                    content,
                    response_id: _,
                } => {
                    let input = GeminiCompletionRequestInput::Content {
                        role: "user".to_string(),
                        content: content,
                    };
                    inputs.push(input);
                }
                Message::Assistant {
                    content:_,
                    response_id,
                } => {
                    id = response_id;
                }

                Message::ToolCall {
                    arguments,
                    call_id,
                    name,
                } => {

                    let value = serde_json::from_str(&arguments)
                            .context("Failed to serialize arguments for Gemini")?;

                    let input = GeminiCompletionRequestFunctionCall {
                        arguments: value,
                        id: call_id,    
                        name: name,                    
                        r#type: "function_call".to_string(),
                    };
                    function_call_contents.push(input);

                }
                Message::ToolOutput { call_id, output, name } => {

                    let arg_string = serde_json::to_string(&output)
                        .context("Failed to serialize arguments for Gemini")?;

                    let input = GeminiCompletionRequestFunctionResult {
                        result: arg_string,
                        call_id: call_id,    
                        name: name,
                        r#type: "function_result".to_string(),
                    };
                    function_result_contents.push(input);
                }
            }
        }

        if function_call_contents.len() > 0 {
            let input = GeminiCompletionRequestInput::FunctionCall { 
                role: "model".to_string(),
                content: function_call_contents,
            };
            inputs.push(input);
        }

        if function_result_contents.len() > 0 {
            let input = GeminiCompletionRequestInput::FunctionCallResult { 
                role: "user".to_string(),
                content: function_result_contents,
            };
            inputs.push(input);
        }
        
        let grequest = GeminiInteractionsRequest {
            model: MODEL_GEMINI_3_FLASH_PREVIEW.to_string(),
            input: inputs,
            system_instruction: request.system.unwrap_or(String::new()),
            previous_interaction_id: id,
            stream: request.stream,
            tools: request.definitions,
        };

        Ok(grequest)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiCompletionRequest {
    #[serde(rename = "system_instruction")] // Override to keep snake_case
    system_instruction: GeminiCompletionRequestSystemInstruction,
    contents: Vec<GeminiCompletionRequestContent>,
    generation_config: GeminiCompletionRequestConfig,
    stream: bool,
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestSystemInstruction {
    parts: Vec<GeminiCompletionRequestPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestContent {
    role: String,
    parts: Vec<GeminiCompletionRequestPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestPart {
    text: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiCompletionRequestConfig {
    temperature: f32,
    max_output_tokens: i32,
}

// impl GeminiCompletionRequest {
//     pub fn new(request: CompletionRequest) -> GeminiCompletionRequest {
//         let mut contents: Vec<GeminiCompletionRequestContent> = Vec::new();
//         let mut parts: Vec<GeminiCompletionRequestPart> = Vec::new();

//         if request.system.is_some() {
//             let part: GeminiCompletionRequestPart = GeminiCompletionRequestPart {
//                 text: request.system.unwrap(),
//             };
//             parts.push(part);
//         }
//         let sinstruction = GeminiCompletionRequestSystemInstruction { parts: parts };

//         for message in request.messages {
//             let mut parts: Vec<GeminiCompletionRequestPart> = Vec::new();
//             let part: GeminiCompletionRequestPart = GeminiCompletionRequestPart {
//                 text: message.content,
//             };
//             parts.push(part);
//             let role = MessageRole::as_str(&message.role);
//             let nrole = role.replace("assistant", "model");
//             let content = GeminiCompletionRequestContent {
//                 role: nrole.to_string(),
//                 parts: parts,
//             };
//             contents.push(content);
//         }
//         let config = GeminiCompletionRequestConfig {
//             temperature: request.temperature,
//             max_output_tokens: request.max_tokens,
//         };

//         let request = GeminiCompletionRequest {
//             system_instruction: sinstruction,
//             contents: contents,
//             generation_config: config,
//             stream: request.stream,
//         };
//         debug!("GeminiCompletionRequest {:#?}", request);

//         request
//     }
// }
