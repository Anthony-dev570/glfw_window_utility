use glfw::{Action, MouseButton};
use crate::glfw_input::glfw_mouse_input::GlfwMouseInput;

impl Default for GlfwMouseInput {
    fn default() -> Self {
        Self {
            mouse_position: [0_f64; 2],
            mouse_delta: [0_f64; 2],
            relative_mouse_position: [0_f64; 2],
            relative_mouse_position_delta: [0_f64; 2],
            mouse_button_pressed: [false; 128],
            mouse_button_released: [false; 128],
            mouse_button_held: [false; 128],
        }
    }
}

impl GlfwMouseInput {

    pub fn update_mouse_position(&mut self, x: f64, y: f64, window_height: f64) {
        {
            let pos = [x, y];
            self.mouse_delta = [
                x - self.mouse_position[0],
                y - self.mouse_position[1],
            ];
            self.mouse_position = pos;
        }//mouse position & mouse delta.
        {
            let y = window_height - y;
            let pos = [x, y];
            self.relative_mouse_position_delta = [
                x - self.relative_mouse_position[0],
                y - self.relative_mouse_position[1]
            ];
            self.relative_mouse_position = pos;
        }//mouse relative position & relative mouse delta
    }

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
    pub(crate) fn mouse_button_pressed(&self) -> [bool; 128] {
        self.mouse_button_pressed
    }
    pub(crate) fn mouse_button_released(&self) -> [bool; 128] {
        self.mouse_button_released
    }
    pub(crate) fn mouse_button_held(&self) -> [bool; 128] {
        self.mouse_button_held
    }
}