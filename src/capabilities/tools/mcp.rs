use anyhow::{Context, Result};
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};
use tracing::debug;

use crate::{
    capabilities::{
        client::mcp::MCPServerAdapter,
        rcp::{JsonRpcRequest, JsonRpcResponse},
        tools::{
            request::{MCPToolCallParamsRequest, MCPToolGetParamsRequest, MCPToolListRequest},
            response::{
                MCPToolCallResponse, MCPToolGetDefinition, MCPToolListDefinition,
                MCPToolListResponse,
            },
            tool::ToolDefinition,
        },
    },
    http::HttpClient,
};

#[derive(Debug, Clone)]
pub struct MCPServerConfig {
    pub name: String,
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Clone)]
pub struct MCPRegistry {
    pub registry: HashMap<String, MCPClient>,
    pub definitions: HashMap<String, ToolDefinition>,
}

impl MCPRegistry {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            definitions: HashMap::new(),
        }
    }

    // register server with standard adapter
    pub async fn register_server(
        &mut self,
        config: MCPServerConfig,
    ) -> Result<Vec<ToolDefinition>> {
        let adapter = Box::new(StandardAdapter {});
        self.register_server_with_adapter(config, adapter).await
    }

    // register server with an adapter
    // Creates a new MCP client for the server
    // Calls the tool_list method and returns a list of tools without the paramter definition.
    pub async fn register_server_with_adapter(
        &mut self,
        config: MCPServerConfig,
        adapter: Box<dyn MCPServerAdapter>,
    ) -> Result<Vec<ToolDefinition>> {
        // add code to check if server already inserted
        let client =
            MCPClient::new(config.clone(), adapter).context("Error connecting the MCPClient")?;
        let mcp_definitions = client.tool_list().await?;
        // debug!("Definitions: {:#?}", mcp_definitions);
        let mut definitions = Vec::new();
        for mcp_definition in mcp_definitions {
            let name = format!("{}___{}", config.name.clone(), mcp_definition.name);
            let definition = ToolDefinition::default_for_mcp(
                "function",
                &name,
                &mcp_definition.description,
                json!({}),
            );
            definitions.push(definition);
        }

        self.registry.insert(config.name, client);
        Ok(definitions)
    }

    // Register a tool for the server.
    // Get complete definition with parameters for the tool and cache it
    pub async fn register_tool(
        &mut self,
        server_name: &str,
        tool_name: &str,
    ) -> Result<ToolDefinition> {
        if let Some(client) = self.registry.get(server_name) {
            let mcp_get_definition = client.tool_get(&tool_name).await?;
            let name = format!("{}___{}", server_name, tool_name);

            let tool_definition = ToolDefinition::default_for_mcp(
                "function",
                &name,
                &mcp_get_definition.description,
                mcp_get_definition.parameters,
            );
            self.definitions.insert(name, tool_definition.clone());

            Ok(tool_definition)
        } else {
            return Err(anyhow::anyhow!(
                "Server '{}' has not been registered.",
                server_name
            ));
        }
    }

    // Call the tool.
    // Split the tool_name to server and the tool_name
    // Call tool_call and return the result
    pub async fn call_tool(&self, tool_name: &str, params: Value) -> Result<Value> {
        let server: Vec<&str> = tool_name.split("___").collect();
        // debug!("Server: {:#?}", server);
        let server_name = server[0];
        let tool_call_name = server[1];
        if let Some(client) = self.registry.get(server_name) {
            client.tool_call(tool_call_name, params).await
        } else {
            return Err(anyhow::anyhow!(
                "Server '{}' has not been registered.",
                server_name
            ));
        }
    }
}

#[derive(Debug, Clone)]
pub struct MCPClient {
    pub name: String,
    pub url: String,
    pub api_key: String,
    http_client: HttpClient,
    server_adapter: Arc<Box<dyn MCPServerAdapter>>,
}

impl MCPClient {
    pub fn new(config: MCPServerConfig, adapter: Box<dyn MCPServerAdapter>) -> Result<Self> {
        Ok(Self {
            name: config.name,
            url: config.url,
            api_key: config.api_key,
            http_client: HttpClient::new()?,
            server_adapter: Arc::new(adapter),
        })
    }

