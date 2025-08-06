use super::structs::WriteToolParams;
use super::super::{FsOperationLog, FsOperationType};
use crate::tools::{ToolResult, tool};
//use crate::tools::highlight::highlight_content;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct WriteTool {
    operation_log: Arc<FsOperationLog>,
}

impl WriteTool {
    pub fn new(operation_log: Arc<FsOperationLog>) -> Self {
        Self { operation_log }
    }

    fn perform_write(&self, params: &WriteToolParams) -> Result<String, String> {
        let path = Path::new(&params.path);

        // Check if file exists before writing
        let file_existed = path.exists();

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
            }
        }

        // Write content to file (overwrites if exists)
        fs::write(path, &params.content).map_err(|e| e.to_string())?;

        let action = if file_existed { "updated" } else { "created" };
        
        Ok(format!("Successfully {} file '{}' with {} bytes", 
                  action, params.path, params.content.len()))
    }
}

#[tool(name = "write", description = r#"Creates a new file with specified content or completely overwrites an existing file. This tool should be used with caution.

**Guidelines**
- To overwrite an existing file, you must first have read it with the `read` tool. This is a safety measure to ensure you are aware of the content being replaced.
- This tool is primarily for creating new files when explicitly instructed. For modifying existing files, the `edit` or `multiedit` tools are the correct choice.
- Do not create files proactively, especially documentation. Only create files when the user's request cannot be fulfilled by modifying existing ones."#, capabilities = [ToolCapability::Write])]
impl WriteTool {

    async fn execute_preview(&self, params: WriteToolParams) -> Option<ToolResult> {
        //let highlighted_content = highlight_content(&params.content, &params.path);

        let mut metadata = HashMap::new();
        metadata.insert("path".to_string(), json!(params.path));
        metadata.insert("content_length".to_string(), json!(params.content.len()));
        metadata.insert("line_count".to_string(), json!(params.content.lines().count()));
        metadata.insert("operation".to_string(), json!("write_preview"));

        Some(ToolResult::Success {
            output: params.content,
            metadata: Some(metadata),
        })
    }

    async fn execute(&self, params: WriteToolParams) -> ToolResult {
        match self.perform_write(&params) {
            Ok(message) => {
                // Log the write operation
                self.operation_log.log_operation(FsOperationType::Write, params.path.clone()).await;

                let output = format!("{}\n{}", message, params.content);
                let mut meta = HashMap::new();
                meta.insert("path".to_string(), json!(params.path));
                meta.insert("content_length".to_string(), json!(params.content.len()));
                meta.insert("operation".to_string(), json!("write"));

                // Add file size information
                if let Ok(metadata) = std::fs::metadata(&params.path) {
                    meta.insert("file_size_bytes".to_string(), json!(metadata.len()));
                }

                // Add line count information
                let line_count = params.content.lines().count();
                meta.insert("line_count".to_string(), json!(line_count));

                ToolResult::Success {
                    output,
                    metadata: Some(meta),
                }
            },
            Err(e) => {
                ToolResult::error(format!("Write failed: {}", e))
            }
        }
    }
}

