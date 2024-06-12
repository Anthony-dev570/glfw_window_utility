use crate::glfw_input::glfw_keyboard_input::GlfwKeyboardInput;
use crate::glfw_input::glfw_mouse_input::GlfwMouseInput;

pub mod imp;
pub mod glfw_mouse_input;
pub mod glfw_keyboard_input;

#[derive(Debug, Clone)]
pub struct GlfwInput {
    mouse: GlfwMouseInput,
    keyboard: GlfwKeyboardInput
}