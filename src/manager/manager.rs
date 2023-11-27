use std::{thread, time::Duration};

use crate::adapter::context::Context;

use super::{
    ui::controller::{
        control::Controller, create_dataset::DatasetView, create_model::CreateModel,
    },
    ui::{
        frame::window_frame,
        viewer::{costs::CostsView, nodes::NodesView},
    },
};
use eframe::{
    egui::{self, ScrollArea},
    NativeOptions,
};

pub struct Manager {
    context: Context,
    create_model: CreateModel,
    create_dataset: DatasetView,
    node_view: NodesView,
    cost_view: CostsView,
}

impl Manager {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let cc_clone = cc.egui_ctx.clone();
        thread::spawn(move || {
            auto_refresh(cc_clone);
        });
        Manager {
            context: Context::default(),
            create_model: CreateModel::new(),
            create_dataset: DatasetView::new(),
            node_view: NodesView { is_open: false },
            cost_view: CostsView { is_open: false },
        }
    }
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
                    self.node_view.view(ctx, ui, context);
                    if ui.button("Toggle Nodes Viewer").clicked() {
                        self.node_view.is_open = !self.node_view.is_open;
                    }

                    self.cost_view.view(ctx, ui, context);
                    if ui.button("Toggle Cost Viewer").clicked() {
                        self.cost_view.is_open = !self.cost_view.is_open;
                    }

                    ui.label(format!("{:?}", context.state));

                    self.create_model.view(ui, context);

                    self.create_dataset.view(ui, context);

                    Controller::view(ui, context);

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
                            context.session.as_mut().unwrap().model =
                                w2g.model.unwrap();
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
    eframe::run_native("", options, Box::new(|cc| Box::new(Manager::new(&cc))))
}

pub fn auto_refresh(ctx: egui::Context) {
    loop {
        ctx.request_repaint();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
