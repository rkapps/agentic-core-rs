# agentic-core

Multi-LLM client library for Rust with support for OpenAI, Anthropic, and Google Gemini.

## Features

- ✅ **Multi-provider support**: OpenAI, Anthropic (Claude), Google Gemini
- ✅ **Streaming responses**: Server-Sent Events (SSE) support
- ✅ **Async/await**: Built on tokio
- ✅ **Type-safe**: Strongly typed requests and responses
- ✅ **Agent pattern**: Lifetime-bound builder pattern where AgentBuilder borrows from AgentService, allowing safe      concurrent access and lazy resource initialization.

## Installation

```toml
[dependencies]
agentic-core = "0.1.0"
```

## Quick Start

```rust
use agentic_core::{AgentService, AgentBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    // Create agent service
    let mut service = AgentService::new();
    
    // Configure provider
    service.set_client("Anthropic", "your-api-key")?;
    
    // Create agent
    let agent = service.get_chat_agent("Anthropic")?;
    
    // Make completion request
    let request = CompletionRequest {
        model: "claude-sonnet-4-5".to_string(),
        messages: vec![/* your messages */],
        // ...
    };
    
    let response = agent.complete(request).await?;
    println!("Response: {}", response.content);
    
    Ok(())
}
```

## Usage

### Regular Completion

```rust
// Example for each provider
```

### Streaming Responses

```rust
// SSE streaming example
```

### Supported Providers

#### Anthropic (Claude)

- Models: `claude-sonnet-4-5`
- API: Messages API
- Features: Streaming, system prompts

#### OpenAI

- Models: `gpt-4o`, etc.
- API: Responses API (stateless)
- Features: Streaming with response IDs

#### Google Gemini  

- Models: `gemini-2.0-flash-exp`
- API: Interactions API (stateless)
- Features: Streaming with interaction IDs

## API Reference

### Core Types

- `LlmClient` - Trait for LLM providers
- `Agent` - Configured LLM client
- `AgentService` - Manages multiple clients
- `CompletionRequest` / `CompletionResponse`

### Configuration

```rust
    let openai_api_key =
        env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    let agent = agent_service
        .builder()
        .with_system(system_prompt)
        .with_openai(&openai_api_key)?
        .build()?;
```

## Examples

See `examples/` directory for complete examples.

## Requirements

- Rust 1.75+
- Tokio runtime

## License

MIT

## Contributing

Pull requests welcome!