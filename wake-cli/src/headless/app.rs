use std::sync::Arc;

use crate::headless::tools::ToolConfig;

use super::tools::{ToolName, list_all_tools, parse_tools_list};
use wake_core::agent::{Agent, AgentBuilder, AgentError, AgentResult, Brain, LoggingConfig, StdoutEventManager};
use wake_core::config::config::WakeConfig;
use wake_core::runners::coder::coder::CoderBrain;
use wake_core::runners::searcher::searcher::SearcherBrain;
use wake_llm::{ChatMessage, ChatMessageContent, LlmClient};

pub enum AgentKind {
    Coder,
    Searcher,
}

pub struct AppHeadless {
    kind: AgentKind
}

impl AppHeadless {
    pub fn new() -> Self {
        Self {
            kind: AgentKind::Coder
        }
    }

    pub async fn run(&self,
        initial_trace: Vec<ChatMessage>,
        list_tools: bool, 
        tools: Option<String>, 
        remove: Option<String>,
        trace: bool
    ) -> Result<(), Box<dyn std::error::Error>> {   
        // Configure internal debug logging to file
        /*
        let _ = LoggingConfig::default()
            .level("debug")
            .file_path("agent_debug.log")
            .init();
        */

        // Handle --list-tools flag
        if list_tools {
            list_all_tools();
            return Ok(());
        }
    
        let (llm_client, model) = WakeConfig::get_llm().await?;
        eprintln!("\x1b[2m{} on {}\x1b[0m", model, llm_client.provider().name());
    
        // Validate that we have some input
        if initial_trace.is_empty() {
            eprintln!("Error: Please provide a prompt for the coder agent");
            eprintln!("Usage: wake \"your prompt here\" or using pipe echo \"your prompt here\" | wake");
            return Ok(());
        }
    
        // Handle tool selection
        let tools = match (tools, remove) {
            (Some(tools_str), _) => {
                let selected_tools = parse_tools_list(&tools_str)?;
                ToolConfig::new().add_tools(selected_tools)
            }
            (None, Some(remove_str)) => {
                let tools_to_remove = parse_tools_list(&remove_str)?;
                ToolConfig::new().remove_tools(tools_to_remove)
            }
            (None, None) => ToolConfig::new(),
        };

        // Run the agent
        let model = llm_client.default_model().await
            .map_err(|e| format!("Failed to get default model: {}", e))?;

        let toolbox = tools.build_toolbox();
        let brain: Box<dyn Brain> = match self.kind {
            AgentKind::Coder => Box::new(CoderBrain::new(Arc::new(llm_client), model)),
            AgentKind::Searcher => Box::new(SearcherBrain::new(Arc::new(llm_client), model)),
        };

        let agent = AgentBuilder::new(brain)
            .with_traces(initial_trace)
            .tools(toolbox)
            .sudo()
            .build();

        let result = agent
            .with_event_handler(StdoutEventManager::new())
            .run().await;

        match result {
            Ok(AgentResult { success, message, trace: agent_trace }) => {
                if trace {
                    println!("{}", serde_json::to_string_pretty(&agent_trace)?);
                } else {
                    if let Some(message) = agent_trace.last() {
                        match message {
                            ChatMessage::Assistant { content: Some(ChatMessageContent::Text(content)), .. } => {
                                println!("{}",content);
                            }
                            ChatMessage::Tool { content, .. } => {
                                println!("{}",content);
                            }
                            _ => {}
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Agent failed: {}", e);
            }
        }
        Ok(())
    }
}