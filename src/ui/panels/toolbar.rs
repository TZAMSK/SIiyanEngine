use egui::{Id, Panel, Popup, PopupCloseBehavior, RectAlign};

use crate::{app::app_state::AppState, ui::dimensions};

pub struct ToolbarPanel;

impl ToolbarPanel {
    pub fn new() -> Self {
        Self
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        Panel::top("toolbar")
            .exact_size(dimensions::TOOLBAR_HEIGHT)
            .show(ui, |ui| {
                ui.horizontal_centered(|ui| {
                    let response = ui.button("Add Shape");

                    self.apply_options(Popup::menu(&response))
                        .show(|ui| self.menu(ui, state));

                    let _ = ui.button("Iso");
                    let _ = ui.button("Top XY");
                });
            });
    }

    fn apply_options<'a>(&self, popup: Popup<'a>) -> Popup<'a> {
        popup
            .align(RectAlign::default())
            .gap(4.0)
            .close_behavior(PopupCloseBehavior::CloseOnClick)
    }

    fn menu(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        ui.menu_button("2D", |ui| {
            self.nested_2d_menu(ui, state);
        });

        ui.menu_button("3D", |ui| {
            self.nested_3d_menu(ui, state);
        });
    }

    fn nested_2d_menu(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        if ui.button("Triangle").clicked() {
            state.selection.arm_triangle_placement();
            state
                .console
                .append("Triangle placement armed. Clicke inside the viewport");
        };

        if ui.button("Rectangle").clicked() {
            state.selection.arm_rectangle_placement();
            state
                .console
                .append("Rectangle placement armed. Clicke inside the viewport");
        }

        if ui.button("Circle").clicked() {
            state.selection.arm_circle_placement();
            state
                .console
                .append("Circle placement armed. Clicke inside the viewport");
        }
    }

    fn nested_3d_menu(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        if ui.button("Cube").clicked() {
            state.selection.arm_cube_placement();
            state
                .console
                .append("Cube placement armed. Clicke inside the viewport");
        }

        if ui.button("Sphere").clicked() {
            state.selection.arm_sphere_placement();
            state
                .console
                .append("Sphere placement armed. Clicke inside the viewport");
        }
    }
}
