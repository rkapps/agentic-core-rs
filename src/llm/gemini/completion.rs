use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::capabilities::{completion::CompletionRequest, messages::MessageRole};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiCompletionRequest {
    #[serde(rename = "system_instruction")] // Override to keep snake_case
    system_instruction: GeminiCompletionRequestSystemInstruction,
    contents: Vec<GeminiCompletionRequestContent>,
    generation_config: GeminiCompletionConfig,
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
pub struct GeminiCompletionConfig {
    temperature: f32,
    max_output_tokens: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiResponseCandidate>,
    // model_version: String,
    // response_id: String
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponseCandidate {
    pub content: GeminiResponseContent,
    // finish_reason: String,
    // index: i32
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseContent {
    pub parts: Vec<GeminiResponseContentPart>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseContentPart {
    pub text: String,
}

impl GeminiCompletionRequest {
    pub fn new(request: CompletionRequest) -> GeminiCompletionRequest {
        let mut contents: Vec<GeminiCompletionRequestContent> = Vec::new();
        let mut parts: Vec<GeminiCompletionRequestPart> = Vec::new();

        if request.system.is_some() {
            let part: GeminiCompletionRequestPart = GeminiCompletionRequestPart {
                text: request.system.unwrap(),
            };
            parts.push(part);
        }
        let sinstruction = GeminiCompletionRequestSystemInstruction { parts: parts };

        for message in request.messages {
            let mut parts: Vec<GeminiCompletionRequestPart> = Vec::new();
            let part: GeminiCompletionRequestPart = GeminiCompletionRequestPart {
                text: message.content,
            };
            parts.push(part);
            let role = MessageRole::as_str(&message.role);
            let nrole = role.replace("assistant", "model");
            let content = GeminiCompletionRequestContent {
                role: nrole.to_string(),
                parts: parts,
            };
            contents.push(content);
        }
        let config = GeminiCompletionConfig {
            temperature: request.temperature,
            max_output_tokens: request.max_tokens,
        };

        let request = GeminiCompletionRequest {
            system_instruction: sinstruction,
            contents: contents,
            generation_config: config,
        };
        debug!("GeminiCompletionRequest {:#?}", request);
        request
    }
}
