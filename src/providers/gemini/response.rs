use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsResponse {
    pub id: String,
    pub outputs: Vec<GeminiInteractionsResponseOutput>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GeminiInteractionsResponseOutput {
    #[serde(rename = "text")]
    Text { text: String },

    #[serde(rename = "thought")]
    Thought { signature: String },

    #[serde(rename = "function_call")]
    FunctionCall {
        id: String,
        arguments: Value,
        name: String,
    },
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponse {
    pub event_type: String,
    pub delta: Option<GeminiInteractionsChunkResponseDelta>,
    pub interaction: Option<GeminiInteractionsChunkResponseInteraction>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponseDelta {
    pub r#type: String,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiInteractionsChunkResponseInteraction {
    pub id: String,
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


#[derive(Debug, Deserialize, Clone)]
pub (super) struct GeminiEmbeddingsResponse {
    pub (super) embedding: GeminiEmbeddingsResponseEmbedding
}

#[derive(Debug, Deserialize, Clone)]
pub (super) struct GeminiEmbeddingsResponseEmbedding {
    pub (super) values: Vec<f32>
}
