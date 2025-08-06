#[cfg(test)]
mod integration_tests {
    use std::sync::Arc;
    use tempfile::tempdir;
    use crate::tools::{
        Tool, FsOperationLog, 
        LsTool, FindTool, WriteTool, ReadTool, EditTool, MultiEditTool
    };
    use crate::tools::fs::{
        ls::structs::LsToolParams,
        find::structs::FindToolParams,
        write::structs::WriteToolParams,
        read::structs::ReadToolParams,
        edit::structs::EditToolParams,
        multiedit::structs::{MultiEditToolParams, EditOperation}
    };

    /// Test 1: Basic file operations workflow
    /// Tests: write -> ls -> read -> edit -> multiedit in sequence
    #[tokio::test]
    async fn test_basic_file_operations_workflow() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create shared operation log
        let fs_log = Arc::new(FsOperationLog::new());
        
        // Initialize tools
        let ls_tool = LsTool::new();
        let write_tool = WriteTool::new(fs_log.clone());
        let read_tool = ReadTool::new(fs_log.clone());
        let edit_tool = EditTool::new(fs_log.clone());
        let multiedit_tool = MultiEditTool::new(fs_log.clone());
        
        // 1. List empty directory
        let ls_result = ls_tool.execute(LsToolParams {
            directory: temp_path.to_string_lossy().to_string(),
            recursive: false,
            show_hidden: false,
            long_format: false,
            max_depth: None,
            max_files: None,
        }).await;
        assert!(ls_result.is_success());
        
        // 2. Write a file
        let file_path = temp_path.join("test.txt");
        let write_result = write_tool.execute(WriteToolParams {
            path: file_path.to_string_lossy().to_string(),
            content: "Hello, World!\nThis is a test file.".to_string(),
        }).await;
        assert!(write_result.is_success());
        
        // 3. List directory again - should show the new file
        let ls_result = ls_tool.execute(LsToolParams {
            directory: temp_path.to_string_lossy().to_string(),
            recursive: false,
            show_hidden: false,
            long_format: false,
            max_depth: None,
            max_files: None,
        }).await;
        assert!(ls_result.is_success());
        if let crate::tools::types::ToolResult::Success { output, .. } = ls_result {
            assert!(output.contains("test.txt"));
        }
        
