use zed_extension_api as zed;

mod models;
mod clipboard;
mod folders;

use models::{ExtensionState, FileSelection};
use clipboard::ClipboardManager;
use folders::FolderManager;

struct CopyPathWithCodeExtension {
    clipboard_manager: ClipboardManager,
    folder_manager: FolderManager,
}

impl CopyPathWithCodeExtension {
    fn new() -> Self {
        Self {
            clipboard_manager: ClipboardManager::new(),
            folder_manager: FolderManager::new(),
        }
    }

    fn copy_current_file(&mut self, file_path: String, content: String, selection: Option<FileSelection>) -> Result<String, String> {
        let combined_content = self.clipboard_manager.copy_file_with_content(file_path, content, selection)?;

        // In a real Zed extension, we would use the actual clipboard API
        // For now, we return the combined content that would be copied
        Ok(format!("Copied {} files to clipboard", self.clipboard_manager.get_file_count()))
    }

    fn clear_clipboard(&mut self) -> String {
        self.clipboard_manager.clear();
        "Clipboard cleared".to_string()
    }

    fn create_folder(&mut self, name: String, initial_files: Vec<String>) -> Result<String, String> {
        self.folder_manager.create_folder(name, initial_files)
    }

    fn delete_folder(&mut self, folder_id: String) -> Result<String, String> {
        self.folder_manager.delete_folder(&folder_id)
    }

    fn rename_folder(&mut self, folder_id: String, new_name: String) -> Result<String, String> {
        self.folder_manager.rename_folder(&folder_id, new_name)
    }

    fn add_file_to_folder(&mut self, folder_id: String, file_path: String) -> Result<String, String> {
        self.folder_manager.add_file_to_folder(&folder_id, file_path)
    }

    fn remove_file_from_folder(&mut self, folder_id: String, file_path: String) -> Result<String, String> {
        self.folder_manager.remove_file_from_folder(&folder_id, &file_path)
    }

    fn copy_folder_contents(&mut self, folder_id: String) -> Result<String, String> {
        let combined_content = self.folder_manager.copy_folder_contents(&folder_id)?;

        // In a real implementation, this would be copied to the system clipboard
        Ok(format!("Copied folder contents to clipboard"))
    }

    fn list_folders(&self) -> Vec<String> {
        self.folder_manager
            .list_folders()
            .iter()
            .map(|folder| format!("{}: {} ({} files)", folder.id, folder.name, folder.file_count()))
            .collect()
    }

    fn get_status(&self) -> String {
        format!(
            "Clipboard: {} | Folders: {} | Total folder files: {}",
            self.clipboard_manager.get_status_message(),
            self.folder_manager.get_folder_count(),
            self.folder_manager.get_total_files_count()
        )
    }

    fn handle_command(&mut self, command: &str, args: Vec<String>) -> Result<String, String> {
        match command {
            "copy_path_with_content" => {
                if args.len() >= 2 {
                    let file_path = args[0].clone();
                    let content = args[1].clone();
                    let selection = if args.len() >= 4 {
                        let start_line: u32 = args[2].parse().map_err(|_| "Invalid start line")?;
                        let end_line: u32 = args[3].parse().map_err(|_| "Invalid end line")?;
                        let selected_content = if args.len() >= 5 { args[4].clone() } else { content.clone() };
                        Some(FileSelection::new(start_line, end_line, selected_content))
                    } else {
                        None
                    };
                    self.copy_current_file(file_path, content, selection)
                } else {
                    Err("Missing arguments: file_path and content required".to_string())
                }
            }
            "clear_clipboard" => {
                Ok(self.clear_clipboard())
            }
            "create_folder" => {
                if !args.is_empty() {
                    let name = args[0].clone();
                    let initial_files = if args.len() > 1 { args[1..].to_vec() } else { vec![] };
                    self.create_folder(name, initial_files)
                } else {
                    Err("Missing argument: folder name required".to_string())
                }
            }
            "delete_folder" => {
                if !args.is_empty() {
                    self.delete_folder(args[0].clone())
                } else {
                    Err("Missing argument: folder_id required".to_string())
                }
            }
            "rename_folder" => {
                if args.len() >= 2 {
                    self.rename_folder(args[0].clone(), args[1].clone())
                } else {
                    Err("Missing arguments: folder_id and new_name required".to_string())
                }
            }
            "add_file_to_folder" => {
                if args.len() >= 2 {
                    self.add_file_to_folder(args[0].clone(), args[1].clone())
                } else {
                    Err("Missing arguments: folder_id and file_path required".to_string())
                }
            }
            "remove_file_from_folder" => {
                if args.len() >= 2 {
                    self.remove_file_from_folder(args[0].clone(), args[1].clone())
                } else {
                    Err("Missing arguments: folder_id and file_path required".to_string())
                }
            }
            "copy_folder_contents" => {
                if !args.is_empty() {
                    self.copy_folder_contents(args[0].clone())
                } else {
                    Err("Missing argument: folder_id required".to_string())
                }
            }
            "list_folders" => {
                let folders = self.list_folders();
                Ok(folders.join("\n"))
            }
            "status" => {
                Ok(self.get_status())
            }
            _ => {
                Err(format!("Unknown command: {}", command))
            }
        }
    }
}

impl zed::Extension for CopyPathWithCodeExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        command: &str,
        args: Vec<String>,
    ) -> zed::Result<Option<String>> {
        match self.handle_command(command, args) {
            Ok(result) => Ok(Some(result)),
            Err(error) => {
                eprintln!("Extension error: {}", error);
                Ok(Some(format!("Error: {}", error)))
            }
        }
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(None)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(None)
    }
}

zed::register_extension!(CopyPathWithCodeExtension);
