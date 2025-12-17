//! # ProcessManagerApp - bring_window_to_foreground_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn bring_window_to_foreground(&mut self, window_id: u64) {
        for window in &mut self.windows {
            window.is_foreground = window.window_id == window_id;
        }
        self.foreground_window_id = Some(window_id);
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                SetForegroundWindow, ShowWindow, SW_RESTORE,
            };
            unsafe {
                let hwnd = HWND(window_id as isize as *mut std::ffi::c_void);
                ShowWindow(hwnd, SW_RESTORE);
                SetForegroundWindow(hwnd);
            }
        }
        #[cfg(target_os = "linux")]
        {
            println!("Bringing window {} to foreground", window_id);
        }
    }
}
