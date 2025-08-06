use std::sync::Arc;
use async_trait::async_trait;
use wake_llm::{ChatMessage, ToolCallMethod};
use tokio::sync::RwLock;

use crate::tools::types::AnyToolBox;
use super::error::AgentError;


/// ThinkerContext is the agent internal state
pub struct ThinkerContext {
    pub trace:           Arc<RwLock<Vec<ChatMessage>>>,
    pub available_tools: AnyToolBox,
    pub method:          ToolCallMethod
}

/// ThinkerFlowControl drives the agentic flow
#[derive(Debug, Clone)]
pub enum ThinkerFlowControl {
    AgentContinue,
    AgentPause
}

/// This structure pilot the flow of the Agent
/// If tool_call are present in the chat message, the flow attribute is ignored
/// If no tool_call is present in the chat message, flow will pilot wether the agent pause or continue
#[derive(Debug, Clone)]
pub struct ThinkerDecision {
    pub message: ChatMessage,
    pub flow:    ThinkerFlowControl
}

impl ThinkerDecision {
    pub fn new(message: ChatMessage) -> Self {
        ThinkerDecision{
            message,
            flow: ThinkerFlowControl::AgentPause
        }
    }

    pub fn agent_continue(message: ChatMessage) -> Self {
        ThinkerDecision{
            message,
            flow: ThinkerFlowControl::AgentContinue
        }
    }

    pub fn agent_pause(message: ChatMessage) -> Self {
        ThinkerDecision{
            message,
            flow: ThinkerFlowControl::AgentPause
        }
    }

    pub fn unwrap(self) -> ChatMessage {
        self.message
    }
}

/// Core thinking interface - pure decision making
#[async_trait]
pub trait Brain: Send + Sync {
    /// This method is called at every step of the agent to decide next step
    /// note that if the message contains toolcall, it will always continue
    async fn next_step(&mut self, context: ThinkerContext) -> Result<ThinkerDecision, AgentError>;
}


