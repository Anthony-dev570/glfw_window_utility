use crate::glfw_input::glfw_mouse_input::GlfwMouseInput;

pub mod glfw_mouse_input;

pub struct GlfwInput {
    mouse: GlfwMouseInput
}