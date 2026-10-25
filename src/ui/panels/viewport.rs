use egui::containers::panel::CentralPanel;

use crate::ui::panels::title_panel;

pub struct ViewportPanel;

impl ViewportPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, viewport_texture_id: Option<egui::TextureId>) {
        CentralPanel::default().show(ui, |ui| {
            title_panel(ui, "Viewport");

            if let Some(texture_id) = viewport_texture_id {
                let available_size = ui.available_size();
                ui.image(egui::load::SizedTexture::new(texture_id, available_size));
            }
        });
    }
}
