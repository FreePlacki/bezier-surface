use crate::{color::Color, point::Vector3, texture::Texture};

#[derive(Debug)]
pub enum Coloring {
    Solid(Color),
    Texture(Texture),
}

pub struct Material {
    /// base surface color
    pub coloring: Coloring,
    /// map altering normal vectors (if `None` the normals remain unchanged)
    pub normal_map: Option<Texture>,
    /// diffuse fraction
    pub kd: f32,
    /// specular fraction
    pub ks: f32,
    /// specular exponent
    pub m: i32,
}

impl Material {
    pub fn color_at(&self, u: f32, v: f32) -> Color {
        match &self.coloring {
            Coloring::Solid(c) => *c,
            Coloring::Texture(t) => {
                let c = t.sample(u, v);
                Color::from_slice([
                    c[0] as f32 / 255.0,
                    c[1] as f32 / 255.0,
                    c[2] as f32 / 255.0,
                ])
            }
        }
    }

    pub fn normal_at(&self, u: f32, v: f32, pu: Vector3, pv: Vector3, n: Vector3) -> Vector3 {
        match &self.normal_map {
            None => n,
            Some(t) => {
                let m = t.sample_normal(u, v);
                Vector3 {
                    x: m.x * pu.x + m.y * pv.x + m.z * n.x,
                    y: m.x * pu.y + m.y * pv.y + m.z * n.y,
                    z: m.x * pu.z + m.y * pv.z + m.z * n.z,
                }
            }
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        let solid = Coloring::Solid(Color::new(0.0, 1.0, 0.0));
        Self {
            coloring: solid,
            normal_map: None,
            kd: 0.5,
            ks: 0.5,
            m: 4,
        }
    }
}
