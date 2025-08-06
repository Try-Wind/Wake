#!/bin/bash

echo "Applying build fixes..."

# Fix the main issue - the ratatui dependency pointing to a non-existent branch
sed -i 's|ratatui = { git = "https://github.com/Marlinski/ratatui", branch = "feature/viewport-resize-v29" }|ratatui = "0.29"|g' Cargo.toml
sed -i 's|ratatui = { git = "https://github.com/Marlinski/ratatui", branch = "feature/viewport-resize-v29", features = \["crossterm"\] }|ratatui = { version = "0.29", features = ["crossterm"] }|g' wake-cli/Cargo.toml

echo "Fixes applied. Attempting build..."
cargo build --release 2>&1 | tail -20