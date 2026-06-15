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

    pub fn draw(&mut self, width: u32, height: u32, _brush: Brush) {
        // Proof of concept to update pixel buffer and correct pixel locations
        // widhth and height are from mouse coordinates, so we need to convert them to pixel indices
        let x = width as usize;
        let y = height as usize;
        if x < self.width as usize && y < self.height as usize {
            let index = (y * self.width as usize + x) * 4; // RGBA
            if index + 3 < self.buffer.len() {
                // Simple brush effect: set the pixel to black with full opacity
                self.buffer[index] = 0;     // R
                self.buffer[index + 1] = 0; // G
                self.buffer[index + 2] = 0; // B
                self.buffer[index + 3] = 255; // A
                self.trigger_update();
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
