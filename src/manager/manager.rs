use crate::{
    adapter::{
        context::{self, Context, State},
        session::Session,
    },
    core::nn::{dataset::DataSet, nn::NN},
};

use super::{ui::frame::window_frame, ui::visualize::draw};
use eframe::{egui, NativeOptions};

#[derive(Default)]
pub struct Manager {
    context: Context,
}

impl eframe::App for Manager {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        window_frame(
            ctx,
            frame,
            "Rust ML Manager",
            &mut self.context,
            |ui, context| {
                ui.label("Content");
                draw(ui, context);
                ui.label(format!("{:?}", context.state));
                if ui.button("load").clicked() {
                    if context.state == State::Empty {
                        let layers = [4, 8, 8, 8, 4];
                        let mut orgin = NN::new(&layers);
                        orgin.rand();
                        context.session = Some(Session {
                            model: orgin,
                            dataset: DataSet {
                                inputs: vec![vec![]],
                                outputs: vec![vec![]],
                            },
                            option: crate::adapter::session::SessionOption {
                                train_method:
                                    crate::adapter::session::TrainingMethod::BackProp,
                                post_x: crate::adapter::session::PostX::Sigmoid,
                            },
                        });
                        context.state = State::Ready;
                    }
                }
                if ui.button("drop").clicked() {
                    context.session = None;
                    context.state = State::Empty;
                }
                if ui.button("rand").clicked() {
                    if context.state != State::Empty {
                        context.session.as_mut().unwrap().model.rand();
                    }
                }
            },
        );
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
