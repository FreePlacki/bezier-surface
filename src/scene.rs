use std::{
    f32::consts::{FRAC_PI_2, PI},
    fs::File,
    io::Read,
    str::FromStr,
};

use eframe::egui::{Color32, Painter, Stroke};

use crate::{canvas::Canvas, color::Color, light::Light, material::Material, mesh::Mesh, point::Point3, surface::BezierSurface};


pub struct Scene {
    pub material: Material,
    pub light: Light,
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
        let mesh = surface.triangulate(30);

        let mut s = Self {
            surface,
            mesh,
            light: Light::new(Point3::new(-600.0, 700.0, 0.0), Color::new(1.0, 1.0, 1.0)),
            material: Material::default(),
            rot_ox: 0.0,
            rot_oz: 0.0,
        };

        let rot_ox = 104.0f32.to_radians();
        let rot_oz = 10.0f32.to_radians();

        s.rotate_ox(rot_ox);
        s.rotate_oz(rot_oz);
        Ok(s)
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

    pub fn material_color(&self) -> [f32; 3] {
        self.material.color.as_slice()
    }

    pub fn set_material_color(&mut self, color: [f32; 3]) {
        self.material.color = Color::from_slice(color);
    }

    pub fn light_color(&self) -> [f32; 3] {
        self.light.color.as_slice()
    }

    pub fn set_light_color(&mut self, color: [f32; 3]) {
        self.light.color = Color::from_slice(color);
    }

    pub fn set_mesh_resolution(&mut self, res: usize) {
        self.mesh = self.surface.triangulate(res);
    }

    pub fn draw_fillings(&self, canvas: &mut Canvas) {
        self.mesh.draw_fillings(canvas, &self.light, &self.material);
    }

    pub fn draw_outlines(&self, canvas: &Canvas, painter: &Painter) {
        self.mesh.draw_outlines(canvas, &painter);
    }

    pub fn draw_light_pos(&self, canvas: &Canvas, painter: &Painter) {
        painter.line(
            vec![
                self.light.pos.to_screen(canvas).projection(),
                Point3::origin().to_screen(canvas).projection(),
            ],
            Stroke::new(3.0, Color32::YELLOW),
        );
    }

    pub fn draw_points(&self, canvas: &Canvas, painter: &Painter) {
        self.surface.draw_points(canvas, &painter);
    }
}
