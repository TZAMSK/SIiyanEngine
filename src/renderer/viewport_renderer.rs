use wgpu::{
    Device, Extent3d, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    TextureView, TextureViewDescriptor,
};

pub struct ViewportRenderer {
    pub texture: Texture,
    pub view: TextureView,
    pub depth_texture: Texture,
    pub depth_view: TextureView,
    pub width: u32,
    pub height: u32,
}

impl ViewportRenderer {
    pub fn new(device: &Device, format: TextureFormat, width: u32, height: u32) -> Self {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Viewport"),
            size: Extent3d {
                depth_or_array_layers: 1,
                width,
                height,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Viewport Depth"),
            size: Extent3d {
                depth_or_array_layers: 1,
                width,
                height,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });

        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        Self {
            texture,
            view,
            depth_texture,
            depth_view,
            width,
            height,
        }
    }
}