        // 4. Read the file
        let read_result = read_tool.execute(ReadToolParams {
            path: file_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(read_result.is_success());
        if let crate::tools::types::ToolResult::Success { output, .. } = read_result {
            assert!(output.contains("Hello, World!"));
            assert!(output.contains("This is a test file."));
        }
        
        // 5. Edit the file (should work since we read it)
        let edit_result = edit_tool.execute(EditToolParams {
            path: file_path.to_string_lossy().to_string(),
            old_string: "Hello, World!".to_string(),
            new_string: "Hello, Universe!".to_string(),
            replace_all: false,
        }).await;
        assert!(edit_result.is_success());
        
        // 6. Use multiedit for multiple replacements
        let multiedit_result = multiedit_tool.execute(MultiEditToolParams {
            file_path: file_path.to_string_lossy().to_string(),
            edits: vec![
                EditOperation {
                    old_string: "Universe".to_string(),
                    new_string: "Galaxy".to_string(),
                    replace_all: false,
                },
                EditOperation {
                    old_string: "test file".to_string(),
                    new_string: "example document".to_string(),
                    replace_all: false,
                },
            ],
        }).await;
        assert!(multiedit_result.is_success());
        
        // 7. Read final result to verify all edits
        let final_read = read_tool.execute(ReadToolParams {
            path: file_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(final_read.is_success());
        if let crate::tools::types::ToolResult::Success { output, .. } = final_read {
            assert!(output.contains("Hello, Galaxy!"));
            assert!(output.contains("example document"));
        }
    }

    /// Test 2: Edit validation - files must be read before editing
    /// Tests that edit and multiedit fail if file hasn't been read first
    #[tokio::test]
    async fn test_edit_requires_read_first() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create shared operation log
        let fs_log = Arc::new(FsOperationLog::new());
        
        // Initialize tools
        let write_tool = WriteTool::new(fs_log.clone());
        let read_tool = ReadTool::new(fs_log.clone());
        let edit_tool = EditTool::new(fs_log.clone());
        let multiedit_tool = MultiEditTool::new(fs_log.clone());
        
        // Create two test files
        let file1_path = temp_path.join("file1.txt");
        let file2_path = temp_path.join("file2.txt");
        
        // Write both files
        let _ = write_tool.execute(WriteToolParams {
            path: file1_path.to_string_lossy().to_string(),
            content: "Content of file 1".to_string(),
        }).await;
        
        let _ = write_tool.execute(WriteToolParams {
            path: file2_path.to_string_lossy().to_string(),
            content: "Content of file 2".to_string(),
        }).await;
        
        // Try to edit file1 without reading it first - should fail
        let edit_result = edit_tool.execute(EditToolParams {
            path: file1_path.to_string_lossy().to_string(),
            old_string: "Content".to_string(),
            new_string: "Modified content".to_string(),
            replace_all: false,
        }).await;
        assert!(edit_result.is_error());
        if let crate::tools::types::ToolResult::Error { error, .. } = edit_result {
            assert!(error.contains("must be read first"));
        }
        
        // Try to multiedit file2 without reading it first - should fail
        let multiedit_result = multiedit_tool.execute(MultiEditToolParams {
            file_path: file2_path.to_string_lossy().to_string(),
            edits: vec![
                EditOperation {
                    old_string: "Content".to_string(),
                    new_string: "Modified content".to_string(),
                    replace_all: false,
                },
            ],
        }).await;
        assert!(multiedit_result.is_error());
        if let crate::tools::types::ToolResult::Error { error, .. } = multiedit_result {
            assert!(error.contains("must be read first"));
        }
        
        // Now read file1 and try editing again - should succeed
        let _ = read_tool.execute(ReadToolParams {
            path: file1_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        
        let edit_result = edit_tool.execute(EditToolParams {
            path: file1_path.to_string_lossy().to_string(),
            old_string: "Content".to_string(),
            new_string: "Modified content".to_string(),
            replace_all: false,
        }).await;
        assert!(edit_result.is_success());
        
        // Now read file2 and try multiediting - should succeed
        let _ = read_tool.execute(ReadToolParams {
            path: file2_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        
        let multiedit_result = multiedit_tool.execute(MultiEditToolParams {
            file_path: file2_path.to_string_lossy().to_string(),
            edits: vec![
                EditOperation {
                    old_string: "Content".to_string(),
                    new_string: "Modified content".to_string(),
                    replace_all: false,
                },
            ],
        }).await;
        assert!(multiedit_result.is_success());
    }

    /// Test 3: Complex multi-file operations with find tool
    /// Tests find, create multiple files, and perform various operations
    #[tokio::test]
    async fn test_complex_multi_file_operations() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        
        // Create shared operation log
        let fs_log = Arc::new(FsOperationLog::new());
        
        // Initialize tools
        let find_tool = FindTool::new();
        let write_tool = WriteTool::new(fs_log.clone());
        let read_tool = ReadTool::new(fs_log.clone());
        let edit_tool = EditTool::new(fs_log.clone());
        
        // Create multiple files with different extensions
        let files = vec![
            ("config.json", r#"{"name": "test", "version": "1.0"}"#),
            ("readme.txt", "This is a readme file\nWith multiple lines"),
            ("script.py", "print('Hello, Python!')\nprint('Second line')"),
            ("data.json", r#"{"items": [1, 2, 3], "active": true}"#),
        ];
        
        // Write all files
        for (filename, content) in &files {
            let file_path = temp_path.join(filename);
            let write_result = write_tool.execute(WriteToolParams {
                path: file_path.to_string_lossy().to_string(),
                content: content.to_string(),
            }).await;
            assert!(write_result.is_success());
        }
        
        // Use find to search for content in files (since that's the default)
        let find_result = find_tool.execute(FindToolParams {
            pattern: "name".to_string(), // Search for "name" in file contents
            path: Some(temp_path.to_string_lossy().to_string()),
            include_extensions: Some("json".to_string()),
            exclude_patterns: None,
            max_results: 100,
            case_sensitive: false,
            find_type: crate::tools::fs::find::structs::FindType::Content,
            show_line_numbers: false,
            context_lines: None,
            whole_word: false,
        }).await;
        assert!(find_result.is_success());
        
        // Read and edit the config.json file
        let config_path = temp_path.join("config.json");
        let read_result = read_tool.execute(ReadToolParams {
            path: config_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(read_result.is_success());
        
        // Edit the version in config.json
        let edit_result = edit_tool.execute(EditToolParams {
            path: config_path.to_string_lossy().to_string(),
            old_string: r#""version": "1.0""#.to_string(),
            new_string: r#""version": "2.0""#.to_string(),
            replace_all: false,
        }).await;
        assert!(edit_result.is_success());
        
        // Read and modify the Python script
        let script_path = temp_path.join("script.py");
        let read_result = read_tool.execute(ReadToolParams {
            path: script_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(read_result.is_success());
        
        let edit_result = edit_tool.execute(EditToolParams {
            path: script_path.to_string_lossy().to_string(),
            old_string: "Hello, Python!".to_string(),
            new_string: "Hello, World from Python!".to_string(),
            replace_all: false,
        }).await;
        assert!(edit_result.is_success());
        
        // Verify final state by reading modified files
        let final_config_read = read_tool.execute(ReadToolParams {
            path: config_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(final_config_read.is_success());
        if let crate::tools::types::ToolResult::Success { output, .. } = final_config_read {
            assert!(output.contains(r#""version": "2.0""#));
        }
        
        let final_script_read = read_tool.execute(ReadToolParams {
            path: script_path.to_string_lossy().to_string(),
            line_start: None,
            line_end: None,
            show_line_numbers: false,
        }).await;
        assert!(final_script_read.is_success());
        if let crate::tools::types::ToolResult::Success { output, .. } = final_script_read {
            assert!(output.contains("Hello, World from Python!"));
        }
        
        // Verify operation log tracked everything
        let operations = fs_log.get_all_operations().await;
        assert!(operations.len() >= 8); // At least 4 writes + 4 reads + 2 edits
        
        let read_files = fs_log.get_read_files().await;
        assert!(read_files.contains(&config_path.to_string_lossy().to_string()));
        assert!(read_files.contains(&script_path.to_string_lossy().to_string()));
    }
}