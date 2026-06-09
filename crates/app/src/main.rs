use eframe::egui;

struct StiftApp;

impl Default for StiftApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for StiftApp {
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Intentionally empty for now, just shows a blank window
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Stift"),
        ..Default::default()
    };

    eframe::run_native(
        "stift-app",
        native_options,
        Box::new(|_cc| Ok(Box::new(StiftApp::default()))),
    )
}
