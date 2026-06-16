use eframe::egui::{Color32, ColorImage};

/// Converts a raw RGBA8 slice into an egui::ColorImage.
///
/// This conversion is UI-framework specific, so it lives in the `app` crate
/// (the egui edge) rather than in the UI-agnostic `renderer` crate.
pub fn convert_to_egui_image(width: u32, height: u32, raw_pixels: &[u8]) -> ColorImage {
    let size = [width as usize, height as usize];

    // Map raw byte chunks straight into egui Color32 structures
    let pixels = raw_pixels
        .chunks_exact(4)
        .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();

    ColorImage {
        size,
        pixels,
        ..Default::default()
    }
}
