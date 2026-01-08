use serde::{Deserialize, Serialize};

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
    pub signature: String,
    pub text: String,
    pub r#type: String
}

impl GeminiInteractionsRequest {

    pub fn new(request:CompletionRequest)  -> Self{

        let mut input = String::new();
        let mut id: Option<String> = None;

        if let Some(message) = request.messages.last() {
            input = message.clone().content.to_string();
            id = message.response_id.clone();
        }


        let grequest = GeminiInteractionsRequest{
            model: MODEL_GEMINI_3_FLASH_PREVIEW.to_string(),
            input: input,
            system_instruction: request.system.unwrap(),
            previous_interaction_id: id,
            stream: false
        };

        grequest

    }
}