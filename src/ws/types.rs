//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashSet};

#[derive(PartialEq, Clone, Copy)]
enum ViewMode {
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
    pid: u32,
    process_name: String,
    window_title: String,
    window_id: u64,
    is_foreground: bool,
}
pub struct AppConfig {
    programs: Vec<CustomProgram>,
    font_path: String,
    use_noto: bool,
    theme: Theme,
    live_grid_size: usize,
    live_detail_percent: f32,
    attempt_start_as_admin: bool,
}
#[derive(Clone, Debug)]
pub struct FileHandle {
    pid: u32,
    process_name: String,
    path: String,
    size: u64,
    access_type: String,
}
#[derive(Clone, Debug)]
pub struct NetworkConnection {
    pid: u32,
    process_name: String,
    protocol: String,
    local_addr: String,
    remote_addr: String,
    state: String,
    connection_id: String,
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
    pid: u32,
    name: String,
    memory: u64,
    cpu_usage: f32,
    parent_pid: Option<u32>,
    status: String,
    run_time: u64,
    is_foreground: bool,
    exe_path: Option<String>,
}
#[derive(PartialEq, Clone, Copy)]
enum SortColumn {
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
enum FilepaneCommand {
    CopyFile { source: String, destination: String },
    MoveFile { source: String, destination: String },
    DeleteFile { path: String },
    CreateDirectory { path: String },
    RenameFile { old_path: String, new_path: String },
    ChangeDirectory { panel: usize, new_path: String },
    CalculateChecksum { path: String, algorithm: ChecksumAlgorithm },
}
#[derive(Clone, Debug, PartialEq)]
enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    CRC32,
}
impl ChecksumAlgorithm {
    fn name(&self) -> &'static str {
        match self {
            ChecksumAlgorithm::MD5 => "MD5",
            ChecksumAlgorithm::SHA1 => "SHA1",
            ChecksumAlgorithm::SHA256 => "SHA256",
            ChecksumAlgorithm::CRC32 => "CRC32",
        }
    }
}
#[derive(PartialEq, Clone, Copy)]
enum Theme {
    Dark,
    Light,
}
#[derive(Clone, Debug)]
pub struct CustomProgram {
    name: String,
    path: String,
    args: String,
    admin: bool,
}
pub struct FontPickerState {
    is_open: bool,
    directory: String,
    files: Vec<String>,
    filter: String,
    selected_file: Option<String>,
    preview_text: String,
    error_msg: Option<String>,
}
#[derive(Clone, Debug)]
pub struct FilepaneTab {
    name: String,
    left_path: String,
    right_path: String,
    selected_left: Vec<String>,
    selected_right: Vec<String>,
    filter: String,
    left_checkboxes: HashSet<String>,
    right_checkboxes: HashSet<String>,
    show_checkboxes: bool,
    command_history: Vec<FilepaneCommand>,
    undo_stack: Vec<FilepaneCommand>,
    redo_stack: Vec<FilepaneCommand>,
    copy_speed_limit_mb_per_sec: f64,
    checksum_algorithm: ChecksumAlgorithm,
}
impl FilepaneTab {
    fn new(name: String, left_path: String, right_path: String) -> Self {
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
enum ContextAction {
    NavigateToDirectory { path: String, panel_index: usize },
    OpenFile { path: String },
    CopyFile { source: String, destination: String, speed_limit: f64 },
    Cut,
    ShowProperties { file_info: FileInfo },
    LogMessage(String),
    CloseMenu,
}
