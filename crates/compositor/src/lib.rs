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
            match brush {
                Brush::Round { size, color } => {
                    self.draw_round_brush(x, y, size as usize, color);
                    self.trigger_update();
                }
                Brush::Square { size, color } => {
                    self.draw_square_brush(x, y, size as usize, color);
                    self.trigger_update();
                }
                Brush::Diamond { size, color } => {
                    self.draw_diamond_brush(x, y, size as usize, color);
                    self.trigger_update();
                }
            };
        }
    }

    fn draw_round_brush(&mut self, x: usize, y: usize, size: usize, color: [u8; 4]) {
        let radius = size / 2;
        for i in y.saturating_sub(radius)..=y + radius {
            for j in x.saturating_sub(radius)..=x + radius {
                if i < self.height as usize && j < self.width as usize {
                    let dx = j as isize - x as isize;
                    let dy = i as isize - y as isize;
                    if dx * dx + dy * dy <= (radius as isize * radius as isize) {
                        let index = (i * self.width as usize + j) * 4;
                        if index + 3 < self.buffer.len() {
                            self.buffer[index] = color[0]; // R
                            self.buffer[index + 1] = color[1]; // G
                            self.buffer[index + 2] = color[2]; // B
                            self.buffer[index + 3] = color[3]; // A
                        }
                    }
                }
            }
        }
    }

    fn draw_square_brush(&mut self, x: usize, y: usize, size: usize, color: [u8; 4]) {
        for i in y.saturating_sub(size / 2)..=y + size / 2 {
            for j in x.saturating_sub(size / 2)..=x + size / 2 {
                if i < self.height as usize && j < self.width as usize {
                    let index = (i * self.width as usize + j) * 4;
                    if index + 3 < self.buffer.len() {
                        self.buffer[index] = color[0]; // R
                        self.buffer[index + 1] = color[1]; // G
                        self.buffer[index + 2] = color[2]; // B
                        self.buffer[index + 3] = color[3]; // A
                    }
                }
            }
        }
    }

    fn draw_diamond_brush(&mut self, x: usize, y: usize, size: usize, color: [u8; 4]) {
        let radius = size / 2;
        for i in y.saturating_sub(radius)..=y + radius {
            for j in x.saturating_sub(radius)..=x + radius {
                if i < self.height as usize && j < self.width as usize {
                    let dx = (j as isize - x as isize).abs();
                    let dy = (i as isize - y as isize).abs();
                    if dx + dy <= radius as isize {
                        let index = (i * self.width as usize + j) * 4;
                        if index + 3 < self.buffer.len() {
                            self.buffer[index] = color[0]; // R
                            self.buffer[index + 1] = color[1]; // G
                            self.buffer[index + 2] = color[2]; // B
                            self.buffer[index + 3] = color[3]; // A
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
