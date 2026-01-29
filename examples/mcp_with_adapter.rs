use agentic_core::{
    agent::service::AgentService,
    capabilities::{client::mcp::MCPServerAdapter, completion::{
        message::Message,
    }, rcp::JsonRpcRequest, tools::mcp::{MCPRegistry, MCPServerConfig}},
};
use anyhow::Result;
use py_literal::Value as PyValue;
use serde_json::{json, Map, Value as JsonValue};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use std::env;
use tracing::{Level, debug};

#[tokio::main]
async fn main() -> Result<()> {
    
    let filter = filter::Targets::new()
        .with_target("agentic_core::providers::gemini", Level::DEBUG)
        .with_target("agentic_core::tools", Level::DEBUG)
        .with_target("agentic_core::agent", Level::DEBUG)
        .with_target("agentic_core::examples", Level::DEBUG)
        ;

     tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact().pretty())  // Compact format
        .with(filter)
        .init();    

    let alpha_api_key = env::var("ALPHA_API_KEY").expect("Alpha Vantage API Key not found");
    let config = MCPServerConfig {
        name: "Alpha".to_string(),
        url: "https://mcp.alphavantage.co/mcp".to_string(),
        api_key: alpha_api_key,
    };

    // let api_key =
    //     env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY environment variable not set");

    // Create the alpha adapter.

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let adapter = AlphaMCPServerAdapter {};

    let agent_service = AgentService::new();
    let agent = agent_service
        .builder()
        .with_mcp_registry(config, adapter).await?
        .with_mcp_tool("Alpha", "COMPANy_OVERVIEW").await?
        // .with_gemini(&api_key)?
        .with_openai(&api_key)?
        .build()?;

    let content = "Tell me about apple stock".to_string();
    let message = Message::User {
        content: content.clone(),
        response_id: None,
    };

    let response = agent.complete_with_tools(&vec![message]).await?;
    println!("Response: {:#?}", response);

    Ok(())
}

#[derive(Debug)]
struct AlphaMCPServerAdapter {}
impl AlphaMCPServerAdapter {
    fn parse_text(&self, text: String) -> Result<String> {
        let value: PyValue = text.parse().expect("Failed to parse Python literal");

        // 2. Convert PyValue to serde_json::Value
        let json_val = self.convert_value(value);

        // 3. Serialize to a valid JSON string
        Ok(serde_json::to_string(&json_val).expect("Failed to serialize to JSON"))
    }

    fn convert_value(&self, py: PyValue) -> JsonValue {
        match py {
            PyValue::String(s) => json!(s),
            PyValue::Integer(i) => json!(i.to_string().parse::<i64>().unwrap_or(0)),
            PyValue::Boolean(b) => json!(b),
            PyValue::Dict(items) => {
                let mut map = Map::new();
                for (k, v) in items {
                    if let PyValue::String(key_str) = k {
                        map.insert(key_str, self.convert_value(v));
                    }
                }
                JsonValue::Object(map)
            }
            PyValue::List(list) => {
                JsonValue::Array(list.into_iter().map(|v| self.convert_value(v)).collect())
            }
            _ => JsonValue::Null,
        }
    }
}

impl MCPServerAdapter for AlphaMCPServerAdapter {
    fn build_tool_list_request(&self) -> JsonRpcRequest {
        let params = serde_json::json!({
            "name": "TOOL_LIST"
        });

        let request = JsonRpcRequest::default("tools/call".to_string(), Some(params));
        request
    }

    fn parse_tool_list_response(&self, text: String) -> Result<String> {
        self.parse_text(text)
    }

    fn build_tool_get_request(&self, name: &str) -> JsonRpcRequest {
        let params = serde_json::json!({
            "name": "TOOL_GET",
            "arguments" : {
                "tool_name" : name
            }
        });

        let request = JsonRpcRequest::default("tools/call".to_string(), Some(params));
        request
    }

    fn parse_tool_get_response(&self, text: String) -> Result<String> {
        // Ok(text)
        self.parse_text(text)
    }

    fn build_tool_call_request(&self, name: &str, params: JsonValue) -> JsonRpcRequest {
        let params = serde_json::json!({
            "name": "TOOL_CALL",
            "arguments" : {
                "tool_name": name,
                "arguments": params
            }
        });

        let request = JsonRpcRequest::default("tools/call".to_string(), Some(params));
        request
    }
    fn parse_tool_call_response(&self, text: String) -> Result<String> {
        let cleaned = text.replace("\n", "");
        self.parse_text(cleaned)
    }
}
