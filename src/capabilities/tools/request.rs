use serde::Serialize;
use serde_json::Value;


#[derive(Debug, Clone)]
pub struct ToolCallRequest {
    pub name: String,
    pub id: String,
    pub arguments: Value
}


#[derive(Debug, Serialize)]
pub (super) struct MCPToolListRequest {}


#[derive(Debug, Serialize)]
pub (super) struct MCPToolGetParamsRequest {
    pub (super) tool_name: String,
}

#[derive(Debug, Serialize)]
pub (super) struct MCPToolCallParamsRequest {
    pub (super) tool_name: String,
    pub (super) arguments: Value,
}


