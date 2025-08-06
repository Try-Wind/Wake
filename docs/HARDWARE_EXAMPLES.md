# Wake Hardware Examples

This guide provides practical examples of using Wake for hardware development tasks.

## Table of Contents
- [Driver Generation](#driver-generation)
- [Protocol Debugging](#protocol-debugging)
- [Circuit Analysis](#circuit-analysis)
- [Timing Calculations](#timing-calculations)
- [Pinout Mapping](#pinout-mapping)
- [Real-World Scenarios](#real-world-scenarios)

## Driver Generation

### Generate Arduino Driver for MPU6050

```bash
wake "Generate an Arduino driver for the MPU6050 accelerometer using I2C"
```

Wake will create a complete driver with:
- Register definitions
- Initialization functions
- Data reading methods
- Calibration routines
- Example usage code

### Generate STM32 Driver for BMP280

```bash
wake "Create a driver for BMP280 pressure sensor for STM32F4 using SPI"
```

Output includes:
- HAL-based implementation
- DMA support options
- Interrupt handling
- Power management

### Generate Rust Driver for SSD1306 OLED

```bash
wake "Generate a Rust no_std driver for SSD1306 OLED display using I2C"
```

Features:
- `embedded-hal` traits
- No heap allocation
- Async support
- Graphics primitives

## Protocol Debugging

### Debug I2C Communication Issues

```bash
wake "My I2C device is not responding. Using 3.3V Arduino with MPU6050 at address 0x68"
```

Wake analyzes:
- Address configuration (7-bit vs 8-bit)
- Pull-up resistor values
- Voltage level compatibility
- Clock stretching requirements
- Bus capacitance issues

### Fix UART Garbage Data

```bash
wake "Getting garbage characters on UART at 115200 baud. ESP32 to USB serial adapter"
```

Wake checks:
- Baud rate mismatch
- Frame format (8N1, 8E1, etc.)
- TX/RX connection swap
- Ground connection
- Voltage level differences

### Troubleshoot SPI Communication

```bash
wake "SPI device returns all 0xFF. Using Raspberry Pi with SD card module"
```

Wake investigates:
- CS (chip select) polarity
- SPI mode (CPOL/CPHA)
- Clock speed limitations
- MISO/MOSI connections
- Power supply issues

## Circuit Analysis

### Review Pull-up Resistor Values

```bash
wake "Calculate I2C pull-up resistor values for 3.3V system at 400kHz with 200pF bus capacitance"
```

Wake provides:
- Minimum resistance calculation
- Maximum resistance calculation
- Rise time analysis
- Power consumption
- Recommended values

### Analyze Voltage Divider

```bash
wake "Design voltage divider to convert 5V signal to 3.3V for GPIO input"
```

Wake calculates:
- Resistor ratios
- Input impedance effects
- Power dissipation
- Tolerance analysis
- Safety margins

### Filter Design

```bash
wake "Design RC low-pass filter for 10kHz cutoff frequency"
```

Wake delivers:
- Component values
- Frequency response
- Phase shift
- Attenuation curves
- Component selection guide

## Timing Calculations

### UART Baud Rate Calculation

```bash
wake "Calculate UART prescaler for 115200 baud with 72MHz system clock"
```

Wake computes:
- Prescaler value
- Actual baud rate
- Error percentage
- Alternative settings
- Register configuration

### PWM Frequency Setup

```bash
wake "Configure PWM for 20kHz frequency with 10-bit resolution on 16MHz MCU"
```

Wake provides:
- Timer prescaler
- Period value
- Duty cycle range
- Actual frequency
- Resolution trade-offs

### Timer Configuration

```bash
wake "Setup timer for 1ms interrupt on STM32F103 at 72MHz"
```

Wake calculates:
- Prescaler value
- Auto-reload value
- Interrupt frequency
- Clock source selection
- Code example

## Pinout Mapping

### Map ESP32 Peripherals

```bash
wake "Show available pins for I2C, SPI, and UART on ESP32 DevKit"
```

Wake displays:
- Default pin assignments
- Alternative pin options
- Pin conflicts
- Strapping pins to avoid
- Recommended configurations

### STM32 Alternate Functions

```bash
wake "List alternate functions for PA0-PA7 on STM32F4"
```

Wake provides:
- Timer channels
- UART/USART pins
- I2C/SPI options
- ADC channels
- Special functions

## Real-World Scenarios

### Complete Sensor Integration

```bash
wake "Help me integrate BME280 sensor with ESP8266 for weather station"
```

Wake provides:
1. **Hardware Setup**
   - Wiring diagram
   - Pull-up resistors
   - Power supply considerations

2. **Driver Code**
   - Complete driver implementation
   - Initialization sequence
   - Data reading functions

3. **Example Application**
   - Temperature, humidity, pressure reading
   - Data formatting
   - Error handling

4. **Optimization Tips**
   - Power saving modes
   - Measurement intervals
   - Calibration procedures

### Drone Flight Controller

```bash
wake "Setup MPU6050 and BMP280 for drone flight controller on STM32F4"
```

Wake assists with:
1. **Sensor Configuration**
   - High-speed I2C setup
   - DMA configuration
   - Interrupt-driven reading

2. **Real-time Constraints**
   - Loop timing
   - Priority management
   - Buffer strategies

3. **Sensor Fusion**
   - Complementary filter
   - Kalman filter basics
   - Calibration routines

### IoT Device Development

```bash
wake "Create MQTT client for ESP32 with TLS to send sensor data"
```

Wake provides:
1. **Network Setup**
   - WiFi configuration
   - TLS certificates
   - Keep-alive settings

2. **MQTT Implementation**
   - Client setup
   - Topic structure
   - QoS levels
   - Retry logic

3. **Power Management**
   - Deep sleep modes
   - Wake on timer
   - Wake on interrupt

## Advanced Hardware Debugging

### Logic Analyzer Integration

```bash
wake "Analyze this I2C capture: START, 0x68 W, ACK, 0x75, ACK, START, 0x69 R, NACK, STOP"
```

Wake interprets:
- Transaction flow
- Address recognition
- Register access pattern
- Error conditions
- Suggested fixes

### Oscilloscope Measurements

```bash
wake "My I2C signals show slow rise time, about 2us to reach 3.3V"
```

Wake analyzes:
- Pull-up resistance too high
- Bus capacitance issues
- Wire length problems
- Suggested solutions

## Tips for Best Results

1. **Provide Context**: Include voltage levels, clock speeds, and specific hardware
2. **Mention Error Messages**: Share exact error codes or symptoms
3. **Specify Platform**: Indicate your microcontroller or development board
4. **Include Constraints**: Mention power, size, or cost limitations
5. **Ask Follow-ups**: Wake maintains context for deeper troubleshooting

## Common Hardware Patterns

### Sensor Reading Pattern
```c
// Wake-generated pattern for reliable sensor reading
bool read_sensor_with_retry(uint8_t *data, int max_retries) {
    for (int i = 0; i < max_retries; i++) {
        if (sensor_read(data) == SUCCESS) {
            return true;
        }
        delay_ms(10);  // Back-off delay
    }
    return false;
}
```

### State Machine for Protocols
```c
// Wake-generated SPI transaction state machine
typedef enum {
    SPI_IDLE,
    SPI_SELECT,
    SPI_TRANSFER,
    SPI_DESELECT,
    SPI_ERROR
} spi_state_t;
```

### Interrupt-Safe Buffer
```c
// Wake-generated ring buffer for UART
typedef struct {
    volatile uint8_t buffer[256];
    volatile uint8_t head;
    volatile uint8_t tail;
} uart_buffer_t;
```

## Getting Help

If you encounter issues:
1. Describe your hardware setup clearly
2. Include relevant voltage levels and frequencies
3. Mention what you've already tried
4. Provide error messages or symptoms

Wake is designed to understand hardware context and provide practical, implementable solutions for embedded systems development.

---

Built with ❤️ by Wind - Empowering hardware developers with AI assistance