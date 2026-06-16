//! Shared paint primitives and the document data model.
//!
//! This crate is intentionally pure data + self-contained CPU rasterization. It
//! has no UI, GPU, or I/O dependencies so it can be reused anywhere.

pub mod paint;

#[derive(Debug, Clone, Copy)]
pub struct StrokePoint {
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
}

impl StrokePoint {
    pub fn new(x: f32, y: f32, pressure: f32) -> Self {
        Self { x, y, pressure }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Brush {
    Square { size: f32, color: [u8; 4] },
    Round { size: f32, color: [u8; 4] },
    Diamond { size: f32, color: [u8; 4] },
}

/// How a layer's pixels combine with the layers below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
}

impl BlendMode {
    /// All variants, in UI display order.
    pub const ALL: [BlendMode; 4] = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
        }
    }
}

/// A single editable image layer: an RGBA8 pixel buffer plus its compositing
/// properties. Brushes rasterize onto exactly one layer; the compositor later
/// flattens the whole stack.
#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub width: u32,
    pub height: u32,
    /// RGBA8, row-major, `width * height * 4` bytes.
    pub buffer: Vec<u8>,
    /// Layer-wide opacity in `0.0..=1.0`, applied on top of per-pixel alpha.
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub visible: bool,
}

impl Layer {
    /// Creates a fully transparent layer.
    pub fn transparent(width: u32, height: u32, name: impl Into<String>) -> Self {
        let buffer = vec![0; (width * height) as usize * 4];
        Self {
            name: name.into(),
            width,
            height,
            buffer,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            visible: true,
        }
    }

    /// Creates a layer filled with a solid opaque color (e.g. a background).
    pub fn filled(width: u32, height: u32, name: impl Into<String>, color: [u8; 4]) -> Self {
        let mut buffer = vec![0; (width * height) as usize * 4];
        for px in buffer.chunks_exact_mut(4) {
            px.copy_from_slice(&color);
        }
        Self {
            name: name.into(),
            width,
            height,
            buffer,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            visible: true,
        }
    }
}

/// The full editable document: an ordered stack of layers (index `0` is the
/// bottom-most) plus which layer is currently active for editing.
#[derive(Debug, Clone)]
pub struct Document {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub active: usize,
}

impl Document {
    /// Creates a document with an opaque white background and one transparent
    /// drawing layer on top (which starts active).
    pub fn new(width: u32, height: u32) -> Self {
        let layers = vec![
            Layer::filled(width, height, "Background", [255, 255, 255, 255]),
            Layer::transparent(width, height, "Layer 1"),
        ];
        let active = layers.len().saturating_sub(1);
        Self {
            width,
            height,
            layers,
            active,
        }
    }

    /// Mutable access to the currently active layer, if any.
    pub fn active_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.get_mut(self.active)
    }
}
