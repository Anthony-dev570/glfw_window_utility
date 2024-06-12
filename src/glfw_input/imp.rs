use glfw::{Action, Key, MouseButton};
use crate::glfw_input::GlfwInput;

impl GlfwInput {
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