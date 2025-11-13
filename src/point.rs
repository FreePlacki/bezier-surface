use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use eframe::egui::{self, Pos2, pos2};

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

    pub fn to_screen(self, canvas: &Canvas) -> Self {
        Self {
            x: self.x + (canvas.width() as f32) * 0.5,
            y: -self.y + (canvas.height() as f32) * 0.5,
            z: self.z,
        }
    }

    pub fn to_viewport_center(self, ctx: &egui::Context) -> Self {
        let sz = ctx.used_size();
        Self {
            x: self.x + sz.x * 0.5,
            y: -self.y + sz.y * 0.5,
            z: self.z,
        }
    }

    pub fn projection(&self) -> Pos2 {
        pos2(self.x, self.y)
    }

    pub fn rotate_ox(&mut self, rot: f32) {
        let (s, c) = rot.sin_cos();
        let (y, z) = (self.y, self.z);
        self.y = y * c - z * s;
        self.z = y * s + z * c;
    }

    pub fn rotate_oz(&mut self, rot: f32) {
        let (s, c) = rot.sin_cos();
        let (x, y) = (self.x, self.y);
        self.x = x * c - y * s;
        self.y = x * s + y * c;
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zeros() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn normalized(&self) -> Self {
        let l = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Self {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    pub fn rotate_ox(&mut self, rot: f32) {
        let mut p = Point3::new(self.x, self.y, self.z);
        p.rotate_ox(rot);
        Self::new(p.x, p.y, p.z);
    }

    pub fn rotate_oz(&mut self, rot: f32) {
        let mut p = Point3::new(self.x, self.y, self.z);
        p.rotate_oz(rot);
        Self::new(p.x, p.y, p.z);
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: -self.x * rhs.z + self.z * rhs.x,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add<Point3> for Point3 {
    type Output = Self;
    fn add(self, rhs: Point3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Point3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Point3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vector3;
    fn sub(self, rhs: Point3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Self;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
