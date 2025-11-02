use core::f32;

use eframe::egui::{self, Context, Painter, Pos2, Rect, TextureOptions, pos2, vec2};

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
    depths: Vec<f32>,
    texture: Option<egui::TextureHandle>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; width * height * 4],
            depths: vec![f32::INFINITY; width * height],
            texture: None,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, z: f32, rgba: [u8; 4]) {
        debug_assert!(x < self.width && y < self.height);

        let idx = y * self.width + x;
        if z < self.depths[idx] {
            self.depths[idx] = z;
            let bidx = idx * 4;
            self.buffer[bidx..bidx + 4].copy_from_slice(&rgba);
        }
    }

    pub fn clear(&mut self, rgba: impl Into<Option<[u8; 4]>>) {
        match rgba.into() {
            Some(ref c) => {
                for px in self.buffer.chunks_exact_mut(4) {
                    px.copy_from_slice(c);
                }
            }
            None => self.buffer.fill(0),
        }
        self.depths.fill(f32::INFINITY);
    }

    pub fn from_screen(&self, pos: Pos2) -> Pos2 {
        pos2(
            pos.x - (self.width as f32) * 0.5,
            (self.height as f32) * 0.5 - pos.y,
        )
    }

    pub fn draw(&mut self, ctx: &Context, painter: &Painter) {
        let img = eframe::egui::ColorImage::from_rgba_unmultiplied(
            [self.width, self.height],
            &self.buffer,
        );

        match &mut self.texture {
            Some(tex) => tex.set(img, TextureOptions::default()),
            None => {
                self.texture = Some(ctx.load_texture("surface", img, Default::default()));
            }
        }

        let tex = self.texture.as_ref().unwrap();

        let size = vec2(self.width as f32, self.height as f32);

        painter.image(
            tex.id(),
            Rect::from_min_size(pos2(0.0, 0.0), size),
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            eframe::egui::Color32::WHITE,
        );
    }
}
