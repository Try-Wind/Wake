pub mod client;
pub mod providers;
pub mod provider;
pub mod chat;
pub mod tool;

// Re-export our client
pub use client::LlmClient;

pub use tool::{
    ToolDescription, 
    ToolCallMethod,
    ToolBox,
    ContainsTool,
    StructuredOutputBuilder, 
    AssistantResponse, 
    IntoChatMessage, 
    FunctionCallingAutoBuilder, 
    FunctionCallingRequiredBuilder};

// Re-export commonly used openai_dive types for consumers
pub use openai_dive::v1::resources::chat::{
    ChatCompletionParameters,
    ChatCompletionResponse, 
    ChatMessage,
    ChatMessageContent,
    ChatCompletionTool,
    ChatCompletionToolType,
    ChatCompletionFunction,
    ToolCall,
    Function,
    ChatCompletionChoice,
};