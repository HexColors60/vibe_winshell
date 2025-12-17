//! # ProcessManagerApp - configure_fonts_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn configure_fonts(&self, ctx: &egui::Context, preview_only: bool) {
        let path_to_load = if preview_only {
            if let Some(selected) = &self.font_picker.selected_file {
                std::path::Path::new(&self.font_picker.directory).join(selected)
            } else {
                return;
            }
        } else {
            if self.font_path.is_empty() {
                return;
            }
            std::path::PathBuf::from(&self.font_path)
        };
        if !path_to_load.exists() {
            return;
        }
        if let Ok(data) = std::fs::read(&path_to_load) {
            let mut fonts = egui::FontDefinitions::default();
            let font_name = if preview_only { "preview_font" } else { "custom_font" }
                .to_owned();
            fonts
                .font_data
                .insert(
                    font_name.clone(),
                    egui::FontData::from_owned(data)
                        .tweak(egui::FontTweak {
                            scale: 1.0,
                            ..Default::default()
                        }),
                );
            if preview_only {
                if !self.font_path.is_empty()
                    && std::path::Path::new(&self.font_path).exists()
                {
                    if let Ok(main_data) = std::fs::read(&self.font_path) {
                        fonts
                            .font_data
                            .insert(
                                "custom_font".to_owned(),
                                egui::FontData::from_owned(main_data),
                            );
                        fonts
                            .families
                            .entry(egui::FontFamily::Proportional)
                            .or_default()
                            .insert(0, "custom_font".to_owned());
                        fonts
                            .families
                            .entry(egui::FontFamily::Monospace)
                            .or_default()
                            .push("custom_font".to_owned());
                    }
                }
                fonts
                    .families
                    .insert(egui::FontFamily::Name("Preview".into()), vec![font_name]);
            } else {
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, font_name.clone());
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .push(font_name);
            }
            ctx.set_fonts(fonts);
        }
    }
}
