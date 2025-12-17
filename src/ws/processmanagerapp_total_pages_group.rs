//! # ProcessManagerApp - total_pages_group Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    pub(crate) fn total_pages<T>(&self, items: &[T]) -> usize {
        (items.len() + self.items_per_page - 1) / self.items_per_page
    }
}
