// Hardware-specific tools for Wake
pub mod circuit_analyzer;
pub mod datasheet_analyzer;
pub mod driver_generator;
pub mod pinout_mapper;
pub mod protocol_debugger;
pub mod timing_calculator;

// Re-export all hardware tools
pub use circuit_analyzer::CircuitAnalyzer;
pub use datasheet_analyzer::DatasheetAnalyzer;
pub use driver_generator::DriverGenerator;
pub use pinout_mapper::PinoutMapper;
pub use protocol_debugger::ProtocolDebugger;
pub use timing_calculator::TimingCalculator;
