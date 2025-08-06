use super::edit::EditTool;
use super::structs::EditToolParams;
use crate::tools::{FsOperationLog, Tool, ToolCapability};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use wake_llm::ToolDescription;

#[test]
fn test_edit_tool_permissions() {
    let log = Arc::new(FsOperationLog::new());
    let tool = EditTool::new(log);
    let perms = tool.capabilities();
    assert!(perms.contains(&ToolCapability::Read));
    assert!(perms.contains(&ToolCapability::Write));
    assert_eq!(perms.len(), 2);
}

#[tokio::test]
async fn test_edit_tool_creation() {
    let log = Arc::new(FsOperationLog::new());
    let tool = EditTool::new(log);
    assert_eq!(tool.name(), "edit");
    assert!(!tool.description().is_empty());
}

#[tokio::test]
async fn test_edit_file_replacement() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "Hello World").unwrap();

    let log = Arc::new(FsOperationLog::new());
    // First read the file to satisfy the logging requirement
    log.log_operation(
        crate::tools::FsOperationType::Read,
        file_path.to_string_lossy().to_string(),
    )
    .await;

    let tool = EditTool::new(log);
    let params = EditToolParams {
        path: file_path.to_string_lossy().to_string(),
        old_string: "Hello".to_string(),
        new_string: "Hi".to_string(),
        replace_all: false,
    };

    let result = tool.execute(params).await;
    assert!(result.is_success());

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Hi World");
}

#[tokio::test]
async fn test_edit_preview_functionality() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "Hello World\nSecond line\nThird line").unwrap();

    let log = Arc::new(FsOperationLog::new());
    // First read the file to satisfy the logging requirement
    log.log_operation(
        crate::tools::FsOperationType::Read,
        file_path.to_string_lossy().to_string(),
    )
    .await;

    let tool = EditTool::new(log);
    let params = EditToolParams {
        path: file_path.to_string_lossy().to_string(),
        old_string: "Hello".to_string(),
        new_string: "Hi".to_string(),
        replace_all: false,
    };

    // Test preview - should return Some(ToolResult) with diff
    let preview_result = tool.execute_preview(params.clone()).await;
    assert!(preview_result.is_some());

    let preview = preview_result.unwrap();
    assert!(preview.is_success());

    // Preview should contain diff output
    let output = match preview {
        crate::tools::ToolResult::Success { output, .. } => output,
        _ => panic!("Expected success result"),
    };

    // Should contain diff markers
    assert!(output.contains("-")); // Deletion marker
    assert!(output.contains("+")); // Addition marker
    assert!(output.contains("Hello")); // Old content
    assert!(output.contains("Hi")); // New content

    // Original file should be unchanged after preview
    let original_content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(original_content, "Hello World\nSecond line\nThird line");
}

#[test]
fn test_myers_diff_algorithm() {
    let log = Arc::new(FsOperationLog::new());
    let tool = EditTool::new(log);

    let before = "line1\nline2\nline3";
    let after = "line1\nmodified line2\nline3";

    let diff = tool.myers_diff(before, after);
    println!("Single change diff output:\n{}", diff);

    // Should contain ANSI color codes for deletions and additions
    assert!(diff.contains("\x1b[48;5;88;37m")); // Red background for deletions
    assert!(diff.contains("\x1b[48;5;28;37m")); // Green background for additions
    assert!(diff.contains("line2")); // Original line
    assert!(diff.contains("modified line2")); // Modified line
    assert!(diff.contains("\x1b[2;37m")); // Dim gray for line numbers
}

#[test]
fn test_myers_diff_no_changes() {
    let log = Arc::new(FsOperationLog::new());
    let tool = EditTool::new(log);

    let content = "line1\nline2\nline3";
    let diff = tool.myers_diff(content, content);
    println!("No changes diff output:\n{}", diff);

    // Should indicate no changes
    assert_eq!(diff, "No changes");
}

#[test]
fn test_myers_diff_multiple_changes() {
    let log = Arc::new(FsOperationLog::new());
    let tool = EditTool::new(log);

    let before = "line1\nline2\nline3\nline4";
    let after = "line1\nmodified line2\nline3\nmodified line4\nextra line";

    let diff = tool.myers_diff(before, after);

    // Debug: print the actual diff to see what we get
    println!("Actual diff output:\n{}", diff);

    // Should contain multiple change markers
    assert!(diff.contains("line2"));
    assert!(diff.contains("modified line2"));
    assert!(diff.contains("line4"));
    assert!(diff.contains("modified line4"));
    assert!(diff.contains("extra line"));

    // Should have line numbers
    assert!(diff.contains("1")); // Line number 1
    assert!(diff.contains("2")); // Line number 2
    assert!(diff.contains("4")); // Line number 4
}

#[tokio::test]
async fn test_execute_vs_preview_behavior() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.txt");
    fs::write(&file_path, "Original content").unwrap();

    let log = Arc::new(FsOperationLog::new());
    log.log_operation(
        crate::tools::FsOperationType::Read,
        file_path.to_string_lossy().to_string(),
    )
    .await;

    let tool = EditTool::new(log);
    let params = EditToolParams {
        path: file_path.to_string_lossy().to_string(),
        old_string: "Original".to_string(),
        new_string: "Modified".to_string(),
        replace_all: false,
    };

    // Preview should not modify file
    let preview_result = tool.execute_preview(params.clone()).await;
    assert!(preview_result.is_some());
    let content_after_preview = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content_after_preview, "Original content");

    // Execute should modify file
    let execute_result = tool.execute(params).await;
    assert!(execute_result.is_success());
    let content_after_execute = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content_after_execute, "Modified content");
}
