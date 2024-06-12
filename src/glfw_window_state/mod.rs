use glfw::{Glfw, Window};

use crate::glfw_input::GlfwInput;

pub struct GlfwWindowState<'a> {
    pub(crate) glfw: &'a mut Glfw,
    pub(crate) window: &'a mut Window,
    pub(crate) input: &'a mut GlfwInput
}