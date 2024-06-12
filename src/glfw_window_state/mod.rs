use glfw::{Glfw, Window};
use crate::glfw_clock::GlfwClock;

use crate::glfw_input::GlfwInput;

pub mod imp;

pub struct GlfwWindowState<'a> {
    pub(crate) glfw: &'a mut Glfw,
    pub(crate) window: &'a mut Window,
    pub(crate) input: &'a mut GlfwInput,
    pub(crate) clock: &'a GlfwClock,
    pub(crate) window_position: [i32; 2]
}