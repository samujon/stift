use eframe::egui;
use stift_compositor::Compositor;
use stift_renderer::convert_to_egui_image;

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_title("Stift Editor"),
        ..Default::default()
    };

    eframe::run_native(
        "Stift Application Engine",
        options,
        Box::new(|cc| Ok(Box::new(StiftApp::new(cc)))),
    )
}

struct StiftApp {
    compositor: Compositor,
    canvas_texture: Option<egui::TextureHandle>,
}

impl StiftApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            compositor: Compositor::new(1800, 920),
            canvas_texture: None,
        }
    }
}

impl eframe::App for StiftApp {
   fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Check if we need to initialize or update the GPU texture asset
            if self.canvas_texture.is_none() {
                let image_data = convert_to_egui_image(
                    self.compositor.width(),
                    self.compositor.height(),
                    self.compositor.output_buffer(),
                );
                self.canvas_texture = Some(ui.ctx().load_texture(
                    "main_workspace_canvas",
                    image_data,
                    egui::TextureOptions::LINEAR,
                ));
                self.compositor.clear_redraw_flag();
            } else if self.compositor.needs_redraw() {
                let image_data = convert_to_egui_image(
                    self.compositor.width(),
                    self.compositor.height(),
                    self.compositor.output_buffer(),
                );
                if let Some(texture) = &mut self.canvas_texture {
                    texture.set(image_data, egui::TextureOptions::LINEAR);
                }
                self.compositor.clear_redraw_flag();
            }

            // Render the GPU texture inside a scrollable area to support panning/zooming later
            egui::ScrollArea::both()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    if let Some(texture) = &self.canvas_texture {
                        // Display texture image widget
                        ui.image(texture);
                    }
                });
        });
    }
}
