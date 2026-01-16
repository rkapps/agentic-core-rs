use serde::Deserialize;



#[derive(Deserialize, Debug)]
pub struct OpenAICompletionResponse {
    pub id: String,
    pub output: Vec<OpenAICompletionResponseOutput>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum OpenAICompletionResponseOutput {

    #[serde(rename = "message")]
    Message {
        id: String,
        status: String,
        content: Vec<OpenAICompletionResponseContent>
    },

    #[serde(rename = "function_call")]
    FunctionCall {
        status: String,
        arguements: String,
        call_id: String,
        name: String
    },

    #[serde(rename = "reasoning")]
    Reasoning {
        id: String,
        summary: Vec<String>,
    },

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
    pub response: Option<OpenAIChunkResponseDataResponse>,
    pub delta: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChunkResponseDataResponse {
    pub id: String
}
