use image::ImageBuffer;

use crate::point::Vector3;

#[derive(Debug)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn from_img(img: ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> Self {
        let (width, height) = img.dimensions();
        let data = img.into_raw();
        Self {
            width,
            height,
            data,
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> [u8; 4] {
        let x = (u.clamp(0.0, 1.0) * (self.width - 1) as f32) as usize;
        let y = ((1.0 - v.clamp(0.0, 1.0)) * (self.height - 1) as f32) as usize; // flip y
        let idx = (y * self.width as usize + x) * 4;
        self.data[idx..idx + 4].try_into().unwrap()
    }

    /// Interprets the texture as a normal map with
    /// Nx \in [-1, 1], Ny \in [-1, 1], Nz \in [0, 1]
    /// from r, g, b respectively
    pub fn sample_normal(&self, u: f32, v: f32) -> Vector3 {
        let col = self.sample(u, v);
        let map = |c| (c / 255.0 - 0.5) * 2.0;
        Vector3 {
            x: map(col[0] as f32),
            y: map(col[1] as f32),
            z: map(col[2] as f32).max(0.0),
        }
    }
}
