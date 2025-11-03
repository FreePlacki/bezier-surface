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

