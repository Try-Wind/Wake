use crate::agent::{
    AgentCore, AgentError, InternalAgentEvent
};
use super::InternalAgentState;

impl AgentCore {
    pub async fn state_processing_handle_event(&mut self, event: InternalAgentEvent) -> Result<(), AgentError> {
        match event {
            InternalAgentEvent::CancelTask => {
                self.cancel_task().await
            },
            InternalAgentEvent::BrainResult { result } => {
                self.process_next_step(result).await
            },
            InternalAgentEvent::ToolsCompleted => {
                self.set_state(InternalAgentState::Running).await;
                Ok(())
            },
            _ => {
                Ok(())
            }
        }
    }

    /// cancel all pending tasks
    async fn cancel_task(&mut self) -> Result<(), AgentError> {
        let InternalAgentState::Processing { cancellation_token, .. } = &self.state else {
            return Err(AgentError::InvalidState(format!("state Processing expected but current state is : {:?}", self.state.to_public())));
        };

        cancellation_token.cancel();
        Ok(())
    }
}