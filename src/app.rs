use std::sync::Arc;

use winit::{application::ApplicationHandler, window::Window};

use crate::{
    renderer::Renderer,
    scene::{shape::three_dimensions::cube::Cube, Scene},
};

pub struct Application {
    scene: Option<Scene>,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
}

impl Application {
    pub async fn new() -> Self {
        let mut cube = Cube::new(
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
        }
    }

    pub async fn init(&mut self, window: Window) -> anyhow::Result<()> {
        let window = Arc::new(window);
        let scene = self.scene.as_mut().expect("Scene must be set before init");
        let renderer = Renderer::new(&window, &mut scene.shapes).await?;

        self.renderer = Some(renderer);
        self.window = Some(window);

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
        if let Some(window) = self.window.as_ref() {
            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                winit::event::WindowEvent::RedrawRequested => {
                    window.request_redraw();
                }
                _ => {}
            }
        }
    }
}
