use std::collections::HashSet;

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::math::Vec2;

#[derive(Debug, Default)]
pub struct Input {
    keys_down: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,

    mouse_buttons_down: HashSet<MouseButton>,
    mouse_buttons_pressed: HashSet<MouseButton>,

    mouse_position_physical: Option<Vec2>,
    mouse_position_canvas: Option<Vec2>,
}

impl Input {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
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

            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position_physical =
                    Some(Vec2::new(position.x as f32, position.y as f32));
            }

            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    if self.mouse_buttons_down.insert(*button) {
                        self.mouse_buttons_pressed.insert(*button);
                    }
                }
                ElementState::Released => {
                    self.mouse_buttons_down.remove(button);
                }
            },

            _ => {}
        }
    }

    pub fn update_mouse_canvas_position(
        &mut self,
        window_size: PhysicalSize<u32>,
        canvas_size: Vec2,
    ) {
        let Some(mouse_position_physical) = self.mouse_position_physical else {
            self.mouse_position_canvas = None;
            return;
        };

        if window_size.width == 0 || window_size.height == 0 {
            self.mouse_position_canvas = None;
            return;
        }

        let x = mouse_position_physical.x / window_size.width as f32 * canvas_size.x;
        let y = mouse_position_physical.y / window_size.height as f32 * canvas_size.y;

        self.mouse_position_canvas = Some(Vec2::new(x, y));
    }

    #[must_use]
    pub fn key_just_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_pressed.contains(&key_code)
    }

    #[must_use]
    pub fn key_down(&self, key_code: KeyCode) -> bool {
        self.keys_down.contains(&key_code)
    }

    #[must_use]
    pub fn mouse_position(&self) -> Option<Vec2> {
        self.mouse_position_canvas
    }

    #[must_use]
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    #[must_use]
    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn end_frame(&mut self) {
        self.keys_pressed.clear();
        self.mouse_buttons_pressed.clear();
    }
}
