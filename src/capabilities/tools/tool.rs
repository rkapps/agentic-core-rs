use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capabilities::client::tool::Tool;

#[derive(Debug, Clone)]
pub struct ToolRegistry {
    pub registry: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> ToolRegistry {
        Self {
            registry: HashMap::new(),
        }
    }

    pub fn register_tool<T: Tool + 'static>(&mut self, tool: T) {
        // let mut guard = self.registry.write().await;
        self.registry.insert(tool.name(), Arc::new(tool));
    }

    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        // let guard = self.registry.read().await;
        self.registry.get(name).cloned()
    }

    pub fn get_tools(&self) -> Vec<Arc<dyn Tool>> {
        // let guard = self.registry.read().await;
        let values  = self.registry.values();
        values.cloned().collect()
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
    pub fn new<T: Tool + 'static>(tool: &T) -> Self {
        Self {
            r#type: "function".to_string(),
            name: tool.name(),
            description: tool.description(),
            parameters: tool.parameters(),
        }
    }

    //added new here
    pub fn from_tool(tool: &dyn Tool) -> Self {
        Self {
            r#type: "function".to_string(),
            name: tool.name(),
            description: tool.description(),
            parameters: tool.parameters(),
        }
    }    

    pub fn default_for_mcp(r#type: &str, name: &str, description: &str, parameters: Value) -> Self {
        Self {
            r#type: r#type.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            parameters: parameters,
        }
    }
}
