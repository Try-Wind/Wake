use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::tools::{Tool, ToolResult, ToolCapability};
use wake_llm::ToolDescription;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProtocolDebuggerArgs {
    /// Communication protocol to debug (I2C, SPI, UART, CAN, OneWire)
    pub protocol: String,
    
    /// The issue or error being experienced
    pub issue: String,
    
    /// Hardware setup details (voltage levels, pull-up resistors, etc.)
    pub hardware_setup: Option<String>,
    
    /// Communication parameters (baud rate, clock speed, etc.)
    pub parameters: Option<String>,
    
    /// Error messages or symptoms
    pub error_messages: Option<String>,
    
    /// Captured data or logic analyzer output (if available)
    pub captured_data: Option<String>,
}

pub struct ProtocolDebugger;

impl ProtocolDebugger {
    pub fn new() -> Self {
        Self
    }
    
    fn analyze_i2c_issue(&self, args: &ProtocolDebuggerArgs) -> String {
        let mut analysis = String::from("## I2C Protocol Debug Analysis\n\n");
        
        analysis.push_str("### Common I2C Issues Check:\n\n");
        
        // Check for common I2C problems
        if args.issue.to_lowercase().contains("nack") || args.issue.to_lowercase().contains("not acknowledged") {
            analysis.push_str("**NACK (Not Acknowledged) Issues:**\n");
            analysis.push_str("1. **Wrong Device Address**: Verify the I2C address (7-bit vs 8-bit confusion)\n");
            analysis.push_str("   - Many datasheets show 8-bit address, but most libraries expect 7-bit\n");
            analysis.push_str("   - Try shifting address right by 1 bit: `address >> 1`\n\n");
            analysis.push_str("2. **Device Not Powered**: Check VCC and GND connections\n");
            analysis.push_str("3. **Device in Reset**: Some devices need time after power-up\n");
            analysis.push_str("   - Add 100ms delay after power-on\n\n");
        }
        
        if args.issue.to_lowercase().contains("garbage") || args.issue.to_lowercase().contains("wrong data") {
            analysis.push_str("**Data Corruption Issues:**\n");
            analysis.push_str("1. **Pull-up Resistors**: Check pull-up resistor values\n");
            analysis.push_str("   - Typical: 4.7kΩ for 100kHz, 2.2kΩ for 400kHz\n");
            analysis.push_str("   - Formula: R_min = (V_dd - 0.4V) / 3mA\n\n");
            analysis.push_str("2. **Clock Stretching**: Some devices need clock stretching support\n");
            analysis.push_str("3. **Bus Capacitance**: Long wires increase capacitance\n");
            analysis.push_str("   - Maximum: 400pF for standard mode\n\n");
        }
        
        if args.issue.to_lowercase().contains("stuck") || args.issue.to_lowercase().contains("hang") {
            analysis.push_str("**Bus Stuck/Hanging Issues:**\n");
            analysis.push_str("1. **SDA Stuck Low**: Device might be holding SDA low\n");
            analysis.push_str("   ```c\n");
            analysis.push_str("   // Recovery sequence: Toggle SCL 9 times\n");
            analysis.push_str("   for(int i = 0; i < 9; i++) {\n");
            analysis.push_str("       SCL_HIGH();\n");
            analysis.push_str("       delay_us(5);\n");
            analysis.push_str("       SCL_LOW();\n");
            analysis.push_str("       delay_us(5);\n");
            analysis.push_str("   }\n");
            analysis.push_str("   // Send STOP condition\n");
            analysis.push_str("   ```\n\n");
            analysis.push_str("2. **Multi-Master Conflict**: Check if multiple masters on bus\n");
        }
        
        // Add hardware setup specific advice
        if let Some(setup) = &args.hardware_setup {
            if setup.contains("3.3") && setup.contains("5") {
                analysis.push_str("**⚠️ Voltage Level Mismatch Detected:**\n");
                analysis.push_str("- Using 3.3V and 5V devices together requires level shifting\n");
                analysis.push_str("- Options:\n");
                analysis.push_str("  1. Use I2C-safe level shifter (e.g., PCA9306)\n");
                analysis.push_str("  2. Use MOSFETs for bidirectional level shifting\n");
                analysis.push_str("  3. Check if 5V device is 3.3V tolerant\n\n");
            }
        }
        
        analysis.push_str("### Debugging Steps:\n\n");
        analysis.push_str("1. **Scan for Devices**:\n");
        analysis.push_str("   ```c\n");
        analysis.push_str("   for(uint8_t addr = 1; addr < 127; addr++) {\n");
        analysis.push_str("       if(i2c_start(addr << 1) == 0) {\n");
        analysis.push_str("           printf(\"Device found at 0x%02X\\n\", addr);\n");
        analysis.push_str("       }\n");
        analysis.push_str("       i2c_stop();\n");
        analysis.push_str("   }\n");
        analysis.push_str("   ```\n\n");
        
        analysis.push_str("2. **Check Signal Quality**: Use oscilloscope or logic analyzer\n");
        analysis.push_str("3. **Verify Timing**: Ensure setup and hold times are met\n");
        analysis.push_str("4. **Test with Lower Speed**: Try 10kHz or 50kHz first\n");
        
        analysis
    }
    
