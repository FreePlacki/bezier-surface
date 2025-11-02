use std::str::FromStr;

use eframe::{
    egui::{Color32, Painter, Stroke, pos2},
};

use crate::{
    canvas::Canvas,
    mesh::Mesh,
    point::{Point3, Vector3},
    triangle::{Triangle, Vertex},
};

pub struct BezierSurface {
    points: [[Point3; 4]; 4],
}

impl BezierSurface {
    pub fn rotate_ox(&mut self, delta: f32) {
        self.points
            .iter_mut()
            .for_each(|r| r.iter_mut().for_each(|p| p.rotate_ox(delta)));
    }

    pub fn rotate_oz(&mut self, delta: f32) {
        self.points
            .iter_mut()
            .for_each(|r| r.iter_mut().for_each(|p| p.rotate_oz(delta)));
    }

    pub fn evaluate(&self, u: f32, v: f32) -> Vertex {
        fn bernstein(i: usize, t: f32) -> f32 {
            match i {
                0 => (1.0 - t).powi(3),
                1 => 3.0 * t * (1.0 - t).powi(2),
                2 => 3.0 * t * t * (1.0 - t),
                3 => t.powi(3),
                _ => unreachable!(),
            }
        }

        fn deriv(i: usize, t: f32) -> f32 {
            match i {
                0 => -3.0 * (1.0 - t).powi(2),
                1 => 3.0 * (1.0 - t).powi(2) - 6.0 * t * (1.0 - t),
                2 => 6.0 * t * (1.0 - t) - 3.0 * t.powi(2),
                3 => 3.0 * t.powi(2),
                _ => unreachable!(),
            }
        }

        let mut p = Point3::origin();
        let mut du = Vector3::zeros();
        let mut dv = Vector3::zeros();
        for j in 0..4 {
            let bv = bernstein(j, v);
            let dbv = deriv(j, v);
            for i in 0..4 {
                let bu = bernstein(i, u);
                let dbu = deriv(i, u);
                let pt = self.points[j][i];
                let w = bu * bv;
                p.x += pt.x * w;
                p.y += pt.y * w;
                p.z += pt.z * w;
                du.x += pt.x * dbu * bv;
                du.y += pt.y * dbu * bv;
                du.z += pt.z * dbu * bv;
                dv.x += pt.x * bu * dbv;
                dv.y += pt.y * bu * dbv;
                dv.z += pt.z * bu * dbv;
            }
        }
        let n = du.cross(dv).normalized();
        Vertex::new(p, n)
    }

    pub fn triangulate(&self, resolution: usize) -> Mesh {
        let mut triangles = Vec::new();

        let n = resolution - 1;

        let u = |x| x as f32 / n as f32;
        let v = |y| y as f32 / n as f32;

        for y in 0..n {
            for x in 0..n {
                let p00 = self.evaluate(u(x), v(y));
                let p10 = self.evaluate(u(x + 1), v(y));
                let p01 = self.evaluate(u(x), v(y + 1));
                let p11 = self.evaluate(u(x + 1), v(y + 1));

                triangles.push(Triangle::new(p00, p10, p11));
                triangles.push(Triangle::new(p00, p11, p01));
            }
        }
        Mesh::new(triangles, resolution)
    }

    pub fn draw_points(&self, canvas: &Canvas, painter: &Painter) {
        for y in 0..4 {
            for x in 0..4 {
                let p = self.points[y][x].to_screen(canvas);
                painter.circle_filled(pos2(p.x, p.y), 6.0, Color32::RED);

                if x + 1 < 4 {
                    let p_next = self.points[y][x + 1].to_screen(canvas);
                    painter.line_segment(
                        [pos2(p.x, p.y), pos2(p_next.x, p_next.y)],
                        Stroke::new(1.0, Color32::LIGHT_RED),
                    );
                }

                if y + 1 < 4 {
                    let p_next = self.points[y + 1][x].to_screen(canvas);
                    painter.line_segment(
                        [pos2(p.x, p.y), pos2(p_next.x, p_next.y)],
                        Stroke::new(1.0, Color32::LIGHT_RED),
                    );
                }
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
