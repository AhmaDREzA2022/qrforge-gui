mod app;
mod state;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("QR Forge"),
        ..Default::default()
    };

    eframe::run_native(
        "QR Forge",
        options,
        Box::new(|cc| Ok(Box::new(app::QrForgeApp::new(cc)))),
    )
}
