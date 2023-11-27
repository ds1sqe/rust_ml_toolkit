use eframe::egui::Ui;
use eframe::egui::{self, Slider};

use crate::adapter::context::Context;
use crate::adapter::session::{PostX, TrainingMethod};

#[derive(PartialEq)]
enum ModelMenu {
    Create,
    Load,
    Save,
}

pub struct ModelWindow {
    is_open: bool,
    menu: ModelMenu,
    model_create: ModelCreate,
}

impl ModelWindow {
    pub fn new() -> Self {
        Self {
            is_open: false,
            menu: ModelMenu::Create,
            model_create: ModelCreate::new(),
        }
    }
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }

    pub fn view(
        &mut self,
        ctx: &eframe::egui::Context,
        ui: &mut Ui,
        context: &mut Context,
    ) {
        eframe::egui::Window::new("Model Management")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.menu,
                        ModelMenu::Create,
                        "Create New",
                    );
                    ui.selectable_value(
                        &mut self.menu,
                        ModelMenu::Save,
                        "Save Current",
                    )
                });
                ui.separator();

                eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.menu {
                        ModelMenu::Create => {
                            self.model_create.view(ui, context);
                        }
                        _ => {}
                    }
                })
            });
    }
}

pub struct ModelCreate {
    layout: String,
    train_method: TrainingMethod,
    rate: f64,
    eps: f64,
    post_x: PostX,
    cycle: usize,
}

impl ModelCreate {
    pub fn new() -> Self {
        ModelCreate {
            layout: String::new(),
            train_method: TrainingMethod::BackProp,
            rate: 1e-3,
            eps: 1e-3,
            post_x: PostX::Sigmoid,
            cycle: 1000,
        }
    }
    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        ui.horizontal(|ui| {
            ui.label("Layout");
            ui.add(
                egui::TextEdit::singleline(&mut self.layout)
                    .hint_text("ex) 8,8,8,10"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("TrainingMethod");
            ui.radio_value(
                &mut self.train_method,
                TrainingMethod::BackProp,
                "BackProp",
            );
            ui.radio_value(
                &mut self.train_method,
                TrainingMethod::FiniteDiff {
                    rate: self.rate,
                    eps: self.eps,
                },
                "FiniteDiff",
            );
        });

        if let TrainingMethod::FiniteDiff { .. } = self.train_method {
            ui.horizontal(|ui| {
                ui.label("Rate: ");
                ui.add(
                    Slider::new(&mut self.rate, 1e-10..=1.0)
                        .smallest_positive(1e-10)
                        .logarithmic(true)
                        .text("rate"),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Epsilon: ");
                ui.add(
                    Slider::new(&mut self.eps, 1e-10..=1.0)
                        .smallest_positive(1e-10)
                        .logarithmic(true)
                        .text("epsilon"),
                );
            });
        }

        ui.horizontal(|ui| {
            ui.label("Post process");
            ui.radio_value(&mut self.post_x, PostX::Sigmoid, "Sigmoid");
        });

        ui.horizontal(|ui| {
            ui.label("Cycle Chunk Size");
            ui.add(Slider::new(&mut self.cycle, 1..=10000));
        });

        if ui.button("Create").clicked() {
            let layers: Vec<&str> = self.layout.split(',').collect();
            let layers: Vec<usize> =
                layers.iter().map(|s| s.parse::<usize>().unwrap()).collect();
            let train_method = self.train_method.clone();
            let post_x = self.post_x.clone();
            let cycle = self.cycle.clone();
            context.create_model(&layers, train_method, post_x, cycle);
        }
    }
}
