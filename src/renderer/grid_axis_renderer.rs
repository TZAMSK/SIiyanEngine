use wgpu::{util::DeviceExt, BufferAddress, VertexBufferLayout, VertexStepMode};

const VERTEX_ATTRIBS: [wgpu::VertexAttribute; 2] =
    wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

pub struct GridAxisRenderer {
    pub vertex_buffer: wgpu::Buffer,
    pub vertex_count: u32,
}

impl GridAxisRenderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let vertices = Self::build_vertices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Grid vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            vertex_buffer,
            vertex_count: (vertices.len() / 6) as u32,
        }
    }

    pub fn build_vertices() -> Vec<f32> {
        let mut vertices = Vec::new();
        let half: i32 = 60;
        let h = half as f32;

        let mut line = |a: [f32; 3], b: [f32; 3], c: [f32; 3]| {
            vertices.extend_from_slice(&a);
            vertices.extend_from_slice(&c);
            vertices.extend_from_slice(&b);
            vertices.extend_from_slice(&c);
        };

        for i in -half..=half {
            let c = i as f32;
            let g = if i % 10 == 0 { 0.55 } else { 0.42 };
            let col = [g, g, g];

            if i == 0 {
                line([-h, 0.0, 0.0], [h, 0.0, 0.0], [1.0, 0.0, 0.0]);
                line([0.0, 0.0, -h], [0.0, 0.0, h], [0.0, 0.0, 1.0]);
            } else {
                line([-h, 0.0, c], [h, 0.0, c], col);
                line([c, 0.0, -h], [c, 0.0, h], col);
            }
        }

        line([0.0, 0.0, 0.0], [0.0, h, 0.0], [0.0, 1.0, 0.0]);

        vertices
    }

    pub fn vertex_layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: (6 * std::mem::size_of::<f32>()) as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &VERTEX_ATTRIBS,
        }
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }
}
