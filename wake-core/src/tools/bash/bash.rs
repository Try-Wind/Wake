use super::structs::BashToolParams;
use crate::tools::{tool, ToolResult};
use serde_json::json;
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tokio::time::timeout;

pub struct BashTool;

impl BashTool {
    pub fn new() -> Self {
        Self
    }

    async fn execute_command(&self, params: &BashToolParams) -> Result<(String, String, i32), Box<dyn std::error::Error + Send + Sync>> {       
        // Validate command is not empty
        if params.command.trim().is_empty() {
            return Err("Command cannot be empty".into());
        }

        // Create the command
        let mut cmd = Command::new("bash");
        cmd.args(["-c", &params.command]);

        // Set working directory if specified
        if let Some(working_dir) = &params.working_dir {
            cmd.current_dir(working_dir);
        }

        // Set environment variables
        for (key, value) in &params.env {
            cmd.env(key, value);
        }

        // Configure stdio
        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped());

        // Execute with optional timeout
        match params.timeout {
            Some(timeout_secs) => {
                let timeout_duration = Duration::from_secs(timeout_secs as u64);
                
                let result = timeout(timeout_duration, async {
                    let output = cmd.output()?;
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    let exit_code = output.status.code().unwrap_or(-1);
                    
                    Ok::<(String, String, i32), Box<dyn std::error::Error + Send + Sync>>((stdout, stderr, exit_code))
                }).await;

                match result {
                    Ok(Ok(output)) => Ok(output),
                    Ok(Err(e)) => Err(format!("Command execution failed: {}", e).into()),
                    Err(_) => Err(format!("Command timed out after {} seconds", timeout_secs).into()),
                }
            },
            None => {
                // No timeout - run indefinitely
                let output = cmd.output()?;
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);
                
                Ok((stdout, stderr, exit_code))
            }
        }
    }
}

#[tool(name = "bash", description = r#"
Executes shell commands within the user's environment. This tool is powerful and requires careful handling to ensure safety and predictability. It is your primary tool for compiling code, running tests, and managing version control with git.

SECURITY WARNING:
 - You are operating in a live user environment without a sandbox.
 - NEVER execute commands that could have unintended consequences, such as deleting files (rm), modifying system-wide configurations, or installing software without explicit, step-by-step user consent.
- When in doubt, ask the user for confirmation before proceeding with any command that modifies the file system.

Development Workflow:
- Compiling and Building: Frequently use this tool to run build commands (e.g., make, npm run build, cargo build) to validate your changes and ensure the code compiles successfully.
- Running Tests: After making changes, always run the project's test suite (e.g., npm test, pytest, cargo test) to verify that your changes haven't introduced any regressions.

Usage Guidelines:
- this tool always runs from the same path. If you need to execute command in another directory, chain the commands with && for instance "cd subcrate && cargo test"
- For file system navigation and inspection, prefer the built-in ls, read, and find tools. Use bash for executing other programs or scripts.
- Always provide a clear, concise description of the command's purpose for the user.
- Chain commands using && to ensure that subsequent commands only run if the previous ones succeed.
- Enclose file paths and arguments in double quotes (") to handle spaces and special characters correctly.

Examples:
- Good: cargo build (Compiles the project)
- Good: npm test (Runs the test suite)
- Good: git status (Checks the repository status)
- Good: git add . && git commit -m "feat: Implement the new feature" (Stages and commits changes)
- DANGEROUS: rm -rf / (Deletes the root directory)
- DANGEROUS: curl http://example.com/install.sh | sh (Executes a script from the internet without inspection)
"#, capabilities = [ToolCapability::Read, ToolCapability::Write, ToolCapability::Network])]
impl BashTool {
    async fn execute(&self, params: BashToolParams) -> ToolResult {
        let start_time = Instant::now();
        
        match self.execute_command(&params).await {
            Ok((stdout, stderr, exit_code)) => {
                let execution_time = start_time.elapsed();
                let mut metadata = HashMap::new();
                
                metadata.insert("command".to_string(), json!(params.command));
                metadata.insert("exit_code".to_string(), json!(exit_code));
                metadata.insert("execution_time_ms".to_string(), json!(execution_time.as_millis()));
                if let Some(timeout_val) = params.timeout {
                    metadata.insert("timeout".to_string(), json!(timeout_val));
                } else {
                    metadata.insert("timeout".to_string(), json!("none"));
                }
                metadata.insert("success".to_string(), json!(exit_code == 0));
                
                if let Some(working_dir) = &params.working_dir {
                    metadata.insert("working_dir".to_string(), json!(working_dir));
                }
                
                if !params.env.is_empty() {
                    metadata.insert("env_vars".to_string(), json!(params.env));
                }
                
                // Include stderr info if present
                let has_stderr = !stderr.is_empty();
                if has_stderr {
                    metadata.insert("has_stderr".to_string(), json!(true));
                    metadata.insert("stderr_length".to_string(), json!(stderr.len()));
                }
                
                // Prepare error message if needed
                let error_message = if exit_code != 0 && has_stderr {
                    Some(format!("Command failed with exit code {}: {}", exit_code, stderr))
                } else if exit_code != 0 {
                    Some(format!("Command failed with exit code {}", exit_code))
                } else {
                    None
                };
                
                // Combine stdout and stderr for output
                let output = if stderr.is_empty() {
                    stdout
                } else if stdout.is_empty() {
                    stderr
                } else {
                    format!("{}\n--- STDERR ---\n{}", stdout, stderr)
                };
                
                if exit_code == 0 {
                    ToolResult::Success {
                        output,
                        metadata: Some(metadata),
                    }
                } else {
                    ToolResult::Error {
                        error: error_message.unwrap_or_else(|| format!("Command failed with exit code {}", exit_code)),
                        metadata: Some(metadata),
                    }
                }
            },
            Err(e) => {
                let execution_time = start_time.elapsed();
                let mut metadata = HashMap::new();
                
                metadata.insert("command".to_string(), json!(params.command));
                metadata.insert("execution_time_ms".to_string(), json!(execution_time.as_millis()));
                if let Some(timeout_val) = params.timeout {
                    metadata.insert("timeout".to_string(), json!(timeout_val));
                } else {
                    metadata.insert("timeout".to_string(), json!("none"));
                }
                metadata.insert("success".to_string(), json!(false));
                
                ToolResult::Error {
                    error: e.to_string(),
                    metadata: Some(metadata),
                }
            }
        }
    }
}