use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub output: Vec<OpenAICompletionResponseOutput>,
}

#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponseOutput {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    pub content: Vec<OpenAICompletionResponseContent>,
    #[serde(default)]
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponseContent {
    pub r#type: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct OpentAIChunkResponse {
    pub event: String,
    pub data : Option<OpenAIChunkResponseData>
}


#[derive(Debug, Deserialize)]
pub struct OpenAIChunkResponseData {
    pub r#type: String,
    pub delta: Option<String>,
}

