#!/bin/bash

# Lume macOS Setup Script
# This script installs all dependencies required for Lume development on macOS.

set -e

echo "🚀 Starting Lume macOS Setup..."

# 1. Check for Homebrew
if ! command -v brew &> /dev/null; then
    echo "Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
else
    echo "✅ Homebrew already installed."
fi

# 2. Install system dependencies
echo "Installing system dependencies..."
brew install rustup cmake pkg-config

# 3. Initialize Rust
if ! command -v cargo &> /dev/null; then
    echo "Initializing Rust toolchain..."
    rustup-init -y
    source $HOME/.cargo/env
else
    echo "✅ Rust already installed."
fi

# 4. WGPU/Metal Note
echo ""
echo "------------------------------------------------------------------"
echo "✅ Setup Complete!"
echo "On macOS, WGPU will automatically use the Metal backend."
echo "You do not need to install Vulkan drivers manually."
echo "------------------------------------------------------------------"
echo "To start developing:"
echo "  cd /Users/nwani/personal-projects/lume"
echo "  cargo run"
echo "------------------------------------------------------------------"
