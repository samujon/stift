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
