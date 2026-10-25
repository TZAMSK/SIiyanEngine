use egui::Panel;
use egui_winit::egui::Color32;

use crate::{
    app::app_state::AppState,
    ui::{dimensions, panels::title_panel},
};

pub struct ConsolePanel;

impl ConsolePanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &AppState) {
        Panel::bottom("properties_panel")
            .resizable(true)
            .default_size(dimensions::CONSOLE_HEIGHT)
            .min_size(40.0)
            .frame(egui::Frame::new().fill(Color32::from_rgb(15, 15, 15)))
            .show(ui, |ui| {
                title_panel(ui, "Console");
                self.scene_properties(ui, state);
            });
    }

    fn scene_properties(&mut self, ui: &mut egui::Ui, _state: &AppState) {
        egui::Frame::new()
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                if ui.button("Clean").clicked() {};
                ui.label("Ready.")
            });
    }
}
