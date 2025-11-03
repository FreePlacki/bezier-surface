use eframe::egui::Painter;

use crate::{canvas::Canvas, scene::{Light, Material}, triangle::Triangle};

pub struct Mesh {
    triangles: Vec<Triangle>,
    resolution: usize,
}

impl Mesh {
    pub fn new(triangles: Vec<Triangle>, resolution: usize) -> Self {
        Self {
            triangles,
            resolution,
        }
    }

    pub fn resolution(&self) -> usize {
        self.resolution
    }

    pub fn rotate_ox(&mut self, delta: f32) {
        self.triangles.iter_mut().for_each(|t| {
            t.p0.rotate_ox(delta);
            t.p1.rotate_ox(delta);
            t.p2.rotate_ox(delta);
        });
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        self.triangles.iter_mut().for_each(|t| {
            t.p0.rotate_oz(delta);
            t.p1.rotate_oz(delta);
            t.p2.rotate_oz(delta);
        });
    }

    pub fn draw_outlines(&self, canvas: &Canvas, painter: &Painter) {
        self.triangles
            .iter()
            .for_each(|t| t.draw_outline(canvas, painter));
    }

    pub fn draw_fillings(&self, canvas: &mut Canvas, light: &Light, material: &Material) {
        self.triangles.iter().for_each(|t| t.draw_filling(canvas, light, material));
    }
}
