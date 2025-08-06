use std::sync::Arc;
use async_trait::async_trait;

use openai_dive::v1::resources::chat::{ChatCompletionFunction, ChatCompletionParameters, ChatCompletionParametersBuilder, ChatCompletionResponse, ChatCompletionTool, ChatCompletionToolChoice, ChatCompletionToolType, ChatMessage};

use crate::{provider::LlmError, tool::ToolBox, LlmClient, ToolDescription};

pub trait FunctionCallingAutoBuilder {
    fn with_function_calling_auto(&mut self, tools: &ToolBox) -> &mut Self;
}

impl FunctionCallingAutoBuilder for ChatCompletionParametersBuilder {
    fn with_function_calling_auto(&mut self, tools: &ToolBox) -> &mut Self {
        self
        .tools(tools.iter().map(|t| {
            ChatCompletionTool {
                r#type: ChatCompletionToolType::Function,
                function: ChatCompletionFunction {
                    name: t.name().to_string(),
                    description: Some(t.description().to_string()),
                    parameters: t.parameters_schema(),
                },
            }
        }).collect::<Vec<_>>())
        .tool_choice(ChatCompletionToolChoice::Auto)
    }
}

#[async_trait]
pub trait ToolCallFunctionCallingAuto {
    async fn chat_with_tools_fc_auto(
        &self,
        request: ChatCompletionParameters,
        tools: &ToolBox
    ) -> Result<ChatCompletionResponse, LlmError>;
}

#[async_trait]
impl ToolCallFunctionCallingAuto for LlmClient {
    async fn chat_with_tools_fc_auto(
        &self,
        request: ChatCompletionParameters,
        tools: &ToolBox
    ) -> Result<ChatCompletionResponse, LlmError> {
        let request = ChatCompletionParametersBuilder::default()
            .model(&request.model)
            .messages(request.messages.clone())
            .with_function_calling_auto(&tools)
            .temperature(0.3)
            .build()
            .map_err(|e| LlmError::from(e.to_string()))?;

        let response = self
            .chat(request.clone())
            .await
            .inspect_err(|e| {
                // Save failed request to file for debugging
                let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
                if let Ok(json) = serde_json::to_string_pretty(&request) {
                    let filename = format!("logs/request_{}.json", timestamp);
                    let _ = std::path::Path::new(&filename).parent()
                    .map(std::fs::create_dir_all).unwrap_or(Ok(()))
                    .and_then(|_| std::fs::write(&filename, json));
                }
            })
            .map_err(|e| LlmError::from(e.to_string()))?;

        Ok(response)
    }
}