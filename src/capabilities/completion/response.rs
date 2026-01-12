use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub struct CompletionResponse {
    pub response_id: String,
    pub content: String,
}

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct CompletionChunkResponse {
    pub response_id: String,
    pub content: String,
    pub thinking: String,
    pub is_final: bool,
}

impl CompletionChunkResponse {
    
    pub fn default() -> CompletionChunkResponse {
        CompletionChunkResponse {
            response_id: String::new(),
            content: String::new(),
            thinking: String::new(),
            is_final: false,
        }
    }

    pub fn stop(id: String) -> CompletionChunkResponse {
        CompletionChunkResponse {
            response_id: id,
            content: String::new(),
            thinking: String::new(),
            is_final: true,
        }
    }

    pub fn content(content: String, thinking: String) -> CompletionChunkResponse {
        CompletionChunkResponse {
            response_id: String::new(),
            content: content,
            thinking: thinking,
            is_final: false,
        }
    }

}
