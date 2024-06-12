use glfw::{Action, Key};
use crate::glfw_input::glfw_keyboard_input::GlfwKeyboardInput;

impl GlfwKeyboardInput {
    pub fn process_key(&mut self, key: Key, action: Action) -> bool {
        let index = key as u32 as usize;
        if index >= self.pressed.len() {
            return false;
        }
        match action {
            Action::Release => {
                self.released[index] = true;
                self.held[index] = false;
            }
            Action::Press => {
                self.pressed[index] = true;
                self.held[index] = true;
            }
            _ => {}
        }
        true
    }

    pub fn is_key_pressed(&self, button: u32) -> Option<bool> {
        if button as usize >= self.pressed.len() {
            return None;
        }
        Some(self.pressed[button as usize])
    }

    pub fn is_key_released(&self, button: u32) -> Option<bool> {
        if button as usize >= self.pressed.len() {
            return None;
        }
        Some(self.released[button as usize])
    }

    pub fn is_key_held(&self, button: u32) -> Option<bool> {
        if button as usize >= self.pressed.len() {
            return None;
        }
        Some(self.held[button as usize])
    }

    pub fn update(&mut self) {
        self.pressed = [false; 512];
        self.released = [false; 512];
    }
}