    fn analyze_spi_issue(&self, args: &ProtocolDebuggerArgs) -> String {
        let mut analysis = String::from("## SPI Protocol Debug Analysis\n\n");
        
        analysis.push_str("### Common SPI Issues Check:\n\n");
        
        if args.issue.to_lowercase().contains("miso") || args.issue.to_lowercase().contains("no response") {
            analysis.push_str("**MISO/No Response Issues:**\n");
            analysis.push_str("1. **CS (Chip Select) Polarity**: Most devices need CS LOW to communicate\n");
            analysis.push_str("2. **MISO Connection**: Verify MISO is connected (not MOSI-MOSI)\n");
            analysis.push_str("3. **Device Power**: Check if device is properly powered\n\n");
        }
        
        if args.issue.to_lowercase().contains("wrong") || args.issue.to_lowercase().contains("shifted") {
            analysis.push_str("**Data Corruption/Shifted Issues:**\n");
            analysis.push_str("1. **SPI Mode Mismatch**: Check CPOL and CPHA settings\n");
            analysis.push_str("   - Mode 0: CPOL=0, CPHA=0 (most common)\n");
            analysis.push_str("   - Mode 1: CPOL=0, CPHA=1\n");
            analysis.push_str("   - Mode 2: CPOL=1, CPHA=0\n");
            analysis.push_str("   - Mode 3: CPOL=1, CPHA=1\n\n");
            analysis.push_str("2. **Bit Order**: Check MSB vs LSB first\n");
            analysis.push_str("3. **Clock Speed Too High**: Try reducing SPI clock\n\n");
        }
        
        analysis.push_str("### SPI Timing Diagram:\n");
        analysis.push_str("```\n");
        analysis.push_str("CS    ‾‾‾\\_______________/‾‾‾\n");
        analysis.push_str("CLK   ___/‾\\_/‾\\_/‾\\_/‾\\___\n");
        analysis.push_str("MOSI  ___X___X___X___X_____\n");
        analysis.push_str("MISO  ___X___X___X___X_____\n");
        analysis.push_str("```\n\n");
        
        analysis.push_str("### Debug Code Example:\n");
        analysis.push_str("```c\n");
        analysis.push_str("// Test SPI communication\n");
        analysis.push_str("uint8_t test_spi_device(void) {\n");
        analysis.push_str("    uint8_t tx_data = 0x55; // Test pattern\n");
        analysis.push_str("    uint8_t rx_data;\n");
        analysis.push_str("    \n");
        analysis.push_str("    CS_LOW();\n");
        analysis.push_str("    delay_us(1);\n");
        analysis.push_str("    rx_data = spi_transfer(tx_data);\n");
        analysis.push_str("    delay_us(1);\n");
        analysis.push_str("    CS_HIGH();\n");
        analysis.push_str("    \n");
        analysis.push_str("    return rx_data;\n");
        analysis.push_str("}\n");
        analysis.push_str("```\n");
        
        analysis
    }
    
    fn analyze_uart_issue(&self, args: &ProtocolDebuggerArgs) -> String {
        let mut analysis = String::from("## UART Protocol Debug Analysis\n\n");
        
        analysis.push_str("### Common UART Issues Check:\n\n");
        
        if args.issue.to_lowercase().contains("garbage") || args.issue.to_lowercase().contains("symbols") {
            analysis.push_str("**Garbage Data/Wrong Characters:**\n");
            analysis.push_str("1. **Baud Rate Mismatch**: Most common issue!\n");
            analysis.push_str("   - Verify both devices use same baud rate\n");
            analysis.push_str("   - Common rates: 9600, 19200, 38400, 57600, 115200\n\n");
            analysis.push_str("2. **Frame Format Mismatch**:\n");
            analysis.push_str("   - Check data bits (usually 8)\n");
            analysis.push_str("   - Check parity (usually None)\n");
            analysis.push_str("   - Check stop bits (usually 1)\n");
            analysis.push_str("   - Format notation: 8N1 = 8 data, No parity, 1 stop\n\n");
        }
        
        if args.issue.to_lowercase().contains("no data") || args.issue.to_lowercase().contains("not receiving") {
            analysis.push_str("**No Data Received:**\n");
            analysis.push_str("1. **TX/RX Swapped**: Most common wiring issue\n");
            analysis.push_str("   - Device A TX → Device B RX\n");
            analysis.push_str("   - Device A RX → Device B TX\n\n");
            analysis.push_str("2. **Ground Connection**: Devices must share ground\n");
            analysis.push_str("3. **Voltage Levels**: Check if levels match\n");
            analysis.push_str("   - TTL/CMOS: 0V/3.3V or 0V/5V\n");
            analysis.push_str("   - RS-232: ±3V to ±15V (inverted!)\n\n");
        }
        
        // Check for baud rate in parameters
        if let Some(params) = &args.parameters {
            if params.contains("115200") || params.contains("high") {
                analysis.push_str("**High Baud Rate Considerations:**\n");
                analysis.push_str("- At 115200 baud, each bit is only 8.68μs\n");
                analysis.push_str("- Long cables can cause issues (keep under 15 feet)\n");
                analysis.push_str("- Consider using lower baud rate for testing\n\n");
            }
        }
        
        analysis.push_str("### UART Signal Example:\n");
        analysis.push_str("```\n");
        analysis.push_str("     Start  D0 D1 D2 D3 D4 D5 D6 D7 Stop\n");
        analysis.push_str("      ___                           ___\n");
        analysis.push_str("         \\___|‾|_|‾|‾|_|‾|_|_|‾|__/\n");
        analysis.push_str("      Idle  0  1  0  1  1  0  1  0  0  Idle\n");
        analysis.push_str("      \n");
        analysis.push_str("Character 'U' (0x55) = 01010101 binary\n");
        analysis.push_str("```\n\n");
        
        analysis.push_str("### Debug Tips:\n");
        analysis.push_str("1. **Loopback Test**: Connect TX to RX on same device\n");
        analysis.push_str("2. **Send Known Pattern**: Send 'U' (0x55) for 01010101 pattern\n");
        analysis.push_str("3. **Use Terminal Software**: Test with known-good terminal first\n");
        analysis.push_str("4. **Check with Oscilloscope**: Measure actual bit timing\n");
        
        analysis
    }
    
