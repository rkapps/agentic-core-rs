use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::{
    capabilities::{model::CompletionRequest, messages::MessageRole},
    llm::gemini::client::MODEL_GEMINI_3_FLASH_PREVIEW,
};

#[derive(Debug, Serialize)]
pub struct GeminiInteractionsRequest {
    model: String,
    input: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    previous_interaction_id: Option<String>,
    system_instruction: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsResponse {
    pub id: String,
    pub outputs: Vec<GeminiInteractionsResponseOutput>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsResponseOutput {
    #[serde(skip_serializing)]
    pub signature: Option<String>,
    pub text: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponse {
    pub event_type: String,
    pub delta: Option<GeminiInteractionsChunkResponseDelta>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponseDelta {
    pub r#type: String,
    pub text: Option<String>,
}

impl GeminiInteractionsRequest {
    pub fn new(request: CompletionRequest) -> Self {
        let mut input = String::new();
        let mut id: Option<String> = None;
        // let system: String = request.system.filter(|_| request.messages.len() == 0).unwrap_or_default();

        // if request.messages.len() > 0 {
        //     system = None;
        // }
        // if let Some(message) = request.messages.last() {
        //     input = message.clone().content.to_string();
        //     id = message.response_id.clone();
        // }

        // Get only the last message for the interactions api.
        // Set the id for the last id for the assistant.
        for message in request.messages {
            input = message.content;
            if message.role.as_str() == "assistant" {
                id = message.response_id;
            }
        }
        // debug!("system: {:#?}", system);

        let grequest = GeminiInteractionsRequest {
            model: MODEL_GEMINI_3_FLASH_PREVIEW.to_string(),
            input: input,
            system_instruction: request.system.unwrap_or(String::new()),
            previous_interaction_id: id,
            stream: request.stream,
        };

        grequest
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiCompletionRequest {
    #[serde(rename = "system_instruction")] // Override to keep snake_case
    system_instruction: GeminiCompletionRequestSystemInstruction,
    contents: Vec<GeminiCompletionRequestContent>,
    generation_config: GeminiCompletionConfig,
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
            stream: request.stream,
        };
        debug!("GeminiCompletionRequest {:#?}", request);

        request
    }
}
