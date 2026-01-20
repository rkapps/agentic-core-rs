use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use crate::capabilities::client::tool::Tool;



#[derive(Debug, Clone)]
pub struct ToolRegistry {
    pub registry: Arc<RwLock<HashMap<String, Arc<dyn Tool>>>>,
}


impl ToolRegistry {
    pub fn new() -> ToolRegistry{
        Self{
            registry: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub async fn register_tool<T: Tool + 'static>(&mut self, tool: T) {
        let mut guard = self.registry.write().await;
        guard.insert(tool.name(), Arc::new(tool));
    }

    pub async fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>>{
        let guard = self.registry.read().await;
        guard.get(name).cloned()
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn default_for_mcp(r#type: &str, name: &str, description: &str, parameters: Value) -> Self{
        Self {
            r#type: r#type.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            parameters: parameters
        }
    }

}


#[derive(Debug, Clone)]
pub struct ToolCallRequest {
    pub name: String,
    pub id: String,
    pub arguments: Value
}

