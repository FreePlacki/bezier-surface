#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::PolygonApp;

mod app;
mod canvas;
mod light;
mod material;
mod mesh;
mod point;
mod scene;
mod color;
mod texture;
mod surface;
mod triangle;

fn main() -> eframe::Result {
    let app = PolygonApp::new();

    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_maximized(true);
    eframe::run_native(
        "Bezier Surface",
        native_options,
        Box::new(|_| Ok(Box::new(app))),
    )
}
