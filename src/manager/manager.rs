use super::{ui::frame::window_frame, ui::visualize::draw};
use eframe::{egui, NativeOptions};

#[derive(Default)]
pub struct Manager {}

impl eframe::App for Manager {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        window_frame(ctx, frame, "Rust ML Manager", |ui| {
            ui.label("Content");
            draw(ui);
        });
    }
}

pub fn get_option() -> NativeOptions {
    NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        fullscreen: true,
        decorated: false,
        min_window_size: Some(egui::vec2(400.0, 100.0)),
        initial_window_size: Some(egui::vec2(800.0, 480.0)),
        ..Default::default()
    }
}

pub fn startapp(options: eframe::NativeOptions) -> Result<(), eframe::Error> {
    eframe::run_native("", options, Box::new(|_cc| Box::<Manager>::default()))
}
