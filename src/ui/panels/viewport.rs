use egui::containers::panel::CentralPanel;
use transform_gizmo_egui::Gizmo;

use crate::{
    app::app_state::AppState,
    editor::gizmo::draw_gizmo,
    scene::{camera::CameraManager, Scene},
    ui::panels::title_panel,
};

pub struct ViewportPanel;

impl ViewportPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        viewport_texture_id: Option<egui::TextureId>,
        state: &mut AppState,
        scene: &mut Scene,
        camera: &CameraManager,
        gizmo: &mut Gizmo,
    ) -> bool {
        let mut transform_changed = false;

        CentralPanel::default().show(ui, |ui| {
            title_panel(ui, "Viewport");

            if let Some(texture_id) = viewport_texture_id {
                let size = ui.available_size();
                let response = ui.add(
                    egui::Image::new(egui::load::SizedTexture::new(texture_id, size))
                        .sense(egui::Sense::click()),
                );

                let ppp = ui.ctx().pixels_per_point();
                let rect = response.rect;

                state.viewport.width = (size.x * ppp).round() as u32;
                state.viewport.height = (size.x * ppp).round() as u32;
                state.viewport.hovered = response.hovered();
                state.viewport.clicked = response.clicked();

                let r = response.rect;
                state.viewport.position = glm::Vec2::new(r.min.x * ppp, r.min.y * ppp);
                let r = response.rect;
                state.viewport.rect = [r.min.x, r.min.y, r.max.x, r.max.y];

                transform_changed = draw_gizmo(ui, gizmo, scene, state, camera, rect);
            }
        });

        transform_changed
    }
}
