mod console;
mod inspector;
mod scene;
mod toolbar;
mod viewport;

pub use console::ConsolePanel;
use egui::{vec2, Align2, Color32, FontId, Sense};
pub use inspector::InspectorPanel;
pub use scene::ScenePanel;
pub use toolbar::ToolbarPanel;
pub use viewport::ViewportPanel;

pub fn title_panel(ui: &mut egui::Ui, title: &str) {
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());

    ui.painter()
        .rect_filled(rect, 0.0, Color32::from_rgb(10, 10, 10));
    ui.painter().text(
        rect.left_center() + vec2(6.0, 0.0),
        Align2::LEFT_CENTER,
        title,
        FontId::proportional(12.0),
        Color32::WHITE,
    );
}
