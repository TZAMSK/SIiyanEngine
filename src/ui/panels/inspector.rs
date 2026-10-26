use egui::Panel;
use egui_winit::egui::Color32;

use crate::{
    app::app_state::AppState,
    ui::{dimensions, panels::title_panel},
};

pub struct InspectorPanel;

impl InspectorPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &AppState) {
        Panel::right("inspector_panel")
            .resizable(true)
            .default_size(dimensions::RIGHT_PANEL_WIDTH)
            .frame(egui::Frame::new().fill(Color32::from_rgb(15, 15, 15)))
            .show(ui, |ui| {
                title_panel(ui, "Inspector");
                self.scene_properties(ui, state);
            });
    }

    fn scene_properties(&mut self, ui: &mut egui::Ui, state: &AppState) {
        egui::Frame::new()
            .inner_margin(egui::Margin::same(8))
            .show(ui, |ui| {
                ui.label("Render");
                ui.separator();

                ui.label("Camera");
                ui.separator();
                ui.label(format!("Position: {}", format_vec3(state.camera.position)));
                ui.label(format!("Target: {}", format_vec3(state.camera.target)));
                ui.label("View mode:");
                ui.label("Placement mode:");

                ui.label("Selected");
                ui.separator();
            });
    }
}

fn format_vec3(v: [f32; 3]) -> String {
    format!("{:2}, {:2}, {:2}", v[0], v[1], v[2])
}
