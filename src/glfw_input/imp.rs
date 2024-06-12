use glfw::{Action, Key, MouseButton};
use crate::glfw_input::glfw_mouse_input::GlfwMouseInput;
use crate::glfw_input::GlfwInput;

impl Default for GlfwInput {
    fn default() -> Self {
        Self {
            mouse: Default::default(),
            keyboard: Default::default(),
        }
    }
}

impl GlfwInput {
    pub fn mouse(&self) -> &GlfwMouseInput {
        &self.mouse
    }

    pub fn update_mouse_position(&mut self, x: f64, y: f64, window_height: f64) {
        self.mouse.update_mouse_position(x, y, window_height);
    }

    pub fn process_mouse_button(&mut self, button: MouseButton, action: Action) -> bool {
        self.mouse.process_mouse_button(button, action)
    }
    pub fn process_key(&mut self, key: Key, action: Action) -> bool {
        self.keyboard.process_key(key, action)
    }

    pub fn is_mouse_button_held(&self, button: u32) -> Option<bool> {
        self.mouse.is_mouse_button_held(button)
    }

    pub fn is_mouse_button_released(&self, button: u32) -> Option<bool> {
        self.mouse.is_mouse_button_released(button)
    }

    pub fn is_mouse_button_pressed(&self, button: u32) -> Option<bool> {
        self.mouse.is_mouse_button_pressed(button)
    }

    pub fn is_key_held(&self, button: u32) -> Option<bool> {
        self.keyboard.is_key_held(button)
    }

    pub fn is_key_released(&self, button: u32) -> Option<bool> {
        self.keyboard.is_key_released(button)
    }

    pub fn is_key_pressed(&self, button: u32) -> Option<bool> {
        self.keyboard.is_key_pressed(button)
    }

    pub fn update(&mut self) {
        self.mouse.update();
        self.keyboard.update();
    }
}