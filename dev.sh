#!/bin/bash

# Copy Path with Code - Zed Extension Development Script
# This script provides a development environment with automatic rebuilding

set -e

echo "🚀 Starting Copy Path with Code development environment..."

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
WATCH_MODE=""
TEST_ON_CHANGE=""
VERBOSE=""
PORT="3000"

while [[ $# -gt 0 ]]; do
    case $1 in
        -w|--watch)
            WATCH_MODE="true"
            shift
            ;;
        -t|--test)
            TEST_ON_CHANGE="true"
            shift
            ;;
        -v|--verbose)
            VERBOSE="--verbose"
            shift
            ;;
        --port)
            PORT="$2"
            shift
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -w, --watch       Watch files and rebuild on changes"
            echo "  -t, --test        Run tests after each rebuild"
            echo "  -v, --verbose     Show detailed output"
            echo "  --port PORT       Port for development server (default: 3000)"
            echo "  -h, --help        Show this help message"
            echo ""
            echo "Development workflow:"
            echo "1. Make changes to source files"
            echo "2. Extension automatically rebuilds"
            echo "3. Reload dev extension in Zed to test changes"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h or --help for usage information"
            exit 1
            ;;
    esac
done

# Function to build extension
build_extension() {
    echo "🔨 Building extension..."

    if cargo build --target wasm32-wasi --release; then
        echo "✅ Build successful at $(date)"

        # Show file size
        WASM_FILE="target/wasm32-wasi/release/copy_path_with_code.wasm"
        if [ -f "$WASM_FILE" ]; then
            size=$(du -h "$WASM_FILE" | cut -f1)
            echo "📦 Extension size: $size"
        fi

        return 0
    else
        echo "❌ Build failed at $(date)"
        return 1
    fi
}

# Function to run tests
run_tests() {
    echo "🧪 Running tests..."

    if cargo test $VERBOSE; then
        echo "✅ Tests passed at $(date)"
        return 0
    else
        echo "❌ Tests failed at $(date)"
        return 1
    fi
}

# Function to handle file changes
handle_change() {
    local file="$1"
    echo "📝 File changed: $file"

    if build_extension; then
        if [ "$TEST_ON_CHANGE" = "true" ]; then
            run_tests
        fi

        echo "🔄 Ready for testing in Zed. Reload your dev extension."
        echo "   To reload: Cmd+Shift+P → 'Extensions: Reload Dev Extensions'"
    fi

    echo "⏰ Watching for changes... (Press Ctrl+C to stop)"
}

# Initial build
echo "🔨 Initial build..."
if ! build_extension; then
    echo "❌ Initial build failed. Please fix errors before continuing."
    exit 1
fi

if [ "$TEST_ON_CHANGE" = "true" ]; then
    echo "🧪 Running initial tests..."
    run_tests
fi

# Watch mode
if [ "$WATCH_MODE" = "true" ]; then
    echo "👀 Starting file watcher..."
    echo "   Watching: src/, Cargo.toml, extension.toml"
    echo "   Press Ctrl+C to stop"
    echo ""

    # Check if file watching tools are available
    if command -v fswatch &> /dev/null; then
        # macOS fswatch
        fswatch -o src/ Cargo.toml extension.toml | while read event; do
            handle_change "detected"
        done
    elif command -v inotifywait &> /dev/null; then
        # Linux inotify
        while inotifywait -r -e modify,create,delete src/ Cargo.toml extension.toml; do
            handle_change "detected"
        done
    elif command -v entr &> /dev/null; then
        # Cross-platform entr
        find src/ -name "*.rs" -o -name "Cargo.toml" -o -name "extension.toml" | entr -r sh -c 'echo "File changed"; ./dev.sh --build-only'
    else
        echo "⚠️  No file watcher available. Please install one of:"
        echo "   macOS: brew install fswatch"
        echo "   Linux: apt-get install inotify-tools"
        echo "   Cross-platform: brew install entr or apt-get install entr"
        echo ""
        echo "Manual mode: Run './build.sh' after making changes"
        echo "Press Enter to exit..."
        read
        exit 0
    fi
else
    echo ""
    echo "🎉 Development environment ready!"
    echo ""
    echo "📋 Development workflow:"
    echo "1. Make changes to your source files"
    echo "2. Run './build.sh' to rebuild"
    echo "3. In Zed: Cmd+Shift+P → 'Extensions: Reload Dev Extensions'"
    echo "4. Test your changes"
    echo ""
    echo "💡 Pro tips:"
    echo "• Use './dev.sh --watch' for automatic rebuilding"
    echo "• Use './test.sh' to run all tests"
    echo "• Check './build.sh --help' for build options"
    echo ""
    echo "🔧 Current extension status:"

    WASM_FILE="target/wasm32-wasi/release/copy_path_with_code.wasm"
    if [ -f "$WASM_FILE" ]; then
        size=$(du -h "$WASM_FILE" | cut -f1)
        echo "✅ Extension built: $size"
    else
        echo "❌ Extension not built"
    fi

    echo ""
    echo "📁 Extension directory: $(pwd)"
    echo "🌐 Install in Zed: Extensions → Install Dev Extension → Select this directory"
fi

# Cleanup function
cleanup() {
    echo ""
    echo "🛑 Development environment stopped"
    echo "✨ Thanks for developing Copy Path with Code!"
    exit 0
}

# Set up signal handlers for graceful shutdown
trap cleanup SIGINT SIGTERM

# If not in watch mode, just show the development info and exit
if [ "$WATCH_MODE" != "true" ]; then
    echo ""
    echo "🏁 Development setup complete!"
fi
