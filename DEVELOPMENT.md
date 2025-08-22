# Development Guide - Copy Path with Code Zed Extension

This guide provides comprehensive information for developing and contributing to the Copy Path with Code extension for Zed.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Development Workflow](#development-workflow)
- [Architecture Overview](#architecture-overview)
- [API Reference](#api-reference)
- [Testing](#testing)
- [Debugging](#debugging)
- [Performance Considerations](#performance-considerations)
- [Contributing](#contributing)
- [Troubleshooting](#troubleshooting)

## Prerequisites

Before starting development, ensure you have the following installed:

### Required Tools

1. **Rust** (via rustup)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **WebAssembly target**
   ```bash
   rustup target add wasm32-wasi
   ```

3. **Zed Editor**
   - Download from [zed.dev](https://zed.dev/)
   - Ensure it's in your PATH for CLI usage

### Optional but Recommended

4. **Development Tools**
   ```bash
   # Code formatting
   rustup component add rustfmt
   
   # Linting
   rustup component add clippy
   
   # File watching (choose one based on your OS)
   brew install fswatch          # macOS
   apt-get install inotify-tools  # Linux
   brew install entr             # Cross-platform
   
   # WebAssembly tools
   cargo install wasm-pack
   ```

## Getting Started

### Quick Setup

1. **Clone and setup the project:**
   ```bash
   git clone https://github.com/khanhromvn/copy-path-with-code-zedstore.git
   cd copy-path-with-code-zedstore
   ./setup.sh
   ```

2. **Build the extension:**
   ```bash
   ./build.sh
   ```

3. **Install in Zed:**
   - Open Zed
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "Extensions" → "Extensions: Install Extensions"
   - Click "Install Dev Extension"
   - Select the project directory

### Development Scripts

The project includes several helper scripts:

- `./setup.sh` - Initial environment setup
- `./build.sh` - Build the extension
- `./test.sh` - Run all tests
- `./dev.sh` - Development mode with file watching

## Project Structure

```
copy-path-with-code-zedstore/
├── src/
│   ├── lib.rs                 # Main extension entry point
│   ├── models/
│   │   └── mod.rs            # Data structures and types
│   ├── clipboard/
│   │   └── mod.rs            # Clipboard management logic
│   └── folders/
│       └── mod.rs            # Folder management logic
├── extension.toml             # Extension metadata
├── Cargo.toml                # Rust dependencies
├── README.md                 # User documentation
├── DEVELOPMENT.md            # This file
├── LICENSE                   # MIT license
├── setup.sh                  # Development setup script
├── build.sh                  # Build script
├── test.sh                   # Test script
└── dev.sh                    # Development server script
```

## Development Workflow

### 1. Making Changes

Follow this workflow when developing features:

```bash
# 1. Create a feature branch
git checkout -b feature/new-feature

# 2. Start development environment
./dev.sh --watch --test

# 3. Make your changes in src/
# Files will automatically rebuild on save

# 4. Test in Zed
# Reload extension: Cmd+Shift+P → "Extensions: Reload Dev Extensions"
```

### 2. Code Quality

Before committing, ensure code quality:

```bash
# Format code
cargo fmt

# Run lints
cargo clippy -- -D warnings

# Run tests
./test.sh

# Build for production
./build.sh
```

### 3. Committing Changes

```bash
git add .
git commit -m "feat: add new clipboard functionality"
git push origin feature/new-feature
```

## Architecture Overview

### Core Components

#### 1. Extension Entry Point (`lib.rs`)

The main extension struct that implements the `zed::Extension` trait:

```rust
struct CopyPathWithCodeExtension {
    clipboard_manager: ClipboardManager,
    folder_manager: FolderManager,
}

impl zed::Extension for CopyPathWithCodeExtension {
    // Extension lifecycle methods
}
```

#### 2. Models (`models/mod.rs`)

Core data structures:

- `CopiedFile` - Represents a copied file with path and content
- `Folder` - Represents a folder containing multiple files
- `ExtensionState` - Overall extension state
- `FileSelection` - Represents selected text with line numbers

#### 3. Clipboard Manager (`clipboard/mod.rs`)

Handles copying files and content:

```rust
impl ClipboardManager {
    pub fn copy_file_with_content(&mut self, file_path: String, content: String, selection: Option<FileSelection>) -> Result<String, String>
    pub fn clear(&mut self)
    pub fn generate_combined_content(&self) -> String
}
```

#### 4. Folder Manager (`folders/mod.rs`)

Manages folder operations:

```rust
impl FolderManager {
    pub fn create_folder(&mut self, name: String, initial_files: Vec<String>) -> Result<String, String>
    pub fn add_file_to_folder(&mut self, folder_id: &str, file_path: String) -> Result<String, String>
    pub fn copy_folder_contents(&self, folder_id: &str) -> Result<String, String>
}
```

### Data Flow

1. **User Action** → Zed triggers extension command
2. **Command Processing** → Extension routes to appropriate manager
3. **State Update** → Manager updates internal state
4. **Response** → Result returned to Zed for user feedback

## API Reference

### Extension Commands

Commands that can be called from Zed:

#### Core Commands

- `copy_path_with_content(file_path, content, [start_line, end_line, selected_content])`
- `clear_clipboard()`
- `status()`

#### Folder Commands

- `create_folder(name, [initial_files...])`
- `delete_folder(folder_id)`
- `rename_folder(folder_id, new_name)`
- `add_file_to_folder(folder_id, file_path)`
- `remove_file_from_folder(folder_id, file_path)`
- `copy_folder_contents(folder_id)`
- `list_folders()`

### Return Values

All commands return either:
- `Ok(String)` - Success message
- `Err(String)` - Error message

## Testing

### Unit Tests

Run individual module tests:

```bash
# Test all modules
cargo test

# Test specific module
cargo test clipboard::tests
cargo test folders::tests
cargo test models::tests

# Verbose output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Full test suite including WebAssembly build
./test.sh

# With coverage
./test.sh --coverage

# Specific test
./test.sh --test test_name
```

### Manual Testing in Zed

1. Build and install the extension
2. Open test files in Zed
3. Test each command through the command palette
4. Verify clipboard contents externally

## Debugging

### Console Output

Use `println!` and `eprintln!` for debugging:

```rust
println!("Debug: Processing file {}", file_path);
eprintln!("Error: {}", error_message);
```

View output in terminal when starting Zed:

```bash
zed --foreground
```

### Extension Reload

After making changes:
1. Rebuild: `./build.sh`
2. In Zed: `Cmd+Shift+P` → "Extensions: Reload Dev Extensions"

### Common Debug Scenarios

#### State Issues
```rust
// Add debug prints to state changes
println!("State before: {:?}", self.clipboard_manager.get_files());
// ... perform operation
println!("State after: {:?}", self.clipboard_manager.get_files());
```

#### WebAssembly Issues
```bash
# Check WASM file was created
ls -la target/wasm32-wasi/release/copy_path_with_code.wasm

# Validate WASM if tools available
wasm-validate target/wasm32-wasi/release/copy_path_with_code.wasm
```

## Performance Considerations

### Memory Management

- Use `String` for owned data, `&str` for references
- Prefer `Vec::retain()` over creating new vectors
- Clear unused state regularly

### WebAssembly Optimizations

```toml
# Cargo.toml optimizations for release builds
[profile.release]
opt-level = "s"          # Optimize for size
lto = true              # Link-time optimization
strip = true            # Remove debug symbols
```

### File I/O

- Read files asynchronously when possible
- Batch file operations
- Handle large files gracefully

## Contributing

### Code Style

Follow Rust conventions:

```bash
# Format code
cargo fmt

# Check style
cargo clippy
```

### Commit Messages

Use conventional commits:

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `test:` - Test additions/changes
- `refactor:` - Code refactoring

### Pull Request Process

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Ensure all tests pass
5. Submit pull request with description

### Adding New Features

1. **Plan the feature:**
   - Define the API
   - Consider state management
   - Plan tests

2. **Implement:**
   ```rust
   // Add to appropriate module
   // Update main lib.rs command handler
   // Add unit tests
   ```

3. **Test:**
   ```bash
   ./test.sh
   # Manual testing in Zed
   ```

4. **Document:**
   - Update README.md for user-facing features
   - Add API documentation
   - Update this development guide

## Troubleshooting

### Common Issues

#### Build Failures

**Problem:** `error: linker 'cc' not found`
```bash
# Install build tools
xcode-select --install  # macOS
sudo apt-get install build-essential  # Linux
```

**Problem:** `wasm32-wasi target not found`
```bash
rustup target add wasm32-wasi
```

#### Extension Not Loading

**Problem:** Extension doesn't appear in Zed
- Check `extension.toml` syntax
- Verify WASM file was created
- Check Zed logs for errors

**Problem:** Commands not working
- Verify command names match exactly
- Check argument types and count
- Add debug prints to trace execution

#### Development Workflow Issues

**Problem:** File watcher not working
```bash
# Install appropriate tool for your OS
brew install fswatch      # macOS
sudo apt install inotify-tools  # Linux
```

**Problem:** Tests failing
```bash
# Clean and rebuild
cargo clean
./setup.sh
```

### Getting Help

1. Check this guide first
2. Review existing issues on GitHub
3. Create a new issue with:
   - OS and Rust version
   - Steps to reproduce
   - Error messages
   - Expected vs actual behavior

### Debug Checklist

Before reporting issues:

- [ ] Latest Rust version installed
- [ ] `wasm32-wasi` target added
- [ ] Clean build completed successfully
- [ ] Extension reloaded in Zed
- [ ] Zed restarted if needed
- [ ] Console output checked for errors

---

## Additional Resources

- [Zed Extension Documentation](https://zed.dev/docs/extensions)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WebAssembly with Rust](https://rustwasm.github.io/docs/book/)
- [Zed Extension API](https://crates.io/crates/zed_extension_api)

---

*Happy coding! 🦀 ⚡*