    fn analyze_general_issue(&self, args: &ProtocolDebuggerArgs) -> String {
        format!(
            "## {} Protocol Debug Analysis\n\n\
            ### Issue Description:\n{}\n\n\
            ### General Debugging Steps:\n\
            1. **Verify Connections**: Check all wiring and connections\n\
            2. **Power Supply**: Ensure adequate power for all devices\n\
            3. **Ground Connection**: All devices must share common ground\n\
            4. **Signal Integrity**: Use oscilloscope to check signal quality\n\
            5. **Timing Requirements**: Verify setup/hold times are met\n\
            6. **Protocol Analyzer**: Use logic analyzer to capture communication\n\n\
            ### Hardware Checks:\n\
            - Measure voltages with multimeter\n\
            - Check for shorts or open circuits\n\
            - Verify pull-up/pull-down resistors\n\
            - Test with minimal setup first\n\n\
            ### Software Checks:\n\
            - Verify initialization sequence\n\
            - Check timing delays\n\
            - Confirm protocol parameters\n\
            - Add debug output at each step\n",
            args.protocol, args.issue
        )
    }
}

#[async_trait]
impl Tool for ProtocolDebugger {
    type Params = ProtocolDebuggerArgs;
    
    fn capabilities(&self) -> &'static [ToolCapability] {
        &[ToolCapability::Read]
    }
    
    async fn execute(&self, args: Self::Params) -> ToolResult {
        
        let analysis = match args.protocol.to_uppercase().as_str() {
            "I2C" | "IIC" | "TWI" => self.analyze_i2c_issue(&args),
            "SPI" => self.analyze_spi_issue(&args),
            "UART" | "SERIAL" | "RS232" | "RS-232" => self.analyze_uart_issue(&args),
            _ => self.analyze_general_issue(&args),
        };
        
        let mut result = analysis;
        
        // Add captured data analysis if provided
        if let Some(data) = &args.captured_data {
            result.push_str("\n### Captured Data Analysis:\n");
            result.push_str(&format!("```\n{}\n```\n", data));
            result.push_str("- Look for timing violations\n");
            result.push_str("- Check for signal integrity issues\n");
            result.push_str("- Verify protocol compliance\n");
        }
        
        // Add specific recommendations based on error messages
        if let Some(errors) = &args.error_messages {
            result.push_str("\n### Error Message Analysis:\n");
            if errors.contains("timeout") {
                result.push_str("- **Timeout Error**: Device not responding in time\n");
                result.push_str("  - Check if device is powered and not in reset\n");
                result.push_str("  - Increase timeout value\n");
                result.push_str("  - Verify clock/baud rate settings\n");
            }
            if errors.contains("busy") {
                result.push_str("- **Bus Busy Error**: Another transaction in progress\n");
                result.push_str("  - Check for multi-master conflicts\n");
                result.push_str("  - Ensure proper transaction completion\n");
                result.push_str("  - Add delays between transactions\n");
            }
        }
        
        ToolResult::success(result)
    }
}

impl ToolDescription for ProtocolDebugger {
    fn name(&self) -> &'static str {
        "protocol_debugger"
    }
    
    fn description(&self) -> &'static str {
        "Debug hardware communication protocol issues. Analyzes I2C, SPI, UART, CAN and other protocol problems, providing specific troubleshooting steps and solutions."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(ProtocolDebuggerArgs))
            .unwrap_or_else(|_| serde_json::json!({}))
    }
}