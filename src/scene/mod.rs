pub mod camera;
pub mod shape;

use crate::{
    scene::shape::{
        three_dimensions::{cube::Cube, sphere::Sphere},
        two_dimensions::{circle::Circle, rectangle::Rectangle, triangle::Triangle},
        Shape,
    },
    selection::{PlacementMode, SelectionState},
};

pub struct Scene {
    pub shapes: Vec<Shape>,
}

impl Scene {
    pub fn add_shape(&mut self, shape_selected: &SelectionState, position: &glm::Vec3) {
        match shape_selected.placement_mode {
            PlacementMode::Triangle => self.shapes.push(
                Triangle::new("Triangle", *position, glm::Vec4::new(0.5, 1.0, 1.0, 1.0)).shape,
            ),
            PlacementMode::Rectangle => self.shapes.push(
                Rectangle::new("Rectangle", *position, glm::Vec4::new(1.0, 0.5, 1.0, 1.0)).shape,
            ),
            PlacementMode::Circle => self
                .shapes
                .push(Circle::new("Circle", *position, glm::Vec4::new(1.0, 1.0, 0.5, 1.0)).shape),
            PlacementMode::Cube => self
                .shapes
                .push(Cube::new("Cube", *position, glm::Vec4::new(1.0, 0.5, 0.5, 1.0)).shape),
            PlacementMode::Sphere => self
                .shapes
                .push(Sphere::new("Sphere", *position, glm::Vec4::new(0.5, 0.5, 1.0, 1.0)).shape),
            _ => {}
        }
    }
}
