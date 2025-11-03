use std::{
    f32::consts::{FRAC_PI_2, PI},
    fs::File,
    io::Read,
    str::FromStr,
};

use eframe::egui::{Color32, Painter, Stroke};

use crate::{canvas::Canvas, mesh::Mesh, point::Point3, surface::BezierSurface};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        debug_assert!(r >= 0.0 && r <= 1.0);
        debug_assert!(g >= 0.0 && g <= 1.0);
        debug_assert!(b >= 0.0 && b <= 1.0);

        Self { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }
    pub fn g(&self) -> f32 {
        self.g
    }
    pub fn b(&self) -> f32 {
        self.b
    }

    pub fn as_slice(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn from_slice(color: [f32; 3]) -> Self {
        Self::new(color[0], color[1], color[2])
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pos: Point3,
    color: Color,
}

impl Light {
    pub fn new(pos: Point3, color: Color) -> Self {
        Self { pos, color }
    }

    pub fn pos(&self) -> Point3 {
        self.pos
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

pub struct Material {
    /// base surface color
    pub color: Color,
    /// diffuse fraction
    pub kd: f32,
    /// specular fraction
    pub ks: f32,
    /// specular exponent
    pub m: i32,
}

impl Material {
    pub fn new(color: Color, kd: f32, ks: f32, m: i32) -> Self {
        debug_assert!(kd >= 0.0 && kd <= 1.0);
        debug_assert!(ks >= 0.0 && ks <= 1.0);

        Self { color, kd, ks, m }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(0.0, 1.0, 0.0),
            kd: 0.5,
            ks: 0.5,
            m: 4,
        }
    }
}

pub struct Scene {
    pub material: Material,
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
