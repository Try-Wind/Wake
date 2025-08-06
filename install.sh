#!/bin/bash

# Wake Installation Script
# Hardware-First Coding Agent by Wind

set -e

echo "================================"
echo "  Wake Installation Script"
echo "  Hardware-First Coding Agent"
echo "  Built by Wind"
echo "================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
   echo -e "${RED}Please do not run this script as root${NC}"
   exit 1
fi

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust is not installed. Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

echo "Checking Rust version..."
rustc --version
cargo --version

# Clone or update repository
if [ -d "$HOME/.wake" ]; then
    echo "Updating Wake..."
    cd $HOME/.wake
    git pull origin main
else
    echo "Cloning Wake repository..."
    git clone https://github.com/Try-Wind/Wake.git $HOME/.wake
    cd $HOME/.wake
fi

# Build Wake
echo ""
echo "Building Wake (this may take a few minutes)..."
cargo build --release

# Install Wake
echo ""
echo "Installing Wake..."
cargo install --path wake-cli --force

# Add to PATH if not already there
if ! grep -q '.cargo/bin' ~/.bashrc 2>/dev/null; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
fi

if ! grep -q '.cargo/bin' ~/.zshrc 2>/dev/null; then
    echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc 2>/dev/null || true
fi

# Create default config directory
mkdir -p $HOME/.config/wake

# Success message
echo ""
echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}  Wake Installation Complete!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo "To get started:"
echo "  1. Reload your shell: source ~/.bashrc (or ~/.zshrc)"
echo "  2. Configure your AI provider: wake auth"
echo "  3. Start using Wake: wake"
echo ""
echo "Example commands:"
echo "  wake \"Generate Arduino driver for MPU6050\""
echo "  wake \"Debug my I2C communication issue\""
echo "  wake \"Calculate UART baud rate for 16MHz clock\""
echo ""
echo "For more information, visit: https://github.com/Try-Wind/Wake"
echo ""