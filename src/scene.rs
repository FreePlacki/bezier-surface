use std::{fs::File, io::Read, str::FromStr};

use eframe::egui;

use crate::{
    canvas::Canvas,
    surface::{BezierSurface, Mesh},
};

pub struct Scene {
    surface: BezierSurface,
    mesh: Mesh,
    rot_ox: f32,
    rot_oz: f32,
}

impl Scene {
    pub fn from_file(name: &str) -> Result<Self, String> {
        let mut f = File::open(name).map_err(|e| e.to_string())?;
        let mut buf = String::new();
        f.read_to_string(&mut buf).map_err(|e| e.to_string())?;
        let surface = BezierSurface::from_str(&buf)?;
        let mesh = surface.triangulate();

        Ok(Self {
            surface,
            mesh,
            rot_ox: 0.0,
            rot_oz: 0.0,
        })
    }

    pub fn rot_ox(&self) -> f32 {
        self.rot_ox
    }

    pub fn rot_oz(&self) -> f32 {
        self.rot_oz
    }

    pub fn rotate_ox(&mut self, delta: f32) {
        self.rot_ox += delta;
        self.surface.rotate_ox(delta);
        self.mesh.rotate_ox(delta);
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        self.rot_oz += delta;
        self.surface.rotate_oz(delta);
        self.mesh.rotate_oz(delta);
    }

    pub fn draw(&self, ui: &mut egui::Ui, canvas: &mut Canvas) {
        let (response, painter) =
            ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

        self.mesh.draw(canvas, &painter);
        self.surface.draw_points(canvas, &painter);
    }
}
