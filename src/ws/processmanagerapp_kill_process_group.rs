//! # ProcessManagerApp - kill_process_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub fn kill_process(&mut self, pid: u32) -> bool {
        if let Some(process) = self.system.process(sysinfo::Pid::from_u32(pid)) {
            process.kill()
        } else {
            false
        }
    }
}
