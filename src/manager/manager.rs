use std::{sync::mpsc::channel, thread, time::Duration};

use crate::{
    adapter::{
        context::{self, Context, State, Transceiver},
        learner::{spawn_learner, ControlSignal, G2w},
        nodes::Nodes,
        session::Session,
    },
    core::nn::{dataset::DataSet, nn::NN},
};

use super::{
    ui::frame::window_frame,
    ui::visualize::{draw_cost, draw_node},
};
use eframe::{
    egui::{self, ScrollArea},
    NativeOptions,
};

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
                ScrollArea::vertical().show(ui, |ui| {
                    ui.label("Content");
                    draw_node(ui, context);
                    draw_cost(ui, context);
                    ui.label(format!("{:?}", context.state));
                    if ui.button("load").clicked() {
                        if context.state == State::Empty {
                            let layers = [2, 4, 4, 1];
                            let mut origin = NN::new(&layers);
                            origin.rand();
                            context.nodes = Some(Nodes::from(&origin));
                            context.session = Some(Session {
                                model: origin,
                                dataset: DataSet {
                                    inputs: vec![
                                        vec![0.0, 0.0],
                                        vec![0.0, 1.0],
                                        vec![1.0, 0.0],
                                        vec![1.0, 1.0],
                                    ],
                                    outputs: vec![vec![1.0], vec![0.0], vec![0.0], vec![1.0]],
                                },
                                option: crate::adapter::session::SessionOption {
                                    train_method: crate::adapter::session::TrainingMethod::BackProp,
                                    post_x: crate::adapter::session::PostX::Sigmoid,
                                    cycle: 1000,
                                },
                            });
                            context.state = State::Ready;
                        }
                    }
                    if ui.button("drop").clicked() {
                        context.session = None;
                        if context.trcv.is_some() {
                            let res = context.trcv.as_ref().unwrap().snd.send(G2w {
                                sig: ControlSignal::Stop,
                            });
                        }
                        context.state = State::Empty;
                    }
                    if ui.button("rand").clicked() {
                        if context.state != State::Empty {
                            context.session.as_mut().unwrap().model.rand();
                        }
                    }
                    if ui.button("train").clicked() {
                        if context.state == State::Ready {
                            let (snd, rx) = channel();
                            let rec = spawn_learner(context.session.clone().unwrap(), rx);
                            context.state = State::Running;
                            context.trcv = Some(Transceiver { snd, rec })
                        }
                    }
                    if context.trcv.is_some() {
                        let w2g = context
                            .trcv
                            .as_ref()
                            .unwrap()
                            .rec
                            .recv_timeout(Duration::from_micros(1000));
                        if w2g.is_ok() {
                            let w2g = w2g.unwrap();
                            context.nodes = w2g.nodes;
                            context.costs.push(w2g.cost);
                            println!("updated cost {:?}", w2g.cost)
                        }
                    }
                });
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
