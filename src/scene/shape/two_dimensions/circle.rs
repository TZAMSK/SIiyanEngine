use crate::scene::shape::round::RoundProperties;
use crate::scene::shape::ShapeBehavior;
use crate::scene::shape::{Shape, ShapeType};

pub struct Circle {
    pub shape: Shape,
    pub properties: RoundProperties,
}

impl Circle {
    pub fn new(name: &str, position: glm::Vec3, color: glm::Vec4) -> Self {
        let mut circle = Self {
            shape: Shape::new(name.to_string(), position, color),
            properties: RoundProperties {
                radius: 1.0,
                segments: 6,
            },
        };
        circle.rebuild_mesh();
        circle
    }
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
            vertices.push(glm::Vec3::new(angle1.cos() * radius, 0.0, angle1.sin()));
            vertices.push(glm::Vec3::new(angle2.cos() * radius, 0.0, angle2.sin()));
        }
    }
}
