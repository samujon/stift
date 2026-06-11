use eframe::egui;
use stift_core::{Brush, StrokePoint, Canvas};
use stift_renderer::{NoopRenderer, Renderer};

pub fn run() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_title("Stift"),
        ..Default::default()
    };

    eframe::run_native(
        "stift-app",
        native_options,
        Box::new(|_cc| Ok(Box::new(StiftApp::default()))),
    )
}

struct StiftApp {
    renderer: NoopRenderer,
    selected_brush: Brush,
    is_drawing: bool,
    preview_points: Vec<egui::Pos2>,
    canvas: Canvas,
}

impl Default for StiftApp {
    fn default() -> Self {
        Self {
            renderer: NoopRenderer::new(),
            selected_brush: Brush::Round { size: 8.0 },
            is_drawing: false,
            preview_points: Vec::new(),
            canvas: Canvas { width: 900, height: 650 },
        }
    }
}

impl eframe::App for StiftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            ui.label("Tools:");
            if ui.button("Round 8px").clicked() {
                self.selected_brush = Brush::Round { size: 8.0 };
            }
            if ui.button("Round 24px").clicked() {
                self.selected_brush = Brush::Round { size: 24.0 };
            }
        });

        ui.separator();

        let available = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(available, egui::Sense::drag());
        ui.painter()
            .rect_filled(rect, 0.0, egui::Color32::from_gray(24));

        if response.drag_started()
            && let Some(pos) = response.interact_pointer_pos() {
                self.renderer
                    .begin_stroke(self.selected_brush, StrokePoint::new(pos.x, pos.y, 1.0));
                self.is_drawing = true;
                self.preview_points.push(pos);
            }

        if response.dragged() && self.is_drawing
            && let Some(pos) = response.interact_pointer_pos() {
                self.renderer
                    .push_point(StrokePoint::new(pos.x, pos.y, 1.0));
                self.preview_points.push(pos);
            }

        let stroke_width = match self.selected_brush {
            Brush::Round { size } => size,
        };

        for segment in self.preview_points.windows(2) {
            ui.painter().line_segment(
                [segment[0], segment[1]],
                egui::Stroke::new(stroke_width, egui::Color32::WHITE),
            );
        }

        if self.is_drawing && !ui.input(|i| i.pointer.primary_down()) {
            self.renderer.end_stroke();
            self.is_drawing = false;
        }

        self.renderer.present();
    }
}
