use super::structs::MultiEditToolParams;
use super::super::{FsOperationLog, FsOperationType, EditTool};
use crate::tools::{tool, ToolResult};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::fs;
use std::path::Path;

#[derive(Clone)]
pub struct MultiEditTool {
    operation_log: Arc<FsOperationLog>,
    edit_tool: EditTool,
}

impl MultiEditTool {
    pub fn new(operation_log: Arc<FsOperationLog>) -> Self {
        let edit_tool = EditTool::new(operation_log.clone());
        Self { operation_log, edit_tool }
    }
    
    async fn perform_multi_edit(&self, params: &MultiEditToolParams, preview: bool) -> Result<(String, Vec<usize>), String> {
        let path = Path::new(&params.file_path);

        // Check if file exists
        if !path.exists() {
            return Err(format!("File does not exist: {}", params.file_path));
        }

        // Read initial content
        let mut current_content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let original_content = current_content.clone();
        let mut replacements_per_edit = Vec::new();

        // Apply each edit operation sequentially on content
        for (index, edit) in params.edits.iter().enumerate() {
            match self.edit_tool.perform_edit_on_content(&current_content, &edit.old_string, &edit.new_string, edit.replace_all) {
                Ok((new_content, replacements)) => {
                    current_content = new_content;
                    replacements_per_edit.push(replacements);
                },
                Err(error) => {
                    return Err(format!("Edit #{}: {}", index + 1, error));
                }
            }
        }

        // Generate comprehensive diff
        let diff = self.edit_tool.myers_diff(&original_content, &current_content);
        
        // Only write to file if not preview mode
        if !preview {
            self.edit_tool.commit_edit(&params.file_path, &current_content)?;
        }

        Ok((diff, replacements_per_edit))
    }
}

#[tool(name = "multiedit", description = r#"Executes a batch of sequential find-and-replace operations on a single file within one atomic transaction. This is the preferred tool for making numerous, distinct changes to one file efficiently.

**Execution Logic:**
- Edits are applied in the exact order they are provided. The second edit operates on the result of the first, the third on the result of the second, and so on.
- The entire sequence is atomic. If any single edit fails (e.g., its `old_string` is not found), the whole operation is rolled back, and the file remains unmodified.

**Critical Considerations:**
- You must first use the `read` tool to understand the file's contents.
- Plan your sequence of edits carefully. An earlier edit might alter the text that a later edit is intended to match, which could cause the later edit to fail."#, capabilities = [ToolCapability::Read, ToolCapability::Write])]
impl MultiEditTool {
    async fn execute_preview(&self, params: MultiEditToolParams) -> Option<ToolResult> {
        Some(self.execute_internal(params, true).await)
    }

    async fn execute(&self, params: MultiEditToolParams) -> ToolResult {
        self.execute_internal(params, false).await
    }

    async fn execute_internal(&self, params: MultiEditToolParams, preview: bool) -> ToolResult {
        // Validate that we have at least one edit operation
        if params.edits.is_empty() {
            return ToolResult::error("At least one edit operation is required".to_string());
        }

        // Validate that the file has been read first
        if let Err(err) = self.operation_log.validate_edit_permission(&params.file_path).await {
            return ToolResult::error(err);
        }

        match self.perform_multi_edit(&params, preview).await {
            Ok((message, replacements_per_edit)) => {
                // Log the multiedit operation only if not preview
                if !preview {
                    self.operation_log.log_operation(FsOperationType::MultiEdit, params.file_path.clone()).await;
                }
                
                let mut meta = HashMap::new();
                meta.insert("path".to_string(), json!(params.file_path));
                meta.insert("edit_count".to_string(), json!(params.edits.len()));
                meta.insert("total_replacements".to_string(), json!(replacements_per_edit.iter().sum::<usize>()));
                meta.insert("replacements_per_edit".to_string(), json!(replacements_per_edit));
                meta.insert("preview_mode".to_string(), json!(preview));

                // Add detailed information about each edit
                let edit_details: Vec<serde_json::Value> = params.edits.iter().enumerate().map(|(i, edit)| {
                    json!({
                        "index": i,
                        "old_string": edit.old_string,
                        "new_string": edit.new_string,
                        "replace_all": edit.replace_all,
                        "replacements_made": replacements_per_edit[i]
                    })
                }).collect();
                meta.insert("edit_details".to_string(), json!(edit_details));

                // Add file size information
                if let Ok(metadata) = std::fs::metadata(&params.file_path) {
                    meta.insert("file_size_bytes".to_string(), json!(metadata.len()));
                }

                ToolResult::Success {
                    output: message,
                    metadata: Some(meta),
                }
            },
            Err(e) => {
                ToolResult::error(format!("MultiEdit {} failed: {}", if preview { "preview" } else { "" }, e))
            }
        }
    }
}