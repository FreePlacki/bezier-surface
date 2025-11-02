use eframe::egui::{Color32, Painter, Stroke};

use crate::{canvas::Canvas, point::Triangle};

pub struct Mesh {
    triangles: Vec<Triangle>,
    resolution: usize,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, resolution: usize) -> Self {
        Self { triangles, resolution }
    }

    pub fn resolution(&self) -> usize {
        self.resolution
    }

    pub fn rotate_ox(&mut self, delta: f32) {
        self.triangles.iter_mut().for_each(|t| {
            t.p0 = t.p0.rotate_ox(delta);
            t.p1 = t.p1.rotate_ox(delta);
            t.p2 = t.p2.rotate_ox(delta);
        });
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        self.triangles.iter_mut().for_each(|t| {
            t.p0 = t.p0.rotate_oz(delta);
            t.p1 = t.p1.rotate_oz(delta);
            t.p2 = t.p2.rotate_oz(delta);
        });
    }

    pub fn draw(&self, canvas: &Canvas, painter: &Painter) {
        let stroke = Stroke::new(1.0, Color32::WHITE);
        for t in &self.triangles {
            let p0 = t.p0.to_screen(canvas).projection();
            let p1 = t.p1.to_screen(canvas).projection();
            let p2 = t.p2.to_screen(canvas).projection();
            painter.line(vec![p0, p1], stroke);
            painter.line(vec![p1, p2], stroke);
            painter.line(vec![p2, p0], stroke);
        }
    }
}
