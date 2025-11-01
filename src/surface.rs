use std::str::FromStr;

use eframe::egui::{Color32, Painter, pos2};

use crate::{canvas::Canvas, point::Point3};

pub struct BezierSurface {
    points: [[Point3; 4]; 4],
}

impl BezierSurface {
    pub fn draw_points(&self, canvas: &Canvas, painter: &Painter) {
        for y in 0..4 {
            for x in 0..4 {
                let p = self.points[y][x].to_screen(canvas);
                painter.circle_filled(pos2(p.x, p.y), 6.0, Color32::BLUE);
            }
        }
    }
}

impl FromStr for BezierSurface {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = [[Point3::origin(); 4]; 4];
        let mut lines = s.lines().filter(|l| l.len() > 2); // TODO: better way to filter empty

        for y in 0..4 {
            for x in 0..4 {
                let p: Point3 = lines.next().ok_or("expected point3")?.parse()?;
                points[y][x] = p;
            }
        }

        Ok(Self { points })
    }
}

// impl BezierSurface {
//     pub fn
// }
