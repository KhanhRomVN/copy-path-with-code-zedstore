#!/bin/bash

# Copy Path with Code - Zed Extension Build Script
# This script builds the extension for development and production

set -e

echo "🔨 Building Copy Path with Code Zed Extension..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please run ./setup.sh first."
    exit 1
fi

# Check if wasm32-wasi target is available
if ! rustup target list --installed | grep -q "wasm32-wasi"; then
    echo "📦 Installing wasm32-wasi target..."
    rustup target add wasm32-wasi
fi

# Clean previous builds if --clean flag is passed
if [ "$1" = "--clean" ] || [ "$1" = "-c" ]; then
    echo "🧹 Cleaning previous builds..."
    cargo clean
fi

# Determine build mode
BUILD_MODE="release"
CARGO_FLAGS="--release"

if [ "$1" = "--debug" ] || [ "$1" = "-d" ]; then
    BUILD_MODE="debug"
    CARGO_FLAGS=""
    echo "🐛 Building in debug mode..."
else
    echo "⚡ Building in release mode..."
fi

# Build the extension
echo "🚀 Compiling WebAssembly module..."
cargo build --target wasm32-wasi $CARGO_FLAGS

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful!"

    # Show build output details
    WASM_FILE="target/wasm32-wasi/$BUILD_MODE/copy_path_with_code.wasm"

    if [ -f "$WASM_FILE" ]; then
        echo "📦 WebAssembly module: $WASM_FILE"

        # Get file size
        if command -v du &> /dev/null; then
            size=$(du -h "$WASM_FILE" | cut -f1)
            echo "   Size: $size"
        fi

        # Get file info
        if command -v file &> /dev/null; then
            file_info=$(file "$WASM_FILE")
            echo "   Type: $file_info"
        fi
    else
        echo "⚠️  Warning: Expected WebAssembly file not found at $WASM_FILE"
    fi

    echo ""
    echo "🎉 Build complete!"
    echo ""

    if [ "$BUILD_MODE" = "release" ]; then
        echo "📋 To install in Zed:"
        echo "1. Open Zed editor"
        echo "2. Open Extensions (Cmd+Shift+P → 'Extensions')"
        echo "3. Click 'Install Dev Extension'"
        echo "4. Select this directory: $(pwd)"
    else
        echo "🐛 Debug build created for development testing"
    fi

else
    echo "❌ Build failed!"
    echo ""
    echo "Common issues:"
    echo "- Check that all dependencies are correctly specified in Cargo.toml"
    echo "- Ensure zed_extension_api version is compatible"
    echo "- Verify Rust syntax in source files"
    echo ""
    echo "Run 'cargo check' for detailed error information"
    exit 1
fi

# Optional: Run basic checks
if command -v cargo &> /dev/null; then
    echo ""
    echo "🔍 Running additional checks..."

    # Check for common issues
    echo "   Checking syntax..."
    cargo check --target wasm32-wasi $CARGO_FLAGS --quiet

    if [ $? -eq 0 ]; then
        echo "   ✅ Syntax check passed"
    else
        echo "   ⚠️  Syntax check found issues"
    fi

    # Check formatting if rustfmt is available
    if command -v cargo-fmt &> /dev/null || cargo fmt --version &> /dev/null 2>&1; then
        echo "   Checking formatting..."
        if cargo fmt --check &> /dev/null; then
            echo "   ✅ Code formatting is correct"
        else
            echo "   ℹ️  Code formatting could be improved. Run 'cargo fmt' to fix."
        fi
    fi
fi

echo ""
echo "🏁 Build process complete!"
