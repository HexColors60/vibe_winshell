//! # ProcessManagerApp - simple_truncate_filename_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn simple_truncate_filename(&self, name: &str, max_width: f32) -> String {
        let max_chars = (max_width / 8.0) as usize;
        if name.len() <= max_chars {
            return name.to_string();
        }
        if max_chars <= 3 {
            return "...".to_string();
        }
        format!("{}...", & name[..max_chars.saturating_sub(3)])
    }
}
