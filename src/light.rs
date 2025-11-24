use crate::{color::Color, point::Point3};

#[derive(Debug, Clone)]
pub struct Light {
    pub pos: Point3,
    pub color: Color,
    /// advancement in animation
    pub t: f32,
    pub is_animating: bool,
    /// r = 0 for point light, otherwise I = I0 * cos^r(theta)
    pub r: i32,
}

impl Light {
    pub fn new(pos: Point3, color: Color) -> Self {
        Self {
            pos,
            color,
            t: 0.0,
            is_animating: true,
            r: 0,
        }
    }

    pub fn advance_animation(&mut self, dt: f32) {
        if self.is_animating {
            self.t += dt * 1e-1;
            if self.t > 1.0 {
                self.t = 0.0;
            }
        }

        let base_radius = 300.0;
        let r = base_radius + 40.0 * (self.t * std::f32::consts::TAU * 3.0).sin(); // 3 inner loops
        let angle = self.t * std::f32::consts::TAU;
        self.pos.x = r * angle.cos();
        self.pos.y = r * angle.sin();
    }
}
