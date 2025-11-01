use std::str::FromStr;

use crate::canvas::Canvas;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn to_screen(&self, canvas: &Canvas) -> Self {
        Self {
            x: self.x + (canvas.width() as f32) * 0.5,
            y: self.y + (canvas.height() as f32) * 0.5,
            z: self.z,
        }
    }
}

impl FromStr for Point3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split_whitespace();
        let x: f32 = pts
            .next()
            .ok_or("expected x coordinate")?
            .parse()
            .map_err(|e| format!("x should be a real number, {e}"))?;
        let y: f32 = pts
            .next()
            .ok_or("expected y coordinate")?
            .parse()
            .map_err(|e| format!("y should be a real number, {e}"))?;
        let z: f32 = pts
            .next()
            .ok_or("expected z coordinate")?
            .parse()
            .map_err(|e| format!("z should be a real number, {e}"))?;

        if pts.next().is_some() {
            return Err("to many coordinates provided".into());
        }

        Ok(Self { x, y, z })
    }
}
