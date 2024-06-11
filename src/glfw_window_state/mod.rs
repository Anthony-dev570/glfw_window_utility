use glfw::{Glfw, Window};

pub struct GlfwWindowState<'a> {
    pub(crate) glfw: &'a mut Glfw,
    pub(crate) window: &'a mut Window
}