use glfw::{Glfw, Window};
use crate::glfw_clock::GlfwClock;
use crate::glfw_input::GlfwInput;
use crate::glfw_window_state::GlfwWindowState;

impl <'a> GlfwWindowState<'a> {
    pub fn force_next_render(&mut self, render: bool) {
        self.force_render = render;
    }

    pub fn glfw(&self) -> &'a mut Glfw {
        self.glfw
    }
    pub fn window(&self) -> &'a mut Window {
        self.window
    }
    pub fn input(&self) -> &'a mut GlfwInput {
        self.input
    }
    pub fn clock(&self) -> &'a GlfwClock {
        self.clock
    }
    pub fn window_position(&self) -> [i32; 2] {
        self.window_position
    }
    pub fn force_render(&self) -> bool {
        self.force_render
    }
}