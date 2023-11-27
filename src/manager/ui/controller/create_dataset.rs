use eframe::egui;
use eframe::egui::Ui;

use crate::{
    adapter::context::{Context, State},
    core::nn::dataset::DataSet,
};

#[derive(PartialEq)]
enum DataSetMenu {
    View,
    Edit,
    Load,
    Save,
}

pub struct DatasetWindow {
    is_open: bool,
    menu: DataSetMenu,
    dataset_view: DatasetView,
    dataset_update: Option<DatasetUpdate>,
}

impl DatasetWindow {
    pub fn new() -> Self {
        Self {
            is_open: false,
            menu: DataSetMenu::View,
            dataset_view: DatasetView::new(),
            dataset_update: None,
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
        eframe::egui::Window::new("Dataset Management")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.menu,
                        DataSetMenu::View,
                        "View Current",
                    );
                    ui.selectable_value(
                        &mut self.menu,
                        DataSetMenu::Edit,
                        "Edit Current",
                    )
                });
                ui.separator();
                match self.menu {
                    DataSetMenu::View => {
                        self.dataset_update = None;
                        self.dataset_view.view(ui, context);
                    }
                    DataSetMenu::Edit => {
                        if self.dataset_update.is_none() {
                            self.dataset_update = Some(DatasetUpdate::new(
                                self.dataset_view.inputs.clone(),
                                self.dataset_view.outputs.clone(),
                            ));
                        }
                        self.dataset_update.as_mut().unwrap().view(ui, context);
                    }
                    _ => {}
                }
            });
    }
}

pub struct DatasetView {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl DatasetView {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
impl DatasetView {
    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        match context.state {
            State::Empty => {
                return;
            }
            _ => {
                let ds = context.session.clone().unwrap().dataset;
                if ds.is_some() {
                    let ds = ds.unwrap();

                    self.inputs = ds
                        .inputs
                        .iter()
                        .map(|input| {
                            let mut str = String::new();
                            for (idx, el) in input.iter().enumerate() {
                                str.push_str(&el.to_string());
                                if idx != input.len() - 1 {
                                    str.push(',');
                                }
                            }
                            str
                        })
                        .collect();
                    self.outputs = ds
                        .outputs
                        .iter()
                        .map(|output| {
                            let mut str = String::new();
                            for (idx, el) in output.iter().enumerate() {
                                str.push_str(&el.to_string());
                                if idx != output.len() - 1 {
                                    str.push(',');
                                }
                            }
                            str
                        })
                        .collect();
                }
            }
        }

        debug_assert!(self.inputs.len() == self.outputs.len());

        for idx in 0..self.inputs.len() {
            ui.label(format!("Index:{idx}"));
            ui.label("Input");
            ui.label(format!("{}", self.inputs[idx]));
            ui.label("Output");
            ui.label(format!("{}", self.outputs[idx]));
        }
    }
}

pub struct DatasetUpdate {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl DatasetUpdate {
    fn new(inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Self { inputs, outputs }
    }

    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        for idx in 0..self.inputs.len() {
            ui.label(format!("Index:{idx}"));
            ui.label("Input");
            ui.add(egui::TextEdit::multiline(&mut self.inputs[idx]));
            ui.label("Output");
            ui.add(egui::TextEdit::multiline(&mut self.outputs[idx]));
        }

        if ui.button("Add New").clicked() {
            self.inputs.push(String::new());
            self.outputs.push(String::new());
        }

        if ui.button("Update DataSet").clicked() {
            let mut new_inputs = Vec::new();
            let mut new_outputs = Vec::new();
            for idx in 0..self.inputs.len() {
                let inputs: Vec<&str> = self.inputs[idx].split(',').collect();

                let mut input_buffer = Vec::new();
                for input in inputs {
                    if !input.is_empty() {
                        input_buffer.push(input.parse::<f64>().unwrap())
                    }
                }
                new_inputs.push(input_buffer);

                let outputs: Vec<&str> = self.outputs[idx].split(',').collect();
                let mut output_buffer = Vec::new();
                for output in outputs {
                    if !output.is_empty() {
                        output_buffer.push(output.parse::<f64>().unwrap())
                    }
                }
                new_outputs.push(output_buffer);
            }
            let ds = DataSet::new(new_inputs, new_outputs);
            context.attach_dataset(ds);
            context.state = State::Ready;
        }
    }
}
