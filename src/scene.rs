use std::{
    f32::consts::{FRAC_PI_2, PI},
    fs::File,
    io::Read,
    path::PathBuf,
    str::FromStr,
};

use eframe::egui::{Color32, Painter, Stroke};

use crate::{
    canvas::Canvas,
    color::Color,
    light::Light,
    material::{Coloring, Material},
    mesh::Mesh,
    point::Point3,
    surface::BezierSurface,
    texture::Texture,
};

pub struct Scene {
    pub material: Material,
    pub light: Light,
    pub is_animating_surface: bool,
    surface: BezierSurface,
    mesh: Mesh,
    resolution: usize,
    rot_ox: f32,
    rot_oz: f32,
}

impl Scene {
    pub fn from_file(name: Option<String>) -> Result<Self, String> {
        let points_str = if let Some(name) = name {
            let mut f = File::open(name).map_err(|e| e.to_string())?;
            let mut buf = String::new();
            f.read_to_string(&mut buf).map_err(|e| e.to_string())?;
            buf
        } else {
            eprintln!("No points file provided, using default.");
            include_str!("../assets/points.txt").to_string()
        };
        let surface = BezierSurface::from_str(&points_str)?;
        let resolution = 30;
        let mesh = surface.triangulate(resolution);

        let mut s = Self {
            surface,
            mesh,
            light: Light::new(Point3::new(-600.0, 700.0, 300.0), Color::new(1.0, 1.0, 1.0)),
            material: Material::default(),
            is_animating_surface: true,
            resolution,
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
        match self.material.coloring {
            Coloring::Solid(c) => c.as_slice(),
            Coloring::Texture(_) => [0.0, 0.0, 0.0],
        }
    }

    pub fn set_material_color(&mut self, color: [f32; 3]) {
        self.material.coloring = Coloring::Solid(Color::from_slice(color));
    }

    pub fn light_color(&self) -> [f32; 3] {
        self.light.color.as_slice()
    }

    pub fn set_light_color(&mut self, color: [f32; 3]) {
        self.light.color = Color::from_slice(color);
    }

    pub fn set_texture(&mut self, path: PathBuf) {
        if let Ok(img) = image::open(path) {
            let img = img.to_rgba8();
            let texture = Texture::from_img(img);
            self.material.coloring = Coloring::Texture(texture);
        }
    }

    pub fn set_normal_map(&mut self, path: PathBuf) {
        if let Ok(img) = image::open(path) {
            let img = img.to_rgba8();
            let texture = Texture::from_img(img);
            self.material.normal_map = Some(texture);
        }
    }

    pub fn set_mesh_resolution(&mut self, res: usize) {
        self.resolution = res;
        self.mesh = self.surface.triangulate(res);
    }

    pub fn draw_fillings(&self, canvas: &mut Canvas, draw_normals: bool) {
        self.mesh
            .draw_fillings(canvas, &self.light, &self.material, draw_normals);
    }

    pub fn draw_outlines(&self, painter: &Painter) {
        self.mesh.draw_outlines(painter);
    }

    pub fn draw_light_pos(&self, painter: &Painter) {
        let stroke = Stroke::new(3.0, Color32::YELLOW);
        let pos = self
            .light
            .pos
            .to_viewport_center(painter.ctx())
            .projection();

        painter.circle_filled(pos, 6.0, stroke.color);
        painter.line(
            vec![
                pos,
                Point3::origin()
                    .to_viewport_center(painter.ctx())
                    .projection(),
            ],
            stroke,
        );
    }

    pub fn draw_points(&self, painter: &Painter) {
        self.surface.draw_points(painter);
    }

    pub fn advance_surface_animation(&mut self, dt: f32) {
        self.surface.advance_animation(dt);
        self.mesh = self.surface.triangulate(self.resolution);
    }
}
