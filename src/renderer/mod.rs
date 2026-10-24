mod pipeline;

use std::sync::Arc;

use wgpu::{
    hal::SurfaceError, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, LoadOp,
    Operations, PowerPreference, PresentMode, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RequestAdapterOptions, StoreOp, Surface,
    SurfaceColorSpace, SurfaceConfiguration, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::window::Window;

use crate::{
    app::app_state::AppState,
    renderer::pipeline::RenderPipelineBuilder,
    scene::shape::{Mesh, Shape},
    ui::UIManager,
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

    pub fn surface_format(&self) -> TextureFormat {
        self.config.format
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(
        &mut self,
        ui: &mut UIManager,
        state: &mut AppState,
        window: &Window,
        shapes: &Vec<Shape>,
    ) -> Result<(), SurfaceError> {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err(SurfaceError::Lost);
            }
        };

        let view = output
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // First: Render 3D content with Clear to set the background
        // This creates the base layer that everything else renders on top of
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("3D Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        // Clear sets the background color for the entire surface
                        load: LoadOp::Clear(Color {
                            r: 0.2,
                            g: 0.2,
                            b: 0.25,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.pipeline);

            for shape in shapes {
                if let (Some(buf), num_vertices) =
                    (&shape.mesh.vertex_buffer, &shape.mesh.vertex_count())
                {
                    render_pass.set_vertex_buffer(0, buf.slice(..));
                    render_pass.draw(0..*num_vertices as u32, 0..1);
                }
            }
        }

        // Second: Render UI on top with Load to preserve the 3D content underneath
        // The transparent CentralPanel in ViewportPanel allows the 2D triangle to show through
        ui.render(
            &self.device,
            &self.queue,
            &mut encoder,
            window,
            &view,
            state,
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}
