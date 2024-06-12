use glfw::{Action, MouseButton};
use crate::glfw_input::glfw_mouse_input::GlfwMouseInput;

impl GlfwMouseInput {
    pub fn process_mouse_button(&mut self, button: MouseButton, action: Action) -> bool {
        let index = button as u32 as usize;
        if index >= self.mouse_button_pressed.len() {
            return false;
        }
        match action {
            Action::Release => {
                self.mouse_button_released[index] = true;
                self.mouse_button_held[index] = false;
            }
            Action::Press => {
                self.mouse_button_pressed[index] = true;
                self.mouse_button_held[index] = true;
            }
            _ => {}
        }
        true
    }

    pub fn is_mouse_button_pressed(&self, button: u32) -> Option<bool> {
        if button as usize >= self.mouse_button_pressed.len() {
            return None;
        }
        Some(self.mouse_button_pressed[button as usize])
    }

    pub fn is_mouse_button_released(&self, button: u32) -> Option<bool> {
        if button as usize >= self.mouse_button_pressed.len() {
            return None;
        }
        Some(self.mouse_button_released[button as usize])
    }

    pub fn is_mouse_button_held(&self, button: u32) -> Option<bool> {
        if button as usize >= self.mouse_button_pressed.len() {
            return None;
        }
        Some(self.mouse_button_held[button as usize])
    }


    pub fn update(&mut self) {
        self.mouse_delta = [0_f64; 2];
        self.relative_mouse_position_delta = [0_f64; 2];
        self.mouse_button_released = [false; 128];
        self.mouse_button_pressed = [false; 128];
    }
}