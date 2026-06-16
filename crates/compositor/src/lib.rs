//! Layer compositing.
//!
//! The compositor's single job is to flatten a [`Document`]'s layer stack into
//! one final RGBA8 image, applying each layer's opacity and blend mode. It does
//! not draw brush strokes — that is the brush engine's job (`stift_core::paint`).

use stift_core::{BlendMode, Document, Layer};

pub struct Compositor {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
    needs_redraw: bool,
}

impl Compositor {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer = vec![0; (width * height) as usize * 4];
        Self {
            width,
            height,
            buffer,
            needs_redraw: true,
        }
    }

    /// Flattens `document` into the output buffer, bottom layer first, applying
    /// each visible layer's opacity and blend mode via source-over compositing.
    pub fn composite(&mut self, document: &Document) {
        // Start from a transparent canvas.
        self.buffer.iter_mut().for_each(|b| *b = 0);

        for layer in document
            .layers
            .iter()
            .filter(|l| l.visible && l.opacity > 0.0)
        {
            self.blend_layer(layer);
        }

        self.needs_redraw = true;
    }

    fn blend_layer(&mut self, layer: &Layer) {
        let w = self.width.min(layer.width) as usize;
        let h = self.height.min(layer.height) as usize;
        let opacity = layer.opacity.clamp(0.0, 1.0);

        for y in 0..h {
            for x in 0..w {
                let dst_i = (y * self.width as usize + x) * 4;
                let src_i = (y * layer.width as usize + x) * 4;

                let src = &layer.buffer[src_i..src_i + 4];
                let sa = (src[3] as f32 / 255.0) * opacity;
                if sa <= 0.0 {
                    continue;
                }

                let dst = [
                    self.buffer[dst_i],
                    self.buffer[dst_i + 1],
                    self.buffer[dst_i + 2],
                    self.buffer[dst_i + 3],
                ];
                let da = dst[3] as f32 / 255.0;

                // Apply the blend mode to the color channels, then source-over.
                let out_a = sa + da * (1.0 - sa);
                for c in 0..3 {
                    let s = src[c] as f32 / 255.0;
                    let d = dst[c] as f32 / 255.0;
                    let blended = blend_channel(layer.blend_mode, s, d);
                    let out = blended * sa + d * (1.0 - sa);
                    self.buffer[dst_i + c] = (out.clamp(0.0, 1.0) * 255.0).round() as u8;
                }
                self.buffer[dst_i + 3] = (out_a.clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn output_buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn clear_redraw_flag(&mut self) {
        self.needs_redraw = false;
    }
}

/// Combines a single source/destination channel pair for a blend mode.
/// Inputs and output are normalized to `0.0..=1.0`.
fn blend_channel(mode: BlendMode, s: f32, d: f32) -> f32 {
    match mode {
        BlendMode::Normal => s,
        BlendMode::Multiply => s * d,
        BlendMode::Screen => 1.0 - (1.0 - s) * (1.0 - d),
        BlendMode::Overlay => {
            if d < 0.5 {
                2.0 * s * d
            } else {
                1.0 - 2.0 * (1.0 - s) * (1.0 - d)
            }
        }
    }
}
