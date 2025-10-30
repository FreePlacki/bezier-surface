#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::PolygonApp;

mod app;

fn main() -> eframe::Result {
    let app = PolygonApp::default();

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_maximized(true);
    eframe::run_native(
        "Bezier Surface",
        native_options,
        Box::new(|_| Ok(Box::new(app))),
    )
}
