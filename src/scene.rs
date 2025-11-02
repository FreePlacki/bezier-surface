use std::{f32::consts::FRAC_PI_2, fs::File, io::Read, str::FromStr};

use eframe::egui::Painter;

use crate::{app::Visible, canvas::Canvas, mesh::Mesh, surface::BezierSurface};

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
        let mesh = surface.triangulate(20);

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
        let new_rot = (self.rot_ox + delta).clamp(-FRAC_PI_2, FRAC_PI_2);
        if (new_rot - self.rot_ox).abs() < 1e-3 {
            return;
        }
        self.rot_ox = new_rot;
        self.surface.rotate_ox(delta);
        self.mesh.rotate_ox(delta);
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        let new_rot = (self.rot_oz + delta).clamp(-FRAC_PI_2, FRAC_PI_2);
        if (new_rot - self.rot_oz).abs() < 1e-3 {
            return;
        }
        self.rot_oz = new_rot;
        self.surface.rotate_oz(delta);
        self.mesh.rotate_oz(delta);
    }

    pub fn mesh_resolution(&self) -> usize {
        self.mesh.resolution()
    }

    pub fn set_mesh_resolution(&mut self, res: usize) {
        self.mesh = self.surface.triangulate(res);
    }

    pub fn draw(&self, canvas: &mut Canvas, painter: &Painter, visible: &Visible) {
        if visible.mesh {
            self.mesh.draw(canvas, &painter);
        }
        if visible.polygon {
            self.surface.draw_points(canvas, &painter);
        }
    }
}
