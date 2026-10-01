use crate::state::AppState;
use eframe::egui;
use qrforge::EccLevel;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) -> bool {
    let mut changed = false;

    ui.heading("Settings");
    ui.add_space(8.0);

    ui.label("Content");
    if ui
        .add(
            egui::TextEdit::multiline(&mut state.data)
                .desired_width(f32::INFINITY)
                .desired_rows(4),
        )
        .changed()
    {
        changed = true;
    }

    ui.add_space(12.0);

    // ECC
    ui.label("Error Correction");
    egui::ComboBox::from_id_salt("ecc")
        .selected_text(format!("{:?}", state.ecc))
        .show_ui(ui, |ui| {
            for level in [EccLevel::L, EccLevel::M, EccLevel::Q, EccLevel::H] {
                if ui
                    .selectable_value(&mut state.ecc, level, format!("{:?}", level))
                    .changed()
                {
                    changed = true;
                }
            }
        });

    ui.add_space(8.0);

    // Scale
    ui.label(format!("Scale: {}", state.scale));
    if ui
        .add(egui::Slider::new(&mut state.scale, 4..=30))
        .changed()
    {
        changed = true;
    }

    // Quiet zone
    ui.label(format!("Quiet Zone: {}", state.quiet_zone));
    if ui
        .add(egui::Slider::new(&mut state.quiet_zone, 0..=10))
        .changed()
    {
        changed = true;
    }

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    // Colors
    ui.label("Colors");
    ui.horizontal(|ui| {
        ui.label("Foreground");
        let mut fg = [
            state.fg[0] as f32 / 255.0,
            state.fg[1] as f32 / 255.0,
            state.fg[2] as f32 / 255.0,
        ];
        if ui.color_edit_button_rgb(&mut fg).changed() {
            state.fg = [
                (fg[0] * 255.0) as u8,
                (fg[1] * 255.0) as u8,
                (fg[2] * 255.0) as u8,
            ];
            changed = true;
        }
    });

    ui.horizontal(|ui| {
        ui.label("Background");
        let mut bg = [
            state.bg[0] as f32 / 255.0,
            state.bg[1] as f32 / 255.0,
            state.bg[2] as f32 / 255.0,
        ];
        if ui.color_edit_button_rgb(&mut bg).changed() {
            state.bg = [
                (bg[0] * 255.0) as u8,
                (bg[1] * 255.0) as u8,
                (bg[2] * 255.0) as u8,
            ];
            changed = true;
        }
    });

    if ui.checkbox(&mut state.invert, "Invert colors").changed() {
        changed = true;
    }

    // Error display
    if let Some(err) = &state.last_error {
        ui.add_space(16.0);
        ui.colored_label(egui::Color32::RED, format!("Error: {err}"));
    }

    changed
}
