use serde::Deserialize;

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