    async fn tool_list(&self) -> Result<Vec<MCPToolListDefinition>> {
        let request = self.server_adapter.build_tool_list_request();
        let body = serde_json::json!(request);
        let headers = self.get_header()?;
        let response = self
            .http_client
            .post_request::<JsonRpcResponse<MCPToolListResponse>>(
                self.url.clone(),
                Some(headers),
                body,
            )
            .await?;

        let json_text = self
            .server_adapter
            .parse_tool_list_response(response.result.content[0].clone().text)
            .with_context(|| "Server Adapter Tool List parsing error")?;

        // Parse JSON and extracts definitions
        let definitions: Vec<MCPToolListDefinition> = serde_json::from_str(&json_text)
            .map_err(|e| anyhow::anyhow!("Server Adapter error converting text {:#?}", e))?;
        Ok(definitions)
    }

    async fn tool_get(&self, name: &str) -> Result<MCPToolGetDefinition> {
        let request = self.server_adapter.build_tool_get_request(name);
        let body = serde_json::json!(request);
        let headers = self.get_header()?;
        debug!("Tool_get request: {:#?}", request);
        let response = self
            .http_client
            .post_request::<JsonRpcResponse<MCPToolListResponse>>(
                self.url.clone(),
                Some(headers),
                body,
            )
            .await?;

        let json_text = self
            .server_adapter
            .parse_tool_get_response(response.result.content[0].clone().text)
            .with_context(|| "Server Adapter Tool Get parsing error")?;

        // debug!("Json Text: {:#?}", json_text);
        let definition: MCPToolGetDefinition = serde_json::from_str(&json_text)
            .map_err(|e| anyhow::anyhow!("Server Adapter error converting text {:#?}", e))?;

        debug!("tool_get Response: {:#?}", definition);
        Ok(definition)
    }

    async fn tool_call(&self, name: &str, params: Value) -> Result<Value> {
        let request = self.server_adapter.build_tool_call_request(name, params);
        let body = serde_json::json!(request);
        let headers = self.get_header()?;
        debug!("Tool_call request: {:#?}", request);
        let response = self
            .http_client
            .post_request::<JsonRpcResponse<MCPToolCallResponse>>(
                self.url.clone(),
                Some(headers),
                body,
            )
            .await?;

        let json_text = self
            .server_adapter
            .parse_tool_call_response(response.result.content[0].clone().text)
            .with_context(|| "Server Adapter Tool Get parsing error")?;

        debug!("tool_call Response: {:#?}", json_text);

        let value: Value = serde_json::from_str(&json_text)?;

        Ok(value)
    }

    fn get_header(&self) -> Result<HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);
        Ok(headers)
    }
}

#[derive(Debug)]
struct StandardAdapter {}

impl MCPServerAdapter for StandardAdapter {
    fn build_tool_list_request(&self) -> JsonRpcRequest {
        let request = JsonRpcRequest::default(
            "tools/list".to_string(),
            serde_json::to_value(MCPToolListRequest {}).ok(),
        );
        request
    }

    fn parse_tool_list_response(&self, text: String) -> Result<String> {
        Ok(text)
    }

    fn build_tool_get_request(&self, name: &str) -> JsonRpcRequest {
        let tool_get = MCPToolGetParamsRequest {
            tool_name: name.to_string(),
        };
        let request =
            JsonRpcRequest::default("tools/get".to_string(), serde_json::to_value(tool_get).ok());
        request
    }
    fn parse_tool_get_response(&self, text: String) -> Result<String> {
        Ok(text)
    }

    fn build_tool_call_request(&self, name: &str, params: Value) -> JsonRpcRequest {
        let tool_call = MCPToolCallParamsRequest {
            tool_name: name.to_string(),
            arguments: params,
        };
        let request = JsonRpcRequest::default(
            "tools/get".to_string(),
            serde_json::to_value(tool_call).ok(),
        );
        request
    }

    fn parse_tool_call_response(&self, text: String) -> Result<String> {
        let data = serde_json::from_str(&text)?;
        Ok(data)
    }
}
