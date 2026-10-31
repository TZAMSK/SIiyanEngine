use egui::Panel;

use crate::app::app_state::AppState;

pub struct MenuPanel;

impl MenuPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _state: &AppState) {
        Panel::top("menu_bar").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| if ui.button("New").clicked() {});
                ui.menu_button("Create", |_ui| {});
                ui.menu_button("Camera", |_ui| {});
                ui.menu_button("View", |_ui| {});
            });
        });
    }
}
