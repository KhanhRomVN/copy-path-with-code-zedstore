#!/bin/bash

# Copy Path with Code - Zed Extension Setup Script
# This script helps set up the development environment for the extension

set -e

echo "🚀 Setting up Copy Path with Code Zed Extension..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust via rustup:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "   Then restart your terminal and run this script again."
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Check if wasm32-wasi target is installed
if ! rustup target list --installed | grep -q "wasm32-wasi"; then
    echo "📦 Installing wasm32-wasi target..."
    rustup target add wasm32-wasi
else
    echo "✅ wasm32-wasi target already installed"
fi

# Check if Zed is installed
if ! command -v zed &> /dev/null; then
    echo "⚠️  Zed editor not found in PATH. Please install Zed from https://zed.dev/"
    echo "   The extension will still build, but you won't be able to test it without Zed."
else
    echo "✅ Zed found: $(zed --version 2>/dev/null || echo "version unknown")"
fi

# Build the extension
echo "🔨 Building extension..."
cargo build --target wasm32-wasi --release

if [ $? -eq 0 ]; then
    echo "✅ Extension built successfully!"

    # Check if target file exists
    if [ -f "target/wasm32-wasi/release/copy_path_with_code.wasm" ]; then
        echo "📦 WebAssembly module created: target/wasm32-wasi/release/copy_path_with_code.wasm"

        # Get file size
        size=$(du -h target/wasm32-wasi/release/copy_path_with_code.wasm | cut -f1)
        echo "   Size: $size"
    fi

    echo ""
    echo "🎉 Setup complete!"
    echo ""
    echo "Next steps:"
    echo "1. Open Zed editor"
    echo "2. Press Cmd+Shift+P (macOS) or Ctrl+Shift+P (Linux/Windows)"
    echo "3. Type 'Extensions' and select 'Extensions: Install Extensions'"
    echo "4. Click 'Install Dev Extension'"
    echo "5. Select this directory: $(pwd)"
    echo ""
    echo "Development commands:"
    echo "  ./build.sh         - Build the extension"
    echo "  ./test.sh          - Run tests"
    echo "  ./dev.sh           - Development mode with file watching"
    echo ""
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi
