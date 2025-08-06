use std::sync::Arc;
use async_trait::async_trait;
use schemars::json_schema;
use serde_json::json;

use openai_dive::v1::resources::chat::{ChatCompletionFunction, ChatCompletionParameters, ChatCompletionParametersBuilder, ChatCompletionResponse, ChatCompletionTool, ChatCompletionToolChoice, ChatCompletionToolType, ChatMessage, Function, ToolCall};
use crate::{provider::LlmError, tool::ToolBox, LlmClient, ToolDescription};


pub struct NoOp {}

impl ToolDescription for NoOp {
    fn name(&self) -> &'static str {
        "no_op"
    }

    fn description(&self) -> &'static str {
        "this tool is a no_op and does nothing. This tool must be called if you don't want to call any tool."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}

pub trait FunctionCallingRequiredBuilder {
    fn with_function_calling_required(&mut self, tools: &ToolBox) -> &mut Self;
}

impl FunctionCallingRequiredBuilder for ChatCompletionParametersBuilder {
    fn with_function_calling_required(&mut self, tools: &ToolBox) -> &mut Self {
        let mut tools = tools.clone();
        tools.push(Arc::new(NoOp{}));

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
        .tool_choice(ChatCompletionToolChoice::Required)
    }
}

#[async_trait]
pub trait ToolCallFunctionCallingRequired {
    async fn chat_with_tools_fc_required(
        &self,
        request: ChatCompletionParameters,
        tools: &ToolBox
    ) -> Result<ChatCompletionResponse, LlmError>;
}

#[async_trait]
impl ToolCallFunctionCallingRequired for LlmClient {
    async fn chat_with_tools_fc_required(
        &self,
        request: ChatCompletionParameters,
        tools: &ToolBox
    ) -> Result<ChatCompletionResponse, LlmError> {
        let request = ChatCompletionParametersBuilder::default()
            .model(&request.model)
            .messages(request.messages.clone())
            .with_function_calling_required(&tools)
            .temperature(0.3)
            .build()
            .map_err(|e| LlmError::from(e.to_string()))?;

        let mut response = self
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

        let mut response = response;
        match &mut response.choices[0].message {
            ChatMessage::Assistant { tool_calls, .. } => {
                if let Some(calls) = tool_calls {
                    if let [ToolCall { function: Function { name, .. }, .. }] = calls.as_slice() {
                        if name == "no_op" {
                            *tool_calls = None;
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(response)  
    }
}