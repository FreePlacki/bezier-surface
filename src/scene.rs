use std::{fs::File, io::Read, str::FromStr};

use eframe::egui;

use crate::{canvas::Canvas, surface::BezierSurface};

pub struct Scene {
    surface: BezierSurface,
}

impl Scene {
    pub fn from_file(name: &str) -> Result<Self, String> {
        let mut f = File::open(name).map_err(|e| e.to_string())?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).map_err(|e| e.to_string())?;

        Ok(Self {
            surface: BezierSurface::from_str(&buf)?,
        })
    }

    pub fn draw(&self, ui: &mut egui::Ui, canvas: &mut Canvas) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        self.surface.draw_points(canvas, &painter);
    }
}
