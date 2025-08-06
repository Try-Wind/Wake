use crate::agent::{AgentCore, InternalAgentEvent, AgentError};
use tracing::error;

impl AgentCore {
    pub async fn state_terminal_handle_event(&mut self, event: InternalAgentEvent) -> Result<(), AgentError> {
        match event {
            _ => {
                // ignore all events but log error
                error!("event {:?} unexpected in state {:?}", event, self.state.to_public());
                Ok(())
            }
        }
    }
}