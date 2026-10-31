use glam::{DMat4, EulerRot};
use transform_gizmo_egui::{
    math::{DQuat, DVec3, Transform},
    Gizmo, GizmoConfig, GizmoExt, GizmoMode, GizmoOrientation,
};

use crate::{
    app::app_state::AppState,
    scene::{camera::CameraManager, Scene},
};

pub fn draw_gizmo(
    ui: &mut egui::Ui,
    gizmo: &mut Gizmo,
    scene: &mut Scene,
    state: &AppState,
    camera: &CameraManager,
    rect: egui::Rect,
) -> bool {
    let Some(selection) = &state.selection else {
        return false;
    };

    let id = selection.picker.selected_shape_id;

    if id == 0 {
        return false;
    }

    let Some(shape) = scene.find_shape_by_id_mut(id as u64) else {
        return false;
    };

    let position = shape.transform.position;
    let rotation = shape.transform.rotation;
    let scale = shape.transform.scale;

    let translation = DVec3::new(position.x as f64, position.y as f64, position.z as f64);
    let rotation = DQuat::from_euler(
        EulerRot::XYZ,
        rotation.x as f64,
        rotation.y as f64,
        rotation.z as f64,
    );
    let scale = DVec3::new(scale.x as f64, scale.y as f64, scale.z as f64);

    let transform = Transform::from_scale_rotation_translation(scale, rotation, translation);

    let view_matrix = to_dmat4(camera.camera.view());
    let projection_matrix = to_dmat4(camera.camera.projection());

    gizmo.update_config(GizmoConfig {
        view_matrix: view_matrix.into(),
        projection_matrix: projection_matrix.into(),
        viewport: rect,
        modes: GizmoMode::all(),
        orientation: GizmoOrientation::Local,
        ..Default::default()
    });

    if let Some((_result, transforms)) = gizmo.interact(ui, &[transform]) {
        if let Some(updated) = transforms.first() {
            let translation = updated.translation;
            let scale = updated.scale;

            let (rx, ry, rz) = DQuat::from_xyzw(
                updated.rotation.v.x,
                updated.rotation.v.y,
                updated.rotation.v.z,
                updated.rotation.s,
            )
            .to_euler(EulerRot::XYZ);

            shape.set_position(glm::Vec3::new(
                translation.x as f32,
                translation.y as f32,
                translation.z as f32,
            ));

            shape.set_rotation(glm::Vec3::new(rx as f32, ry as f32, rz as f32));

            shape.set_scale(glm::Vec3::new(
                scale.x as f32,
                scale.y as f32,
                scale.z as f32,
            ));

            return true;
        }
    }

    false
}

fn to_dmat4(matrix: glm::Mat4) -> DMat4 {
    let values: [f64; 16] = std::array::from_fn(|i| matrix.as_slice()[i] as f64);
    DMat4::from_cols_array(&values)
}
