# Copy Path with Code - Zed Extension

A Zed editor extension that allows you to copy file paths along with their content to the clipboard, with advanced folder management capabilities for organizing your code files.

## Features

- **Copy Path with Content**: Copy the current file's path and content to clipboard
- **Selection Support**: Copy only selected text with line number annotations
- **Folder Management**: Create and manage folders to organize related files
- **Batch Operations**: Copy contents from multiple files at once
- **Persistent Storage**: Your folders and preferences are saved between sessions

## Installation

### From Zed Extension Store (Coming Soon)

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows) to open command palette
3. Type "Extensions" and select "Extensions: Install Extensions"
4. Search for "Copy Path with Code"
5. Click "Install"

### Development Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/khanhromvn/copy-path-with-code-zedstore.git
   ```

2. Open Zed and go to Extensions
3. Click "Install Dev Extension"
4. Select the `copy-path-with-code-zedstore` directory

## Usage

### Basic Commands

#### Copy Path with Content
- **Keyboard**: `Cmd+Alt+C` (macOS) or `Ctrl+Alt+C` (Linux/Windows)
- **Command Palette**: "Copy Path and Content"

This will copy the current file's relative path and its entire content to the clipboard. If you have text selected, it will only copy the selected portion with line number annotations.

#### Clear Clipboard
- **Keyboard**: `Cmd+Alt+Z` (macOS) or `Ctrl+Alt+Z` (Linux/Windows)
- **Command Palette**: "Clear Copied Paths and Content"

### Folder Management

#### Create Folder
- **Command Palette**: "Create Code Folder"

Creates a new folder and adds all currently open files to it.

#### Add File to Folder
- **Keyboard**: `Cmd+Alt+A` (macOS) or `Ctrl+Alt+A` (Linux/Windows)
- **Command Palette**: "Add Current File to Folder"

#### Remove File from Folder
- **Keyboard**: `Cmd+Alt+D` (macOS) or `Ctrl+Alt+D` (Linux/Windows)
- **Command Palette**: "Remove Current File from Folder"

### Advanced Features

#### Copy Folder Contents
Copy all files from a folder to the clipboard in a structured format with file separators.

#### Folder Operations
- Rename folders
- Delete folders
- Open all files in a folder
- Manage folder contents

## Output Format

When copying multiple files, the content is formatted as:

```
path/to/file1.js

// File content here

---

path/to/file2.py:10-25

# Selected content with line numbers

---

path/to/file3.md

<!-- Another file's content -->
```

## Configuration

The extension stores its configuration and folder data in Zed's extension storage. No additional setup is required.

## Development

This extension is written in Rust and compiled to WebAssembly for Zed.

### Prerequisites

- [Rust](https://rustup.rs/) installed via rustup
- Zed editor

### Building

```bash
cargo build --target wasm32-wasi --release
```

### Development Workflow

1. Make your changes
2. Build the extension
3. In Zed, go to Extensions → Reload Dev Extensions
4. Test your changes

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have feature requests, please [open an issue](https://github.com/khanhromvn/copy-path-with-code-zedstore/issues) on GitHub.

## Changelog

### v0.1.0
- Initial release
- Basic copy path with content functionality
- Folder management system
- Keyboard shortcuts
- Selection support with line numbers