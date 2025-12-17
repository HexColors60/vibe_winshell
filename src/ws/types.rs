//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashSet};

#[derive(PartialEq, Clone, Copy)]
pub enum ViewMode {
    Processes,
    Files,
    Network,
    FilesNetwork,
    Windows,
    Taskbar,
    Live,
    New,
    Logs,
    Settings,
    Filepane,
}
#[derive(Clone, Debug)]
pub struct WindowInfo {
    pub pid: u32,
    pub process_name: String,
    pub window_title: String,
    pub window_id: u64,
    pub is_foreground: bool,
}
pub struct AppConfig {
    pub programs: Vec<CustomProgram>,
    pub font_path: String,
    pub use_noto: bool,
    pub theme: Theme,
    pub live_grid_size: usize,
    pub live_detail_percent: f32,
    pub attempt_start_as_admin: bool,
}
#[derive(Clone, Debug)]
pub struct FileHandle {
    pub pid: u32,
    pub process_name: String,
    pub path: String,
    pub size: u64,
    pub access_type: String,
}
#[derive(Clone, Debug)]
pub struct NetworkConnection {
    pub pid: u32,
    pub process_name: String,
    pub protocol: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub connection_id: String,
}
#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: std::time::SystemTime,
    pub extension: Option<String>,
}
impl FileInfo {
    pub fn new(path: std::path::PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = std::fs::metadata(&path)?;
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        let path_str = path.to_string_lossy().to_string();
        let is_directory = metadata.is_dir();
        let size = if is_directory { 0 } else { metadata.len() };
        let modified = metadata.modified()?;
        let extension = if !is_directory {
            path.extension().and_then(|ext| ext.to_str()).map(|s| s.to_lowercase())
        } else {
            None
        };
        Ok(FileInfo {
            name,
            path: path_str,
            is_directory,
            size,
            modified,
            extension,
        })
    }
}
#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub memory: u64,
    pub cpu_usage: f32,
    pub parent_pid: Option<u32>,
    pub status: String,
    pub run_time: u64,
    pub is_foreground: bool,
    pub exe_path: Option<String>,
}
#[derive(PartialEq, Clone, Copy)]
pub enum SortColumn {
    Pid,
    Name,
    Memory,
    Cpu,
    ParentPid,
    Status,
    Runtime,
    FilePath,
    FileSize,
    FileAccess,
    Protocol,
    LocalAddr,
    RemoteAddr,
    NetState,
    WindowTitle,
}
#[derive(Clone, Debug)]
pub enum FilepaneCommand {
    CopyFile { source: String, destination: String },
    MoveFile { source: String, destination: String },
    DeleteFile { path: String },
    CreateDirectory { path: String },
    RenameFile { old_path: String, new_path: String },
    ChangeDirectory { panel: usize, new_path: String },
    CalculateChecksum { path: String, algorithm: ChecksumAlgorithm },
}
#[derive(Clone, Debug, PartialEq)]
pub enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    CRC32,
}
impl ChecksumAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            ChecksumAlgorithm::MD5 => "MD5",
            ChecksumAlgorithm::SHA1 => "SHA1",
            ChecksumAlgorithm::SHA256 => "SHA256",
            ChecksumAlgorithm::CRC32 => "CRC32",
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
pub enum Theme {
    Dark,
    Light,
}
#[derive(Clone, Debug)]
pub struct CustomProgram {
    pub name: String,
    pub path: String,
    pub args: String,
    pub admin: bool,
}
pub struct FontPickerState {
    pub is_open: bool,
    pub directory: String,
    pub files: Vec<String>,
    pub filter: String,
    pub selected_file: Option<String>,
    pub preview_text: String,
    pub error_msg: Option<String>,
}
#[derive(Clone, Debug)]
pub struct FilepaneTab {
    pub name: String,
    pub left_path: String,
    pub right_path: String,
    pub selected_left: Vec<String>,
    pub selected_right: Vec<String>,
    pub filter: String,
    pub left_checkboxes: HashSet<String>,
    pub right_checkboxes: HashSet<String>,
    pub show_checkboxes: bool,
    pub command_history: Vec<FilepaneCommand>,
    pub undo_stack: Vec<FilepaneCommand>,
    pub redo_stack: Vec<FilepaneCommand>,
    pub copy_speed_limit_mb_per_sec: f64,
    pub checksum_algorithm: ChecksumAlgorithm,
}
impl FilepaneTab {
    pub fn new(name: String, left_path: String, right_path: String) -> Self {
        Self {
            name,
            left_path,
            right_path,
            selected_left: Vec::new(),
            selected_right: Vec::new(),
            filter: String::new(),
            left_checkboxes: HashSet::new(),
            right_checkboxes: HashSet::new(),
            show_checkboxes: false,
            command_history: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            copy_speed_limit_mb_per_sec: 10.0,
            checksum_algorithm: ChecksumAlgorithm::SHA256,
        }
    }
}
#[derive(Debug)]
pub enum ContextAction {
    NavigateToDirectory { path: String, panel_index: usize },
    OpenFile { path: String },
    CopyFile { source: String, destination: String, speed_limit: f64 },
    MoveFile { source: String, destination: String },
    DeleteFile { path: String },
    Cut,
    ShowProperties { file_info: FileInfo },
    LogMessage(String),
    CloseMenu,
}

#[derive(Clone, Debug)]
pub struct FileOperation {
    pub operation_type: FileOperationType,
    pub source_path: String,
    pub destination_path: Option<String>,
    pub original_path: Option<String>, // For undo/restore
    pub timestamp: std::time::SystemTime,
}

#[derive(Clone, Debug)]
pub enum FileOperationType {
    Copy,
    Move,
    Delete,
    Create,
    Rename,
}

#[derive(Clone, Debug)]
pub struct TrashItem {
    pub original_path: String,
    pub trash_path: String,
    pub deletion_time: std::time::SystemTime,
    pub file_type: FileOperationType,
}
