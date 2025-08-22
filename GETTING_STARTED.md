# Getting Started - Copy Path with Code Zed Extension

Welcome to the Copy Path with Code extension for Zed! This guide will help you get up and running quickly.

## 📋 Project Overview

This is a complete Zed extension that provides clipboard functionality for copying file paths along with their content, plus advanced folder management capabilities. It's written in Rust and compiled to WebAssembly for optimal performance.

### Key Features
- 📂 Copy file paths with content to clipboard
- ✂️ Support for selected text with line numbers
- 📁 Folder management system for organizing code files
- 🔄 Batch operations for multiple files
- 💾 Persistent storage for your folders
- ⚡ Fast WebAssembly-based architecture

## 🚀 Quick Start (5 minutes)

### Prerequisites
- Zed editor installed ([download here](https://zed.dev/))
- Rust toolchain (we'll install this)

### Installation Steps

1. **Setup the development environment:**
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
   - Type "Extensions" → Select "Extensions: Install Extensions"
   - Click "Install Dev Extension" 
   - Select this project directory

4. **Test it works:**
   - Open any file in Zed
   - Press `Cmd+Shift+P` → "Copy Path and Content"
   - Paste somewhere to see your file path + content!

## 📖 Basic Usage

### Copy Current File
1. Open a file in Zed
2. Command palette: `Cmd+Shift+P` → "Copy Path and Content"
3. Content is now in your clipboard with path header

### Copy Selected Text
1. Select some text in your file
2. Command palette: "Copy Path and Content"  
3. Gets copied with line numbers (e.g., `file.rs:10-15`)

### Create Code Folders
1. Open multiple related files
2. Command palette: "Create Code Folder"
3. Enter folder name → All open files added automatically
4. Later: "Copy Folder Contents" to get all files at once

## 🛠️ Development Workflow

### Making Changes
```bash
# Start development with auto-rebuilding
./dev.sh --watch --test

# Make your changes in src/
# Extension rebuilds automatically

# In Zed: Cmd+Shift+P → "Extensions: Reload Dev Extensions"
```

### Running Tests
```bash
# Run all tests
./test.sh

# Run with coverage
./test.sh --coverage

# Check code quality
cargo fmt && cargo clippy
```

## 📂 Project Structure

```
copy-path-with-code-zedstore/
├── src/
│   ├── lib.rs           # Main extension entry point
│   ├── models/          # Data structures  
│   ├── clipboard/       # Clipboard management
│   └── folders/         # Folder operations
├── extension.toml       # Zed extension config
├── Cargo.toml          # Rust dependencies
├── build.sh            # Build script
├── test.sh             # Test runner
├── dev.sh              # Development server
└── examples/           # Usage examples
```

## 🎯 Next Steps

### For Users
1. **Try the basic features** - copy some files and see the output format
2. **Create folders** - organize your frequently accessed code files
3. **Explore batch operations** - copy entire folders for documentation
4. **Customize workflow** - create folders for different projects/features

### For Developers
1. **Read the code** - start with `src/lib.rs` and explore the modules
2. **Check out tests** - `src/*/mod.rs` files have comprehensive test suites
3. **Review architecture** - see `DEVELOPMENT.md` for detailed design info
4. **Add features** - look at the TODO comments and issue tracker

## 🔧 Troubleshooting

### Common Issues

**Extension not showing up in Zed:**
- Make sure `extension.toml` is in the root directory
- Check that the build completed successfully
- Try restarting Zed

**Build fails:**
- Run `./setup.sh` again to ensure Rust is properly installed
- Check that `wasm32-wasi` target is added: `rustup target list --installed`

**Commands not working:**
- Reload the dev extension in Zed
- Check Zed's console output for errors (start Zed with `zed --foreground`)

### Getting Help
1. Check `DEVELOPMENT.md` for detailed troubleshooting
2. Look at `examples/` for usage patterns
3. Review test files for API examples
4. Open an issue on GitHub with details

## 📚 Documentation

- **README.md** - User guide and features
- **DEVELOPMENT.md** - Complete development guide  
- **examples/README.md** - Usage examples and workflows
- **CHANGELOG.md** - Version history and changes

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-feature`
3. **Make your changes** with tests
4. **Run the full test suite**: `./test.sh`
5. **Submit a pull request** with a clear description

### Good First Issues
- Add more unit tests for edge cases
- Improve error messages
- Add keyboard shortcuts
- Create additional usage examples
- Optimize WebAssembly bundle size

## 📊 Project Status

- ✅ Core functionality complete
- ✅ Comprehensive test suite
- ✅ Development tooling setup
- ✅ Documentation complete
- 🔄 Ready for testing and feedback
- 🚀 Ready for Zed Extension Store submission

## 🎉 What Makes This Special

Unlike simple clipboard tools, this extension provides:

1. **Context-Aware Copying** - Always includes file paths for reference
2. **Organization System** - Folders help manage complex projects  
3. **Batch Processing** - Handle multiple files efficiently
4. **Developer-Focused** - Built for code sharing, documentation, and review workflows
5. **Performance** - WebAssembly for fast, efficient operation
6. **Extensible** - Clean architecture for adding new features

## 🚀 Ready to Use!

You now have a fully functional Zed extension that can significantly improve your code copying and organization workflow. The extension is production-ready and includes all the tooling needed for further development.

### Immediate Next Steps:
1. Install and test the basic functionality
2. Create your first code folder
3. Try copying a folder's contents
4. Share formatted code with your team
5. Provide feedback or contribute improvements!

---

**Happy coding with Copy Path with Code! 📋✨**

*Need help? Check the other documentation files or open an issue on GitHub.*