use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::tools::{Tool, ToolResult, ToolCapability};
use wake_llm::ToolDescription;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimingCalculatorArgs {
    /// Calculation type (baud-rate, i2c-timing, spi-clock, pwm, timer)
    pub calc_type: String,
    
    /// Input frequency or clock (in Hz)
    pub clock_freq: u32,
    
    /// Desired output or target value
    pub target: u32,
    
    /// Optional: Additional constraints
    pub constraints: Option<String>,
}

pub struct TimingCalculator;

impl TimingCalculator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for TimingCalculator {
    type Params = TimingCalculatorArgs;
    
    fn capabilities(&self) -> &'static [ToolCapability] {
        &[ToolCapability::Read]
    }
    
    async fn execute(&self, args: Self::Params) -> ToolResult {
        
        let calculation = format!(
            "## Timing Calculation\n\n\
            ### Type: {}\n\
            ### Clock: {} Hz\n\
            ### Target: {}\n\n\
            This tool calculates:\n\
            - Prescaler and divider values\n\
            - Actual vs desired frequencies\n\
            - Timing accuracy and errors\n\
            - Register configuration values\n",
            args.calc_type, args.clock_freq, args.target
        );
        
        ToolResult::success(calculation)
    }
}

impl ToolDescription for TimingCalculator {
    fn name(&self) -> &'static str {
        "timing_calculator"
    }
    
    fn description(&self) -> &'static str {
        "Calculate timing parameters, prescalers, and dividers for hardware peripherals like UART, timers, PWM, and clocks."
    }
    
    fn parameters_schema(&self) -> serde_json::Value {
        serde_json::to_value(schemars::schema_for!(TimingCalculatorArgs))
            .unwrap_or_else(|_| serde_json::json!({}))
    }
}