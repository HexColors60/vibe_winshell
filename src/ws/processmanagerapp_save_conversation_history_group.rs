//! # ProcessManagerApp - save_conversation_history_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn save_conversation_history(
        &self,
        conversation_text: &str,
    ) -> Result<String, String> {
        use std::fs;
        let now = chrono::Local::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("history_{}.txt", timestamp);
        let winshell_path = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| std::path::PathBuf::from("."));
        let file_path = winshell_path.join(&filename);
        let mut content = format!(
            "WinShell Conversation History\n\
            =============================\n\
            Timestamp: {}\n\
            Session Date: {}\n\n\
            Conversation:\n\
            ------------\n\
            {}\n\n\
            === End of History ===\n",
            timestamp, now.format("%Y-%m-%d %H:%M:%S"), conversation_text
        );
        content
            .push_str(
                &format!(
                    "\nSystem Info:\n\
            - WinShell Version: 0.1.0\n\
            - Platform: {}\n\
            - Total Log Entries: {}\n",
                    std::env::consts::OS, self.logs.len()
                ),
            );
        if !self.filepane_tabs.is_empty() {
            content
                .push_str(
                    &format!(
                        "\nFilepane Info:\n\
                - Active Tab: {}\n\
                - Total Tabs: {}\n\
                - Columns Swapped: {}\n",
                        self.filepane_active_tab + 1, self.filepane_tabs.len(), if self
                        .filepane_swap_columns { "Yes" } else { "No" }
                    ),
                );
            if let Some(active_tab) = self.filepane_tabs.get(self.filepane_active_tab) {
                content
                    .push_str(
                        &format!(
                            "  Active Tab Paths:\n\
                    - Left: {}\n\
                    - Right: {}\n",
                            active_tab.left_path, active_tab.right_path
                        ),
                    );
            }
        }
        match fs::write(&file_path, content) {
            Ok(_) => {
                Ok(format!("Conversation history saved to: {}", file_path.display()))
            }
            Err(e) => Err(format!("Failed to save conversation history: {}", e)),
        }
    }
}
