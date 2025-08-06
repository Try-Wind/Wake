use super::coder::CoderBrain;
use crate::agent::{Agent, Brain, StdoutEventManager, ThinkerContext};
use crate::logging::LoggingConfig;
use crate::tools::AnyTool;
use wake_llm::ToolCallMethod;
use wake_llm::{ChatMessage, ChatMessageContent, client::LlmClient};
use tokio::sync::RwLock;
use std::sync::Arc;
use tempfile::TempDir;
use std::sync::Once;

static INIT_LOGGING: Once = Once::new();
fn init_test_logging() {
    INIT_LOGGING.call_once(|| {
        let _ = LoggingConfig::from_env().init();
    });
}

// Helper function to create a coder agent with full toolbox
async fn create_coder_agent_with_goal(goal: &str) -> impl Agent {
    let llm_client = Arc::new(LlmClient::first_from_env().expect("No LLM provider available"));
    let model = llm_client.default_model().await.expect("default model");
    println!("using model: {:?}", model);
    
    // Create shared storage for todo tools
    let todo_storage = Arc::new(crate::tools::TodoStorage::new());
    
    // Create shared operation log for file system tools
    let fs_log = Arc::new(crate::tools::FsOperationLog::new());
    
    let bash = Box::new(crate::tools::BashTool::new());
    let edit = Box::new(crate::tools::EditTool::new(fs_log.clone()));
    let multiedit = Box::new(crate::tools::MultiEditTool::new(fs_log.clone()));
    let fetch = Box::new(crate::tools::FetchTool::new());
    let find = Box::new(crate::tools::FindTool::new());
    let ls = Box::new(crate::tools::LsTool::new());
    let read = Box::new(crate::tools::ReadTool::new(fs_log.clone()));
    let todoread = Box::new(crate::tools::TodoReadTool::new(todo_storage.clone()));
    let todowrite = Box::new(crate::tools::TodoWriteTool::new(todo_storage.clone()));
    let write = Box::new(crate::tools::WriteTool::new(fs_log.clone()));
    let toolbox: Vec<Box<dyn AnyTool>> = vec![bash, edit, multiedit, fetch, find, ls, read, todoread, todowrite, write];
    
    crate::agent::AgentBuilder::new(Box::new(CoderBrain::new(llm_client, model)))
        .goal(goal)
        .tools(toolbox)
        .sudo()
        .build()
}


#[tokio::test]
async fn test_coder_brain_creation() {
    let llm_client = Arc::new(LlmClient::first_from_env().expect("No LLM provider available"));
    let model = llm_client.default_model().await.expect("default model");
    
    let brain = CoderBrain::new(llm_client, model.clone());
    
    assert_eq!(brain.model, model);
}

#[tokio::test]
async fn test_coder_brain_think_simple() {
    let llm_client = Arc::new(LlmClient::first_from_env().expect("No LLM provider available"));
    let model = llm_client.default_model().await.expect("default model");
    let mut brain = CoderBrain::new(llm_client, model);
    
    // Create test context with a simple message
    let context = ThinkerContext {
        trace: Arc::new(RwLock::new(vec![ChatMessage::User {
            content: ChatMessageContent::Text("Say hello".to_string()),
            name: None,
        }])),
        available_tools: vec![],
        method: ToolCallMethod::FunctionCall
    };
    
    let result = brain.next_step(context).await;
    assert!(result.is_ok(), "Brain should successfully process simple message {:?}", result);
    let response = result.unwrap().unwrap();
    match response {
        ChatMessage::Assistant { content, .. } => {
            assert!(content.is_some(), "Assistant should provide content");
        }
        _ => panic!("Expected Assistant message"),
    }
}

// Integration tests with real coding tasks and temporary files

#[tokio::test]
async fn test_coder_integration_simple_file_creation() {
    init_test_logging();
    // Create a temporary directory for this test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Change to the temporary directory
    std::env::set_current_dir(temp_path).expect("Failed to change directory");
    
    // Create a coder agent with full toolbox and goal
    let agent = create_coder_agent_with_goal(
        "Create a Python file called 'hello.py' that prints 'Hello, World!' when executed. The file should contain a proper function and a main guard."
    ).await;
    
    println!("🧪 Test: Creating hello.py in temp directory: {:?}", temp_path);
    
    // Run the agent with stdout event streaming
    let result = agent
        .with_event_handler(StdoutEventManager::new())
        .run().await;
    
    // Verify the agent completed successfully
    assert!(result.is_ok(), "Coder agent should complete successfully");
    let agent_result = result.unwrap();
    assert!(agent_result.success, "Agent should report success");
    
    println!("📝 Agent completed with {} messages", agent_result.trace.len());
    
    // Verify the file was created
    let hello_py_path = temp_path.join("hello.py");
    assert!(hello_py_path.exists(), "hello.py should be created in temp directory");
    
    // Read and verify the file content
    let content = std::fs::read_to_string(&hello_py_path)
        .expect("Should be able to read hello.py");
    
    println!("📄 Created file content:\n{}", content);
    
    // Basic checks for expected content
    assert!(content.contains("Hello, World!"), "File should contain 'Hello, World!'");
    assert!(content.contains("def ") || content.contains("print"), "File should contain function or print statement");
    
    // Cleanup is automatic when TempDir is dropped
}



