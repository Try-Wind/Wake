#!/bin/bash

# Wake Installation Script
# Hardware-First Coding Agent by Wind

set -e

REPO_URL="https://github.com/Try-Wind/Wake"
INSTALL_DIR="$HOME/.wake"
BIN_DIR="$HOME/.local/bin"
VERSION="latest"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}"
echo "╔══════════════════════════════════════════╗"
echo "║     Wake - Hardware-First Coding Agent   ║"
echo "║              by Wind                     ║"
echo "╚══════════════════════════════════════════╝"
echo -e "${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$OS" in
    linux*)
        PLATFORM="linux"
        ;;
    darwin*)
        PLATFORM="macos"
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH_SUFFIX="x64"
        ;;
    aarch64|arm64)
        ARCH_SUFFIX="arm64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

BINARY_NAME="wake-${PLATFORM}-${ARCH_SUFFIX}"

echo -e "${YELLOW}Installing Wake for ${PLATFORM} (${ARCH})...${NC}"

# Check for required tools
check_command() {
    if ! command -v "$1" &> /dev/null; then
        echo -e "${RED}Error: $1 is not installed.${NC}"
        echo "Please install $1 and try again."
        exit 1
    fi
}

check_command curl
check_command git

# Method 1: Try downloading pre-built binary
echo -e "${YELLOW}Checking for pre-built binaries...${NC}"

RELEASE_URL="https://api.github.com/repos/Try-Wind/Wake/releases/latest"
DOWNLOAD_URL=$(curl -s "$RELEASE_URL" | grep "browser_download_url.*${BINARY_NAME}" | cut -d '"' -f 4)

if [ -n "$DOWNLOAD_URL" ]; then
    echo -e "${GREEN}Found pre-built binary, downloading...${NC}"
    
    # Create directories
    mkdir -p "$BIN_DIR"
    
    # Download binary
    curl -L "$DOWNLOAD_URL" -o "$BIN_DIR/wake"
    chmod +x "$BIN_DIR/wake"
    
    echo -e "${GREEN}Wake installed successfully!${NC}"
else
    echo -e "${YELLOW}No pre-built binary found. Building from source...${NC}"
    
    # Method 2: Build from source
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        echo -e "${YELLOW}Rust is not installed. Installing Rust...${NC}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    # Clone or update repository
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${YELLOW}Updating existing Wake installation...${NC}"
        cd "$INSTALL_DIR"
        git pull
    else
        echo -e "${YELLOW}Cloning Wake repository...${NC}"
        git clone "$REPO_URL" "$INSTALL_DIR"
        cd "$INSTALL_DIR"
    fi
    
    # Build Wake
    echo -e "${YELLOW}Building Wake... This may take a few minutes.${NC}"
    cargo build --release
    
    # Create directories
    mkdir -p "$BIN_DIR"
    
    # Copy binary
    cp "target/release/wake" "$BIN_DIR/wake"
    chmod +x "$BIN_DIR/wake"
    
    echo -e "${GREEN}Wake built and installed successfully!${NC}"
fi

# Create configuration directory
mkdir -p "$HOME/.config/wake"

# Create default configuration if it doesn't exist
if [ ! -f "$HOME/.config/wake/config.json" ]; then
    cat > "$HOME/.config/wake/config.json" << 'EOF'
{
  "providers": [
    {
      "provider": "openai",
      "env_vars": {
        "OPENAI_API_KEY": "your-api-key-here"
      },
      "model": "gpt-4",
      "tool_method": "FunctionCall"
    }
  ],
  "selected_provider": 0,
  "hardware_focus": {
    "default_platform": "esp32",
    "preferred_language": "c",
    "enable_hardware_tools": true
  }
}
EOF
    echo -e "${YELLOW}Created default configuration at ~/.config/wake/config.json${NC}"
    echo -e "${YELLOW}Please edit this file to add your API keys.${NC}"
fi

# Add to PATH if not already there
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo -e "${YELLOW}Adding Wake to PATH...${NC}"
    
    # Detect shell
    if [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
    
    echo "" >> "$SHELL_RC"
    echo "# Wake - Hardware-First Coding Agent" >> "$SHELL_RC"
    echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
    
    echo -e "${GREEN}Added Wake to PATH in $SHELL_RC${NC}"
    echo -e "${YELLOW}Please run: source $SHELL_RC${NC}"
    echo -e "${YELLOW}Or restart your terminal.${NC}"
fi

echo ""
echo -e "${GREEN}╔══════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     Installation Complete! 🎉            ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Next steps:${NC}"
echo "1. Configure your AI provider: wake auth"
echo "2. Start using Wake: wake"
echo "3. Get help: wake --help"
echo ""
echo -e "${YELLOW}For hardware examples and documentation:${NC}"
echo "https://github.com/Try-Wind/Wake"
echo ""
echo -e "${GREEN}Happy hardware hacking! 🔧${NC}"