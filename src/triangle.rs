use eframe::egui::{Color32, Painter, Pos2, Stroke, pos2};

use crate::{
    canvas::Canvas,
    light::Light,
    material::Material,
    point::{Point3, Vector3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pos: Point3,
    normal: Vector3,
    u: f32,
    v: f32,
}

impl Vertex {
    pub fn new(pos: Point3, normal: Vector3, u: f32, v: f32) -> Self {
        Self { pos, normal, u, v }
    }

    pub fn rotate_ox(&mut self, rot: f32) {
        self.pos.rotate_ox(rot);
        self.normal.rotate_ox(rot);
    }

    pub fn rotate_oz(&mut self, rot: f32) {
        self.pos.rotate_oz(rot);
        self.normal.rotate_oz(rot);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub p0: Vertex,
    pub p1: Vertex,
    pub p2: Vertex,
}

impl Triangle {
    pub fn new(p0: Vertex, p1: Vertex, p2: Vertex) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn draw_outline(&self, canvas: &Canvas, painter: &Painter) {
        let stroke = Stroke::new(1.0, Color32::LIGHT_GREEN);

        let p0 = self.p0.pos.to_screen(canvas).projection();
        let p1 = self.p1.pos.to_screen(canvas).projection();
        let p2 = self.p2.pos.to_screen(canvas).projection();
        painter.line(vec![p0, p1], stroke);
        painter.line(vec![p1, p2], stroke);
        painter.line(vec![p2, p0], stroke);
    }

    fn determinant(&self) -> f32 {
        let (x0, y0) = (self.p0.pos.x, self.p0.pos.y);
        let (x1, y1) = (self.p1.pos.x, self.p1.pos.y);
        let (x2, y2) = (self.p2.pos.x, self.p2.pos.y);

        (y1 - y2) * (x0 - x2) + (x2 - x1) * (y0 - y2)
    }

    fn baryc(&self, x: f32, y: f32, det: f32) -> (f32, f32, f32) {
        let (x0, y0) = (self.p0.pos.x, self.p0.pos.y);
        let (x1, y1) = (self.p1.pos.x, self.p1.pos.y);
        let (x2, y2) = (self.p2.pos.x, self.p2.pos.y);

        let l0 = ((y1 - y2) * (x - x2) + (x2 - x1) * (y - y2)) / det;
        let l1 = ((y2 - y0) * (x - x2) + (x0 - x2) * (y - y2)) / det;
        let l2 = 1.0 - l0 - l1;

        (l0, l1, l2)
    }

    pub fn draw_filling(&self, canvas: &mut Canvas, light: &Light, material: &Material) {
        let v0 = self.p0.pos.to_screen(canvas).projection();
        let v1 = self.p1.pos.to_screen(canvas).projection();
        let v2 = self.p2.pos.to_screen(canvas).projection();

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
                            let xf = x as f32;
                            let yf = scan_y as f32;
                            let Pos2 { x: xf, y: yf } = canvas.from_screen(pos2(xf, yf));

                            let (l0, l1, l2) = self.baryc(xf, yf, self.determinant());

                            let n =
                                (self.p0.normal * l0 + self.p1.normal * l1 + self.p2.normal * l2)
                                    .normalized();
                            let p = self.p0.pos * l0 + self.p1.pos * l1 + self.p2.pos * l2;
                            let u = self.p0.u * l0 + self.p1.u * l1 + self.p2.u * l2;
                            let v = self.p0.v * l0 + self.p1.v * l1 + self.p2.v * l2;

                            let color = self.color_for(u, v, n, p, light, material);
                            canvas.put_pixel(x, y, p.z, color);
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

    fn color_for(&self, u: f32, v: f32, n: Vector3, p: Point3, light: &Light, material: &Material) -> [u8; 4] {
        let light_dir = (light.pos - p).normalized();
        let il = n.dot(light_dir).max(0.0);

        let r = n * (2.0 * n.dot(light_dir)) - light_dir;

        let iz = Vector3::new(0.0, 0.0, 1.0).dot(r).powi(material.m);

        let intensity = (material.kd * il + material.ks * iz) * 255.0;

        let col = material.color_at(u, v);
        [
            (light.color.r() * col.r() * intensity).min(255.0) as u8,
            (light.color.g() * col.g() * intensity).min(255.0) as u8,
            (light.color.b() * col.b() * intensity).min(255.0) as u8,
            255,
        ]
    }
}
