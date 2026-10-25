use std::collections::HashSet;

use winit::window::{Fullscreen, Window};
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Default)]
pub struct Input {
    pressed: HashSet<KeyCode>,
}

impl Input {
    pub fn handle_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
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
