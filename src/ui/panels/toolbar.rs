use egui::Panel;

use crate::{app::app_state::AppState, ui::dimensions};

pub struct ToolbarPanel;

impl ToolbarPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, _state: &AppState) {
        Panel::top("toolbar")
            .exact_size(dimensions::TOOLBAR_HEIGHT)
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let _ = ui.button("Add Shape");
                    let _ = ui.button("Iso");
                    let _ = ui.button("Top XY");
                });
            });
    }
}
