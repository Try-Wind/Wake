pub mod datasheet_analyzer;
pub mod driver_generator;
pub mod protocol_debugger;
pub mod circuit_analyzer;
pub mod pinout_mapper;
pub mod firmware_flasher;

use crate::tools::types::AnyTool;

pub fn hardware_tools() -> Vec<AnyTool> {
    vec![
        Box::new(datasheet_analyzer::DatasheetAnalyzer),
        Box::new(driver_generator::DriverGenerator),
        Box::new(protocol_debugger::ProtocolDebugger),
        Box::new(circuit_analyzer::CircuitAnalyzer),
        Box::new(pinout_mapper::PinoutMapper),
        Box::new(firmware_flasher::FirmwareFlasher),
    ]
}