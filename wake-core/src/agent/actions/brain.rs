use chrono::Utc;
use wake_llm::ChatMessage;
use tracing::info;
use tokio_util::sync::CancellationToken;
use crate::agent::{AgentCore, AgentError, AgentEvent, InternalAgentEvent, InternalAgentState, ThinkerContext, ThinkerDecision, ThinkerFlowControl};

impl AgentCore {
    /// Launch a brain task to decide next step
    pub async fn spawn_next_step(&mut self) {         
        let cancellation_token = CancellationToken::new();
        let cancel_token_clone = cancellation_token.clone();
        let trace = self.trace.clone();
        let tx_clone = self.internal_tx.clone();
        let available_tools = self.available_tools.clone();
        let method = self.method.clone();
        let context = ThinkerContext {
            trace,
            available_tools,
            method
        };
        let brain = self.brain.clone();
        
        //////////////////////// TOKIO SPAWN
        tokio::spawn(async move {
            tokio::select! {
                result = async {
                    brain.write().await.next_step(context).await
                } => {
                    let _ = tx_clone.send(InternalAgentEvent::BrainResult {
                        result
                    });
                }
                _ = cancel_token_clone.cancelled() => {
                    // Brain thinking was cancelled, no need to send result
                }
            }
        });
        //////////////////////// TOKIO SPAWN
        
        self.set_state(InternalAgentState::Processing { 
            task_name: "next_step".to_string(), 
            tools_exec_at: Utc::now(), 
            cancellation_token
        }).await;
    }


    /// Process a brain task result
    pub async fn process_next_step(&mut self, result: Result<ThinkerDecision, AgentError>) -> Result<(), AgentError> {
        let ThinkerDecision{message, flow} = self.handle_brain_error(result).await?;
        let ChatMessage::Assistant { content, reasoning_content, tool_calls, .. } = message.clone() else {
            return self.handle_brain_error::<ThinkerDecision>(
                Err(AgentError::InvalidResponse(format!("ChatMessage::Assistant expected, but got {:?} instead", message)))).await.map(|_| ()
            );
        };
    
        // Add the message to trace
        info!(target: "agent::think", reasoning_content = ?reasoning_content, content = ?content);
        let trace = self.trace.clone();
        trace.write().await.push(message.clone());
        
        // Emit event to external consumers
        let _ = self.emit_event(AgentEvent::BrainResult { 
            timestamp: Utc::now(),
            thought: Ok(message.clone())
        }).await;
    
        // run tool call if any
        let tool_calls_from_brain = tool_calls.unwrap_or(vec![]);
        if !tool_calls_from_brain.is_empty() {
            self.spawn_tools(tool_calls_from_brain).await;
            return Ok(())
        }
    
        // no tool call, thus we rely on flow control
        match flow {
            ThinkerFlowControl::AgentContinue => {
                self.set_state(InternalAgentState::Running).await;
            }
            ThinkerFlowControl::AgentPause => {
                self.set_state(InternalAgentState::Paused).await;
            }
        }
        Ok(())
    }

    // Helper method that emits error events before returning the error
    async fn handle_brain_error<T>(&mut self, result: Result<T, AgentError>) -> Result<T, AgentError> {
        match result {
            Ok(value) => Ok(value),
            Err(error) => {
                self.set_state(InternalAgentState::Paused).await;
                let _ = self.emit_event(AgentEvent::BrainResult { 
                    timestamp: Utc::now(),
                    thought: Err(error.clone())
                }).await;
                Err(error)
            }
        }
    }
}