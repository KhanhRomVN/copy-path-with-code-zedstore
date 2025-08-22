# Examples - Copy Path with Code Zed Extension

This directory contains examples and use cases for the Copy Path with Code extension.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Advanced Features](#advanced-features)
- [Common Workflows](#common-workflows)
- [API Examples](#api-examples)
- [Integration Patterns](#integration-patterns)

## Basic Usage

### Example 1: Copy Single File

1. Open a file in Zed (e.g., `main.rs`)
2. Use command palette: `Cmd+Shift+P` → "Copy Path and Content"
3. Result in clipboard:

```
src/main.rs

fn main() {
    println!("Hello, world!");
}
```

### Example 2: Copy Selected Text

1. Select lines 5-10 in `lib.rs`
2. Use command palette: "Copy Path and Content"
3. Result in clipboard:

```
src/lib.rs:5-10

pub struct Config {
    pub debug: bool,
    pub port: u16,
}
```

## Advanced Features

### Example 3: Create and Manage Folders

```rust
// Simulated command calls (actual usage through Zed commands)

// Create a folder with current open files
create_folder("Frontend Components", vec![
    "src/components/Button.tsx",
    "src/components/Modal.tsx",
    "src/styles/components.css"
]);

// Add more files to the folder
add_file_to_folder("folder_123", "src/components/Input.tsx");

// Copy all folder contents at once
copy_folder_contents("folder_123");
```

### Example 4: Batch Operations

Working with multiple related files:

1. Create folder: "API Layer"
2. Add files:
   - `src/api/users.rs`
   - `src/api/auth.rs` 
   - `src/api/mod.rs`
3. Copy folder contents → Get all API files formatted for documentation

Result:
```
src/api/users.rs

pub async fn get_user(id: u64) -> Result<User, ApiError> {
    // Implementation
}

---

src/api/auth.rs

pub async fn authenticate(token: &str) -> Result<Claims, AuthError> {
    // Implementation  
}

---

src/api/mod.rs

pub mod users;
pub mod auth;
```

## Common Workflows

### Workflow 1: Documentation Generation

**Use Case**: Creating documentation with code examples

1. **Setup Folders by Feature**
   - Create folder: "Authentication Examples"
   - Create folder: "Database Examples" 
   - Create folder: "API Examples"

2. **Collect Code Samples**
   - Add relevant files to each folder
   - Use selection copying for specific functions

3. **Generate Documentation**
   - Copy folder contents
   - Paste into documentation with proper formatting

### Workflow 2: Code Review Preparation

**Use Case**: Preparing code for review or sharing

1. **Collect Changed Files**
   ```bash
   # Get list of changed files
   git diff --name-only HEAD~1
   ```

2. **Create Review Folder**
   - Create folder: "PR #123 - Feature XYZ"
   - Add all changed files to folder

3. **Share Context**
   - Copy folder contents
   - Share formatted code with reviewers

### Workflow 3: Bug Investigation

**Use Case**: Gathering related code for debugging

1. **Identify Problem Area**
   - Create folder: "Login Bug Investigation"

2. **Collect Related Code**
   - Add error-prone files
   - Add test files
   - Add configuration files

3. **Analyze Together**
   - Copy folder contents
   - Review all related code in context

## API Examples

### Extension Command Usage

These examples show how the extension processes commands internally:

#### Copy with Selection
```rust
// Command: copy_path_with_content
// Args: ["src/main.rs", "full_file_content", "10", "15", "selected_content"]

let result = extension.handle_command("copy_path_with_content", vec![
    "src/main.rs".to_string(),
    "fn main() {\n    println!(\"Hello\");\n}".to_string(),
    "10".to_string(),
    "15".to_string(),
    "println!(\"Hello\");".to_string(),
]);

// Result: "Copied 1 files to clipboard"
// Clipboard: "src/main.rs:10-15\n\nprintln!(\"Hello\");"
```

#### Folder Management
```rust
// Create folder
let result = extension.handle_command("create_folder", vec![
    "My Components".to_string(),
    "Button.tsx".to_string(),
    "Modal.tsx".to_string(),
]);

// Add file to folder  
let result = extension.handle_command("add_file_to_folder", vec![
    "folder_123456789".to_string(),
    "Input.tsx".to_string(),
]);

// Copy folder contents
let result = extension.handle_command("copy_folder_contents", vec![
    "folder_123456789".to_string(),
]);
```

## Integration Patterns

### Pattern 1: Multi-Project Development

When working across multiple projects:

1. **Project-Based Folders**
   - Create folder: "Frontend - React App"
   - Create folder: "Backend - Rust API"
   - Create folder: "Shared - Types"

2. **Cross-Reference Code**
   - Copy related implementations
   - Share between team members
   - Document integration points

### Pattern 2: Learning and Teaching

For educational purposes:

1. **Concept Folders**
   - Create folder: "Rust Ownership Examples"
   - Create folder: "Async Programming Patterns"
   - Create folder: "Error Handling Best Practices"

2. **Code Examples**
   - Collect exemplary code
   - Copy with context for tutorials
   - Share learning resources

### Pattern 3: Refactoring Support

During code refactoring:

1. **Before/After Comparison**
   - Create folder: "Before Refactoring"
   - Create folder: "After Refactoring"
   - Copy same files to both folders at different stages

2. **Impact Analysis**
   - Copy affected files
   - Review changes in context
   - Validate refactoring correctness

## Tips and Best Practices

### Naming Conventions

**Good Folder Names:**
- "Frontend Components - Dashboard"
- "API Layer - User Management" 
- "Bug Fix #456 - Memory Leak"
- "Feature - OAuth Integration"

**Avoid:**
- Generic names like "Folder1", "Misc"
- Very long names
- Special characters that might cause issues

### File Organization

1. **Group by Purpose**: Related functionality together
2. **Include Context**: Tests with implementation
3. **Consider Dependencies**: Include imported files
4. **Document Intent**: Clear folder names and purposes

### Performance Tips

1. **Large Files**: Be mindful of memory usage with very large files
2. **Many Files**: Consider breaking large folders into smaller ones
3. **Frequent Updates**: Clear clipboard periodically to free memory

## Troubleshooting Examples

### Common Issues and Solutions

#### Issue: Files Not Found
```
Error: File not found
```
**Solution**: Ensure file paths are relative to workspace root

#### Issue: Folder Already Exists  
```
Error: Folder with this name already exists
```
**Solution**: Use descriptive, unique folder names

#### Issue: Empty Folder Contents
```
Error: No readable files found in folder
```
**Solution**: Check file permissions and paths

### Debug Commands

```rust
// Get extension status
let status = extension.handle_command("status", vec![]);
// Returns: "Clipboard: 3 files copied | Folders: 2 | Total folder files: 8"

// List all folders
let folders = extension.handle_command("list_folders", vec![]);
// Returns: "folder_1: My Components (3 files)\nfolder_2: API Layer (5 files)"
```

## Real-World Scenarios

### Scenario 1: Open Source Contribution

Contributing to a Rust project:

1. Create folder: "Contribution - Add JSON Serialization"
2. Add modified files and new tests
3. Copy folder contents for PR description
4. Include before/after examples

### Scenario 2: Client Code Review

Sharing code with clients:

1. Create folder: "Client Review - Feature Demo"
2. Add core implementation files
3. Include example usage and tests  
4. Copy with professional formatting

### Scenario 3: Technical Interview

Preparing code samples:

1. Create folders by topic:
   - "Algorithms - Sorting"
   - "System Design - Cache Implementation"
   - "Testing - Unit Test Examples"
2. Copy relevant code for discussion
3. Format for presentation

---

For more examples and use cases, check the test files in `src/` which demonstrate the API usage patterns.