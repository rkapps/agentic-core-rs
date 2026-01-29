use serde::Deserialize;
use serde_json::Value;



#[derive(Debug, Deserialize)]
pub (super) struct MCPToolListResponse {
    pub (super) content: Vec<MCPToolListResponseContent>,
}
#[derive(Debug, Deserialize, Clone)]
pub (super) struct MCPToolListResponseContent {
    #[allow(dead_code)]
    r#type: String,
    pub (super) text: String,
}

#[derive(Debug, Deserialize)]
pub (super) struct MCPToolListDefinition {
    pub (super) name: String,
    pub (super) description: String,
}


#[derive(Debug, Deserialize)]
pub (super) struct MCPToolGetDefinition {
    #[allow(dead_code)]
    name: String,
    pub (super)  description: String,
    pub (super) parameters: Value,
}



#[derive(Debug, Deserialize)]
pub (super) struct MCPToolCallResponse {
    pub (super) content: Vec<MCPToolCallResponseContent>,
}

#[derive(Debug, Deserialize, Clone)]
pub (super) struct MCPToolCallResponseContent {
    #[allow(dead_code)]
    r#type: String,
    pub (super) text: String,
}

