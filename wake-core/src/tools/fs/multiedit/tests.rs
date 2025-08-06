use super::multiedit::MultiEditTool;
use super::structs::{EditOperation, MultiEditToolParams};
use crate::tools::{FsOperationLog, Tool, ToolCapability};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use wake_llm::ToolDescription;

#[test]
fn test_multiedit_tool_permissions() {
    let log = Arc::new(FsOperationLog::new());
    let tool = MultiEditTool::new(log);
    let perms = tool.capabilities();
    assert!(perms.contains(&ToolCapability::Read));
    assert!(perms.contains(&ToolCapability::Write));
    assert_eq!(perms.len(), 2);
}

#[tokio::test]
async fn test_multiedit_tool_creation() {
    let log = Arc::new(FsOperationLog::new());
    let tool = MultiEditTool::new(log);
    assert_eq!(tool.name(), "multiedit");
    assert!(!tool.description().is_empty());
}

#[tokio::test]
async fn test_multiedit_multiple_replacements() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "Hello World, Hello Universe").unwrap();

    let log = Arc::new(FsOperationLog::new());
    // First read the file to satisfy the logging requirement
    log.log_operation(
        crate::tools::FsOperationType::Read,
        file_path.to_string_lossy().to_string(),
    )
    .await;

    let tool = MultiEditTool::new(log);
    let params = MultiEditToolParams {
        file_path: file_path.to_string_lossy().to_string(),
        edits: vec![
            EditOperation {
                old_string: "Hello".to_string(),
                new_string: "Hi".to_string(),
                replace_all: false,
            },
            EditOperation {
                old_string: "World".to_string(),
                new_string: "Earth".to_string(),
                replace_all: true,
            },
        ],
    };

    let result = tool.execute(params).await;
    assert!(result.is_success());

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Hi Earth, Hello Universe");
}

#[tokio::test]
async fn test_multiedit_preview_diff_output() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "line1\nHello World\nline3\nGoodbye World").unwrap();

    let log = Arc::new(FsOperationLog::new());
    log.log_operation(
        crate::tools::FsOperationType::Read,
        file_path.to_string_lossy().to_string(),
    )
    .await;

    let tool = MultiEditTool::new(log);
    let params = MultiEditToolParams {
        file_path: file_path.to_string_lossy().to_string(),
        edits: vec![
            EditOperation {
                old_string: "Hello".to_string(),
                new_string: "Hi".to_string(),
                replace_all: false,
            },
            EditOperation {
                old_string: "Goodbye".to_string(),
                new_string: "Farewell".to_string(),
                replace_all: false,
            },
        ],
    };

    // Test preview - should return Some(ToolResult) with diff
    let preview_result = tool.execute_preview(params.clone()).await;
    assert!(preview_result.is_some());

    let preview = preview_result.unwrap();
    assert!(preview.is_success());

    // Preview should contain diff output showing both changes
    let output = match preview {
        crate::tools::ToolResult::Success { output, .. } => output,
        _ => panic!("Expected success result"),
    };

    println!("MultiEdit diff output:\n{}", output);

    // Should contain diff markers for both changes
    assert!(output.contains("-")); // Deletion markers
    assert!(output.contains("+")); // Addition markers
    assert!(output.contains("Hello")); // First old content
    assert!(output.contains("Hi")); // First new content
    assert!(output.contains("Goodbye")); // Second old content
    assert!(output.contains("Farewell")); // Second new content

    // Should contain ANSI color codes
    assert!(output.contains("\x1b[48;5;88;37m")); // Red background for deletions
    assert!(output.contains("\x1b[48;5;28;37m")); // Green background for additions

    // Original file should be unchanged after preview
    let original_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(original_content, "line1\nHello World\nline3\nGoodbye World");
}
