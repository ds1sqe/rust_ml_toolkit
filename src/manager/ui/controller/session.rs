use std::path::Path;

use eframe::egui::Ui;

use crate::adapter::context::Context;

#[derive(PartialEq)]
enum SessionMenu {
    Load,
    Save,
}

pub struct SessionWindow {
    is_open: bool,
    menu: SessionMenu,
    session_load: SessionLoad,
    session_save: SessionSave,
}

impl SessionWindow {
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    pub fn new() -> Self {
        Self {
            is_open: false,
            menu: SessionMenu::Load,
            session_load: SessionLoad::new(),
            session_save: SessionSave::new(),
        }
    }
    pub fn view(
        &mut self,
        ctx: &eframe::egui::Context,
        ui: &mut Ui,
        context: &mut Context,
    ) {
        eframe::egui::Window::new("Session Management")
            .open(&mut self.is_open)
            .resizable(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut self.menu,
                        SessionMenu::Load,
                        "Load Session",
                    );
                    ui.selectable_value(
                        &mut self.menu,
                        SessionMenu::Save,
                        "Save Current",
                    )
                });
                ui.separator();

                eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                    match self.menu {
                        SessionMenu::Load => {
                            self.session_load.view(ui, context);
                        }
                        SessionMenu::Save => {
                            self.session_save.view(ui, context);
                        }
                        _ => {}
                    }
                })
            });
    }
}

pub struct SessionLoad {
    path: String,
}

impl SessionLoad {
    pub fn new() -> Self {
        Self {
            path: String::new(),
        }
    }
    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        ui.add(eframe::egui::TextEdit::singleline(&mut self.path));
        if ui.button("load").clicked() {
            let path = Path::new(&self.path);
            let loaded = Context::load_session(path);
            match loaded {
                None => {
                    println!("loading session failed")
                }
                Some(loaded_ctx) => {
                    *context = loaded_ctx;
                }
            }
        }
    }
}

pub struct SessionSave {
    path: String,
}

impl SessionSave {
    pub fn new() -> Self {
        Self {
            path: String::new(),
        }
    }
    pub fn view(&mut self, ui: &mut Ui, context: &mut Context) {
        ui.add(eframe::egui::TextEdit::singleline(&mut self.path));
        if ui.button("save").clicked() {
            let path = Path::new(&self.path);
            let saved = context.save_session(path);
            match saved {
                None => {
                    println!("saving session failed")
                }
                Some(_) => {
                    println!("saving session success")
                }
            }
        }
    }
}
