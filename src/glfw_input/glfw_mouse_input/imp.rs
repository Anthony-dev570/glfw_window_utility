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
}