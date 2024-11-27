#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

mod gui;
mod mock;
mod plib;

use crate::gui::app;
use eframe::egui;
use gui::app::ProtonCtlApp;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
                        // We've failed to create the database ( or do nothing ). This leaves the rest of the program in a bad state.
                        // This is not a recoverable error so we're going to let the program panic
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ProtonCtl",
        options,
        Box::new(|cc| Ok(Box::new(ProtonCtlApp::new(cc)))),
    )
}
