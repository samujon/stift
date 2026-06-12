use std::env;

use eframe::egui;
use stift_compositor::Compositor;
use stift_renderer::convert_to_egui_image;

pub fn run() -> eframe::Result<()> {

    unsafe { env::set_var("RUST_LOG", "info"); }
    env_logger::init();

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
            compositor: Compositor::new(1000, 1000),
            canvas_texture: None,
        }
    }
}

impl eframe::App for StiftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Right side panel containing layers + toolbar
        egui::Panel::right("right_panel")
            .resizable(true)
            .default_size(240.0)
            .size_range(180.0..=400.0)
            .show_inside(ui, |ui| {
                // Layers panel
                egui::Panel::top("layers_panel")
                    .resizable(true)
                    .default_size(200.0)
                    .size_range(60.0..=f32::INFINITY)
                    .show_inside(ui, |ui| {
                        ui.heading("Layers");
                        ui.separator();
                        ui.label("⚫ Layer 1 (Background)");
                        ui.allocate_space(ui.available_size());
                    });

                // Properties panel
                egui::Panel::top("properties_panel")
                    .resizable(true)
                    .default_size(150.0)
                    .size_range(60.0..=f32::INFINITY)
                    .show_inside(ui, |ui| {
                        ui.heading("Properties");
                        ui.allocate_space(ui.available_size());
                    });

                // Toolbar fills remaining bottom area
                ui.horizontal_top(|ui| {
                    ui.add_space(6.0);
                    let _ = ui.button("🖌"); // Brush tool placeholder
                    let _ = ui.button("⛶"); // Selection tool placeholder
                    let _ = ui.button("🎨"); // Color picker placeholder
                });
            });

        // Central canvas
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
                .content_margin(1000.0) // Add margin to allow panning beyond edges
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
