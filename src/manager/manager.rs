use std::{thread, time::Duration};

use crate::adapter::context::Context;

use super::{
    ui::frame::window_frame,
    ui::{
        controller::create_model::CreateModel,
        visualize::{draw_cost, draw_node},
    },
};
use eframe::{
    egui::{self, ScrollArea},
    NativeOptions,
};

pub struct Manager {
    context: Context,
    create: CreateModel,
}

impl Manager {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        let cc_clone = cc.egui_ctx.clone();
        thread::spawn(move || {
            auto_refresh(cc_clone);
        });
        Manager {
            context: Context::default(),
            create: CreateModel::new(),
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
                    ui.label("Content");
                    draw_node(ui, context);
                    draw_cost(ui, context);
                    ui.label(format!("{:?}", context.state));

                    self.create.view(ui, context);

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
    eframe::run_native("", options, Box::new(|cc| Box::new(Manager::new(&cc))))
}

pub fn auto_refresh(ctx: egui::Context) {
    loop {
        ctx.request_repaint();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
