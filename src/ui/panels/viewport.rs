use egui::containers::panel::CentralPanel;

use crate::{app::app_state::ViewportState, ui::panels::title_panel};

pub struct ViewportPanel;

impl ViewportPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        viewport_texture_id: Option<egui::TextureId>,
        viewport_state: &mut ViewportState,
    ) {
        CentralPanel::default().show(ui, |ui| {
            title_panel(ui, "Viewport");

            if let Some(texture_id) = viewport_texture_id {
                let size = ui.available_size();
                let response = ui.image(egui::load::SizedTexture::new(texture_id, size));

                let ppp = ui.ctx().pixels_per_point();
                viewport_state.width = (size.x * ppp).round() as u32;
                viewport_state.height = (size.y * ppp).round() as u32;
                viewport_state.hovered = response.hovered();
                let r = response.rect;
                viewport_state.rect = [r.min.x, r.min.y, r.max.x, r.max.y];
            }
        });
    }
}
