use crate::scene::shape::round::RoundProperties;
use crate::scene::shape::ShapeBehavior;
use crate::scene::shape::{Shape, ShapeType};

pub struct Circle {
    pub shape: Shape,
    pub properties: RoundProperties,
}

impl ShapeBehavior for Circle {
    fn shape_type(&self) -> ShapeType {
        ShapeType::Circle
    }

    fn rebuild_mesh(&mut self) {
        let vertices = &mut self.shape.mesh.vertices;
        let radius = self.properties.radius;
        let segments = self.properties.segments;

        vertices.clear();
        vertices.reserve(segments * 3);

        let pi = std::f32::consts::PI;

        for i in 0..segments {
            let angle1 = (i as f32 / (segments) as f32) * 2.0 * pi;
            let angle2 = ((i + 1) as f32 / (segments) as f32) * 2.0 * pi;

            vertices.push(glm::Vec3::new(0.0, 0.0, 0.0));
            vertices.push(glm::Vec3::new(
                glm::cos(angle1) * radius,
                glm::sin(angle1),
                0.0,
            ));
            vertices.push(glm::Vec3::new(
                glm::cos(angle2) * radius,
                glm::sin(angle2),
                0.0,
            ));
        }
    }
}
