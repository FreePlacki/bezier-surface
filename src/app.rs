use std::{
    process::exit,
    sync::mpsc::{self, Sender},
};

use eframe::egui::{self, Context, Slider, Ui};

use crate::{canvas::Canvas, scene::Scene};

struct Visible {
    polygon: bool,
    mesh: bool,
    filling: bool,
    light_pos: bool,
}

impl Default for Visible {
    fn default() -> Self {
        Self {
            polygon: false,
            mesh: false,
            filling: true,
            light_pos: true,
        }
    }
}

pub struct PolygonApp {
    canvas: Canvas,
    scene: Scene,
    visible: Visible,
    rx_tex: Option<mpsc::Receiver<String>>,
    rx_nor: Option<mpsc::Receiver<String>>,
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
                visible: Visible::default(),
                rx_tex: None,
                rx_nor: None,
            },
        }
    }
}

impl PolygonApp {
    fn rotations(&mut self, ui: &mut Ui) {
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
    }

    fn mesh_resolution(&mut self, ui: &mut Ui) {
        let mut n = self.scene.mesh_resolution();
        ui.label("Dokładność");
        ui.add(Slider::new(&mut n, 2..=50));
        self.scene.set_mesh_resolution(n);
    }

    fn visibility(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.visible.polygon, "wielobok");
        ui.checkbox(&mut self.visible.mesh, "siatka");
        ui.checkbox(&mut self.visible.filling, "wypełnienie");
        ui.checkbox(&mut self.visible.light_pos, "pozycja światła");
    }

    fn surface_props(&mut self, ui: &mut Ui) {
        ui.label("Matowość (kd)");
        ui.add(Slider::new(&mut self.scene.material.kd, 0.0..=1.0).fixed_decimals(2));
        ui.label("Połysk (ks)");
        ui.add(Slider::new(&mut self.scene.material.ks, 0.0..=1.0).fixed_decimals(2));
        ui.label("Wykładnik zwierciadlany (m)");
        ui.add(Slider::new(&mut self.scene.material.m, 1..=100));
    }

    fn pick_image(&mut self, tx: Sender<String>) {
        std::thread::spawn(move || {
            // https://docs.rs/image/latest/image/codecs/index.html#supported-formats
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(
                    "image",
                    &[
                        "png", "jpg", "gif", "bmp", "hdr", "ico", "jpeg", "pnm", "tiff", "webp",
                    ],
                )
                .pick_file()
            {
                let s = path.display().to_string();
                let _ = tx.send(s);
            }
        });
    }

    fn pick_surface(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let mut col = self.scene.material_color();
            ui.color_edit_button_rgb(&mut col);
            if col != self.scene.material_color() {
                self.scene.set_material_color(col);
            }

            if ui.button("Tekstura z pliku...").clicked() {
                let (tx, rx) = mpsc::channel();
                self.rx_tex = Some(rx);
                self.pick_image(tx);
            }
        });

        if let Some(rx) = &self.rx_tex {
            if let Ok(path) = rx.try_recv() {
                self.scene.set_texture(path.into());
                self.rx_tex = None;
            }
        }
    }

    fn light_props(&mut self, ui: &mut Ui) {
        ui.label("Pozycja światła (z)");
        ui.add(Slider::new(&mut self.scene.light.pos.z, -600.0..=600.0).fixed_decimals(0));
        ui.label("Kolor światła");
        let mut col = self.scene.light_color();
        ui.color_edit_button_rgb(&mut col);
        self.scene.set_light_color(col);
    }

    fn light_animation(&mut self, ctx: &Context, ui: &mut Ui) {
        let dt = ctx.input(|i| i.stable_dt);
        self.scene.light.advance_animation(dt);
        ui.label("Animacja światła");
        ui.horizontal(|ui| {
            if self.scene.light.is_animating {
                if ui.small_button("⏸").clicked() {
                    self.scene.light.is_animating = false;
                }
            } else {
                if ui.small_button("▶").clicked() {
                    self.scene.light.is_animating = true;
                }
            }
            let mut new_t = self.scene.light.t;
            ui.add(Slider::new(&mut new_t, 0.0..=1.0).fixed_decimals(2));
            if !self.scene.light.is_animating {
                self.scene.light.t = new_t;
                self.scene.light.advance_animation(dt);
            }
        });
    }

    fn normal_map(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if ui.button("Mapa wekt. normalnych...").clicked() {
                let (tx, rx) = mpsc::channel();
                self.rx_nor = Some(rx);
                self.pick_image(tx);
            }

            if self.scene.material.normal_map.is_some() {
                if ui.button("🗑").clicked() {
                    self.scene.material.normal_map = None;
                }
            }
        });

        if let Some(rx) = &self.rx_nor {
            if let Ok(path) = rx.try_recv() {
                self.scene.set_normal_map(path.into());
                self.rx_nor = None;
            }
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
                self.rotations(ui);
                self.mesh_resolution(ui);
                ui.separator();

                self.visibility(ui);
                ui.separator();

                self.surface_props(ui);

                ui.label("Kolor powierzchni");
                self.pick_surface(ui);
                self.normal_map(ui);

                ui.separator();
                self.light_props(ui);
                self.light_animation(ctx, ui);
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
            if self.visible.light_pos {
                self.scene.draw_light_pos(&self.canvas, &painter);
            }
        });
        ctx.request_repaint();
    }
}
