//! # ProcessManagerApp - accessors Methods
//!
//! This module contains method implementations for `ProcessManagerApp`.
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};
use super::processmanagerapp_type::ProcessManagerApp;

impl ProcessManagerApp {
    fn get_paginated_items<'a, T>(&self, items: &'a [T]) -> &'a [T] {
        let start = self.current_page * self.items_per_page;
        let end = (start + self.items_per_page).min(items.len());
        if start >= items.len() { &[] } else { &items[start..end] }
    }
}
