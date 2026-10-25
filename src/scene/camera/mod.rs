use std::sync::Arc;

use egui::Window;
use wgpu::util::DeviceExt;

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
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraManager {
    pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
        let camera = Camera {
            position: glm::Vec3::new(0.0, 1.0, 2.0),
            target: glm::Vec3::new(0.0, 0.0, 0.0),
            up: glm::Vec3::new(0.0, 1.0, 0.0),
            aspect: width as f32 / height as f32,
            fov: 45.0,
            znear: 1.0,
            zfar: 1000.0,
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
}

impl Camera {
    fn build_view_projection_matrix(&self) -> glm::Mat4 {
        let view = glm::look_at(&self.position, &self.target, &self.up);
        let proj = glm::perspective(self.fov, self.aspect, self.znear, self.zfar);

        return opengl_to_wgpu_matrix() * view * proj;
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
