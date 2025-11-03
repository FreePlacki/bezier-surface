use crate::color::Color;

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
