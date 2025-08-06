// Examples module for wake-llm
// 
// This module contains practical examples demonstrating various use cases:
//
// - basic_query.rs: Simple chat completion
// - query_with_history.rs: Multi-turn conversation with context
// - streaming_query.rs: Real-time streaming responses  
// - function_calling.rs: Tool/function calling capabilities
//
// To run examples:
// cargo run --example basic_query
// cargo run --example query_with_history
// cargo run --example streaming_query
// cargo run --example function_calling

pub mod basic_query;
pub mod query_with_history;
pub mod streaming_query;
pub mod function_calling;