use std::fmt::Debug;
use serde_json::Value;
use anyhow::Result;
use crate::capabilities::rcp::JsonRpcRequest;



pub trait MCPServerAdapter: Send + Sync + Debug {
    fn build_tool_list_request(&self) -> JsonRpcRequest;
    fn parse_tool_list_response(&self, text: String) -> Result<String>;
    fn build_tool_get_request(&self, name: &str) -> JsonRpcRequest;
    fn parse_tool_get_response(&self, text: String) -> Result<String>;
    fn build_tool_call_request(&self, name: &str, params: Value) -> JsonRpcRequest;
    fn parse_tool_call_response(&self, text: String) -> Result<String>;
}
