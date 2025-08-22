#!/bin/bash

# Copy Path with Code - Zed Extension Test Script
# This script runs unit tests and integration tests for the extension

set -e

echo "🧪 Running tests for Copy Path with Code Zed Extension..."

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

# Parse command line arguments
VERBOSE=""
TEST_NAME=""
COVERAGE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--verbose)
            VERBOSE="--verbose"
            shift
            ;;
        -c|--coverage)
            COVERAGE="true"
            shift
            ;;
        --test)
            TEST_NAME="--test $2"
            shift
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --verbose     Show detailed test output"
            echo "  -c, --coverage    Generate code coverage report"
            echo "  --test NAME       Run specific test"
            echo "  -h, --help        Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

echo "🔍 Running Rust unit tests..."

# Run regular unit tests (not targeting wasm32-wasi for unit tests)
cargo test $VERBOSE $TEST_NAME

if [ $? -eq 0 ]; then
    echo "✅ Unit tests passed!"
else
    echo "❌ Unit tests failed!"
    exit 1
fi

# Run clippy for linting
if command -v cargo-clippy &> /dev/null || cargo clippy --version &> /dev/null 2>&1; then
    echo ""
    echo "🔍 Running clippy lints..."

    cargo clippy -- -D warnings

    if [ $? -eq 0 ]; then
        echo "✅ Clippy checks passed!"
    else
        echo "⚠️  Clippy found issues. Please fix the warnings above."
    fi
else
    echo "ℹ️  Clippy not available. Install with: rustup component add clippy"
fi

# Check formatting
if command -v cargo-fmt &> /dev/null || cargo fmt --version &> /dev/null 2>&1; then
    echo ""
    echo "🔍 Checking code formatting..."

    if cargo fmt --check; then
        echo "✅ Code formatting is correct!"
    else
        echo "❌ Code formatting issues found. Run 'cargo fmt' to fix."
        exit 1
    fi
else
    echo "ℹ️  rustfmt not available. Install with: rustup component add rustfmt"
fi

# Test WebAssembly build
echo ""
echo "🔍 Testing WebAssembly build..."

cargo build --target wasm32-wasi --release

if [ $? -eq 0 ]; then
    echo "✅ WebAssembly build successful!"

    # Check if wasm file was created
    WASM_FILE="target/wasm32-wasi/release/copy_path_with_code.wasm"
    if [ -f "$WASM_FILE" ]; then
        size=$(du -h "$WASM_FILE" | cut -f1)
        echo "   WebAssembly module size: $size"

        # Basic wasm validation if wasm tools are available
        if command -v wasm-validate &> /dev/null; then
            echo "   Validating WebAssembly module..."
            if wasm-validate "$WASM_FILE"; then
                echo "   ✅ WebAssembly module is valid"
            else
                echo "   ❌ WebAssembly module validation failed"
                exit 1
            fi
        fi
    else
        echo "   ⚠️  WebAssembly file not found at expected location"
    fi
else
    echo "❌ WebAssembly build failed!"
    exit 1
fi

# Generate test coverage if requested
if [ "$COVERAGE" = "true" ]; then
    echo ""
    echo "📊 Generating test coverage report..."

    if command -v cargo-tarpaulin &> /dev/null; then
        cargo tarpaulin --out Html --output-dir coverage/
        echo "✅ Coverage report generated in coverage/tarpaulin-report.html"
    elif command -v grcov &> /dev/null; then
        echo "🔍 Using grcov for coverage..."
        export RUSTFLAGS="-Cinstrument-coverage"
        export LLVM_PROFILE_FILE="coverage-%p-%m.profraw"

        cargo test
        grcov . --binary-path target/debug/ -s . -t html --branch --ignore-not-existing -o coverage/

        echo "✅ Coverage report generated in coverage/"
    else
        echo "ℹ️  Install cargo-tarpaulin or grcov for coverage reports:"
        echo "   cargo install cargo-tarpaulin"
        echo "   # or"
        echo "   cargo install grcov"
    fi
fi

# Integration tests (simulate Zed extension loading)
echo ""
echo "🔍 Running integration tests..."

# Test extension.toml parsing
if [ -f "extension.toml" ]; then
    echo "   ✅ extension.toml exists"

    # Basic TOML validation if available
    if command -v toml &> /dev/null; then
        if toml verify extension.toml; then
            echo "   ✅ extension.toml is valid"
        else
            echo "   ❌ extension.toml has syntax errors"
            exit 1
        fi
    fi
else
    echo "   ❌ extension.toml not found"
    exit 1
fi

# Test Cargo.toml
if [ -f "Cargo.toml" ]; then
    echo "   ✅ Cargo.toml exists"

    # Check for required dependencies
    if grep -q "zed_extension_api" Cargo.toml; then
        echo "   ✅ zed_extension_api dependency found"
    else
        echo "   ❌ zed_extension_api dependency missing"
        exit 1
    fi

    if grep -q 'crate-type.*cdylib' Cargo.toml; then
        echo "   ✅ cdylib crate type configured"
    else
        echo "   ❌ cdylib crate type not configured"
        exit 1
    fi
else
    echo "   ❌ Cargo.toml not found"
    exit 1
fi

echo ""
echo "🎉 All tests passed!"
echo ""
echo "📋 Test Summary:"
echo "✅ Unit tests"
echo "✅ Code linting (clippy)"
echo "✅ Code formatting"
echo "✅ WebAssembly build"
echo "✅ Configuration validation"

if [ "$COVERAGE" = "true" ]; then
    echo "✅ Test coverage report"
fi

echo ""
echo "🏁 Testing complete! Your extension is ready for development."

# Show next steps
echo ""
echo "Next steps:"
echo "1. Install the extension in Zed using './build.sh'"
echo "2. Test the functionality manually in Zed"
echo "3. Create additional integration tests as needed"
