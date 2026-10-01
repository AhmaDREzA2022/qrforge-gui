use crate::state::AppState;
use eframe::egui;
use qrforge::{generate_code, to_png, to_svg, PngOpts, SvgOpts};


pub fn show(ui: &mut egui::Ui, state: &mut AppState, needs_regenerate: &mut bool) {
    ui.horizontal(|ui| {
        ui.heading("QR Forge");
        ui.separator();

        if ui.button("💾 Save PNG").clicked() {
            save_png(state);
        }

        if ui.button("📄 Save SVG").clicked() {
            save_svg(state);
        }

        if ui.button("📋 Copy").clicked() {
            copy_to_clipboard(state);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("🔄 Regenerate").clicked() {
                *needs_regenerate = true;
            }
        });
    });
}

fn save_png(state: &AppState) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("PNG", &["png"])
        .set_file_name("qrcode.png")
        .save_file()
    else {
        return;
    };
 
    if let Ok(code) = generate_code(&state.data, state.ecc) {
        let opts = PngOpts {
            scale: state.scale.max(1),
            quiet_zone: state.quiet_zone,
            colors: state.colors(),
            invert: state.invert,
        };
        if let Ok(img) = to_png(&code, opts) {
            let _ = img.save(path);
        }
    }
}

fn save_svg(state: &AppState) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("SVG", &["svg"])
        .set_file_name("qrcode.svg")
        .save_file()
    else {
        return;
    };
 
    if let Ok(code) = generate_code(&state.data, state.ecc) {
        let opts = SvgOpts {
            scale: state.scale.max(1),
            quiet_zone: state.quiet_zone,
            colors: state.colors(),
            invert: state.invert,
        };
        if let Ok(svg) = to_svg(&code, opts) {
            let _ = std::fs::write(path, svg);
        }
    }
}

fn copy_to_clipboard(state: &AppState) {
    if let Ok(code) = generate_code(&state.data, state.ecc) {
        let opts = PngOpts {
            scale: state.scale.max(1),
            quiet_zone: state.quiet_zone,
            colors: state.colors(),
            invert: state.invert,
            
        };
        if let Ok(img) = to_png(&code, opts) {
            let rgba = img.to_rgb8();
            let (w, h) = rgba.dimensions();

            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_image(arboard::ImageData {
                    width: w as usize,
                    height: h as usize,
                    bytes: rgba.into_raw().into()
                });
            }
        }
    }
}