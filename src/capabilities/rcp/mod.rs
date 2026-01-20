use serde::{Serialize, Deserialize};
use serde_json::Value;


#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    id: Value,
    params: Option<Value>
}


impl JsonRpcRequest {

    pub fn new(jsonrpc: String, method: String, id: Value, params: Option<Value>) -> Self {
        Self { jsonrpc, method, id, params}
    }

    pub fn default(method: String, params: Option<Value>) -> Self{
        Self { jsonrpc: String::from("2.0"), method: method, id: serde_json::from_str("1").unwrap(), params: params}
    }


}


#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse<T> {
    #[allow(dead_code)]
    jsonrpc: String,
    #[allow(dead_code)]
    id: Value,
    pub result: T
}

