// src/main.rs
// Entry point after SplitRS split.
// It forwards to the generated module entry.

#![allow(warnings)]

pub use eframe::egui;

pub mod ws;

fn main() -> eframe::Result<()> {
    ws::functions::main()
}
