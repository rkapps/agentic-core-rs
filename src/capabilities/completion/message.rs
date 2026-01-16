use serde::{Deserialize, Serialize};
use serde_json::Value;

// pub const ROLE_ASSISTANT: &str = "assistant";
// pub const ROLE_SYSTEM: &str = "system";
// pub const ROLE_USER: &str = "user";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Message {
    User {
        content: String,
        response_id: Option<String>,
    },
    Assistant {
        content: String,
        response_id: Option<String>,
    },
    Thought {
        content: String,
    },
    ToolCall{
        arguments: String,
        call_id: String,
        name: String,
    },
    ToolOutput {
        call_id: String,
        output: Value,
        name: String,
    },
}

