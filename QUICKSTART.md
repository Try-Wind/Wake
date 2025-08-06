# Wake Quick Start Guide

Get up and running with Wake in 5 minutes!

## 🚀 Installation

### Option 1: Quick Install (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/Try-Wind/Wake/main/install.sh | bash
source ~/.bashrc  # or ~/.zshrc
```

### Option 2: Build from Source
```bash
git clone https://github.com/Try-Wind/Wake.git
cd Wake
cargo build --release
cargo install --path wake-cli
```

## ⚙️ Configuration

### 1. Configure your AI provider:
```bash
wake auth
```

Choose from:
- OpenAI (GPT-4)
- Anthropic (Claude)
- Google (Gemini)
- Local (Ollama)

### 2. Set your API key:
Enter your API key when prompted, or edit `~/.config/wake/.wake.config`

## 🎯 Your First Commands

### Generate a Driver
```bash
wake "Generate an Arduino driver for MPU6050 accelerometer"
```

### Debug Hardware Issue
```bash
wake "My ESP32 I2C is getting NACK at address 0x68"
```

### Calculate Timing
```bash
wake "Calculate UART baud rate prescaler for 16MHz clock targeting 9600 baud"
```

### Analyze Circuit
```bash
wake "Calculate pull-up resistor for I2C at 3.3V, 400kHz"
```

## 💡 Example: Complete Sensor Integration

```bash
wake "Help me connect and read data from a BME280 sensor with Arduino Uno"
```

Wake will provide:
1. **Wiring diagram** - Which pins to connect
2. **Driver code** - Complete working code
3. **Example usage** - How to read temperature, humidity, pressure
4. **Troubleshooting** - Common issues and solutions

## 🔧 Hardware Tools Available

| Tool | Purpose | Example |
|------|---------|---------|
| `driver_generator` | Create device drivers | "Generate STM32 driver for BMP280" |
| `protocol_debugger` | Fix communication issues | "Debug SPI returning all 0xFF" |
| `circuit_analyzer` | Analyze circuits | "Review my voltage divider circuit" |
| `timing_calculator` | Calculate timings | "PWM frequency for servo control" |
| `pinout_mapper` | Map MCU pins | "Available I2C pins on ESP32" |
| `datasheet_analyzer` | Extract datasheet info | "Register map for MPU6050" |

## 📚 Real-World Projects

### Build a Weather Station
```bash
wake "Create weather station with ESP8266, BME280, and OLED display"
```

### Design a Drone Controller
```bash
wake "Setup flight controller with STM32F4, MPU6050, and GPS"
```

### IoT Sensor Node
```bash
wake "Build battery-powered LoRa sensor node with deep sleep"
```

## 🎨 Tips for Best Results

1. **Be Specific**: Include your MCU, voltage levels, and communication speeds
2. **Provide Context**: Mention what you've tried and what errors you see
3. **Ask Follow-ups**: Wake maintains context for deeper help
4. **Use Examples**: "Like the Arduino Wire library but for STM32"

## 🆘 Common Issues

### "Command not found"
```bash
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bashrc
```

### "Permission denied" (Linux)
```bash
sudo usermod -a -G dialout $USER
# Log out and back in
```

### "API key not set"
```bash
wake auth
# Or edit ~/.config/wake/.wake.config
```

## 🎉 Success Checklist

- [ ] Wake installed and in PATH
- [ ] API provider configured
- [ ] First driver generated
- [ ] Hardware issue debugged
- [ ] Ready to build amazing hardware!

## 📖 Learn More

- **Full Documentation**: [README.md](README.md)
- **Hardware Examples**: [docs/HARDWARE_EXAMPLES.md](docs/HARDWARE_EXAMPLES.md)
- **Contributing**: [CONTRIBUTING.md](CONTRIBUTING.md)
- **Release Guide**: [docs/RELEASE_GUIDE.md](docs/RELEASE_GUIDE.md)

## 💬 Get Help

- **GitHub Issues**: [Report bugs or request features](https://github.com/Try-Wind/Wake/issues)
- **Discussions**: [Ask questions and share projects](https://github.com/Try-Wind/Wake/discussions)

---

**Wake** - Hardware-first coding agent by Wind
Built with ❤️ for the embedded systems community