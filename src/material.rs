use crate::{color::Color, texture::Texture};

#[derive(Debug)]
pub enum Coloring {
    Solid(Color),
    Texture(Texture),
}

pub struct Material {
    /// base surface color
    pub coloring: Coloring,
    /// diffuse fraction
    pub kd: f32,
    /// specular fraction
    pub ks: f32,
    /// specular exponent
    pub m: i32,
}

impl Material {
    pub fn new(coloring: Coloring, kd: f32, ks: f32, m: i32) -> Self {
        debug_assert!(kd >= 0.0 && kd <= 1.0);
        debug_assert!(ks >= 0.0 && ks <= 1.0);

        Self {
            coloring,
            kd,
            ks,
            m,
        }
    }

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
}

impl Default for Material {
    fn default() -> Self {
        let solid = Coloring::Solid(Color::new(0.0, 1.0, 0.0));
        Self {
            coloring: solid,
            kd: 0.5,
            ks: 0.5,
            m: 4,
        }
    }
}
