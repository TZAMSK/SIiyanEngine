mod pipeline;

use std::sync::Arc;

use wgpu::{
    Device, DeviceDescriptor, Instance, PowerPreference, PresentMode, Queue, RenderPipeline,
    RequestAdapterOptions, Surface, SurfaceColorSpace, SurfaceConfiguration, TextureUsages,
};
use winit::window::Window;

use crate::{
    renderer::pipeline::RenderPipelineBuilder,
    scene::shape::{Mesh, Shape},
};

pub struct Renderer {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pipeline: RenderPipeline,
}

fn upload_shapes(shapes: &mut [Shape], device: &Device) {
    for shape in shapes {
        shape.mesh.upload_to_gpu(device, &shape.material.color);
        upload_shapes(&mut shape.children, device);
    }
}

fn write_shapes(shapes: &[Shape], queue: &Queue) {
    for shape in shapes {
        if let Some(buf) = &shape.mesh.vertex_buffer {
            queue.write_buffer(buf, 0, &shape.mesh.vertex_data);
        }
        write_shapes(&shape.children, queue);
    }
}

impl Renderer {
    pub async fn new(window: &Arc<Window>, shapes: &mut [Shape]) -> anyhow::Result<Self> {
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .expect("Failed to find adapter");

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let size = window.inner_size();
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: SurfaceColorSpace::Auto,
        };

        surface.configure(&device, &config);

        upload_shapes(shapes, &device);
        write_shapes(shapes, &queue);

        let pipeline = RenderPipelineBuilder::new(&device, surface_format)
            .with_shader(include_str!("shader.wgsl"))
            .with_vertex_layout(Mesh::vertex_layout())
            .build()
            .expect("Couldn't create pipeline");

        Ok(Self {
            surface,
            device,
            queue,
            config,
            pipeline,
        })
    }
}
