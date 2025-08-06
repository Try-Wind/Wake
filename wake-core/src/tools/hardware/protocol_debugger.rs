use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wake_macros::tool;

use crate::tools::types::{Error, Result, Tool};

#[tool(
    schema = "protocol_debugger",
    description = "Debugs hardware communication protocols (I2C, SPI, UART, CAN) by analyzing signals, timing, and common issues"
)]
pub struct ProtocolDebugger;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProtocolDebuggerInput {
    /// Protocol type (i2c, spi, uart, can, i2s, etc.)
    pub protocol: String,
    /// Problem description or symptoms
    pub issue: String,
    /// Hardware setup details
    pub setup: HardwareSetup,
    /// Captured data or signals if available
    pub captured_data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HardwareSetup {
    /// Master/Controller device
    pub master_device: String,
    /// Slave/Peripheral device
    pub slave_device: String,
    /// Communication speed/baud rate
    pub speed: String,
    /// Voltage levels
    pub voltage: String,
    /// Pull-up resistor values if applicable
    pub pullup_resistors: Option<String>,
    /// Wire length
    pub wire_length: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProtocolDebuggerOutput {
    /// Identified issues
    pub issues_found: Vec<Issue>,
    /// Recommended fixes
    pub recommendations: Vec<Recommendation>,
    /// Protocol analysis
    pub analysis: ProtocolAnalysis,
    /// Debug commands to try
    pub debug_commands: Vec<String>,
    /// Oscilloscope settings if needed
    pub scope_settings: Option<ScopeSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Issue {
    pub severity: String, // critical, warning, info
    pub category: String, // timing, electrical, protocol, configuration
    pub description: String,
    pub likely_cause: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Recommendation {
    pub priority: u8, // 1-5, 1 being highest
    pub action: String,
    pub explanation: String,
    pub code_example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProtocolAnalysis {
    pub timing_check: String,
    pub electrical_check: String,
    pub protocol_compliance: String,
    pub noise_analysis: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ScopeSettings {
    pub time_base: String,
    pub voltage_scale: String,
    pub trigger_settings: String,
    pub channels: Vec<String>,
}

#[async_trait]
impl Tool for ProtocolDebugger {
    type Input = ProtocolDebuggerInput;
    type Output = ProtocolDebuggerOutput;

    async fn run(&self, input: Self::Input) -> Result<Self::Output> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Analyze based on protocol type
        match input.protocol.to_lowercase().as_str() {
            "i2c" => {
                analyze_i2c(&input, &mut issues, &mut recommendations);
            }
            "spi" => {
                analyze_spi(&input, &mut issues, &mut recommendations);
            }
            "uart" => {
                analyze_uart(&input, &mut issues, &mut recommendations);
            }
            "can" => {
                analyze_can(&input, &mut issues, &mut recommendations);
            }
            _ => {
                return Err(Error::ToolExecutionError(
                    format!("Unsupported protocol: {}", input.protocol)
                ));
            }
        }

        // Add common checks
        check_common_issues(&input, &mut issues, &mut recommendations);

        let debug_commands = generate_debug_commands(&input.protocol);
        let scope_settings = generate_scope_settings(&input.protocol, &input.setup.speed);

        Ok(ProtocolDebuggerOutput {
            issues_found: issues,
            recommendations,
            analysis: ProtocolAnalysis {
                timing_check: analyze_timing(&input),
                electrical_check: analyze_electrical(&input),
                protocol_compliance: check_protocol_compliance(&input),
                noise_analysis: analyze_noise(&input),
            },
            debug_commands,
            scope_settings,
        })
    }
}

fn analyze_i2c(input: &ProtocolDebuggerInput, issues: &mut Vec<Issue>, recommendations: &mut Vec<Recommendation>) {
    // Check for common I2C issues
    if input.issue.contains("NACK") || input.issue.contains("not responding") {
        issues.push(Issue {
            severity: "critical".to_string(),
            category: "protocol".to_string(),
            description: "Device not acknowledging (NACK)".to_string(),
            likely_cause: "Wrong address, device not powered, or pull-ups missing".to_string(),
        });
        
        recommendations.push(Recommendation {
            priority: 1,
            action: "Verify I2C address".to_string(),
            explanation: "Use I2C scanner to find correct address".to_string(),
            code_example: Some("i2c_scan() // Scans 0x00-0x7F".to_string()),
        });
    }

    // Check pull-up resistors
    if let Some(pullups) = &input.setup.pullup_resistors {
        if !pullups.contains("k") && !pullups.contains("K") {
            issues.push(Issue {
                severity: "warning".to_string(),
                category: "electrical".to_string(),
                description: "Pull-up resistor value may be incorrect".to_string(),
                likely_cause: "Typical values are 2.2kΩ to 10kΩ".to_string(),
            });
        }
    } else {
        recommendations.push(Recommendation {
            priority: 2,
            action: "Add pull-up resistors".to_string(),
            explanation: "I2C requires pull-ups on SDA and SCL".to_string(),
            code_example: Some("4.7kΩ resistors to VDD on both lines".to_string()),
        });
    }
}

fn analyze_spi(input: &ProtocolDebuggerInput, issues: &mut Vec<Issue>, recommendations: &mut Vec<Recommendation>) {
    if input.issue.contains("garbage") || input.issue.contains("wrong data") {
        issues.push(Issue {
            severity: "critical".to_string(),
            category: "configuration".to_string(),
            description: "SPI mode mismatch or wrong bit order".to_string(),
            likely_cause: "Clock polarity/phase (mode) incorrect".to_string(),
        });
        
        recommendations.push(Recommendation {
            priority: 1,
            action: "Verify SPI mode (0-3)".to_string(),
            explanation: "Check CPOL and CPHA settings match device".to_string(),
            code_example: Some("SPI.begin(SPI_MODE0) // Try modes 0-3".to_string()),
        });
    }
}

fn analyze_uart(input: &ProtocolDebuggerInput, issues: &mut Vec<Issue>, recommendations: &mut Vec<Recommendation>) {
    if input.issue.contains("garbage") || input.issue.contains("corrupted") {
        issues.push(Issue {
            severity: "critical".to_string(),
            category: "configuration".to_string(),
            description: "Baud rate mismatch or framing error".to_string(),
            likely_cause: "Incorrect baud rate or data format".to_string(),
        });
        
        recommendations.push(Recommendation {
            priority: 1,
            action: "Verify baud rate and format".to_string(),
            explanation: "Ensure both devices use same settings".to_string(),
            code_example: Some("Serial.begin(115200, SERIAL_8N1)".to_string()),
        });
    }
}

fn analyze_can(input: &ProtocolDebuggerInput, issues: &mut Vec<Issue>, recommendations: &mut Vec<Recommendation>) {
    if input.issue.contains("bus off") || input.issue.contains("error passive") {
        issues.push(Issue {
            severity: "critical".to_string(),
            category: "electrical".to_string(),
            description: "CAN bus termination issue".to_string(),
            likely_cause: "Missing or incorrect 120Ω termination resistors".to_string(),
        });
        
        recommendations.push(Recommendation {
            priority: 1,
            action: "Check CAN bus termination".to_string(),
            explanation: "Need 120Ω at each end of the bus".to_string(),
            code_example: None,
        });
    }
}

fn check_common_issues(input: &ProtocolDebuggerInput, issues: &mut Vec<Issue>, recommendations: &mut Vec<Recommendation>) {
    // Check voltage compatibility
    if input.setup.voltage.contains("3.3") && input.setup.voltage.contains("5") {
        issues.push(Issue {
            severity: "warning".to_string(),
            category: "electrical".to_string(),
            description: "Voltage level mismatch".to_string(),
            likely_cause: "Mixing 3.3V and 5V devices".to_string(),
        });
        
        recommendations.push(Recommendation {
            priority: 2,
            action: "Add level shifter".to_string(),
            explanation: "Use bidirectional level shifter for safe operation".to_string(),
            code_example: None,
        });
    }

    // Check wire length for high-speed protocols
    if let Some(length) = &input.setup.wire_length {
        if length.contains("m") || length.contains("meter") {
            issues.push(Issue {
                severity: "info".to_string(),
                category: "electrical".to_string(),
                description: "Long wire length may cause issues".to_string(),
                likely_cause: "Signal degradation over distance".to_string(),
            });
        }
    }
}

fn analyze_timing(input: &ProtocolDebuggerInput) -> String {
    format!("Timing analysis for {} at {}", input.protocol, input.setup.speed)
}

fn analyze_electrical(input: &ProtocolDebuggerInput) -> String {
    format!("Electrical analysis: {} voltage levels", input.setup.voltage)
}

fn check_protocol_compliance(input: &ProtocolDebuggerInput) -> String {
    format!("{} protocol compliance check", input.protocol.to_uppercase())
}

fn analyze_noise(input: &ProtocolDebuggerInput) -> String {
    "Noise analysis: Check for EMI sources and add decoupling capacitors".to_string()
}

fn generate_debug_commands(protocol: &str) -> Vec<String> {
    match protocol.to_lowercase().as_str() {
        "i2c" => vec![
            "i2cdetect -y 1".to_string(),
            "i2cdump -y 1 0x68".to_string(),
            "i2cset -y 1 0x68 0x00 0x01".to_string(),
        ],
        "spi" => vec![
            "spi-test -D /dev/spidev0.0".to_string(),
            "echo -ne '\\x01\\x02' > /dev/spidev0.0".to_string(),
        ],
        "uart" => vec![
            "stty -F /dev/ttyUSB0 115200".to_string(),
            "cat /dev/ttyUSB0".to_string(),
            "echo 'test' > /dev/ttyUSB0".to_string(),
        ],
        _ => vec![],
    }
}

fn generate_scope_settings(protocol: &str, speed: &str) -> Option<ScopeSettings> {
    Some(ScopeSettings {
        time_base: match protocol {
            "i2c" => "10µs/div".to_string(),
            "spi" => "1µs/div".to_string(),
            "uart" => "100µs/div".to_string(),
            _ => "auto".to_string(),
        },
        voltage_scale: "1V/div".to_string(),
        trigger_settings: "Rising edge, 50% threshold".to_string(),
        channels: match protocol {
            "i2c" => vec!["CH1: SDA".to_string(), "CH2: SCL".to_string()],
            "spi" => vec!["CH1: MOSI".to_string(), "CH2: MISO".to_string(), "CH3: SCK".to_string(), "CH4: CS".to_string()],
            "uart" => vec!["CH1: TX".to_string(), "CH2: RX".to_string()],
            _ => vec![],
        },
    })
}