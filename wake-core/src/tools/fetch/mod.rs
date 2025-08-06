pub mod structs;
pub mod fetch;

#[cfg(test)]
mod tests;

pub use structs::{FetchToolParams, HttpMethod};
pub use fetch::FetchTool;