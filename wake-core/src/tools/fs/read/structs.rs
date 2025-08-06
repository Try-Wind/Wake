use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct ReadToolParams {
    /// Path to the file to read
    pub path: String,
    /// Starting line number (optional)
    #[serde(default)]
    pub line_start: Option<u32>,
    /// Ending line number (optional)
    #[serde(default)]
    pub line_end: Option<u32>,
    /// Whether to include line numbers in the output
    #[serde(default)]
    pub show_line_numbers: bool,
}