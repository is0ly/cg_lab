use std::collections::HashSet;

use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

#[derive(Debug, Default)]
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
}

impl Input {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        let WindowEvent::KeyboardInput { event, .. } = event else {
            return;
        };

        let PhysicalKey::Code(key_code) = event.physical_key else {
            return;
        };

        match event.state {
            ElementState::Pressed => {
                if !event.repeat && self.keys_down.insert(key_code) {
                    self.keys_pressed.insert(key_code);
                }
            }
            ElementState::Released => {
                self.keys_down.remove(&key_code);
            }
        }
    }

    #[must_use]
    pub fn key_just_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_pressed.contains(&key_code)
    }

    #[must_use]
    pub fn key_down(&self, key_code: KeyCode) -> bool {
        self.keys_down.contains(&key_code)
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
    }
}
