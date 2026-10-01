use crate::state::AppState;
use crate::ui::{preview, sidebar, toolbar};
use eframe::egui;
use qrforge::{generate_code, to_png, EccLevel, PngOpts};

pub struct QrForgeApp {
    pub state: AppState,
    needs_regenerate: bool,
}

impl QrForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
            needs_regenerate: true,
        }
    }

    fn regenerate(&mut self, ctx: &egui::Context) {
        self.state.last_error = None;

        if self.state.data.trim().is_empty() {
            self.state.preview_texture = None;
            return;
        }

        match generate_code(&self.state.data, self.state.ecc) {
            Ok(code) => {
                let opts = PngOpts {
                    scale: self.state.scale.max(1),
                    quiet_zone: self.state.quiet_zone,
                    colors: self.state.colors(),
                    invert: self.state.invert,
                };

                match to_png(&code, opts) {
                    Ok(img) => {
                        let size = [img.width() as usize, img.height() as usize];
                        let rgba = img.to_rgba8();
                        let pixels = rgba.as_flat_samples();

                        let color_image =
                            egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                        self.state.preview_texture =
                            Some(ctx.load_texture("qr_preview", color_image, Default::default()));
                    }
                    Err(e) => {
                        self.state.last_error = Some(e.to_string());
                        self.state.preview_texture = None;
                    }
                }
            }
            Err(e) => {
                self.state.last_error = Some(e.to_string());
                self.state.preview_texture = None;
            }
        }
    }
}

impl eframe::App for QrForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark theme by default
        ctx.set_visuals(egui::Visuals::dark());

        if self.needs_regenerate {
            self.regenerate(ctx);
            self.needs_regenerate = false;
        }

        // Top toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            toolbar::show(ui, &mut self.state, &mut self.needs_regenerate);
        });

        // Left sidebar (controls)
        egui::SidePanel::left("sidebar")
            .resizable(true)
            .default_width(280.0)
            .show(ctx, |ui| {
                if sidebar::show(ui, &mut self.state) {
                    self.needs_regenerate = true;
                }
            });

        // Central preview
        egui::CentralPanel::default().show(ctx, |ui| {
            preview::show(ui, &self.state);
        });
    }
}
