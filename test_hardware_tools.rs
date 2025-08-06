// Test script for Wake hardware tools
use wake_core::tools::hardware::*;
use wake_core::tools::Tool;

#[tokio::main]
async fn main() {
    println!("Testing Wake Hardware Tools\n");
    
    // Test Driver Generator
    println!("1. Testing Driver Generator:");
    let driver_gen = DriverGenerator::new();
    let driver_args = DriverGeneratorArgs {
        component: "MPU6050".to_string(),
        platform: "Arduino".to_string(),
        language: "C++".to_string(),
        protocol: "I2C".to_string(),
        features: None,
        include_examples: Some(true),
    };
    
    match driver_gen.execute(driver_args).await {
        wake_core::tools::ToolResult::Success { output, .. } => {
            println!("✓ Driver Generator working!");
            println!("  Generated {} lines of code\n", output.lines().count());
        }
        _ => println!("✗ Driver Generator failed\n"),
    }
    
    // Test Protocol Debugger
    println!("2. Testing Protocol Debugger:");
    let protocol_debug = ProtocolDebugger::new();
    let protocol_args = ProtocolDebuggerArgs {
        protocol: "I2C".to_string(),
        issue: "Device not responding, getting NACK".to_string(),
        hardware_setup: Some("3.3V MCU with 4.7k pull-ups".to_string()),
        parameters: Some("400kHz clock speed".to_string()),
        error_messages: None,
        captured_data: None,
    };
    
    match protocol_debug.execute(protocol_args).await {
        wake_core::tools::ToolResult::Success { output, .. } => {
            println!("✓ Protocol Debugger working!");
            println!("  Generated {} lines of analysis\n", output.lines().count());
        }
        _ => println!("✗ Protocol Debugger failed\n"),
    }
    
    // Test Circuit Analyzer
    println!("3. Testing Circuit Analyzer:");
    let circuit = CircuitAnalyzer::new();
    let circuit_args = CircuitAnalyzerArgs {
        circuit: "I2C pull-up resistor circuit".to_string(),
        analysis_type: "pull-up".to_string(),
        components: Some("4.7k resistors".to_string()),
        conditions: Some("3.3V, 400kHz".to_string()),
    };
    
    match circuit.execute(circuit_args).await {
        wake_core::tools::ToolResult::Success { .. } => {
            println!("✓ Circuit Analyzer working!\n");
        }
        _ => println!("✗ Circuit Analyzer failed\n"),
    }
    
    println!("Hardware tools test complete!");
}