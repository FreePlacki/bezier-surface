use eframe::egui::{Color32, Painter, Stroke};

use crate::{canvas::Canvas, point::Point3};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub p0: Point3,
    pub p1: Point3,
    pub p2: Point3,
}

impl Triangle {
    pub fn new(p0: Point3, p1: Point3, p2: Point3) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn draw_outline(&self, canvas: &Canvas, painter: &Painter) {
        let stroke = Stroke::new(1.0, Color32::LIGHT_GREEN);

        let p0 = self.p0.to_screen(canvas).projection();
        let p1 = self.p1.to_screen(canvas).projection();
        let p2 = self.p2.to_screen(canvas).projection();
        painter.line(vec![p0, p1], stroke);
        painter.line(vec![p1, p2], stroke);
        painter.line(vec![p2, p0], stroke);
    }

    pub fn draw_filling(&self, canvas: &mut Canvas) {
        let color = [200, 255, 200, 255];

        let v0 = self.p0.to_screen(canvas).projection();
        let v1 = self.p1.to_screen(canvas).projection();
        let v2 = self.p2.to_screen(canvas).projection();

        let verts = [v0, v1, v2];

        let min_yf = verts
            .iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, |a, b| a.min(b));
        let max_yf = verts
            .iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, |a, b| a.max(b));

        if max_yf - min_yf < 1e-2 {
            return;
        }

        let y_min = min_yf.floor() as i32;
        let y_max = max_yf.ceil() as i32;

        let table_height = (y_max - y_min + 1).max(1) as usize;
        if table_height == 0 {
            return;
        }

        #[derive(Clone, Copy)]
        struct Edge {
            y_max: f32,
            x: f32,
            inv_slope: f32,
        }

        let mut edge_table: Vec<Vec<Edge>> = vec![Vec::new(); table_height];

        for i in 0..3 {
            let mut p0 = verts[i];
            let mut p1 = verts[(i + 1) % 3];

            if (p1.y - p0.y).abs() < 1e-2 {
                continue;
            }

            (p0.x, p0.y, p1.x, p1.y) = if p0.y < p1.y {
                (p0.x, p0.y, p1.x, p1.y)
            } else {
                (p1.x, p1.y, p0.x, p0.y)
            };

            let inv_slope = (p1.x - p0.x) / (p1.y - p0.y);

            let bucket_idx = (p0.y.ceil() as i32 - y_min) as isize;
            if bucket_idx < 0 || bucket_idx as usize >= edge_table.len() {
                continue;
            }

            edge_table[bucket_idx as usize].push(Edge {
                y_max: p1.y,
                x: p0.x,
                inv_slope,
            });
        }

        let scan_y_start = y_min.max(0);
        let scan_y_end = ((canvas.height() as i32) - 1).min(y_max);

        if scan_y_start > scan_y_end {
            return;
        }

        let mut active_edges: Vec<Edge> = Vec::new();

        for scan_y in y_min..=y_max {
            let bucket_idx = (scan_y - y_min) as usize;

            if bucket_idx < edge_table.len() {
                for &e in &edge_table[bucket_idx] {
                    active_edges.push(e);
                }
            }

            active_edges.retain(|e| (scan_y as f32) < e.y_max);

            active_edges.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));

            let mut i = 0;
            while i + 1 < active_edges.len() {
                let xa = active_edges[i].x;
                let xb = active_edges[i + 1].x;

                let x_start = xa.ceil() as i32;
                let x_end = xb.floor() as i32;

                if scan_y >= scan_y_start && scan_y <= scan_y_end {
                    let y = scan_y as usize;
                    let x0 = x_start.max(0) as usize;
                    let x1 = (canvas.width() as i32 - 1).min(x_end) as usize;
                    if x0 <= x1 {
                        for x in x0..=x1 {
                            canvas.put_pixel(x, y, color);
                        }
                    }
                }

                i += 2;
            }

            for e in &mut active_edges {
                e.x += e.inv_slope;
            }
        }
    }
}
