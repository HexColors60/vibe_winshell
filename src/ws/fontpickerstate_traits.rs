//! # FontPickerState - Trait Implementations
//!
//! This module contains trait implementations for `FontPickerState`.
//!
//! ## Implemented Traits
//!
//! - `Default`
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::FontPickerState;

impl Default for FontPickerState {
    fn default() -> Self {
        Self {
            is_open: false,
            directory: "C:\\Windows\\Fonts".to_string(),
            files: Vec::new(),
            filter: String::new(),
            selected_file: None,
            preview_text: "The quick brown fox jumps over the lazy dog.\n1234567890\n你好，世界！This is a preview."
                .to_string(),
            error_msg: None,
        }
    }
}

