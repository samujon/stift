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
