use std::env;

use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tab {
    Canvas,
    Layers,
    Properties,
    Toolbar,
}

struct StiftApp {
    compositor: Compositor,
    canvas_texture: Option<egui::TextureHandle>,
    dock_state: DockState<Tab>,
}

impl StiftApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Build initial layout: canvas in center, panels on right split vertically
        let mut dock_state = DockState::new(vec![Tab::Canvas]);

        // Split the root to put panels on the right (30% width)
        let surface = dock_state.main_surface_mut();
        let [_canvas_node, right_node] =
            surface.split_right(NodeIndex::root(), 0.75, vec![Tab::Layers]);

        // Split the right node vertically: properties below layers
        let [_layers_node, properties_node] =
            surface.split_below(right_node, 0.5, vec![Tab::Properties]);

        // Toolbar below properties
        surface.split_below(properties_node, 0.7, vec![Tab::Toolbar]);

        Self {
            compositor: Compositor::new(1000, 1000),
            canvas_texture: None,
            dock_state,
        }
    }
}

struct StiftTabViewer<'a> {
    compositor: &'a mut Compositor,
    canvas_texture: &'a mut Option<egui::TextureHandle>,
}

impl<'a> TabViewer for StiftTabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        match tab {
            Tab::Canvas => "Canvas".into(),
            Tab::Layers => "Layers".into(),
            Tab::Properties => "Properties".into(),
            Tab::Toolbar => "Toolbar".into(),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::Canvas => self.canvas_ui(ui),
            Tab::Layers => self.layers_ui(ui),
            Tab::Properties => self.properties_ui(ui),
            Tab::Toolbar => self.toolbar_ui(ui),
        }
    }
}

impl<'a> StiftTabViewer<'a> {
    fn canvas_ui(&mut self, ui: &mut egui::Ui) {
        if self.canvas_texture.is_none() {
            let image_data = convert_to_egui_image(
                self.compositor.width(),
                self.compositor.height(),
                self.compositor.output_buffer(),
            );
            *self.canvas_texture = Some(ui.ctx().load_texture(
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
            if let Some(texture) = self.canvas_texture.as_mut() {
                texture.set(image_data, egui::TextureOptions::LINEAR);
            }
            self.compositor.clear_redraw_flag();
        }

        egui::ScrollArea::both()
            .content_margin(1000.0)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                if let Some(texture) = self.canvas_texture.as_ref() {
                    ui.image(texture);
                }
            });
    }

    fn layers_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Layers");
        ui.separator();
        ui.label("⚫ Layer 1 (Background)");
    }

    fn properties_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
    }

    fn toolbar_ui(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.add_space(6.0);
            let _ = ui.button("🖌");
            let _ = ui.button("⛶");
            let _ = ui.button("🎨");
        });
    }
}

impl eframe::App for StiftApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut tab_viewer = StiftTabViewer {
            compositor: &mut self.compositor,
            canvas_texture: &mut self.canvas_texture,
        };

        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut tab_viewer);
    }
}
