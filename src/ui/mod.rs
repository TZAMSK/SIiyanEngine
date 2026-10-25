mod dimensions;
mod panels;

use egui_wgpu::wgpu::{
    CommandEncoder, Device, LoadOp, Operations, Queue, RenderPass, RenderPassColorAttachment,
    RenderPassDescriptor, StoreOp, TextureFormat, TextureView,
};
use egui_wgpu::Renderer as EguiRenderer;
use egui_wgpu::RendererOptions;
use egui_winit::egui::{Context, PlatformOutput, RawInput};
use egui_winit::winit::window::Theme;
use egui_winit::winit::window::Window;
use egui_winit::State as EguiWinitState;

use crate::app::app_state::AppState;

pub struct UIManager {
    ctx: Context,
    platform: EguiWinitState,
    components: UIComponents,
    egui_renderer: EguiRenderer,
}

struct UIComponents {
    inspector: panels::InspectorPanel,
    scene: panels::ScenePanel,
    console: panels::ConsolePanel,
    menu_bar: panels::MenuPanel,
    toolbar: panels::ToolbarPanel,
}

impl UIManager {
    pub fn new(window: &Window, device: &Device, surface_format: TextureFormat) -> Self {
        let ctx = Context::default();
        let platform = EguiWinitState::new(
            ctx.clone(),
            egui_winit::egui::ViewportId::ROOT,
            window,
            None,
            Some(Theme::Dark),
            None,
        );

        let egui_renderer = EguiRenderer::new(
            device,
            surface_format,
            RendererOptions {
                ..Default::default()
            },
        );

        Self {
            ctx,
            platform,
            components: UIComponents {
                inspector: panels::InspectorPanel::new(),
                scene: panels::ScenePanel::new(),
                console: panels::ConsolePanel::new(),
                menu_bar: panels::MenuPanel::new(),
                toolbar: panels::ToolbarPanel::new(),
            },
            egui_renderer,
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::WindowEvent) -> bool {
        self.platform.on_window_event(window, event).consumed
    }

    pub fn render_ui(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        self.components.menu_bar.show(ui, state);
        self.components.toolbar.show(ui, state);
        self.components.console.show(ui, state);
        self.components.scene.show(ui, state);
        self.components.inspector.show(ui, state);
    }

    pub fn render(
        &mut self,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        window: &Window,
        texture_view: &TextureView,
        state: &mut AppState,
    ) {
        // Get raw input and run egui
        let input = self.take_input(window);
        let ctx = self.ctx().clone();
        let mut full_output = ctx.run_ui(input, |ui| {
            self.render_ui(ui, state);
        });

        // Handle platform output
        self.handle_platform_output(window, full_output.platform_output);

        // Tessellate paint jobs
        let paint_jobs = ctx.tessellate(full_output.shapes, 4.9);

        // Prepare screen descriptor
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [window.inner_size().width, window.inner_size().height],
            pixels_per_point: window.scale_factor() as f32,
        };

        for (id, image_delta) in full_output.textures_delta.set.drain() {
            match image_delta.first() {
                Some(x) => self.egui_renderer.update_texture(device, queue, id, x),
                None => println!("Image failed to changed"),
            }
        }

        self.egui_renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("egui render pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Load,
                    store: StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        let render_pass_static: &mut RenderPass<'static> =
            unsafe { std::mem::transmute(&mut render_pass) };

        // Render the UI
        self.egui_renderer
            .render(render_pass_static, &paint_jobs, &screen_descriptor);
    }

    pub fn handle_platform_output(&mut self, window: &Window, platform_output: PlatformOutput) {
        self.platform
            .handle_platform_output(window, platform_output);
    }

    pub fn take_input(&mut self, window: &Window) -> RawInput {
        self.platform.take_egui_input(window)
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }
}
