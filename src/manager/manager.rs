use std::{thread, time::Duration};

use crate::adapter::context::Context;

use super::{
    ui::controller::{control::Controller, model::ModelWindow},
    ui::{
        controller::{dataset::DatasetWindow, session::SessionWindow},
        frame::window_frame,
        viewer::{costs::CostsView, nodes::NodesView},
    },
};
use eframe::{
    egui::{self},
    NativeOptions,
};

pub struct Manager {
    context: Context,
    model_window: ModelWindow,
    dataset_window: DatasetWindow,
    session_window: SessionWindow,
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
            model_window: ModelWindow::new(),
            dataset_window: DatasetWindow::new(),
            session_window: SessionWindow::new(),
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
                egui::SidePanel::right("Toggle Windows")
                    .resizable(false)
                    .default_width(160.0)
                    .show_inside(ui, |ui| {
                        ui.heading("Toggle Windows");
                        ui.separator();

                        self.node_view.view(ctx, ui, context);
                        if ui.button("Nodes Viewer").clicked() {
                            self.node_view.is_open = !self.node_view.is_open;
                        }

                        self.cost_view.view(ctx, ui, context);
                        if ui.button("Cost Viewer").clicked() {
                            self.cost_view.is_open = !self.cost_view.is_open;
                        }

                        self.dataset_window.view(ctx, ui, context);
                        if ui.button("Manage Dataset").clicked() {
                            self.dataset_window.toggle();
                        }

                        self.model_window.view(ctx, ui, context);
                        if ui.button("Manage Model").clicked() {
                            self.model_window.toggle();
                        }

                        self.session_window.view(ctx, ui, context);
                        if ui.button("Manage Session").clicked() {
                            self.session_window.toggle();
                        }

                        Controller::view(ui, context);
                    });

                ui.label(format!("{:?}", context.state));

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
            },
        );
    }
}

pub fn get_option() -> NativeOptions {
    NativeOptions {
        decorated: false,
        fullscreen: false,
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
