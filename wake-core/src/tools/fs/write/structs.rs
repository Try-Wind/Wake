use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct WriteToolParams {
    /// Path to the file to write
    pub path: String,
    /// Content to write to the file
    pub content: String,
}