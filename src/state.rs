use std::default;

use qrforge::{ColorOptions, EccLevel}

#[derive(Clone)]
pub struct AppState {
    pub data: String,
    pub ecc: EccLevel,
    pub scale: u32,
    pub quiet_zone: u32,
    pub fg: [u8; 3],
    pub bg: [u8; 3],
    pub invert: bool,

    // generated results
    pub preview_texture: Option<egui::TextureHandle>,
    pub last_error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self { data: "https://example.com".to_string(),
            ecc: EccLevel::M,
            scale: 10,
            quiet_zone: 4,
            fg: [0, 0, 0], bg: [255, 255, 255],
            invert: false,
            preview_texture: None,
            last_error:None
        }
    }
}

impl AppState {
    pub fn colors(&self) -> ColorOptions {
        ColorOptions { fg: self.fg, bg:self.bg }
    }
}