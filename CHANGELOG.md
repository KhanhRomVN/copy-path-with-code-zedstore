# Changelog

All notable changes to the Copy Path with Code Zed extension will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure and documentation
- Development scripts for easier contribution workflow

## [0.1.0] - 2024-12-20

### Added
- Initial release of Copy Path with Code for Zed
- Core clipboard functionality to copy file paths with content
- Support for selected text with line number annotations
- Folder management system for organizing code files
- Persistent storage for folders and settings
- WebAssembly-based extension architecture
- Comprehensive test suite with unit and integration tests
- Development tooling with automatic rebuilding
- Cross-platform support (macOS, Linux, Windows)

### Features
- **Copy Path with Content**: Copy current file path and content to clipboard
- **Selection Support**: Copy only selected text with line number range
- **Folder Management**: Create, rename, delete folders for organizing files
- **Batch Operations**: Add/remove multiple files to/from folders
- **Folder Contents Copy**: Copy all files from a folder at once
- **Status Reporting**: Real-time status of copied files and folders
- **Error Handling**: Graceful error handling with user-friendly messages

### Commands
- `copy_path_with_content` - Copy current file with content
- `clear_clipboard` - Clear all copied content
- `create_folder` - Create new folder with optional initial files
- `delete_folder` - Delete folder by ID
- `rename_folder` - Rename existing folder
- `add_file_to_folder` - Add file to specific folder
- `remove_file_from_folder` - Remove file from folder
- `copy_folder_contents` - Copy all files from folder
- `list_folders` - List all folders with metadata
- `status` - Get extension status information

### Development Tools
- Automated setup script (`setup.sh`)
- Build script with debug/release modes (`build.sh`)
- Comprehensive test runner (`test.sh`)
- Development server with file watching (`dev.sh`)
- Code formatting and linting integration
- WebAssembly build validation

### Documentation
- User guide with installation and usage instructions
- Development guide for contributors
- API documentation with examples
- Troubleshooting guide
- Code examples and best practices

### Architecture
- Modular design with separate managers for clipboard and folders
- Type-safe data structures with serde serialization
- Memory-efficient state management
- WebAssembly optimization for performance
- Comprehensive error handling and logging

### Performance
- Optimized WebAssembly builds for size and speed
- Efficient memory management for large files
- Batch processing for multiple file operations
- Lazy loading of file contents when needed

### Security
- Safe file path handling to prevent directory traversal
- Input validation for all user-provided data
- Secure clipboard operations
- No external network dependencies

---

## Development Notes

### Version Numbering
- Major version: Breaking API changes
- Minor version: New features, backward compatible
- Patch version: Bug fixes, backward compatible

### Release Process
1. Update version in `Cargo.toml` and `extension.toml`
2. Update `CHANGELOG.md` with new version details
3. Run full test suite: `./test.sh --coverage`
4. Build release version: `./build.sh`
5. Create git tag: `git tag v0.1.0`
6. Push to repository: `git push origin main --tags`

### Upcoming Features (Planned)
- Custom keyboard shortcuts configuration
- Syntax highlighting in copied content
- File filtering and search within folders
- Export/import folder configurations
- Integration with external clipboard managers
- Theme customization for folder colors
- Collaborative folder sharing
- Plugin system for custom processors

### Known Limitations
- Currently requires manual clipboard pasting
- No real-time file synchronization
- Limited to text-based files
- No built-in file preview
- WebAssembly size constraints on very large files

---

*For detailed technical changes, see the git commit history.*