use crate::scene::shape::ShapeBehavior;
use crate::scene::shape::{Shape, ShapeType};

pub struct Triangle {
    pub shape: Shape,
}

impl ShapeBehavior for Triangle {
    fn shape_type(&self) -> ShapeType {
        ShapeType::Triangle
    }

    fn rebuild_mesh(&mut self) {
        let vertices = &mut self.shape.mesh.vertices;

        vertices.clear();

        vertices.push(glm::Vec3::new(0.0, 1.0, 0.0));
        vertices.push(glm::Vec3::new(-1.0, -1.0, 0.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, 0.0));
    }
}
