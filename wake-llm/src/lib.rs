pub mod chat;
pub mod client;
pub mod provider;
pub mod providers;
pub mod tool;

// Re-export our client
pub use client::LlmClient;

pub use tool::{
    AssistantResponse, ContainsTool, FunctionCallingAutoBuilder, FunctionCallingRequiredBuilder,
    IntoChatMessage, StructuredOutputBuilder, ToolBox, ToolCallMethod, ToolDescription,
};

// Re-export commonly used openai_dive types for consumers
pub use openai_dive::v1::resources::chat::{
    ChatCompletionChoice, ChatCompletionFunction, ChatCompletionParameters, ChatCompletionResponse,
    ChatCompletionTool, ChatCompletionToolType, ChatMessage, ChatMessageContent, Function,
    ToolCall,
};
