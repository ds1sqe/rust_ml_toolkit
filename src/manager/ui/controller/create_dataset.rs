use eframe::egui::Ui;

use crate::adapter::context::Context;

pub struct CreateDataSet<T> {
    inputs: Vec<Vec<T>>,
    outputs: Vec<Vec<T>>,
}

impl<T> CreateDataSet<T> {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
impl CreateDataSet<f64> {
    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        debug_assert!(self.inputs.len() == self.outputs.len());
        for idx in 0..self.inputs.len() {}
    }
}
