use std::sync::atomic::{AtomicU64, Ordering};

use wgpu::{BufferAddress, VertexBufferLayout, VertexStepMode};

mod round;
pub mod three_dimensions;
pub mod two_dimensions;

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

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Default)]
pub struct Shape {
    pub id: u64,
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh,
    pub material: Material,
    pub physics_body: PhysicsBody,
    pub children: Vec<Shape>,
    pub parent: Option<u64>,
}

pub struct Transform {
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

const VERTEX_ATTRIBS: [wgpu::VertexAttribute; 3] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4, 2 => Uint32];

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<glm::Vec3>,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub vertex_data: Vec<u8>,
}

impl Mesh {
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertex_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: (8 * std::mem::size_of::<f32>()) as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBS,
        }
    }

    pub fn upload_to_gpu(
        &mut self,
        device: &wgpu::Device,
        position: &glm::Vec3,
        color: &glm::Vec4,
        id: u64,
    ) {
        if self.vertices.is_empty() {
            self.vertex_buffer = None;
            self.vertex_data.clear();
            return;
        }

        debug_assert!(id <= u32::MAX as u64);

        let mut raw: Vec<f32> = Vec::with_capacity(self.vertices.len() * 7);
        for v in &self.vertices {
            raw.extend_from_slice(&[
                v.x + position.x,
                v.y + position.y,
                v.z + position.z,
                color.x,
                color.y,
                color.z,
                color.w,
                f32::from_bits(id as u32),
            ]);
        }
        self.vertex_data = bytemuck::cast_slice(&raw).to_vec();

        self.vertex_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex buffer"),
            size: self.vertex_data.len() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }));
    }
}

pub struct Material {
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
        let count = mesh.vertex_count();
        let vertices = &mesh.vertices;
        let mut volume = 0.0;

        if count < 3 || count % 3 != 0 {
            return 0.0;
        }

        for triangle in vertices.chunks_exact(3) {
            let a = triangle[0];
            let b = triangle[1];
            let c = triangle[2];

            volume += glm::dot(&a, &glm::cross(&b, &c));
        }

        (volume / 6.0).abs()
    }
}

impl Shape {
    fn new(name: String, position: glm::Vec3, color: glm::Vec4) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
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
        let delta = position - self.transform.position;
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
        let ratio = scale.component_div(&self.transform.scale);
        self.transform.scale = scale;

        for child in self.children.iter_mut() {
            if child.physics_body.following_parent {
                child.setPosition(child.transform.scale.component_mul(&ratio))
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

    fn removeChild(&mut self, child_id: u64) -> Option<Shape> {
        let index = self
            .children
            .iter()
            .position(|child| child.id == child_id)?;

        let mut child = self.children.remove(index);

        child.parent = None;

        Some(child)
    }
}
