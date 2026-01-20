use std::fmt::Debug;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;


#[async_trait]
pub trait Tool: Send + Sync + Debug{
    
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> serde_json::Value;
    async fn execute(&self, value: serde_json::Value) -> Result<Value>;
}

