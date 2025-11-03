use std::process::exit;

use eframe::egui::{self, Slider};

use crate::{canvas::Canvas, scene::Scene};

struct Visible {
    polygon: bool,
    mesh: bool,
    filling: bool,
}

pub struct PolygonApp {
    canvas: Canvas,
    scene: Scene,
    visible: Visible,
}

impl PolygonApp {
    pub fn new() -> Self {
        let fname = "points.txt";
        let scene = Scene::from_file(fname);
        match scene {
            Err(e) => {
                eprintln!("{e}");
                exit(1)
            }
            Ok(scene) => Self {
                canvas: Canvas::new(600, 600),
                scene,
                visible: Visible {
                    polygon: true,
                    mesh: true,
                    filling: true,
                },
            },
        }
    }
}

impl eframe::App for PolygonApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.style_mut(|style| style.wrap_mode = Some(egui::TextWrapMode::Extend));

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("Plik", |ui| {
                    if ui.button("Wyjdź").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        egui::SidePanel::right("right_panel")
            .resizable(false)
            .show(ctx, |ui| {
                let mut ox = self.scene.rot_ox().to_degrees();
                ui.label("OX");
                ui.add(
                    Slider::new(&mut ox, -90.0..=90.0)
                        .suffix("°")
                        .max_decimals(0),
                );
                self.scene.rotate_ox(ox.to_radians() - self.scene.rot_ox());

                let mut oz = self.scene.rot_oz().to_degrees();
                ui.label("OZ");
                ui.add(
                    Slider::new(&mut oz, -90.0..=90.0)
                        .suffix("°")
                        .fixed_decimals(0),
                );
                self.scene.rotate_oz(oz.to_radians() - self.scene.rot_oz());

                let mut n = self.scene.mesh_resolution();
                ui.label("Dokładność");
                ui.add(Slider::new(&mut n, 2..=50));
                self.scene.set_mesh_resolution(n);

                ui.separator();
                ui.checkbox(&mut self.visible.polygon, "wielobok");
                ui.checkbox(&mut self.visible.mesh, "siatka");
                ui.checkbox(&mut self.visible.filling, "wypełnienie");

                ui.separator();
                ui.label("Matowość (kd)");
                ui.add(Slider::new(&mut self.scene.material.kd, 0.0..=1.0).fixed_decimals(2));
                ui.label("Połysk (ks)");
                ui.add(Slider::new(&mut self.scene.material.ks, 0.0..=1.0).fixed_decimals(2));
                ui.label("Wykładnik zwierciadlany (m)");
                ui.add(Slider::new(&mut self.scene.material.m, 1..=100));

                ui.label("Kolor powierzchni");
                let mut col = self.scene.material_color();
                ui.color_edit_button_rgb(&mut col);
                self.scene.set_material_color(col);

                ui.label("Kolor światła");
                let mut col = self.scene.light_color();
                ui.color_edit_button_rgb(&mut col);
                self.scene.set_light_color(col);
            });

        self.canvas.clear(None);
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let delta = response.drag_delta();
            let strength = 8e-3;

            if delta.y.abs() > 10.0 * strength {
                self.scene.rotate_ox(delta.y * strength);
            }
            if delta.x.abs() > 10.0 * strength {
                self.scene.rotate_oz(delta.x * strength);
            }

            if self.visible.filling {
                self.scene.draw_fillings(&mut self.canvas);
                self.canvas.draw(ctx, &painter);
            }
            if self.visible.mesh {
                self.scene.draw_outlines(&self.canvas, &painter);
            }
            if self.visible.polygon {
                self.scene.draw_points(&self.canvas, &painter);
            }
        });
        ctx.request_repaint();
    }
}
