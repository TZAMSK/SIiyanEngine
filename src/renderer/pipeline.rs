use wgpu::{
    BlendState, ColorTargetState, ColorWrites, CompareFunction, DepthBiasState, Device, Face,
    FragmentState, FrontFace, MultisampleState, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, StencilState, TextureFormat,
    VertexBufferLayout, VertexState,
};

pub struct RenderPipelineBuilder<'a> {
    device: &'a Device,
    format: TextureFormat,
    shader: Option<String>,
    vertex_layout: Option<VertexBufferLayout<'static>>,
}

impl<'a> RenderPipelineBuilder<'a> {
    pub fn new(device: &'a Device, format: TextureFormat) -> Self {
        Self {
            device,
            format,
            shader: None,
            vertex_layout: None,
        }
    }

    pub fn with_shader(mut self, shader_source: &str) -> Self {
        self.shader = Some(shader_source.to_string());
        self
    }

    pub fn with_vertex_layout(mut self, layout: VertexBufferLayout<'static>) -> Self {
        self.vertex_layout = Some(layout);
        self
    }

    pub fn build(self) -> Result<RenderPipeline, PipelineError> {
        let shader = self.shader.ok_or(PipelineError::MissingShader)?;
        let vertex_layout = self
            .vertex_layout
            .ok_or(PipelineError::MissingVertexLayout)?;

        let shader_module = self.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(shader.into()),
        });

        let pipeline_layout = self
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0,
            });

        Ok(self
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(vertex_layout)],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: Some("fs_main"),
                    targets: &[Some(ColorTargetState {
                        format: self.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    polygon_mode: PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: TextureFormat::Depth32Float,
                    depth_write_enabled: Some(true),
                    depth_compare: Some(CompareFunction::Less),
                    stencil: StencilState::default(),
                    bias: DepthBiasState::default(),
                }),
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None,
                cache: None,
            }))
    }
}

#[derive(Debug)]
pub enum PipelineError {
    MissingShader,
    MissingVertexLayout,
}
