# Wake - Hardware-First Coding Agent by Wind

![Wake Logo](./docs/wake-banner.png)

Wake is a powerful hardware-aware coding assistant built by Wind (a startup founded by Adhyaay Karnwal). It's designed specifically for embedded systems, IoT development, circuit design, and hardware programming. Wake understands hardware components, can generate device drivers from datasheets, debug hardware issues, and help with everything from Arduino projects to complex embedded systems.

## 🚀 Features

### Hardware-Specific Capabilities
- **Driver Generation**: Upload a datasheet, get production-ready drivers for STM32, ESP32, Arduino, Raspberry Pi, and more
- **Hardware Debugging**: Intelligent analysis of hardware issues, timing problems, and communication protocol errors
- **Circuit Design Assistance**: Help with PCB design, component selection, and schematic review
- **Embedded Development**: Support for C/C++, Rust, MicroPython for embedded systems
- **Protocol Implementation**: I2C, SPI, UART, CAN bus, and other hardware communication protocols
- **RTOS Support**: FreeRTOS, Zephyr, and other real-time operating systems
- **IoT Development**: MQTT, CoAP, BLE, WiFi, LoRaWAN implementations

### Core Features
- **Multi-Provider Support**: Works with OpenAI, Anthropic, Google, and more
- **Intelligent Context**: Understands your hardware setup and maintains context across sessions
- **Tool System**: Extensible tool system for hardware testing, flashing, and debugging
- **Real-time Assistance**: Get help while you work with hardware projects

## 📋 Prerequisites

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **Git**

## 🛠 Installation

### Quick Install (Linux/macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/Try-Wind/Wake/main/install.sh | bash
```

### Manual Installation

1. **Clone the repository:**
```bash
git clone https://github.com/Try-Wind/Wake.git
cd Wake
```

2. **Build the project:**
```bash
cargo build --release
```

3. **Install Wake:**
```bash
cargo install --path wake-cli
```

4. **Configure your AI provider:**
```bash
wake auth
```

## ⚙️ Configuration

Wake needs to be configured with an AI provider. Run `wake auth` to set up your provider.

### Supported Providers

- **OpenAI** (GPT-4, GPT-3.5)
- **Anthropic** (Claude)
- **Google** (Gemini)
- **Local Models** (Ollama, llama.cpp)
- **Custom Endpoints**

### Example Configuration

Create a `.wake.config` file in your home directory:

```json
{
  "providers": [
    {
      "provider": "openai",
      "api_key": "your-api-key",
      "model": "gpt-4",
      "tool_method": "FunctionCall"
    }
  ],
  "selected_provider": 0
}
```

## 💻 Usage

### Basic Commands

```bash
# Start Wake in interactive mode
wake

# Get help with a specific hardware issue
wake "My ESP32 is not responding to I2C commands at address 0x68"

# Generate a driver from a datasheet
wake "Generate an Arduino driver for the MPU6050 accelerometer"

# Debug hardware communication
wake "Debug SPI communication with my SD card module"

# Help with circuit design
wake "Review my circuit: 3.3V MCU connecting to 5V sensor"
```

### Hardware-Specific Examples

#### Generate Device Driver
```bash
wake "Create a driver for the BMP280 pressure sensor for STM32F4"
```

#### Debug Serial Communication
```bash
wake "My UART is receiving garbage data at 115200 baud"
```

#### IoT Project Help
```bash
wake "Set up MQTT client on ESP8266 with TLS"
```

#### PCB Design Review
```bash
wake "Review my I2C pull-up resistor values for 400kHz operation"
```

## 🔧 Hardware Tools

Wake includes specialized tools for hardware development:

- **`datasheet_analyzer`**: Extract register maps and timing diagrams from PDFs
- **`driver_generator`**: Generate boilerplate driver code
- **`protocol_debugger`**: Analyze communication protocol issues
- **`timing_calculator`**: Calculate delays, frequencies, and timing requirements
- **`pinout_mapper`**: Map microcontroller pins to peripherals
- **`power_analyzer`**: Calculate power consumption and battery life

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## 📚 Documentation

- [User Guide](docs/USER_GUIDE.md)
- [Hardware Examples](docs/HARDWARE_EXAMPLES.md)
- [API Reference](docs/API.md)
- [Tool Development](docs/TOOLS.md)

## 🐛 Troubleshooting

### Common Issues

**Wake command not found:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

**Permission denied on Linux:**
```bash
# Add user to dialout group for serial port access
sudo usermod -a -G dialout $USER
# Logout and login again
```

**Hardware device not detected:**
```bash
# Check device connections
wake "List all connected serial devices"
```

## 📜 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 👥 Team

Wake is developed by **Wind**, a startup founded by Adhyaay Karnwal, focused on building next-generation hardware development tools.

## 🙏 Acknowledgments

- Inspired by the needs of embedded engineers worldwide
- Built for the hardware development community
- Special thanks to all contributors

## 📬 Contact

- **GitHub Issues**: [github.com/Try-Wind/Wake/issues](https://github.com/Try-Wind/Wake/issues)
- **Discussions**: [github.com/Try-Wind/Wake/discussions](https://github.com/Try-Wind/Wake/discussions)

---

**Wake** - Empowering hardware developers with AI-driven assistance. Built with ❤️ by Wind.