use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wake_macros::tool;

use crate::tools::types::{Error, Result, Tool};

#[tool(
    schema = "firmware_flasher",
    description = "Assists with flashing firmware to microcontrollers, bootloader configuration, and debugging flash issues"
)]
pub struct FirmwareFlasher;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FirmwareFlasherInput {
    /// Target device/board
    pub device: String,
    /// Firmware file path or type
    pub firmware: String,
    /// Connection method (serial, jtag, swd, dfu, etc.)
    pub connection: String,
    /// Action (flash, verify, erase, backup)
    pub action: String,
    /// Additional options
    pub options: Option<FlashOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FlashOptions {
    /// Port/interface (e.g., COM3, /dev/ttyUSB0)
    pub port: Option<String>,
    /// Baud rate for serial flashing
    pub baud_rate: Option<u32>,
    /// Memory address to flash to
    pub address: Option<String>,
    /// Bootloader mode
    pub bootloader: Option<String>,
    /// Verify after flash
    pub verify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FirmwareFlasherOutput {
    /// Flash command to execute
    pub command: String,
    /// Step-by-step instructions
    pub instructions: Vec<String>,
    /// Troubleshooting tips
    pub troubleshooting: Vec<TroubleshootingTip>,
    /// Bootloader setup if needed
    pub bootloader_setup: Option<BootloaderSetup>,
    /// Tool recommendations
    pub tools_needed: Vec<FlashTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TroubleshootingTip {
    pub issue: String,
    pub solution: String,
    pub command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BootloaderSetup {
    pub method: String,
    pub steps: Vec<String>,
    pub timing: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FlashTool {
    pub name: String,
    pub purpose: String,
    pub download_url: String,
    pub platform_support: Vec<String>,
}

#[async_trait]
impl Tool for FirmwareFlasher {
    type Input = FirmwareFlasherInput;
    type Output = FirmwareFlasherOutput;

    async fn run(&self, input: Self::Input) -> Result<Self::Output> {
        let command = generate_flash_command(&input);
        let instructions = generate_instructions(&input);
        let troubleshooting = generate_troubleshooting(&input);
        let bootloader_setup = generate_bootloader_setup(&input);
        let tools_needed = recommend_tools(&input);

        Ok(FirmwareFlasherOutput {
            command,
            instructions,
            troubleshooting,
            bootloader_setup,
            tools_needed,
        })
    }
}

fn generate_flash_command(input: &FirmwareFlasherInput) -> String {
    let port = input.options.as_ref()
        .and_then(|o| o.port.as_ref())
        .map(|p| p.as_str())
        .unwrap_or("/dev/ttyUSB0");
    
    let baud = input.options.as_ref()
        .and_then(|o| o.baud_rate)
        .unwrap_or(115200);

    match input.device.to_lowercase().as_str() {
        "esp32" | "esp8266" => {
            match input.action.as_str() {
                "flash" => format!(
                    "esptool.py --chip {} --port {} --baud {} write_flash -z 0x1000 {}",
                    input.device.to_lowercase(), port, baud, input.firmware
                ),
                "erase" => format!(
                    "esptool.py --chip {} --port {} erase_flash",
                    input.device.to_lowercase(), port
                ),
                _ => format!("esptool.py --help")
            }
        }
        "stm32" => {
            match input.connection.as_str() {
                "serial" => format!(
                    "stm32flash -w {} -v -g 0x0 {}",
                    input.firmware, port
                ),
                "swd" | "jtag" => format!(
                    "openocd -f interface/stlink.cfg -f target/stm32f1x.cfg -c \"program {} verify reset exit\"",
                    input.firmware
                ),
                "dfu" => format!(
                    "dfu-util -a 0 -s 0x08000000:leave -D {}",
                    input.firmware
                ),
                _ => "st-flash write firmware.bin 0x8000000".to_string()
            }
        }
        "arduino" | "atmega328" => {
            format!(
                "avrdude -c arduino -p m328p -P {} -b {} -U flash:w:{}:i",
                port, baud, input.firmware
            )
        }
        "rp2040" | "raspberry-pi-pico" => {
            format!(
                "picotool load {} -f",
                input.firmware
            )
        }
        "nrf52" => {
            format!(
                "nrfjprog --program {} --sectorerase --verify --reset",
                input.firmware
            )
        }
        _ => {
            format!("# Generic flash command for {}\n# Adjust parameters as needed", input.device)
        }
    }
}

fn generate_instructions(input: &FirmwareFlasherInput) -> Vec<String> {
    let mut instructions = Vec::new();
    
    match input.device.to_lowercase().as_str() {
        "esp32" => {
            instructions.push("1. Install esptool: pip install esptool".to_string());
            instructions.push("2. Connect ESP32 via USB".to_string());
            instructions.push("3. Hold BOOT button while connecting (if needed)".to_string());
            instructions.push("4. Identify port: ls /dev/tty* (Linux/Mac) or Device Manager (Windows)".to_string());
            instructions.push("5. Run the flash command".to_string());
            instructions.push("6. Press RESET button after flashing".to_string());
        }
        "stm32" => {
            match input.connection.as_str() {
                "serial" => {
                    instructions.push("1. Install stm32flash: apt-get install stm32flash".to_string());
                    instructions.push("2. Set BOOT0 pin HIGH (move jumper)".to_string());
                    instructions.push("3. Reset the device".to_string());
                    instructions.push("4. Run the flash command".to_string());
                    instructions.push("5. Set BOOT0 pin LOW".to_string());
                    instructions.push("6. Reset to run new firmware".to_string());
                }
                "swd" | "jtag" => {
                    instructions.push("1. Install OpenOCD: apt-get install openocd".to_string());
                    instructions.push("2. Connect ST-Link or J-Link programmer".to_string());
                    instructions.push("3. Power the target board".to_string());
                    instructions.push("4. Run the flash command".to_string());
                }
                "dfu" => {
                    instructions.push("1. Install dfu-util: apt-get install dfu-util".to_string());
                    instructions.push("2. Put device in DFU mode (BOOT0=1, Reset)".to_string());
                    instructions.push("3. Verify device detected: dfu-util -l".to_string());
                    instructions.push("4. Run the flash command".to_string());
                }
                _ => {}
            }
        }
        "arduino" => {
            instructions.push("1. Install Arduino IDE or avrdude".to_string());
            instructions.push("2. Connect Arduino via USB".to_string());
            instructions.push("3. Select correct board and port".to_string());
            instructions.push("4. Compile sketch to .hex file".to_string());
            instructions.push("5. Run avrdude command".to_string());
        }
        _ => {
            instructions.push("1. Install appropriate flashing tool".to_string());
            instructions.push("2. Connect device to computer".to_string());
            instructions.push("3. Put device in programming mode".to_string());
            instructions.push("4. Run flash command".to_string());
            instructions.push("5. Reset device to run firmware".to_string());
        }
    }
    
    instructions
}

fn generate_troubleshooting(input: &FirmwareFlasherInput) -> Vec<TroubleshootingTip> {
    vec![
        TroubleshootingTip {
            issue: "Device not detected".to_string(),
            solution: "Check USB cable, install drivers, verify port permissions".to_string(),
            command: Some("ls -l /dev/tty* | grep dialout".to_string()),
        },
        TroubleshootingTip {
            issue: "Permission denied on port".to_string(),
            solution: "Add user to dialout group".to_string(),
            command: Some("sudo usermod -a -G dialout $USER".to_string()),
        },
        TroubleshootingTip {
            issue: "Timeout during flash".to_string(),
            solution: "Check bootloader mode, try lower baud rate".to_string(),
            command: None,
        },
        TroubleshootingTip {
            issue: "Verification failed".to_string(),
            solution: "Erase flash first, check power supply stability".to_string(),
            command: Some(format!("# Erase flash before writing")),
        },
    ]
}

fn generate_bootloader_setup(input: &FirmwareFlasherInput) -> Option<BootloaderSetup> {
    match input.device.to_lowercase().as_str() {
        "esp32" => Some(BootloaderSetup {
            method: "UART Bootloader".to_string(),
            steps: vec![
                "Hold BOOT button (GPIO0 to GND)".to_string(),
                "Press and release RESET".to_string(),
                "Release BOOT after 1 second".to_string(),
            ],
            timing: Some("Hold BOOT for ~1 second after reset".to_string()),
        }),
        "stm32" => Some(BootloaderSetup {
            method: "Built-in Bootloader".to_string(),
            steps: vec![
                "Set BOOT0 pin HIGH".to_string(),
                "Set BOOT1 pin LOW (if present)".to_string(),
                "Reset the microcontroller".to_string(),
            ],
            timing: None,
        }),
        "arduino" => Some(BootloaderSetup {
            method: "Arduino Bootloader".to_string(),
            steps: vec![
                "Bootloader activates automatically on reset".to_string(),
                "DTR signal triggers auto-reset".to_string(),
            ],
            timing: Some("Bootloader active for ~2 seconds after reset".to_string()),
        }),
        _ => None
    }
}

fn recommend_tools(input: &FirmwareFlasherInput) -> Vec<FlashTool> {
    let mut tools = Vec::new();
    
    match input.device.to_lowercase().as_str() {
        "esp32" | "esp8266" => {
            tools.push(FlashTool {
                name: "esptool".to_string(),
                purpose: "Official Espressif flashing tool".to_string(),
                download_url: "pip install esptool".to_string(),
                platform_support: vec!["Windows".to_string(), "macOS".to_string(), "Linux".to_string()],
            });
            tools.push(FlashTool {
                name: "ESP Flash Download Tool".to_string(),
                purpose: "GUI tool for Windows".to_string(),
                download_url: "https://www.espressif.com/en/support/download/other-tools".to_string(),
                platform_support: vec!["Windows".to_string()],
            });
        }
        "stm32" => {
            tools.push(FlashTool {
                name: "STM32CubeProgrammer".to_string(),
                purpose: "Official ST programming tool".to_string(),
                download_url: "https://www.st.com/en/development-tools/stm32cubeprog.html".to_string(),
                platform_support: vec!["Windows".to_string(), "macOS".to_string(), "Linux".to_string()],
            });
            tools.push(FlashTool {
                name: "OpenOCD".to_string(),
                purpose: "Open source debugging and programming".to_string(),
                download_url: "apt-get install openocd".to_string(),
                platform_support: vec!["Linux".to_string(), "macOS".to_string()],
            });
        }
        _ => {
            tools.push(FlashTool {
                name: "Platform-specific tool".to_string(),
                purpose: "Check manufacturer's website".to_string(),
                download_url: "Varies by platform".to_string(),
                platform_support: vec!["Varies".to_string()],
            });
        }
    }
    
    tools
}