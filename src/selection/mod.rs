#[derive(Debug)]
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
    hovered_shape_id: u32,
    selected_shape_id: u32,
}

impl SelectionState {
    pub fn new() -> Self {
        Self {
            placement_mode: PlacementMode::None,
            show_selection_window: false,
            hovered_shape_id: 0,
            selected_shape_id: 0,
        }
    }

    pub fn set_selected_shape_id(&mut self, id: u32) {
        self.selected_shape_id = id
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
