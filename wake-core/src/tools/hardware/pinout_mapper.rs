use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wake_macros::tool;

use crate::tools::types::{Error, Result, Tool};

#[tool(
    schema = "pinout_mapper",
    description = "Maps microcontroller pins to peripherals, generates pin configuration code, and checks for conflicts"
)]
pub struct PinoutMapper;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PinoutMapperInput {
    /// Microcontroller or board type
    pub board: String,
    /// List of peripherals to configure
    pub peripherals: Vec<Peripheral>,
    /// Generate code for this platform/framework
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Peripheral {
    pub name: String,
    pub type_: String, // i2c, spi, uart, gpio, pwm, adc, etc.
    pub pins: Vec<PinAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PinAssignment {
    pub function: String, // SDA, SCL, MOSI, MISO, TX, RX, etc.
    pub pin: String,      // Physical pin number or name
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PinoutMapperOutput {
    /// Pin mapping visualization
    pub pinout_diagram: String,
    /// Configuration code
    pub config_code: String,
    /// Pin conflicts detected
    pub conflicts: Vec<PinConflict>,
    /// Available pins
    pub available_pins: Vec<String>,
    /// Pin usage summary
    pub usage_summary: PinUsageSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PinConflict {
    pub pin: String,
    pub conflict: String,
    pub resolution: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PinUsageSummary {
    pub total_pins: u32,
    pub used_pins: u32,
    pub gpio_available: u32,
    pub special_functions: Vec<String>,
}

#[async_trait]
impl Tool for PinoutMapper {
    type Input = PinoutMapperInput;
    type Output = PinoutMapperOutput;

    async fn run(&self, input: Self::Input) -> Result<Self::Output> {
        let mut conflicts = Vec::new();
        let mut used_pins = Vec::new();

        // Check for pin conflicts
        for peripheral in &input.peripherals {
            for pin_assignment in &peripheral.pins {
                if used_pins.contains(&pin_assignment.pin) {
                    conflicts.push(PinConflict {
                        pin: pin_assignment.pin.clone(),
                        conflict: format!("Pin already used by another peripheral"),
                        resolution: "Choose alternative pin or use multiplexing".to_string(),
                    });
                } else {
                    used_pins.push(pin_assignment.pin.clone());
                }
            }
        }

        let config_code = generate_config_code(&input);
        let pinout_diagram = generate_pinout_diagram(&input);
        let available_pins = get_available_pins(&input.board, &used_pins);

        Ok(PinoutMapperOutput {
            pinout_diagram,
            config_code,
            conflicts,
            available_pins,
            usage_summary: PinUsageSummary {
                total_pins: 40,
                used_pins: used_pins.len() as u32,
                gpio_available: 20,
                special_functions: vec![
                    "I2C0".to_string(),
                    "SPI0".to_string(),
                    "UART1".to_string(),
                    "PWM0-3".to_string(),
                ],
            },
        })
    }
}

fn generate_config_code(input: &PinoutMapperInput) -> String {
    let mut code = String::new();
    
    match input.platform.to_lowercase().as_str() {
        "arduino" => {
            code.push_str("// Pin Configuration for Arduino\n\n");
            for peripheral in &input.peripherals {
                code.push_str(&format!("// {} Configuration\n", peripheral.name));
                for pin in &peripheral.pins {
                    code.push_str(&format!("#define {}_{} {}\n", 
                        peripheral.name.to_uppercase(),
                        pin.function.to_uppercase(),
                        pin.pin
                    ));
                }
                code.push_str("\n");
            }
            
            code.push_str("void setup() {\n");
            for peripheral in &input.peripherals {
                match peripheral.type_.as_str() {
                    "gpio" => {
                        for pin in &peripheral.pins {
                            code.push_str(&format!("  pinMode({}, OUTPUT);\n", pin.pin));
                        }
                    }
                    "i2c" => {
                        code.push_str("  Wire.begin();\n");
                    }
                    "spi" => {
                        code.push_str("  SPI.begin();\n");
                    }
                    _ => {}
                }
            }
            code.push_str("}\n");
        }
        "stm32" => {
            code.push_str("/* Pin Configuration for STM32 */\n\n");
            for peripheral in &input.peripherals {
                code.push_str(&format!("/* {} Configuration */\n", peripheral.name));
                for pin in &peripheral.pins {
                    code.push_str(&format!("#define {}_{}_PIN GPIO_PIN_{}\n", 
                        peripheral.name.to_uppercase(),
                        pin.function.to_uppercase(),
                        pin.pin
                    ));
                    code.push_str(&format!("#define {}_{}_PORT GPIOA\n", 
                        peripheral.name.to_uppercase(),
                        pin.function.to_uppercase()
                    ));
                }
                code.push_str("\n");
            }
        }
        "esp32" => {
            code.push_str("// Pin Configuration for ESP32\n\n");
            for peripheral in &input.peripherals {
                code.push_str(&format!("// {} Pins\n", peripheral.name));
                for pin in &peripheral.pins {
                    code.push_str(&format!("const int {}_{} = {};\n", 
                        peripheral.name.to_lowercase(),
                        pin.function.to_lowercase(),
                        pin.pin
                    ));
                }
                code.push_str("\n");
            }
        }
        _ => {
            code.push_str("// Generic Pin Configuration\n");
            for peripheral in &input.peripherals {
                for pin in &peripheral.pins {
                    code.push_str(&format!("{}.{} = Pin {}\n", 
                        peripheral.name,
                        pin.function,
                        pin.pin
                    ));
                }
            }
        }
    }
    
    code
}

fn generate_pinout_diagram(input: &PinoutMapperInput) -> String {
    let mut diagram = String::new();
    diagram.push_str(&format!("\n{} Pinout Map\n", input.board));
    diagram.push_str("═══════════════════════════════\n\n");
    
    // Simple ASCII representation
    diagram.push_str("       ┌─────────────┐\n");
    diagram.push_str("    3V3│ 1       40 │VBUS\n");
    diagram.push_str("     GP0│ 2       39 │VSYS\n");
    diagram.push_str("     GP1│ 3       38 │GND\n");
    
    for peripheral in &input.peripherals {
        for pin in &peripheral.pins {
            diagram.push_str(&format!("  {}({})│         │\n", 
                pin.function,
                pin.pin
            ));
        }
    }
    
    diagram.push_str("       └─────────────┘\n\n");
    
    diagram.push_str("Legend:\n");
    for peripheral in &input.peripherals {
        diagram.push_str(&format!("  {} - {}\n", peripheral.name, peripheral.type_));
    }
    
    diagram
}

fn get_available_pins(board: &str, used_pins: &[String]) -> Vec<String> {
    // This would normally query a database of board pinouts
    let all_pins: Vec<String> = (0..40).map(|i| format!("GP{}", i)).collect();
    
    all_pins.into_iter()
        .filter(|pin| !used_pins.contains(pin))
        .collect()
}