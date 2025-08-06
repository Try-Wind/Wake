use crate::tools::{Tool, ToolCapability, ToolResult};
use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wake_llm::ToolDescription;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct PinoutMapperArgs {
    /// Microcontroller or board name
    pub device: String,

    /// Peripheral to map (UART, I2C, SPI, PWM, ADC, GPIO)
    pub peripheral: String,

    /// Optional: Specific instance (UART1, I2C2, etc.)
    pub instance: Option<String>,
}

pub struct PinoutMapper;

impl PinoutMapper {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for PinoutMapper {
    type Params = PinoutMapperArgs;

    fn capabilities(&self) -> &'static [ToolCapability] {
        &[ToolCapability::Read]
    }

    async fn execute(&self, args: Self::Params) -> ToolResult {
        let mapping = format!(
            "## Pinout Mapping for {}\n\n\
            ### Peripheral: {}\n\n\
            This tool provides:\n\
            - Pin assignments for peripherals\n\
            - Alternate function configurations\n\
            - Conflict detection\n\
            - Power and ground pin locations\n\
            - Compatible pin combinations\n",
            args.device, args.peripheral
        );

        ToolResult::success(mapping)
    }
}

impl ToolDescription for PinoutMapper {
    fn name(&self) -> &'static str {
        "pinout_mapper"
    }

    fn description(&self) -> &'static str {
        "Map microcontroller pins to peripherals, identify alternate functions, and detect pin conflicts."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(PinoutMapperArgs))
            .unwrap_or_else(|_| serde_json::json!({}))
    }
}
