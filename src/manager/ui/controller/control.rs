use eframe::egui::Ui;

use crate::adapter::context::{Context, State};

pub struct Controller;
impl Controller {
    pub fn view(ui: &mut Ui, context: &mut Context) {
        if context.state == State::Ready && ui.button("Start").clicked() {
            context.start()
        }

        if context.state == State::Running && ui.button("Stop").clicked() {
            context.stop()
        }
    }
}
