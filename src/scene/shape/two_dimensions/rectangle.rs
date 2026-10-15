use crate::scene::shape::shape::ShapeBehavior;
use crate::scene::shape::shape::{Shape, ShapeType};

pub struct Rectangle {
    pub shape: Shape,
}

impl ShapeBehavior for Rectangle {
    fn shape_type(&self) -> ShapeType {
        ShapeType::Rectangle
    }

    fn rebuild_mesh(&mut self) {
        let vertices = &mut self.shape.mesh.vertices;

        vertices.clear();

        vertices.push(glm::Vec3::new(-2.0, -1.0, 0.0));
        vertices.push(glm::Vec3::new(2.0, -1.0, 0.0));
        vertices.push(glm::Vec3::new(2.0, 1.0, 0.0));

        vertices.push(glm::Vec3::new(-2.0, -1.0, 0.0));
        vertices.push(glm::Vec3::new(2.0, 1.0, 0.0));
        vertices.push(glm::Vec3::new(-2.0, 1.0, 0.0));
    }
}
