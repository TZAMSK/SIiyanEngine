use crate::selection::picker::Picker;

pub mod picker;

#[derive(Debug, PartialEq)]
pub enum PlacementMode {
    None,
    Triangle,
    Rectangle,
    Circle,
    Cube,
    Sphere,
}

#[derive(Debug)]
pub struct SelectionState {
    pub placement_mode: PlacementMode,
    show_selection_window: bool,
    pub picker: Picker,
}

impl SelectionState {
    pub fn new(device: &wgpu::Device) -> Self {
        let picker = Picker::new(device);

        Self {
            placement_mode: PlacementMode::None,
            show_selection_window: false,
            picker,
        }
    }

    pub fn disarm_placement(&mut self) {
        self.placement_mode = PlacementMode::None;
        self.show_selection_window = false;
    }

    pub fn arm_triangle_placement(&mut self) {
        self.placement_mode = PlacementMode::Triangle;
        self.show_selection_window = false;
    }

    pub fn arm_rectangle_placement(&mut self) {
        self.placement_mode = PlacementMode::Rectangle;
        self.show_selection_window = false;
    }

    pub fn arm_circle_placement(&mut self) {
        self.placement_mode = PlacementMode::Circle;
        self.show_selection_window = false;
    }

    pub fn arm_cube_placement(&mut self) {
        self.placement_mode = PlacementMode::Cube;
        self.show_selection_window = false;
    }

    pub fn arm_sphere_placement(&mut self) {
        self.placement_mode = PlacementMode::Sphere;
        self.show_selection_window = false;
    }
}
