pub mod bash;
pub mod fetch;
pub mod fs;
pub mod hardware;
pub mod highlight;
pub mod todo;
pub mod types;

#[cfg(test)]
mod tests_llm;

pub use types::{
    AnyTool, AnyToolBox, Tool, ToolCall, ToolCapability, ToolEmptyParams, ToolError, ToolResult,
};
pub use wake_macros::tool;

// Re-export all tools
pub use bash::BashTool;
pub use fetch::FetchTool;
pub use fs::{
    EditTool, FindTool, FsOperation, FsOperationLog, FsOperationSummary, FsOperationType, LsTool,
    MultiEditTool, ReadTool, WriteTool,
};
pub use todo::{
    TodoItem, TodoItemInput, TodoReadTool, TodoStatus, TodoStorage, TodoWriteParams, TodoWriteTool,
};
