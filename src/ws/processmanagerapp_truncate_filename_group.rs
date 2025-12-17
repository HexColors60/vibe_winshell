//! # ProcessManagerApp - truncate_filename_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn truncate_filename(&self, name: &str, max_width: f32, ui: &egui::Ui) -> String {
        if ui
            .painter()
            .layout_no_wrap(
                name.to_string(),
                ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone(),
                egui::Color32::WHITE,
            )
            .size()
            .x <= max_width
        {
            return name.to_string();
        }
        let ellipsis = "...";
        let ellipsis_width = ui
            .painter()
            .layout_no_wrap(
                ellipsis.to_string(),
                ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone(),
                egui::Color32::WHITE,
            )
            .size()
            .x;
        let mut chars = name.chars().collect::<Vec<char>>();
        let mut start_len = chars.len() / 2;
        for _ in 0..10 {
            if start_len <= 3 {
                return name.chars().take(3).collect::<String>() + ellipsis;
            }
            let truncated_name: String = chars.iter().take(start_len).collect::<String>()
                + ellipsis;
            let truncated_width = ui
                .painter()
                .layout_no_wrap(
                    truncated_name.clone(),
                    ui.style().text_styles.get(&egui::TextStyle::Body).unwrap().clone(),
                    egui::Color32::WHITE,
                )
                .size()
                .x;
            if truncated_width <= max_width {
                return truncated_name;
            }
            start_len = (start_len * 3) / 4;
        }
        name.chars().take(5).collect::<String>() + ellipsis
    }
}
