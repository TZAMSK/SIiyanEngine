use egui::Stroke;
use egui_winit::egui::{CentralPanel, Color32, Rect};

pub struct ViewportPanel {
    pub viewport_rect: Option<Rect>,
}

impl ViewportPanel {
    pub fn new() -> Self {
        Self {
            viewport_rect: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        let panel_frame = egui::Frame::new()
            .fill(Color32::GRAY)
            .corner_radius(10)
            .stroke(Stroke {
                width: 1.0,
                color: Color32::DARK_RED,
            })
            .outer_margin(1);

        CentralPanel::default()
            .frame(panel_frame.fill(Color32::TRANSPARENT))
            .show(ui, |ui| {
                let rect = ui.max_rect();

                self.viewport_rect = Some(rect);
            });
    }

    pub fn get_viewport_rect(&self) -> Option<Rect> {
        self.viewport_rect
    }
}
