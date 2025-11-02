use std::process::exit;

use eframe::egui::{self, Slider};

use crate::{canvas::Canvas, scene::Scene};

pub struct PolygonApp {
    canvas: Canvas,
    scene: Scene,
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
                ui.add(Slider::new(&mut ox, -90.0..=90.0).suffix("°"));
                self.scene.rotate_ox(ox.to_radians() - self.scene.rot_ox());

                let mut oz = self.scene.rot_oz().to_degrees();
                ui.label("OZ");
                ui.add(Slider::new(&mut oz, -90.0..=90.0).suffix("°"));
                self.scene.rotate_oz(oz.to_radians() - self.scene.rot_oz());
            });

        self.canvas.clear(None);

        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            let delta = response.drag_delta();

            self.scene.rotate_ox(-delta.y * 8e-3);
            self.scene.rotate_oz(delta.x * 8e-3);

            self.canvas.draw(ui);
            self.scene.draw(&mut self.canvas, &response, &painter);
        });
    }
}
