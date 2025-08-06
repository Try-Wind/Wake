use crate::tools::{ToolResult, tool};
use super::structs::ReadToolParams;
use super::super::{FsOperationLog, FsOperationType};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct ReadTool {
    operation_log: Arc<FsOperationLog>,
}

impl ReadTool {
    pub fn new(operation_log: Arc<FsOperationLog>) -> Self {
        Self { operation_log }
    }

    fn read_file_content(&self, params: &ReadToolParams) -> io::Result<String> {
        let file = fs::File::open(&params.path)?;
        let reader = BufReader::new(file);
        
        match (params.line_start, params.line_end) {
            // Read specific line range
            (Some(start), Some(end)) => {
                let lines: Result<Vec<(u32, String)>, io::Error> = reader
                    .lines()
                    .enumerate()
                    .filter_map(|(i, line)| {
                        let line_num = i as u32 + 1; // 1-based line numbers
                        if line_num >= start && line_num <= end {
                            Some(line.map(|l| (line_num, l)))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                match lines {
                    Ok(filtered_lines) => Ok(self.format_lines(filtered_lines, params.show_line_numbers)),
                    Err(e) => Err(e)
                }
            },
            // Read from start line to end of file
            (Some(start), None) => {
                let lines: Result<Vec<(u32, String)>, io::Error> = reader
                    .lines()
                    .enumerate()
                    .filter_map(|(i, line)| {
                        let line_num = i as u32 + 1; // 1-based line numbers
                        if line_num >= start {
                            Some(line.map(|l| (line_num, l)))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                match lines {
                    Ok(filtered_lines) => Ok(self.format_lines(filtered_lines, params.show_line_numbers)),
                    Err(e) => Err(e)
                }
            },
            // Read from beginning to end line
            (None, Some(end)) => {
                let lines: Result<Vec<(u32, String)>, io::Error> = reader
                    .lines()
                    .enumerate()
                    .filter_map(|(i, line)| {
                        let line_num = i as u32 + 1; // 1-based line numbers
                        if line_num <= end {
                            Some(line.map(|l| (line_num, l)))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                match lines {
                    Ok(filtered_lines) => Ok(self.format_lines(filtered_lines, params.show_line_numbers)),
                    Err(e) => Err(e)
                }
            },
            // Read entire file
            (None, None) => {
                if params.show_line_numbers {
                    let lines: Result<Vec<(u32, String)>, io::Error> = reader
                        .lines()
                        .enumerate()
                        .map(|(i, line)| {
                            let line_num = i as u32 + 1;
                            line.map(|l| (line_num, l))
                        })
                        .collect();
                    
                    match lines {
                        Ok(numbered_lines) => Ok(self.format_lines(numbered_lines, true)),
                        Err(e) => Err(e)
                    }
                } else {
                    fs::read_to_string(&params.path)
                }
            }
        }
    }

    fn format_lines(&self, lines: Vec<(u32, String)>, show_line_numbers: bool) -> String {
        if show_line_numbers {
            lines
                .iter()
                .map(|(line_num, content)| format!("{:4}: {}", line_num, content))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            lines
                .iter()
                .map(|(_, content)| content.clone())
                .collect::<Vec<_>>()
                .join("\n")
        }
    }
}

#[tool(name = "read", description = r#"Retrieves the contents of a specified file. This is your primary method for inspecting code, configuration, or any other text-based file.

**Usage:**
- An absolute `path` to the file is required.
- For large files, you can read a specific portion by specifying `line_start` and `line_end`. If omitted, the entire file is read (within system limits).
- The output is formatted with line numbers for easy reference, which is crucial context for subsequent `edit` operations.

**Best Practices:**
- When investigating a task, it is often effective to read multiple potentially relevant files in a single turn to build a complete understanding of the context."#, capabilities = [Read])]
impl ReadTool {
    async fn execute(&self, params: ReadToolParams) -> ToolResult {
        let path = Path::new(&params.path);
        
        // Check if file exists
        if !path.exists() {
            return ToolResult::error(format!("File does not exist: {}", params.path));
        }

        // Check if it's a file (not a directory)
        if !path.is_file() {
            return ToolResult::error(format!("Path is not a file: {}", params.path));
        }

        // Read the file
        match self.read_file_content(&params) {
            Ok(content) => {
                // Log the read operation
                self.operation_log.log_operation(FsOperationType::Read, params.path.clone()).await;

                let mut meta = HashMap::new();
                meta.insert("path".to_string(), json!(params.path));
                meta.insert("total_lines".to_string(), json!(content.lines().count()));
                
                if let Some(start) = params.line_start {
                    meta.insert("line_start".to_string(), json!(start));
                }
                if let Some(end) = params.line_end {
                    meta.insert("line_end".to_string(), json!(end));
                }

                ToolResult::Success {
                    output: content,
                    metadata: Some(meta),
                }
            },
            Err(e) => ToolResult::error(format!("Failed to read file: {}", e))
        }
    }
}
