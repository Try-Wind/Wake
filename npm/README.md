# Wake CLI

A hardware-first coding agent by Wind - now available via npm!

## 🚀 Quick Installation

```bash
npm install -g @trywind/wake
```

That's it! Wake will be installed globally and available from your command line.

## 📘 Usage

```bash
# Show help and available commands
wake --help

# Configure authentication
wake auth

# Start Wake CLI
wake
```

## 🎯 Features

- **Hardware Interaction**: Direct communication with embedded systems and IoT devices
- **AI-Powered Assistance**: Intelligent coding suggestions and automation
- **Multi-Platform Support**: Works on Linux, Windows, and macOS (coming soon)
- **Intuitive TUI**: Terminal user interface for seamless interaction

## 📦 What Gets Installed?

When you install Wake via npm, it automatically:
1. Detects your operating system and architecture
2. Downloads the appropriate pre-compiled binary from GitHub releases
3. Sets up the `wake` command in your PATH
4. Makes the binary executable (on Unix-like systems)

## 🔧 Supported Platforms

| Platform | Architecture | Status |
|----------|-------------|---------|
| Linux    | x64         | ✅ Supported |
| Windows  | x64         | ✅ Supported |
| macOS    | x64         | 🚧 Coming Soon |
| macOS    | ARM64 (M1/M2) | 🚧 Coming Soon |

## 📚 Documentation

- [GitHub Repository](https://github.com/Try-Wind/Wake)
- [Quick Start Guide](https://github.com/Try-Wind/Wake/blob/main/QUICKSTART.md)
- [Contributing](https://github.com/Try-Wind/Wake/blob/main/CONTRIBUTING.md)
- [Release Notes](https://github.com/Try-Wind/Wake/releases)

## 🛠️ Alternative Installation Methods

### From Source
If npm installation doesn't work for your platform:

```bash
# Clone the repository
git clone https://github.com/Try-Wind/Wake.git
cd Wake

# Install with Cargo
cargo install --path wake-cli
```

### Direct Download
Download pre-built binaries from [GitHub Releases](https://github.com/Try-Wind/Wake/releases/latest)

## 🐛 Troubleshooting

### Installation Failed
- Check your internet connection
- Ensure you have Node.js 14+ installed
- Try clearing npm cache: `npm cache clean --force`
- Install from source as an alternative

### Binary Not Found
```bash
# Reinstall the package
npm uninstall -g @trywind/wake
npm install -g @trywind/wake
```

### Permission Denied (Unix/Linux/macOS)
```bash
# Make the binary executable
chmod +x $(npm root -g)/@trywind/wake/bin/wake
```

## 📮 Support

- **Issues**: [GitHub Issues](https://github.com/Try-Wind/Wake/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Try-Wind/Wake/discussions)

## 📄 License

Apache-2.0 - see [LICENSE](https://github.com/Try-Wind/Wake/blob/main/LICENSE) for details.

---

Built with ❤️ by [Wind](https://wind.ai)