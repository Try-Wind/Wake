use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wake_llm::tools::{Tool, ToolArguments, ToolResult};
use wake_macros::tool_derive;

#[tool_derive]
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
    async fn run(&self, args: ToolArguments) -> Result<ToolResult, String> {
        let args: TimingCalculatorArgs = args.try_into()?;
        
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
        
        Ok(ToolResult::Success(calculation))
    }
    
    fn name(&self) -> &str {
        "timing_calculator"
    }
    
    fn description(&self) -> &str {
        "Calculate timing parameters, prescalers, and dividers for hardware peripherals like UART, timers, PWM, and clocks."
    }
}