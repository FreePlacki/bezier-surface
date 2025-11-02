use std::{
    f32::consts::{FRAC_PI_2, PI},
    fs::File,
    io::Read,
    str::FromStr,
};

use eframe::egui::{Color32, Painter, Stroke, pos2};

use crate::{canvas::Canvas, mesh::Mesh, point::Point3, surface::BezierSurface};

pub struct Light {
    pos: Point3,
    color: (f32, f32, f32),
}

impl Light {
    pub fn new(pos: Point3, color: (f32, f32, f32)) -> Self {
        debug_assert!(color.0 >= 0.0 && color.0 <= 1.0);
        debug_assert!(color.1 >= 0.0 && color.1 <= 1.0);
        debug_assert!(color.2 >= 0.0 && color.2 <= 1.0);
        Self { pos, color }
    }

    pub fn pos(&self) -> Point3 {
        self.pos
    }

    pub fn color(&self) -> (f32, f32, f32) {
        self.color
    }
}

pub struct Scene {
    surface: BezierSurface,
    mesh: Mesh,
    light: Light,
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
            light: Light::new(Point3::new(-600.0, 700.0, 0.0), (1.0, 1.0, 1.0)),
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
        let mut new_rot = self.rot_ox + delta;
        if new_rot < -FRAC_PI_2 {
            new_rot += PI;
        } else if new_rot > FRAC_PI_2 {
            new_rot -= PI;
        }

        self.rot_ox = new_rot;
        self.surface.rotate_ox(delta);
        self.mesh.rotate_ox(delta);
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        let mut new_rot = self.rot_oz + delta;
        if new_rot < -FRAC_PI_2 {
            new_rot += PI;
        } else if new_rot > FRAC_PI_2 {
            new_rot -= PI;
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

    pub fn draw_fillings(&self, canvas: &mut Canvas) {
        self.mesh.draw_fillings(canvas, &self.light);
    }

    pub fn draw_outlines(&self, canvas: &Canvas, painter: &Painter) {
        self.mesh.draw_outlines(canvas, &painter);
    }

    pub fn draw_points(&self, canvas: &Canvas, painter: &Painter) {
        self.surface.draw_points(canvas, &painter);
        painter.line(
            vec![
                self.light.pos().to_screen(canvas).projection(),
                Point3::origin().to_screen(canvas).projection(),
            ],
            Stroke::new(3.0, Color32::YELLOW),
        );
    }
}
