use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopiedFile {
    pub display_path: String,
    pub base_path: String,
    pub content: String,
}

impl CopiedFile {
    pub fn new(display_path: String, base_path: String, content: String) -> Self {
        Self {
            display_path,
            base_path,
            content,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub files: Vec<String>,
    pub color: Option<String>,
}

impl Folder {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            files: Vec::new(),
            color: None,
        }
    }

    pub fn add_file(&mut self, file_path: String) -> bool {
        if !self.files.contains(&file_path) {
            self.files.push(file_path);
            true
        } else {
            false
        }
    }

    pub fn remove_file(&mut self, file_path: &str) -> bool {
        if let Some(index) = self.files.iter().position(|f| f == file_path) {
            self.files.remove(index);
            true
        } else {
            false
        }
    }

    pub fn has_file(&self, file_path: &str) -> bool {
        self.files.contains(&file_path.to_string())
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionState {
    pub copied_files: Vec<CopiedFile>,
    pub folders: Vec<Folder>,
}

impl Default for ExtensionState {
    fn default() -> Self {
        Self {
            copied_files: Vec::new(),
            folders: Vec::new(),
        }
    }
}

impl ExtensionState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_copied_file(&mut self, copied_file: CopiedFile) {
        // Remove existing file with same base path
        self.copied_files.retain(|f| f.base_path != copied_file.base_path);
        self.copied_files.push(copied_file);
    }

    pub fn clear_copied_files(&mut self) {
        self.copied_files.clear();
    }

    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
    }

    pub fn remove_folder(&mut self, folder_id: &str) -> bool {
        if let Some(index) = self.folders.iter().position(|f| f.id == folder_id) {
            self.folders.remove(index);
            true
        } else {
            false
        }
    }

    pub fn find_folder(&self, folder_id: &str) -> Option<&Folder> {
        self.folders.iter().find(|f| f.id == folder_id)
    }

    pub fn find_folder_mut(&mut self, folder_id: &str) -> Option<&mut Folder> {
        self.folders.iter_mut().find(|f| f.id == folder_id)
    }

    pub fn generate_combined_content(&self) -> String {
        self.copied_files
            .iter()
            .map(|f| format!("{}\n\n{}", f.display_path, f.content))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n")
    }

    pub fn copied_files_count(&self) -> usize {
        self.copied_files.len()
    }

    pub fn folders_count(&self) -> usize {
        self.folders.len()
    }
}

#[derive(Debug, Clone)]
pub struct FileSelection {
    pub start_line: u32,
    pub end_line: u32,
    pub content: String,
}

impl FileSelection {
    pub fn new(start_line: u32, end_line: u32, content: String) -> Self {
        Self {
            start_line,
            end_line,
            content,
        }
    }

    pub fn format_path_with_lines(&self, base_path: &str) -> String {
        if self.start_line == self.end_line {
            format!("{}:{}", base_path, self.start_line)
        } else {
            format!("{}:{}-{}", base_path, self.start_line, self.end_line)
        }
    }
}
