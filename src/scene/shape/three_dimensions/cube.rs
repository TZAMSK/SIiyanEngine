use crate::scene::shape::shape::ShapeBehavior;
use crate::scene::shape::shape::{Shape, ShapeType};

pub struct Cube {
    pub shape: Shape,
}

impl ShapeBehavior for Cube {
    fn shape_type(&self) -> ShapeType {
        ShapeType::Cube
    }

    fn rebuild_mesh(&mut self) {
        let vertices = &mut self.shape.mesh.vertices;

        vertices.clear();

        // -Z
        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, -1.0));

        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, -1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, -1.0));

        // +Z
        vertices.push(glm::Vec3::new(-1.0, -1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, 1.0));

        vertices.push(glm::Vec3::new(-1.0, -1.0, 1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));

        // -Y
        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(-1.0, -1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, 1.0));

        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, -1.0));

        // +Y
        vertices.push(glm::Vec3::new(-1.0, 1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, 1.0));

        vertices.push(glm::Vec3::new(-1.0, 1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));

        // -X
        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, -1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, 1.0));

        vertices.push(glm::Vec3::new(-1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(-1.0, 1.0, 1.0));
        vertices.push(glm::Vec3::new(-1.0, -1.0, 1.0));

        // +X
        vertices.push(glm::Vec3::new(1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, -1.0));

        vertices.push(glm::Vec3::new(1.0, -1.0, -1.0));
        vertices.push(glm::Vec3::new(1.0, -1.0, 1.0));
        vertices.push(glm::Vec3::new(1.0, 1.0, 1.0));
    }
}
