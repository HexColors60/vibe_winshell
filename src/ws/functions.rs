//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use eframe::egui;
use sysinfo::System;
use std::time::{Duration, Instant};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
pub fn main() -> eframe::Result {
    let icon_data = eframe::icon_data::from_png_bytes(include_bytes!("../winshell.png"))
        .unwrap_or_default();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(icon_data),
        ..Default::default()
    };
    let config = ProcessManagerApp::load_config();
    if config.attempt_start_as_admin {
        if !ProcessManagerApp::is_user_admin() {
            if let Ok(_) = ProcessManagerApp::restart_as_admin() {
                return Ok(());
            }
        }
    }
    eframe::run_native(
        "WinShell - Process, File & Network Monitor",
        options,
        Box::new(|cc| Ok(Box::new(ProcessManagerApp::new(cc)))),
    )
}
