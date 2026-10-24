pub mod app_state;

use std::sync::Arc;

use wgpu::hal::SurfaceError;
use winit::{application::ApplicationHandler, window::Window};

use crate::{
    app::app_state::AppState,
    renderer::Renderer,
    scene::{shape::three_dimensions::cube::Cube, Scene},
    ui::UIManager,
};

pub struct Application {
    scene: Option<Scene>,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    state: AppState,
    pub ui: Option<UIManager>,
}

impl Application {
    pub async fn new() -> Self {
        let cube = Cube::new(
            "Cube",
            glm::Vec3::new(0.0, 0.0, 0.0),
            glm::Vec4::new(1.0, 1.0, 1.0, 1.0),
        );

        let scene = Scene {
            shapes: vec![cube.shape],
        };
        Self {
            scene: Some(scene),
            window: None,
            renderer: None,
            state: AppState::default(),
            ui: None,
        }
    }

    pub async fn init(&mut self, window: Window) -> anyhow::Result<()> {
        let window = Arc::new(window);
        let scene = self.scene.as_mut().expect("Scene must be set before init");
        let renderer = Renderer::new(&window, &mut scene.shapes).await?;
        let ui = UIManager::new(&window, &renderer.device, renderer.surface_format());

        self.renderer = Some(renderer);
        self.window = Some(window);
        self.ui = Some(ui);

        Ok(())
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        pollster::block_on(self.init(window)).unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let (Some(window), Some(renderer), Some(ui)) = (
            self.window.as_ref(),
            self.renderer.as_mut(),
            self.ui.as_mut(),
        ) {
            if ui.handle_event(window, &event) {
                return;
            }

            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                winit::event::WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    self.state.update();

                    match renderer.render(
                        ui,
                        &mut self.state,
                        window,
                        &self.scene.as_ref().unwrap().shapes,
                    ) {
                        Ok(_) => {}
                        Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                            let size = window.inner_size();
                            renderer.resize(size.width, size.height);
                        }
                        Err(e) => log::error!("Failed to render: {}", e),
                    }
                    window.request_redraw();
                }
                _ => {}
            }
        }
    }
}
