use bytemuck::offset_of;
use wgpu::{util::DeviceExt, wgc::device::queue};

mod view;

pub struct CameraManager {
    pub camera: Camera,
    pub camera_uniform: CameraUniform,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
}

pub struct Camera {
    position: glm::Vec3,
    target: glm::Vec3,
    up: glm::Vec3,
    aspect: f32,
    fov: f32,
    znear: f32,
    zfar: f32,
    ortho_scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraManager {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let camera = Camera {
            position: glm::Vec3::new(5.0, 8.0, -4.0),
            target: glm::Vec3::new(0.0, 0.0, 0.0),
            up: glm::Vec3::new(0.0, 1.0, 0.0),
            aspect: width as f32 / height as f32,
            fov: 45.0_f32.to_radians(),
            znear: 0.1,
            zfar: 1000.0,
            ortho_scale: 12.0,
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        Self {
            camera,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.camera_uniform.update_view_proj(&self.camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }
}

impl Camera {
    fn build_view_projection_matrix(&self) -> glm::Mat4 {
        let view = glm::look_at(&self.position, &self.target, &self.up);
        let proj = glm::perspective(self.aspect, self.fov, self.znear, self.zfar);

        opengl_to_wgpu_matrix() * proj * view
    }

    pub fn position(&self) -> glm::Vec3 {
        self.position
    }

    pub fn target(&self) -> glm::Vec3 {
        self.target
    }

    pub fn zoom_camera(&mut self, scroll: f32) {
        //let orthoZoom = self.ortho_scale - scroll_offset;
        let offset = self.position - self.target;
        let dist = (glm::length(&offset) * (1.0 - scroll * 0.1)).clamp(5.0, 500.0);
        self.position = self.target + glm::normalize(&offset) * dist;
    }

    pub fn set_aspect(&mut self, aspect: f32) {
        self.aspect = aspect
    }
}

impl CameraUniform {
    fn new() -> Self {
        Self {
            view_proj: glm::Mat4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into()
    }
}

#[rustfmt::skip]
pub fn opengl_to_wgpu_matrix() -> glm::Mat4{ 
    glm::Mat4::from_columns(&[
        glm::vec4(1.0, 0.0, 0.0, 0.0),
        glm::vec4(0.0, 1.0, 0.0, 0.0),
        glm::vec4(0.0, 0.0, 0.5, 0.0),
        glm::vec4(0.0, 0.0, 0.5, 1.0),
    ])
}
