use stift_core::{Brush, StrokePoint};

pub trait Renderer {
    fn begin_stroke(&mut self, brush: Brush, point: StrokePoint);
    fn push_point(&mut self, point: StrokePoint);
    fn end_stroke(&mut self);
    fn present(&mut self);
}

#[derive(Debug, Default)]
pub struct NoopRenderer {
    active_stroke: bool,
    active_brush: Option<Brush>,
    points_in_stroke: usize,
}

impl NoopRenderer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Renderer for NoopRenderer {
    fn begin_stroke(&mut self, brush: Brush, _point: StrokePoint) {
        self.active_stroke = true;
        self.active_brush = Some(brush);
        self.points_in_stroke = 1;
    }

    fn push_point(&mut self, _point: StrokePoint) {
        if self.active_stroke {
            self.points_in_stroke += 1;
        }
    }

    fn end_stroke(&mut self) {
        self.active_stroke = false;
        self.active_brush = None;
        self.points_in_stroke = 0;
    }

    fn present(&mut self) {
        // Intentionally no-op for now. Real renderer will submit GPU passes here.
    }
}
