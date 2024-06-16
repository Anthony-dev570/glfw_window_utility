use glfw::{Glfw, Window};
use crate::glfw_clock::GlfwClock;
use crate::glfw_input::GlfwInput;
use crate::glfw_window_state::GlfwWindowState;

impl <'a> GlfwWindowState<'a> {
    pub fn force_next_render(&mut self, render: bool) {
        self.force_render = render;
    }

    pub fn glfw(&mut self) -> &mut &'a mut Glfw {
        &mut self.glfw
    }
    pub fn window(&mut self) -> &mut &'a mut Window {
        &mut self.window
    }
    pub fn input(&mut self) -> &mut &'a mut GlfwInput {
        &mut self.input
    }
    pub fn clock(&mut self) -> &mut &'a GlfwClock {
        &mut self.clock
    }
    pub fn window_position(&self) -> [i32; 2] {
        self.window_position
    }
    pub fn force_render(&self) -> bool {
        self.force_render
    }
    pub fn window_size(&self) -> [u32; 2] {
        self.window_size
    }
}