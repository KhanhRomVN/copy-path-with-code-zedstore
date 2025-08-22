use crate::models::{Folder, CopiedFile};
use std::fs;

pub struct FolderManager {
    pub folders: Vec<Folder>,
}

impl FolderManager {
    pub fn new() -> Self {
        Self {
            folders: Vec::new(),
        }
    }

    pub fn create_folder(&mut self, name: String, initial_files: Vec<String>) -> Result<String, String> {
        if name.trim().is_empty() {
            return Err("Folder name cannot be empty".to_string());
        }

        if self.folders.iter().any(|f| f.name == name) {
            return Err("Folder with this name already exists".to_string());
        }

        let folder_id = self.generate_folder_id();
        let mut folder = Folder::new(folder_id, name.clone());

        for file_path in initial_files {
            folder.add_file(file_path);
        }

        self.folders.push(folder);
        Ok(format!("Folder '{}' created successfully", name))
    }

    pub fn delete_folder(&mut self, folder_id: &str) -> Result<String, String> {
        if let Some(index) = self.folders.iter().position(|f| f.id == folder_id) {
            let folder_name = self.folders[index].name.clone();
            self.folders.remove(index);
            Ok(format!("Folder '{}' deleted successfully", folder_name))
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn rename_folder(&mut self, folder_id: &str, new_name: String) -> Result<String, String> {
        if new_name.trim().is_empty() {
            return Err("Folder name cannot be empty".to_string());
        }

        if self.folders.iter().any(|f| f.name == new_name && f.id != folder_id) {
            return Err("Folder with this name already exists".to_string());
        }

        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            let old_name = folder.name.clone();
            folder.name = new_name.clone();
            Ok(format!("Folder renamed from '{}' to '{}'", old_name, new_name))
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn add_file_to_folder(&mut self, folder_id: &str, file_path: String) -> Result<String, String> {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            if folder.add_file(file_path.clone()) {
                Ok(format!("File '{}' added to folder '{}'", file_path, folder.name))
            } else {
                Err("File already exists in folder".to_string())
            }
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn remove_file_from_folder(&mut self, folder_id: &str, file_path: &str) -> Result<String, String> {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            if folder.remove_file(file_path) {
                Ok(format!("File '{}' removed from folder '{}'", file_path, folder.name))
            } else {
                Err("File not found in folder".to_string())
            }
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn get_folder(&self, folder_id: &str) -> Option<&Folder> {
        self.folders.iter().find(|f| f.id == folder_id)
    }

    pub fn get_folder_mut(&mut self, folder_id: &str) -> Option<&mut Folder> {
        self.folders.iter_mut().find(|f| f.id == folder_id)
    }

    pub fn list_folders(&self) -> &Vec<Folder> {
        &self.folders
    }

    pub fn get_folder_count(&self) -> usize {
        self.folders.len()
    }

    pub fn copy_folder_contents(&self, folder_id: &str) -> Result<String, String> {
        if let Some(folder) = self.get_folder(folder_id) {
            let mut copied_files = Vec::new();

            for file_path in &folder.files {
                match fs::read_to_string(file_path) {
                    Ok(content) => {
                        let copied_file = CopiedFile::new(
                            file_path.clone(),
                            file_path.clone(),
                            content,
                        );
                        copied_files.push(copied_file);
                    }
                    Err(_) => {
                        // Skip files that can't be read
                        continue;
                    }
                }
            }

            if copied_files.is_empty() {
                return Err("No readable files found in folder".to_string());
            }

            let combined = copied_files
                .iter()
                .map(|f| format!("{}\n\n{}", f.display_path, f.content))
                .collect::<Vec<_>>()
                .join("\n\n---\n\n");

            Ok(combined)
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn find_folders_containing_file(&self, file_path: &str) -> Vec<&Folder> {
        self.folders
            .iter()
            .filter(|folder| folder.has_file(file_path))
            .collect()
    }

    pub fn get_folder_by_name(&self, name: &str) -> Option<&Folder> {
        self.folders.iter().find(|f| f.name == name)
    }

    pub fn set_folder_color(&mut self, folder_id: &str, color: Option<String>) -> Result<String, String> {
        if let Some(folder) = self.folders.iter_mut().find(|f| f.id == folder_id) {
            folder.color = color.clone();
            let message = if let Some(color_value) = color {
                format!("Color '{}' set for folder '{}'", color_value, folder.name)
            } else {
                format!("Color removed from folder '{}'", folder.name)
            };
            Ok(message)
        } else {
            Err("Folder not found".to_string())
        }
    }

    pub fn get_total_files_count(&self) -> usize {
        self.folders.iter().map(|f| f.file_count()).sum()
    }

    pub fn validate_folder_name(&self, name: &str, exclude_id: Option<&str>) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Folder name cannot be empty".to_string());
        }

        if name.len() > 100 {
            return Err("Folder name is too long (max 100 characters)".to_string());
        }

        let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err("Folder name contains invalid characters".to_string());
        }

        if let Some(existing) = self.folders.iter().find(|f| f.name == name) {
            if let Some(exclude_id) = exclude_id {
                if existing.id != exclude_id {
                    return Err("Folder with this name already exists".to_string());
                }
            } else {
                return Err("Folder with this name already exists".to_string());
            }
        }

        Ok(())
    }

    fn generate_folder_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        format!("folder_{}", timestamp)
    }
}

impl Default for FolderManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_folder() {
        let mut manager = FolderManager::new();
        let result = manager.create_folder("Test Folder".to_string(), vec!["test.rs".to_string()]);

        assert!(result.is_ok());
        assert_eq!(manager.get_folder_count(), 1);

        let folder = &manager.folders[0];
        assert_eq!(folder.name, "Test Folder");
        assert_eq!(folder.files.len(), 1);
        assert_eq!(folder.files[0], "test.rs");
    }

    #[test]
    fn test_create_duplicate_folder() {
        let mut manager = FolderManager::new();
        manager.create_folder("Test Folder".to_string(), vec![]).unwrap();

        let result = manager.create_folder("Test Folder".to_string(), vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already exists"));
    }

    #[test]
    fn test_delete_folder() {
        let mut manager = FolderManager::new();
        manager.create_folder("Test Folder".to_string(), vec![]).unwrap();

        let folder_id = manager.folders[0].id.clone();
        let result = manager.delete_folder(&folder_id);

        assert!(result.is_ok());
        assert_eq!(manager.get_folder_count(), 0);
    }

    #[test]
    fn test_rename_folder() {
        let mut manager = FolderManager::new();
        manager.create_folder("Old Name".to_string(), vec![]).unwrap();

        let folder_id = manager.folders[0].id.clone();
        let result = manager.rename_folder(&folder_id, "New Name".to_string());

        assert!(result.is_ok());
        assert_eq!(manager.folders[0].name, "New Name");
    }

    #[test]
    fn test_add_file_to_folder() {
        let mut manager = FolderManager::new();
        manager.create_folder("Test Folder".to_string(), vec![]).unwrap();

        let folder_id = manager.folders[0].id.clone();
        let result = manager.add_file_to_folder(&folder_id, "new_file.rs".to_string());

        assert!(result.is_ok());
        assert_eq!(manager.folders[0].files.len(), 1);
        assert_eq!(manager.folders[0].files[0], "new_file.rs");
    }

    #[test]
    fn test_remove_file_from_folder() {
        let mut manager = FolderManager::new();
        manager.create_folder("Test Folder".to_string(), vec!["test.rs".to_string()]).unwrap();

        let folder_id = manager.folders[0].id.clone();
        let result = manager.remove_file_from_folder(&folder_id, "test.rs");

        assert!(result.is_ok());
        assert_eq!(manager.folders[0].files.len(), 0);
    }

    #[test]
    fn test_validate_folder_name() {
        let manager = FolderManager::new();

        // Valid name
        assert!(manager.validate_folder_name("Valid Name", None).is_ok());

        // Empty name
        assert!(manager.validate_folder_name("", None).is_err());
        assert!(manager.validate_folder_name("   ", None).is_err());

        // Invalid characters
        assert!(manager.validate_folder_name("Invalid/Name", None).is_err());
        assert!(manager.validate_folder_name("Invalid\\Name", None).is_err());
        assert!(manager.validate_folder_name("Invalid:Name", None).is_err());
    }

    #[test]
    fn test_find_folders_containing_file() {
        let mut manager = FolderManager::new();
        manager.create_folder("Folder 1".to_string(), vec!["test.rs".to_string()]).unwrap();
        manager.create_folder("Folder 2".to_string(), vec!["test.rs".to_string(), "other.rs".to_string()]).unwrap();
        manager.create_folder("Folder 3".to_string(), vec!["different.rs".to_string()]).unwrap();

        let folders_with_test_rs = manager.find_folders_containing_file("test.rs");
        assert_eq!(folders_with_test_rs.len(), 2);

        let folders_with_different_rs = manager.find_folders_containing_file("different.rs");
        assert_eq!(folders_with_different_rs.len(), 1);

        let folders_with_nonexistent = manager.find_folders_containing_file("nonexistent.rs");
        assert_eq!(folders_with_nonexistent.len(), 0);
    }
}
