use serde::Serialize;
use serde_json::Value;
use crate::capabilities::client::tool::Tool;

#[derive(Serialize, Debug, Clone)]
pub struct ToolDefinition {
    r#type: String,
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

impl ToolDefinition {
    pub fn new<T: Tool + 'static>(tool: T) -> Self {
        Self {
            r#type: "function".to_string(),
            name: tool.name(),
            description: tool.description(),
            parameters:  tool.parameters()
        }
    }
}


#[derive(Debug, Clone)]
pub struct ToolCallRequest {
    pub name: String,
    pub id: String,
    pub arguements: Value
}

