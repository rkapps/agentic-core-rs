use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct CompletionResponse {
    pub id: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct CompletionChunkResponse {
    pub content: String,
    pub thinking: String,
    pub is_final: bool,
}

impl CompletionChunkResponse {
    
    pub fn default() -> CompletionChunkResponse {
        CompletionChunkResponse {
            content: String::new(),
            thinking: String::new(),
            is_final: false,
        }
    }

    pub fn stop() -> CompletionChunkResponse {
        CompletionChunkResponse {
            content: String::new(),
            thinking: String::new(),
            is_final: true,
        }
    }

    pub fn content(content: String, thinking: String) -> CompletionChunkResponse {
        CompletionChunkResponse {
            content: content,
            thinking: thinking,
            is_final: false,
        }
    }

}
