// Hardware-specific tools for Wake
pub mod datasheet_analyzer;
pub mod driver_generator;
pub mod protocol_debugger;
pub mod circuit_analyzer;
pub mod pinout_mapper;
pub mod timing_calculator;

use wake_llm::tools::{Tool, ToolArguments, ToolCall, ToolResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;

// Re-export all hardware tools
pub use datasheet_analyzer::DatasheetAnalyzer;
pub use driver_generator::DriverGenerator;
pub use protocol_debugger::ProtocolDebugger;
pub use circuit_analyzer::CircuitAnalyzer;
pub use pinout_mapper::PinoutMapper;
pub use timing_calculator::TimingCalculator;