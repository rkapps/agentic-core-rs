use serde::{Deserialize, Serialize};
use tracing::debug;
use crate::{capabilities::completion::CompletionRequest, llm::gemini::client::MODEL_GEMINI_3_FLASH_PREVIEW};


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
    pub r#type: String
}


#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponse {
    pub event_type: String,
    pub delta : Option<GeminiInteractionsChunkResponseDelta>
}


#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponseDelta {
    pub r#type: String,
    pub text: Option<String>,
}


impl GeminiInteractionsRequest {

    pub fn new(request:CompletionRequest)  -> Self{

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

        let grequest = GeminiInteractionsRequest{
            model: MODEL_GEMINI_3_FLASH_PREVIEW.to_string(),
            input: input,
            system_instruction: request.system.unwrap_or(String::new()),
            previous_interaction_id: id,
            stream: request.stream
        };

        grequest

    }
}