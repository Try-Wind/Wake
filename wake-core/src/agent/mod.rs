pub mod builder;
pub mod claims;
pub mod error;
pub mod brain;
pub mod agent;
pub mod protocol;
pub mod events;
pub mod states;
pub mod actions;
pub mod output;

#[cfg(test)]
mod tests;

pub use agent::{
    Agent, AgentCore,
    TaskAgentResponse, 
    AgentResult
};
pub use states::{InternalAgentState, PublicAgentState};

pub use protocol::{AgentRequest, AgentResponse, AgentController};

pub use events::{
    InternalAgentEvent, AgentEvent,
    ClosureHandler, AgentEventHandler, DynEventHandler, closure_handler,
    UserRequest, UserResponse, PermissionRequest, PermissionResponse};
pub use output::StdoutEventManager;
    
pub use builder::AgentBuilder;
pub use claims::{ClaimManager, PermissionError};
pub use error::{AgentError, AgentExecutionError};
pub use brain::{Brain, ThinkerContext, ThinkerDecision, ThinkerFlowControl};
pub use crate::logging::LoggingConfig;