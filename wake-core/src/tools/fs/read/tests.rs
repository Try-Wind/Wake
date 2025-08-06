use super::read::ReadTool;
use super::structs::ReadToolParams;
use crate::tools::{Tool, ToolCapability, FsOperationLog};
use wake_llm::ToolDescription;
use tempfile::TempDir;
use std::fs;
use std::sync::Arc;

#[test]
fn test_read_tool_creation() {
    let log = Arc::new(FsOperationLog::new());
    let tool = ReadTool::new(log);
    
    // Verify tool properties
    assert_eq!(tool.name(), "read");
    assert!(!tool.description().is_empty());
    
    // Verify capabilities
    let capabilities = tool.capabilities();
    assert!(capabilities.contains(&ToolCapability::Read));
    assert_eq!(capabilities.len(), 1);
}

#[tokio::test]
async fn test_read_tool_basic_file_reading() {
    // Create a temporary directory with test files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // Create a test file with known content
    let test_content = r#"Hello World
This is a test file
With multiple lines
For reading tests
End of file"#;

    let test_file_path = temp_path.join("test.txt");
    fs::write(&test_file_path, test_content).expect("Failed to write test file");

    let log = Arc::new(FsOperationLog::new());
    let read_tool = ReadTool::new(log);

    // Test 1: Read entire file
    let params = ReadToolParams {
        path: test_file_path.to_string_lossy().to_string(),
        line_start: None,
        line_end: None,
        show_line_numbers: false,
    };

    let result = read_tool.execute(params).await;
    match result {
        crate::tools::ToolResult::Success { output, .. } => {
            assert!(output.contains("Hello World"), "Should contain Hello World");
            assert!(output.contains("End of file"), "Should contain End of file");
            assert!(output.contains("With multiple lines"), "Should contain all lines");
        },
        crate::tools::ToolResult::Error { error, .. } => {
            panic!("Read tool should succeed, got error: {}", error);
        }
    }

    // Test 2: Read file with line numbers
    let params_with_lines = ReadToolParams {
        path: test_file_path.to_string_lossy().to_string(),
        line_start: None,
        line_end: None,
        show_line_numbers: true,
    };

    let result_with_lines = read_tool.execute(params_with_lines).await;
    println!("{}", result_with_lines);
    match result_with_lines {
        crate::tools::ToolResult::Success { output, .. } => {
            assert!(output.contains("   1: "), "Should contain line number 1");
            assert!(output.contains("   5: "), "Should contain line number 5");
            assert!(output.contains("This is a test file"), "Should contain content with line numbers");
        },
        crate::tools::ToolResult::Error { error, .. } => {
            panic!("Read tool with line numbers should succeed, got error: {}", error);
        }
    }
}

#[tokio::test]
async fn test_read_tool_line_range_reading() {
    // Create a temporary directory with test files
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // Create a test file with numbered lines
    let test_content = (1..=20)
        .map(|i| format!("Line {}: Content for line {}", i, i))
        .collect::<Vec<_>>()
        .join("\n");

    let test_file_path = temp_path.join("numbered_test.txt");
    fs::write(&test_file_path, test_content).expect("Failed to write test file");

    let log = Arc::new(FsOperationLog::new());
    let read_tool = ReadTool::new(log);

    // Test 1: Read specific line range (lines 5-10)
    let params_range = ReadToolParams {
        path: test_file_path.to_string_lossy().to_string(),
        line_start: Some(5),
        line_end: Some(10),
        show_line_numbers: true,
    };

    let result_range = read_tool.execute(params_range).await;
    match result_range {
        crate::tools::ToolResult::Success { output, .. } => {
            assert!(output.contains("Line 5: Content for line 5"), "Should contain line 5");
            assert!(output.contains("Line 10: Content for line 10"), "Should contain line 10");
            assert!(!output.contains("Line 4: Content for line 4"), "Should not contain line 4");
            assert!(!output.contains("Line 11: Content for line 11"), "Should not contain line 11");
            
            // Count lines in output to verify range
            let line_count = output.lines().count();
            assert_eq!(line_count, 6, "Should have exactly 6 lines (5-10 inclusive)");
        },
        crate::tools::ToolResult::Error { error, .. } => {
            panic!("Read tool range should succeed, got error: {}", error);
        }
    }

    // Test 2: Read from line 15 to end of file
    let params_from_line = ReadToolParams {
        path: test_file_path.to_string_lossy().to_string(),
        line_start: Some(15),
        line_end: None,
        show_line_numbers: true,
    };

    let result_from_line = read_tool.execute(params_from_line).await;
    match result_from_line {
        crate::tools::ToolResult::Success { output, .. } => {
            assert!(output.contains("Line 15: Content for line 15"), "Should contain line 15");
            assert!(output.contains("Line 20: Content for line 20"), "Should contain line 20");
            assert!(!output.contains("Line 14: Content for line 14"), "Should not contain line 14");
            
            // Should have lines 15-20 (6 lines)
            let line_count = output.lines().count();
            assert_eq!(line_count, 6, "Should have exactly 6 lines (15-20 inclusive)");
        },
        crate::tools::ToolResult::Error { error, .. } => {
            panic!("Read tool from line should succeed, got error: {}", error);
        }
    }

    // Test 3: Test reading non-existent file
    let params_nonexistent = ReadToolParams {
        path: "/nonexistent/path/file.txt".to_string(),
        line_start: None,
        line_end: None,
        show_line_numbers: false,
    };

    let result_nonexistent = read_tool.execute(params_nonexistent).await;
    match result_nonexistent {
        crate::tools::ToolResult::Success { .. } => {
            panic!("Read tool should fail for non-existent file");
        },
        crate::tools::ToolResult::Error { error, .. } => {
            assert!(error.contains("No such file") || error.contains("not found") || error.contains("cannot find") || error.contains("does not exist"), 
                   "Should indicate file not found error, got: {}", error);
        }
    }
}