#[tokio::test]
async fn test_multi_turn_conversation() {
    init_test_logging();
    // Create a temporary directory for this test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    std::env::set_current_dir(temp_path).expect("Failed to change directory");
    
    // Create a coder agent with full toolbox and goal
    let goal = "Create a Python file called 'hello.py' that prints 'Hello, World!' when executed.";
    let mut agent = create_coder_agent_with_goal(goal).await;

    // get a controller
    let controller = agent.controller();

    ////////////// TURN 1
    // Run the agent with stdout event streaming
    println!("> {}",goal);
    let result = tokio::spawn(async move {
        agent
            .with_event_handler(StdoutEventManager::new())
            .run().await
    });
    let _ = controller.wait_turn(None).await;
        
    // Verify the file was created
    let hello_py_path = temp_path.join("hello.py");
    assert!(hello_py_path.exists(), "hello.py should be created in temp directory");
    
    // Read and verify the file content
    let content = std::fs::read_to_string(&hello_py_path)
        .expect("Should be able to read hello.py");
    
    // Basic checks for expected content
    assert!(content.contains("Hello, World!"), "File should contain 'Hello, World!'");
    assert!(content.contains("def ") || content.contains("print"), "File should contain function or print statement");
    
    ////////////// TURN 2
    println!("> {}", "modify the file so that the text output in green");
    let _ = controller.send_user_input("I want the text to be output in green".to_string()).await;
    let _ = controller.wait_turn(None).await;
}


#[tokio::test]
async fn test_coder_integration_bug_fix_task() {
    init_test_logging();
    // Create a temporary directory for this test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();
    
    // Change to the temporary directory
    std::env::set_current_dir(temp_path).expect("Failed to change directory");
    
    // Create a buggy Python file
    let buggy_code = r#"def calculate_average(numbers):
    total = 0
    for num in numbers:
        total += num
    return total / 0  # Bug: division by zero instead of len(numbers)

def main():
    data = [1, 2, 3, 4, 5]
    avg = calculate_average(data)
    print(f"Average: {avg}")

if __name__ == "__main__":
    main()
"#;
    
    let buggy_file_path = temp_path.join("calculator.py");
    std::fs::write(&buggy_file_path, buggy_code)
        .expect("Failed to write buggy file");
    
    println!("🐛 Created buggy file: {:?}", buggy_file_path);
    
    // Create a coder agent with full toolbox and goal
    let agent = create_coder_agent_with_goal(
        "There's a bug in calculator.py. Please read the file, identify the bug, and fix it so the code calculates the average correctly."
    ).await;
    
    println!("🧪 Test: Fixing bug in calculator.py");
    
    // Run the agent with stdout event streaming  
    let result = agent
        .with_event_handler(StdoutEventManager::new())
        .run().await;
    
    // Verify the agent completed successfully
    assert!(result.is_ok(), "Coder agent should complete successfully");
    let agent_result = result.unwrap();
    assert!(agent_result.success, "Agent should report success");
    
    println!("🔧 Agent completed bug fix with {} messages", agent_result.trace.len());
    println!("{:#?}", agent_result.trace);
    
    // Verify the file still exists
    assert!(buggy_file_path.exists(), "calculator.py should still exist");
    
    // Read the fixed content
    let fixed_content = std::fs::read_to_string(&buggy_file_path)
        .expect("Should be able to read fixed calculator.py");
    
    // Verify the bug was fixed
    assert!(!fixed_content.contains("/ 0"), "Division by zero should be fixed");
    assert!(fixed_content.contains("len(numbers)") || fixed_content.contains("count"), 
           "Should use proper length calculation");
    assert!(fixed_content.contains("calculate_average"), "Function should still exist");
    assert!(fixed_content.contains("def main"), "Main function should still exist");
    
    // Cleanup is automatic when TempDir is dropped
}

