use crate::scene::shape::round::RoundProperties;
use crate::scene::shape::shape::ShapeBehavior;
use crate::scene::shape::shape::{Shape, ShapeType};

pub struct Sphere {
    pub shape: Shape,
    pub properties: RoundProperties,
}

impl ShapeBehavior for Sphere {
    fn shape_type(&self) -> ShapeType {
        ShapeType::Sphere
    }

    fn rebuild_mesh(&mut self) {
        let vertices = &mut self.shape.mesh.vertices;
        let radius = self.properties.radius;
        let segments = self.properties.segments;

        vertices.clear();
        vertices.reserve(segments * segments * 6);

        for y in 0..segments {
            for x in 0..segments {
                let vert_at = |xi: usize, yi: usize| -> glm::Vec3 {
                    let u = xi as f32 / segments as f32;
                    let v = yi as f32 / segments as f32;
                    let phi = v * std::f32::consts::PI;
                    let theta = u * 2.0 * std::f32::consts::PI;

                    glm::Vec3::new(
                        radius * glm::sin(phi) * glm::cos(theta),
                        radius * glm::cos(phi),
                        radius * glm::sin(phi) * glm::sin(theta),
                    )
                };

                let tl = vert_at(x, y);
                let tr = vert_at(x + 1, y);
                let bl = vert_at(x, y + 1);
                let br = vert_at(x + 1, y + 1);

                vertices.push(tl);
                vertices.push(bl);
                vertices.push(tr);

                vertices.push(tr);
                vertices.push(bl);
                vertices.push(br);
            }
        }
    }
}
