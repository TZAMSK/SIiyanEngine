pub enum ShapeType {
    Triangle,
    Rectangle,
    Circle,
    Cube,
    Sphere,
}

pub trait ShapeBehavior {
    fn shape_type(&self) -> ShapeType;
    fn rebuild_mesh(&mut self);
}

#[derive(Default)]
pub struct Shape {
    pub id: u32,
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh,
    pub material: Material,
    pub physics_body: PhysicsBody,
    pub children: Vec<Shape>,
    pub parent: Option<u32>,
}

struct Transform {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: glm::Vec3::new(0.0, 0.0, 0.0),
            rotation: glm::Vec3::new(0.0, 0.0, 0.0),
            scale: glm::Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<glm::Vec3>,
    pub vertex_buffer: Option<wgpu::Buffer>,
}

impl Mesh {
    fn vertexCount(&self) -> usize {
        self.vertices.len()
    }

    fn uploadToGpu(&mut self, device: &wgpu::Device) {
        if self.vertices.is_empty() {
            self.vertex_buffer = None;
            return;
        }

        self.vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: (self.vertexCount() * std::mem::size_of::<glm::Vec3>()) as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: true,
        }));
    }
}

struct Material {
    pub color: glm::Vec4,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: glm::Vec4::new(0.608, 0.608, 0.608, 1.0),
        }
    }
}

#[derive(Default)]
struct PhysicsBody {
    pub volume: u32,
    pub following_parent: bool,
    pub has_physics_added: bool,
}

impl PhysicsBody {
    pub fn calculateVolume(&self, mesh: &Mesh) -> f32 {
        let count = mesh.vertexCount();
        let vertices = &mesh.vertices;
        let mut volume = 0.0;

        if count < 3 || count % 3 != 0 {
            return 0.0;
        }

        for triangle in vertices.chunks_exact(3) {
            let a = triangle[0];
            let b = triangle[1];
            let c = triangle[2];

            volume += glm::dot(a, glm::cross(b, c));
        }

        glm::abs(volume / 6.0)
    }
}

impl Shape {
    fn new(name: String, position: glm::Vec3, color: glm::Vec4) -> Self {
        Self {
            name,
            transform: Transform {
                position,
                ..Default::default()
            },
            material: Material { color },
            ..Default::default()
        }
    }

    fn isParent(&self) -> bool {
        self.parent.is_some()
    }

    fn setPosition(&mut self, position: glm::Vec3) {
        let delta = self.transform.position - position;
        self.transform.position = position;

        for child in self.children.iter_mut() {
            if child.physics_body.following_parent {
                child.setPosition(child.transform.position + delta)
            }
        }
    }

    fn setRotation(&mut self, rotation: glm::Vec3) {
        let delta = self.transform.rotation - rotation;
        self.transform.rotation = rotation;

        for child in self.children.iter_mut() {
            if child.physics_body.following_parent {
                child.setRotation(child.transform.rotation + delta)
            }
        }
    }

    fn setScale(&mut self, scale: glm::Vec3) {
        let delta = self.transform.scale - scale;
        self.transform.scale = scale;

        for child in self.children.iter_mut() {
            if child.physics_body.following_parent {
                child.setPosition(child.transform.scale * delta)
            }
        }
    }

    fn addChild(&mut self, mut child: Shape) {
        if child.id == self.id {
            return;
        }

        child.parent = Some(self.id);

        self.children.push(child);
    }

    fn removeChild(&mut self, child_id: u32) -> Option<Shape> {
        let index = self
            .children
            .iter()
            .position(|child| child.id == child_id)?;

        let mut child = self.children.remove(index);

        child.parent = None;

        Some(child)
    }
}
