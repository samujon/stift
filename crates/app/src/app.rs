use eframe::egui;
use egui::scroll_area::ScrollSource;
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};
use log::debug;
use std::env;
use stift_compositor::Compositor;
use stift_core::{Document, paint};

use crate::render::convert_to_egui_image;

pub fn run() -> eframe::Result<()> {
    unsafe {
        env::set_var(
            "RUST_LOG",
            "debug,winit=warn,eframe=warn,egui=warn,egui_glow=warn,wgpu=warn,tracing::span=warn",
        );
    }
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
    document: Document,
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

        let document = Document::new(1000, 1000);
        let mut compositor = Compositor::new(document.width, document.height);
        // Produce the initial flattened image so the canvas has something to show.
        compositor.composite(&document);

        Self {
            document,
            compositor,
            canvas_texture: None,
            dock_state,
        }
    }
}

struct StiftTabViewer<'a> {
    document: &'a mut Document,
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

        const CONTENT_MARGIN: f32 = 1000.0;
        let image_rect = egui::ScrollArea::both()
            // Enable scrolling with scroll bars and mouse wheel, but not dragging the content to scroll
            // This behaviour should change in the future or maybe be configurable depending on
            // the tool selected, but for now we want to be able to scroll with the mouse wheel without dragging the canvas around
            .scroll_source(ScrollSource {
                scroll_bar: (true),
                drag: (false),
                mouse_wheel: (true),
            })
            .content_margin(CONTENT_MARGIN)
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                self.canvas_texture
                    .as_ref()
                    .map(|texture| ui.image(texture).rect)
            });

        let image_rect = image_rect.inner;

        let latest_pos = ui.ctx().input(|i| i.pointer.latest_pos());
        debug!("Latest pointer position: {:?}", latest_pos);
        let is_pointer_down = ui.ctx().input(|i| i.pointer.any_down());

        // if the mouse is pressed, draw on the active layer at the mouse position
        if is_pointer_down && let (Some(pos), Some(rect)) = (latest_pos, image_rect) {
            // correct the screen position into canvas-local coordinates by
            // subtracting the image rect origin (accounts for panel/scroll offset)
            let local = pos - rect.min;
            let x = local.x.clamp(0.0, self.compositor.width() as f32 - 1.0) as u32;
            let y = local.y.clamp(0.0, self.compositor.height() as f32 - 1.0) as u32;

            debug!("Drawing at canvas-local position: ({}, {})", x, y);
            if let Some(layer) = self.document.active_layer_mut() {
                paint::rasterize(
                    layer,
                    x as usize,
                    y as usize,
                    stift_core::Brush::Round {
                        size: 100.0,
                        color: [100, 0, 0, 255],
                    },
                );
            }
            // Re-flatten the stack so the change is visible.
            self.compositor.composite(self.document);
        }
    }

    fn layers_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Layers");
        ui.separator();

        let mut changed = false;
        let active = self.document.active;
        let mut new_active = active;
        // Show top-most layer first, matching how layers stack visually.
        for index in (0..self.document.layers.len()).rev() {
            let layer = &mut self.document.layers[index];
            ui.horizontal(|ui| {
                changed |= ui.checkbox(&mut layer.visible, "").changed();
                if ui.selectable_label(active == index, &layer.name).clicked() {
                    new_active = index;
                }
            });
        }
        self.document.active = new_active;

        if changed {
            self.compositor.composite(self.document);
        }
    }

    fn properties_ui(&mut self, ui: &mut egui::Ui) {
        ui.heading("Properties");
        ui.separator();

        let active = self.document.active;
        let Some(layer) = self.document.layers.get_mut(active) else {
            ui.label("No active layer");
            return;
        };

        let mut changed = false;
        ui.label(format!("Layer: {}", layer.name));

        changed |= ui
            .add(egui::Slider::new(&mut layer.opacity, 0.0..=1.0).text("Opacity"))
            .changed();

        egui::ComboBox::from_label("Blend")
            .selected_text(layer.blend_mode.label())
            .show_ui(ui, |ui| {
                for mode in stift_core::BlendMode::ALL {
                    changed |= ui
                        .selectable_value(&mut layer.blend_mode, mode, mode.label())
                        .changed();
                }
            });

        if changed {
            self.compositor.composite(self.document);
        }
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
            document: &mut self.document,
            compositor: &mut self.compositor,
            canvas_texture: &mut self.canvas_texture,
        };

        DockArea::new(&mut self.dock_state)
            .style(Style::from_egui(ui.style().as_ref()))
            .show_inside(ui, &mut tab_viewer);
    }
}
