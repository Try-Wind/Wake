use super::structs::WriteToolParams;
use super::write::WriteTool;
use crate::tools::{FsOperationLog, Tool, ToolCapability};
use std::fs;
use std::sync::Arc;
use tempfile::tempdir;
use wake_llm::ToolDescription;

#[test]
fn test_write_tool_permissions() {
    let log = Arc::new(FsOperationLog::new());
    let tool = WriteTool::new(log);
    let perms = tool.capabilities();
    assert!(perms.contains(&ToolCapability::Write));
    assert_eq!(perms.len(), 1);
}

#[tokio::test]
async fn test_write_tool_creation() {
    let log = Arc::new(FsOperationLog::new());
    let tool = WriteTool::new(log);
    assert_eq!(tool.name(), "write");
    assert!(!tool.description().is_empty());
}

#[tokio::test]
async fn test_write_new_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("new_file.txt");

    let log = Arc::new(FsOperationLog::new());
    let tool = WriteTool::new(log);
    let params = WriteToolParams {
        path: file_path.to_string_lossy().to_string(),
        content: "Hello, World!".to_string(),
    };

    let result = tool.execute(params).await;
    assert!(result.is_success());
    if let crate::tools::types::ToolResult::Success { output, .. } = result {
        assert!(output.contains("created"));
    } else {
        panic!("Expected success result");
    }

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "Hello, World!");
}
