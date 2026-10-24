use egui::{Panel, Stroke};
use egui_winit::egui::Color32;

use crate::app::app_state::AppState;

pub struct PropertiesPanel;

impl PropertiesPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &AppState) {
        let panel_frame = egui::Frame::new()
            .fill(Color32::GRAY)
            .corner_radius(10)
            .stroke(Stroke {
                width: 1.0,
                color: Color32::DARK_RED,
            })
            .outer_margin(1);

        Panel::right("properties_panel")
            .resizable(true)
            .default_size(250.0)
            .frame(
                panel_frame
                    .fill(Color32::from_rgba_premultiplied(40, 40, 40, 230))
                    .inner_margin(8.0),
            )
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Properties");
                    ui.separator();
                    self.scene_properties(ui, state);
                });
            });
    }

    fn scene_properties(&mut self, ui: &mut egui::Ui, state: &AppState) {
        ui.label("Scene");
        ui.indent("scene_properties", |ui| {
            ui.label(format!("Project: {}", state.project.name));
            ui.label(format!(
                "Viewport: {}x{}",
                state.viewport.width, state.viewport.height
            ));
        });
    }
}
