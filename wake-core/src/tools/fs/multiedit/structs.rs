use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[schemars(inline)]
pub struct EditOperation {
    /// The text pattern to find and replace
    pub old_string: String,
    /// The replacement text
    pub new_string: String,
    /// Whether to replace all occurrences (default: false, replaces only first)
    #[serde(default)]
    pub replace_all: bool,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct MultiEditToolParams {
    /// Path to the file to edit
    pub file_path: String,
    /// Array of edit operations to perform sequentially
    pub edits: Vec<EditOperation>,
}