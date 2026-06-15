use stift_core::Brush;
pub struct Compositor {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
    needs_redraw: bool,
}

impl Compositor {
    pub fn new(width: u32, height: u32) -> Self {
        // Initialize with a solid white canvas (RGBA)
        let num_pixels = (width * height) as usize;
        let buffer = vec![255; num_pixels * 4];

        Self {
            width,
            height,
            buffer,
            needs_redraw: true,
        }
    }

    pub fn draw(&mut self, width: u32, height: u32, brush: Brush) {
        // width and height are from mouse coordinates, so we need to convert them to pixel indices
        let x = width as usize;
        let y = height as usize;
        if x < self.width as usize && y < self.height as usize {
            // Given the brush type, we can determine the area to update. For simplicity, let's assume a round brush with a fixed size.
            // actually a square at the moment
            let brush_size = match brush {
                Brush::Round { size } => size as usize,
            };
            // Given the brush size, we can calculate the area to update. For a round brush, we would typically 
            // update a circular area around the (x, y) position. 
            // For simplicity, let's just update a square area for now
            for i in y.saturating_sub(brush_size / 2)..=y + brush_size / 2 {
                for j in x.saturating_sub(brush_size / 2)..=x + brush_size / 2 {
                    if i < self.height as usize && j < self.width as usize {
                        let index = (i * self.width as usize + j) * 4;
                        if index + 3 < self.buffer.len() {
                            self.buffer[index] = 0;     // R
                            self.buffer[index + 1] = 0; // G
                            self.buffer[index + 2] = 0; // B
                            self.buffer[index + 3] = 255; // A
                        }
                    }
                }
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

    pub fn trigger_update(&mut self) {
        self.needs_redraw = true;
    }
}
