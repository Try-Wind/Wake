use wake_llm::ChatMessage;
use uuid::Uuid;

use crate::tools::AnyTool;
use super::Brain;
use super::AgentCore;
use super::claims::ClaimManager;

/// Builder for AgentCore
pub struct AgentBuilder {
    pub session_id: String,
    pub brain: Box<dyn Brain>,
    pub goal: Option<String>,
    pub trace: Vec<ChatMessage>,
    pub available_tools: Vec<Box<dyn AnyTool>>,
    pub permissions: ClaimManager,
}

impl AgentBuilder {
    pub fn new(brain: Box<dyn Brain>) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            brain: brain,
            goal: None,
            trace: vec![],
            available_tools: vec![],
            permissions: ClaimManager::new(),
        }
    }
}

impl AgentBuilder {
    pub fn id(mut self, session_id: &str) -> Self {
        self.session_id = session_id.to_string();
        self
    }
        
    pub fn brain(mut self, brain: Box<dyn Brain>) -> Self {
        self.brain = brain;
        self
    }
    
    pub fn goal(mut self, goal: &str) -> Self {
        self.goal = Some(goal.to_string());
        self
    }
    
    pub fn with_traces(mut self, trace: Vec<ChatMessage>) -> Self {
        self.trace = trace;
        self
    }

    pub fn tools(mut self, available_tools: Vec<Box<dyn AnyTool>>) -> Self {
        self.available_tools = available_tools;
        self
    }
    
    pub fn permissions(mut self, permissions: ClaimManager) -> Self {
        self.permissions = permissions;
        self
    }

    /// Enable sudo mode - bypasses all permission checks
    pub fn sudo(mut self) -> Self {
        self.permissions.sudo();
        self
    }

    /// Build the AgentCore with required runtime fields
    pub fn build(mut self) -> AgentCore {        
        if let Some(goal) = self.goal {
            self.trace.push(ChatMessage::User { content: wake_llm::ChatMessageContent::Text(goal.clone()), name: None });
        }
        
        AgentCore::new(
            self.session_id.clone(),
            self.brain,
            self.trace,
            self.available_tools,
            self.permissions
        )
    }
}
