use egui::Panel;
use egui_winit::egui::Color32;

use crate::{
    app::app_state::AppState,
    ui::{dimensions, panels::title_panel},
};

pub struct ScenePanel;

impl ScenePanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &AppState) {
        Panel::left("scene_panel")
            .resizable(true)
            .default_size(dimensions::LEFT_PANEL_WIDTH)
            .frame(egui::Frame::new().fill(Color32::from_rgb(15, 15, 15)))
            .show(ui, |ui| {
                title_panel(ui, "Scene");
                self.scene_properties(ui, state);
            });
    }

    fn scene_properties(&mut self, ui: &mut egui::Ui, _state: &AppState) {
        egui::Frame::new()
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.label("Object (1)");
                ui.separator();

                ui.indent("shapes_tree", |ui| {
                    ui.label("Triangle");
                });
            });
    }
}
