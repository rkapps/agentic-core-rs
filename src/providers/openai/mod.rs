pub mod completion;
pub mod request;
pub mod response;
pub mod embeddings;

pub const LLM: &str = "OpenAI";
pub const MODEL_GPT_5_NANO: &str = "gpt-5-nano";
pub const MODEL_TEXT_EMBEDDING_3_SMALL: &str = "text-embedding-3-small";
const OPENAI_BASE_URL: &str = "https://api.openai.com";
