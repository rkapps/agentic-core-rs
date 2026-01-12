use serde::{Deserialize, Serialize};
use std::str::FromStr;

const ROLE_ASSISTANT : &str = "assistant";
const ROLE_SYSTEM : &str = "system";
const ROLE_USER : &str = "user";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: MessageRole,
    pub content: String,
    pub response_id: Option<String>,

}

impl MessageRole {

    pub fn assistant() -> &'static str {
        ROLE_ASSISTANT
    }

    pub fn user() -> &'static str {
        ROLE_USER
    }

    pub fn system() -> &'static str {
        ROLE_SYSTEM
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MessageRole::User => ROLE_USER,
            MessageRole::Assistant => ROLE_ASSISTANT,
            MessageRole::System => ROLE_SYSTEM,
        }
    }
}

impl FromStr for MessageRole {
    type Err = ();
    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role {
            ROLE_USER => Ok(MessageRole::User),
            ROLE_ASSISTANT => Ok(MessageRole::Assistant),
            ROLE_SYSTEM => Ok(MessageRole::System),
            _ => Err(()),
        }
    }
}

//imlement helper function to create role specific message.
impl Message {
    pub fn create_user_message(content: &str, response_id: Option<String>) -> Message {
        Message {
            role: MessageRole::User,
            content: content.to_string(),
            response_id: response_id
        }
    }

    pub fn create_system_message(content: &str, response_id: Option<String>) -> Message {
        Message {
            role: MessageRole::System,
            content: content.to_string(),
            response_id: response_id
        }
    }

    pub fn create_assistant_message(content: &str, response_id: Option<String>) -> Message {
        Message {
            role: MessageRole::Assistant,
            content: content.to_string(),
            response_id: response_id
        }
    }
}
