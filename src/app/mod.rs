pub mod app_state;
mod input;

use std::sync::Arc;

use input::Input;
use wgpu::hal::SurfaceError;
use winit::{application::ApplicationHandler, window::Window};

use crate::{
    app::app_state::AppState,
    renderer::Renderer,
    scene::{camera::CameraManager, shape::three_dimensions::cube::Cube, Scene},
    ui::UIManager,
};

pub struct Application {
    scene: Option<Scene>,
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    state: AppState,
    pub ui: Option<UIManager>,
    camera: Option<CameraManager>,
    input: Input,
}

impl Application {
    pub async fn new() -> Self {
        let cube = Cube::new(
            "Cube",
            glm::Vec3::new(10.0, 0.0, 0.0),
            glm::Vec4::new(0.50, 1.0, 1.0, 1.0),
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
            camera: None,
            input: Input::default(),
        }
    }

    pub async fn init(&mut self, window: Window) -> anyhow::Result<()> {
        let window = Arc::new(window);
        let scene = self.scene.as_mut().expect("Scene must be set before init");

        let mut renderer = Renderer::new(&window, &mut scene.shapes).await?;

        let camera = CameraManager::new(
            &renderer.device,
            window.inner_size().width,
            window.inner_size().height,
        );

        renderer.build_pipeline(&camera);

        let ui = UIManager::new(
            &window,
            &renderer.device,
            renderer.surface_format(),
            renderer.viewport_view(),
        );

        self.renderer = Some(renderer);
        self.window = Some(window);

        self.camera = Some(camera);

        self.ui = Some(ui);

        Ok(())
    }

    pub fn on_viewport_clicked(scene: &mut Scene, state: &mut AppState, world_point: &glm::Vec3) {
        scene.add_shape(&state.selection, world_point);
        state.log_shape_added(&world_point);
        state.selection.disarm_placement();
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attr = Window::default_attributes()
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        let window = event_loop.create_window(attr).unwrap();

        pollster::block_on(self.init(window)).unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let (Some(window), Some(renderer), Some(ui), Some(camera)) = (
            self.window.as_ref(),
            self.renderer.as_mut(),
            self.ui.as_mut(),
            self.camera.as_mut(),
        ) {
            let consumed = ui.handle_event(window, &event);

            if self.input.handle_keyboard(window, &event) {
                event_loop.exit();
                return;
            }

            self.input.handle_mouse(window, &event);

            if consumed {
                return;
            }

            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                winit::event::WindowEvent::Resized(size) => {
                    renderer.resize(size.width, size.height);
                }
                winit::event::WindowEvent::RedrawRequested => {
                    self.state.update();

                    let (w, h) = (self.state.viewport.width, self.state.viewport.height);
                    if w > 0 && h > 0 && (w, h) != renderer.viewport_size() {
                        renderer.resize_viewport(w, h);
                        camera.camera.set_aspect(w as f32 / h as f32);
                        ui.update_viewport_texture(&renderer.device, renderer.viewport_view());
                    }

                    if self.state.viewport.hovered {
                        if let Some(p) = input::mouse_local_to_world_space(
                            &self.state.viewport,
                            &camera.camera,
                            self.input.mouse_pos().x,
                            self.input.mouse_pos().y,
                        ) {
                            if let Some(scene) = self.scene.as_mut() {
                                if self.state.viewport.clicked {
                                    Self::on_viewport_clicked(scene, &mut self.state, &p);
                                }
                                renderer.sync_shapes(&mut scene.shapes);
                            }
                        };
                        camera.camera.zoom_camera(self.input.scroll());
                    }

                    camera.update(&renderer.queue);
                    self.input.end_frame();

                    self.state.camera.position = camera.camera.position().into();
                    self.state.camera.target = camera.camera.target().into();

                    match renderer.render(
                        ui,
                        &mut self.state,
                        window,
                        &self.scene.as_ref().unwrap().shapes,
                        camera,
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
