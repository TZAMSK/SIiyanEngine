use std::collections::HashSet;

use winit::event::{MouseButton, MouseScrollDelta};
use winit::window::{Fullscreen, Window};
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Input {
    pressed: HashSet<KeyCode>,
    buttons: HashSet<MouseButton>,
    mouse_pos: glm::Vec2,
    mouse_delta: glm::Vec2,
    scroll: f32,
}

impl Input {
    pub fn mouse_button_down(&self, b: MouseButton) -> bool {
        self.buttons.contains(&b)
    }

    pub fn mouse_pos(&self) -> glm::Vec2 {
        self.mouse_pos
    }

    pub fn mouse_delta(&self) -> glm::Vec2 {
        self.mouse_delta
    }

    pub fn scroll(&self) -> f32 {
        self.scroll
    }

    pub fn end_frame(&mut self) {
        self.mouse_delta = glm::vec2(0.0, 0.0);
        self.scroll = 0.0
    }

    pub fn handle_mouse(&mut self, _window: &Window, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let new = glm::vec2(position.x as f32, position.y as f32);
                self.mouse_delta += new - self.mouse_pos;
                self.mouse_pos = new;
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    self.buttons.insert(*button);
                }
                ElementState::Released => {
                    self.buttons.remove(button);
                }
            },
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll += match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32 / 50.0,
                };
            }
            _ => {}
        }
        false
    }

    pub fn handle_keyboard(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let WindowEvent::KeyboardInput { event, .. } = event else {
            return false;
        };
        let PhysicalKey::Code(code) = event.physical_key else {
            return false;
        };

        match event.state {
            ElementState::Pressed => {
                if !event.repeat {
                    match code {
                        KeyCode::F11 => {
                            let fs = if window.fullscreen().is_some() {
                                None
                            } else {
                                Some(Fullscreen::Borderless(None))
                            };
                            window.set_fullscreen(fs);
                        }
                        KeyCode::Escape => return true,
                        _ => {}
                    }
                }
                self.pressed.insert(code);
            }

            ElementState::Released => {
                self.pressed.remove(&code);
            }
        }
        false
    }

    pub fn mouseLocalTWorldSpace() {}
}
