use std::{collections::HashMap, fmt::Debug, sync::Arc};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use tokio::sync::RwLock;


#[async_trait]
pub trait Tool: Send + Sync + Debug{
    
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, value: serde_json::Value) -> Result<Value>;
}


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