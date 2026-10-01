use crate::state::AppState;
use eframe::egui;

pub fn show(ui: &mut egui::Ui, state: &AppState) {
    ui.vertical(|ui| {
        ui.add_space(20.0);
        ui.heading("Preview");
        ui.add_space(12.0);

        if let Some(texture) = &state.preview_texture {
            let max_size = ui.available_size().min_elem().min(400.0);
            let size = texture.size_vec2();
            let scale = max_size / size.max_elem();
            let display_size = size * scale;

            ui.image((texture.id(), display_size));
        } else {
            ui.label("Enter some text to generate a QR code");
        }
    });
}