use crate::models::{CopiedFile, FileSelection};
use std::fs;

pub struct ClipboardManager {
    pub copied_files: Vec<CopiedFile>,
}

impl ClipboardManager {
    pub fn new() -> Self {
        Self {
            copied_files: Vec::new(),
        }
    }

    pub fn copy_file_with_content(
        &mut self,
        file_path: String,
        content: String,
        selection: Option<FileSelection>,
    ) -> Result<String, String> {
        let display_path = if let Some(sel) = &selection {
            sel.format_path_with_lines(&file_path)
        } else {
            file_path.clone()
        };

        let file_content = if let Some(sel) = selection {
            sel.content
        } else {
            content
        };

        let copied_file = CopiedFile::new(display_path, file_path, file_content);

        // Remove existing file with same base path
        self.copied_files.retain(|f| f.base_path != copied_file.base_path);
        self.copied_files.push(copied_file);

        Ok(self.generate_combined_content())
    }

    pub fn copy_multiple_files(&mut self, file_paths: Vec<String>) -> Result<String, String> {
        let mut successful_copies = 0;

        for file_path in file_paths {
            match fs::read_to_string(&file_path) {
                Ok(content) => {
                    let copied_file = CopiedFile::new(
                        file_path.clone(),
                        file_path.clone(),
                        content,
                    );

                    // Remove existing file with same base path
                    self.copied_files.retain(|f| f.base_path != copied_file.base_path);
                    self.copied_files.push(copied_file);
                    successful_copies += 1;
                }
                Err(_) => {
                    // Skip files that can't be read
                    continue;
                }
            }
        }

        if successful_copies == 0 {
            return Err("No files could be read successfully".to_string());
        }

        Ok(self.generate_combined_content())
    }

    pub fn clear(&mut self) {
        self.copied_files.clear();
    }

    pub fn generate_combined_content(&self) -> String {
        self.copied_files
            .iter()
            .map(|f| format!("{}\n\n{}", f.display_path, f.content))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    pub fn get_file_count(&self) -> usize {
        self.copied_files.len()
    }

    pub fn has_files(&self) -> bool {
        !self.copied_files.is_empty()
    }

    pub fn get_files(&self) -> &Vec<CopiedFile> {
        &self.copied_files
    }

    pub fn remove_file(&mut self, base_path: &str) -> bool {
        let initial_len = self.copied_files.len();
        self.copied_files.retain(|f| f.base_path != base_path);
        self.copied_files.len() != initial_len
    }

    pub fn contains_file(&self, base_path: &str) -> bool {
        self.copied_files.iter().any(|f| f.base_path == base_path)
    }

    pub fn get_status_message(&self) -> String {
        let count = self.copied_files.len();
        if count == 0 {
            "No files copied".to_string()
        } else if count == 1 {
            format!("1 file copied")
        } else {
            format!("{} files copied", count)
        }
    }
}

impl Default for ClipboardManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::FileSelection;

    #[test]
    fn test_copy_file_with_content() {
        let mut clipboard = ClipboardManager::new();
        let result = clipboard.copy_file_with_content(
            "test.rs".to_string(),
            "fn main() {}".to_string(),
            None,
        );

        assert!(result.is_ok());
        assert_eq!(clipboard.get_file_count(), 1);
    }

    #[test]
    fn test_copy_with_selection() {
        let mut clipboard = ClipboardManager::new();
        let selection = FileSelection::new(10, 20, "selected content".to_string());

        let result = clipboard.copy_file_with_content(
            "test.rs".to_string(),
            "full content".to_string(),
            Some(selection),
        );

        assert!(result.is_ok());
        assert_eq!(clipboard.get_file_count(), 1);

        let files = clipboard.get_files();
        assert_eq!(files[0].display_path, "test.rs:10-20");
        assert_eq!(files[0].content, "selected content");
    }

    #[test]
    fn test_clear_clipboard() {
        let mut clipboard = ClipboardManager::new();
        clipboard.copy_file_with_content(
            "test.rs".to_string(),
            "content".to_string(),
            None,
        ).unwrap();

        assert_eq!(clipboard.get_file_count(), 1);

        clipboard.clear();
        assert_eq!(clipboard.get_file_count(), 0);
    }

    #[test]
    fn test_duplicate_file_replacement() {
        let mut clipboard = ClipboardManager::new();

        // Add first version
        clipboard.copy_file_with_content(
            "test.rs".to_string(),
            "old content".to_string(),
            None,
        ).unwrap();

        // Add updated version of same file
        clipboard.copy_file_with_content(
            "test.rs".to_string(),
            "new content".to_string(),
            None,
        ).unwrap();

        assert_eq!(clipboard.get_file_count(), 1);
        let files = clipboard.get_files();
        assert_eq!(files[0].content, "new content");
    }
}
